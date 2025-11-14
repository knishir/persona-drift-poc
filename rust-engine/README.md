1. RUST_BACKEND.md
(full file inside one code block — COPY AS IS)
# Persona Drift Detector – Rust Backend (Axum)

This backend is the core engine of the Persona Drift Detector system. It receives user behavioral events, stores them, analyzes drift, calculates identity fingerprints, and serves the dashboard API. The backend is written in Rust using Axum, Tokio, and Tower-HTTP.

---

## 🧠 What This Backend Does

### 1. Event Ingestion  
The `/ingest` endpoint accepts user events such as:
- login actions  
- device info  
- browser info  
- OS  
- IP address  
- timestamps  

These events are saved in memory and persisted to `store.json`.

### 2. Profile Summary  
The `/profiles` endpoint returns a map of all users and their total event count.

### 3. Drift Analysis (Future Expansion)  
Although disabled temporarily, the backend is intended to:
- detect identity drift  
- score behavior anomalies  
- highlight risk  
- compare event patterns across time  

### 4. Behavioral Fingerprinting (Future Expansion)  
The backend is structured to support:
- consistent fingerprint generation  
- stability scoring  
- tracking of changes to a user's behavior fingerprint  

### 5. Timeline API (Future Expansion)  
Intended to show:
- chronological event patterns  
- device switching  
- IP movement  
- browser switching  
- OS switching  

---

## 🔧 Tech Stack

### Runtime & Async:
- **Tokio** (async runtime)
- **Axum** (modern Rust web framework)
- **tower-http** (CORS)

### Serialization:
- **serde**  
- **serde_json**

### Data Handling:
- **Arc + RwLock** to safely share state across threads
- **Disk persistence** via `store.json`

---

## 📂 Folder Structure



rust-engine/
│── src/
│ └── main.rs
│── store.json (auto generated)
│── Cargo.toml


---

## 🚀 How It Works (Step-by-Step)

1. On startup:
   - server loads existing `store.json`  
   - initializes `Arc<RwLock<HashMap>>` state  
   - configures CORS  
   - starts listening on **localhost:8080**

2. When `/ingest` is called:
   - timestamp is added
   - event inserted under user key
   - store is persisted to disk
   - returns `{ ok: true, stored: true }`

3. When `/profiles` is called:
   - backend counts number of events per user
   - returns them to dashboard

4. When frontend requests `/user/{id}` (future):
   - backend will supply additional data:
     - fingerprint  
     - drift score  
     - timeline  

---

## 🧪 Testing the Backend (manual)

### Ingest an event:


curl -X POST http://localhost:8080/ingest
 ^
-H "Content-Type: application/json" ^
-d "{"user_id":"alice","kind":"login","meta":{"ip":"1.2.3.4","browser":"chrome","os":"windows","dev_type":"laptop"}}"


### Get profiles:


curl http://localhost:8080/profiles


### Health check:


curl http://localhost:8080/health


---

## 🛠 Usage for Developers

Developers can:
- send behavioral events to `/ingest`
- build any ML / anomaly detection layer on top of event history
- extend fingerprinting logic
- plug into SIEM, IAM, or SSO systems
- integrate into dashboards (Next.js)

---

## 📌 Future Goals

- Add risk scoring model
- Add time-series drift tracking
- Add fingerprint evolution analysis
- Add real-time streaming (NATS or Kafka)
- Add authentication
- Build proper SDK for sending events
- Add device reputation scoring
- Add anomaly heatmaps
- Add GraphView of identity drift
- Add MinIO or S3 storage for event logs

---

## ✔️ Summary

This backend is a clean, simple Axum-based Rust engine designed for behavioral identity tracking.  
It is modular, extendable, and built to serve as a real security-feature POC that can be expanded into enterprise-grade identity drift analytics.
