use axum::http::Method;
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use blake3;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc};
use tokio::{fs, net::TcpListener, sync::RwLock};
use tower_http::cors::{Any, CorsLayer};

// ---------------------------------------------------------
// DATA STRUCTURES
// ---------------------------------------------------------
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Event {
    user_id: String,
    kind: String,
    meta: serde_json::Value,
    ts: Option<String>,
}

type Store = Arc<RwLock<HashMap<String, Vec<Event>>>>;

#[derive(Clone)]
struct AppState {
    store: Store,
}

// ---------------------------------------------------------
// LOAD STORE
// ---------------------------------------------------------
async fn load_store() -> HashMap<String, Vec<Event>> {
    if let Ok(data) = fs::read_to_string("store.json").await {
        if let Ok(parsed) = serde_json::from_str(&data) {
            return parsed;
        }
    }
    HashMap::new()
}

// ---------------------------------------------------------
// SAVE STORE
// ---------------------------------------------------------
async fn save_store(store: &HashMap<String, Vec<Event>>) {
    let _ = fs::write("store.json", serde_json::to_string_pretty(store).unwrap()).await;
}

// ---------------------------------------------------------
// INGEST
// ---------------------------------------------------------
async fn ingest(State(state): State<AppState>, Json(mut ev): Json<Event>) -> Json<Value> {
    if ev.ts.is_none() {
        ev.ts = Some(Utc::now().to_rfc3339());
    }

    let mut lock = state.store.write().await;
    lock.entry(ev.user_id.clone()).or_default().push(ev);
    save_store(&lock).await;

    Json(json!({
        "ok": true,
        "stored": true
    }))
}

// ---------------------------------------------------------
// PROFILES
// ---------------------------------------------------------
async fn profiles(State(state): State<AppState>) -> Json<Value> {
    let lock = state.store.read().await;
    let mut out = serde_json::Map::new();

    for (user, events) in lock.iter() {
        out.insert(user.clone(), json!(events.len()));
    }

    Json(json!({ "profiles": out }))
}

// ---------------------------------------------------------
// TIMELINE
// ---------------------------------------------------------
async fn timeline(State(state): State<AppState>, Path(user): Path<String>) -> Json<Value> {
    let lock = state.store.read().await;

    if let Some(list) = lock.get(&user) {
        return Json(json!({
            "ok": true,
            "events": list
        }));
    }

    Json(json!({
        "ok": false,
        "error": "user not found"
    }))
}

// ---------------------------------------------------------
// DRIFT SCORE (simple demo)
// ---------------------------------------------------------
async fn drift(State(state): State<AppState>, Path(user): Path<String>) -> Json<Value> {
    let lock = state.store.read().await;

    if !lock.contains_key(&user) {
        return Json(json!({ "ok": false, "error": "user not found" }));
    }

    let events = lock.get(&user).unwrap();

    let mut risk = "Low";
    let mut score = 0;

    if events.len() > 1 {
        let last = &events[events.len() - 1].meta;
        let prev = &events[events.len() - 2].meta;

        if last.get("ip") != prev.get("ip") {
            score += 20;
        }
        if last.get("browser") != prev.get("browser") {
            score += 20;
        }
        if last.get("os") != prev.get("os") {
            score += 20;
        }
        if last.get("dev_type") != prev.get("dev_type") {
            score += 20;
        }

        if score >= 60 {
            risk = "High";
        } else if score >= 30 {
            risk = "Medium";
        }
    }

    Json(json!({
        "ok": true,
        "risk": risk,
        "score": score,
        "reasons": [],
    }))
}

// ---------------------------------------------------------
// FINGERPRINT
// ---------------------------------------------------------
async fn fingerprint(State(state): State<AppState>, Path(user): Path<String>) -> Json<Value> {
    let lock = state.store.read().await;

    if !lock.contains_key(&user) {
        return Json(json!({
            "ok": false,
            "error": "user not found"
        }));
    }

    let events = lock.get(&user).unwrap();

    // Extract tokens from last event
    let mut tokens: Vec<String> = Vec::new();
    if let Some(last) = events.last() {
        if let Some(ip) = last.meta.get("ip").and_then(|v| v.as_str()) {
            tokens.push(format!("ip:{}", ip));
        }
        if let Some(browser) = last.meta.get("browser").and_then(|v| v.as_str()) {
            tokens.push(format!("browser:{}", browser));
        }
        if let Some(os) = last.meta.get("os").and_then(|v| v.as_str()) {
            tokens.push(format!("os:{}", os));
        }
        if let Some(dev) = last.meta.get("dev_type").and_then(|v| v.as_str()) {
            tokens.push(format!("dev:{}", dev));
        }
    }

    // Fingerprint hash
    let hash_input = format!("{:?}", tokens);
    let fingerprint = blake3::hash(hash_input.as_bytes()).to_hex().to_string();

    // Stability calculation
    let mut stability = 100;
    if events.len() > 1 {
        let mut changes = 0;

        for w in events.windows(2) {
            let a = &w[0].meta;
            let b = &w[1].meta;

            if a.get("ip") != b.get("ip") {
                changes += 1;
            }
            if a.get("browser") != b.get("browser") {
                changes += 1;
            }
            if a.get("os") != b.get("os") {
                changes += 1;
            }
            if a.get("dev_type") != b.get("dev_type") {
                changes += 1;
            }
        }

        stability = (100 - (changes * 20)).max(0);
    }

    // History
    let history: Vec<Value> = events.iter().map(|e| e.meta.clone()).collect();

    Json(json!({
        "ok": true,
        "user_id": user,
        "fingerprint": fingerprint,
        "tokens": tokens,
        "stability": stability,
        "history": history
    }))
}

// ---------------------------------------------------------
// FEATURES (placeholder)
// ---------------------------------------------------------
async fn features() -> Json<Value> {
    Json(json!({
        "ok": true,
        "features": []
    }))
}

// ---------------------------------------------------------
// HEALTH
// ---------------------------------------------------------
async fn health() -> &'static str {
    "ok"
}

// ---------------------------------------------------------
// MAIN
// ---------------------------------------------------------
#[tokio::main]
async fn main() {
    println!("Loading store…");
    let store = load_store().await;

    let state = AppState {
        store: Arc::new(RwLock::new(store)),
    };

    // Proper CORS for Axum 0.7
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods([Method::GET, Method::POST]);

    let app = Router::new()
        .route("/", get(|| async { "Engine Running" }))
        .route("/health", get(health))
        .route("/ingest", post(ingest))
        .route("/profiles", get(profiles))
        .route("/timeline/:user", get(timeline))
        .route("/drift/:user", get(drift))
        .route("/fingerprint/:user", get(fingerprint))
        .route("/features/:user", get(features))
        .with_state(state)
        .layer(cors);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("Server running on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}
