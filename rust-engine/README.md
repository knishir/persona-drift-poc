# ⚙️ Persona Drift Engine (Rust) — Full Documentation

The **Persona Drift Engine** is a Rust-based behavioral analytics engine that detects unusual user behavior, device fingerprints, and drift over time. It exposes a clean HTTP API that powers the Next.js dashboard.

---

# 📌 1. What This Engine Does

The engine performs **four major tasks**:

### ✔ 1. Ingests behavioral events  
Each event contains:
- `user_id`
- `kind` (login, action, etc.)
- `meta` (browser, OS, device, IP)
- `timestamp`

### ✔ 2. Stores events in-memory and persists to disk (`store.json`)  
Data survives restarts.

### ✔ 3. Computes analytics  
These include:
- Drift score  
- Drift reasons  
- Fingerprint similarity  
- Stability scores  
- Timeline reconstruction  

### ✔ 4. Exposes a simple REST API for frontend consumption  
Easy for any system to integrate.

---

# 📌 2. API Endpoints

### **POST `/ingest`**
Accepts a new behavioral event.

### **GET `/profiles`**
Returns all known users + event count.

### **GET `/drift/:user_id`**
Computes drift score and reasons.

### **GET `/fingerprint/:user_id`**
Returns last computed fingerprint and its stability.

### **GET `/timeline/:user_id`**
Returns the user’s full event history sorted chronologically.

---

# 📌 3. Event Structure

```json
{
  "user_id": "alice",
  "kind": "login",
  "meta": {
    "browser": "chrome",
    "os": "windows",
    "dev_type": "laptop",
    "ip": "1.2.3.4"
  }
}
```

The server automatically adds a timestamp if not provided.

---

# 📌 4. How It Works Internally

### 🧠 1. Data is stored in a global in-memory HashMap  
```
HashMap<String, Vec<Event>>
```

### 💾 2. Data is saved to disk on every ingest  
`store.json` contains all events.

### 🧮 3. Drift Score Logic  
Drift is based on:

- IP changes  
- Browser changes  
- OS changes  
- Device type changes  
- Abrupt jumps in sequence  

Each contributes to a weighted drift score:
- Multiple IPs → +20  
- Multiple OS → +5  
- Multiple browsers → +5  
- Multiple device types → +5  
- Abrupt device switch → +10  

### 🔐 4. Fingerprint Stability  
Uses token intersection logic:
- High overlap = stable behavior  
- Large changes = unstable behavior  

### 📜 5. Timeline  
Sorted by timestamps so analysts can visually understand sequence.

---

# 📌 5. Technology Used

| Component | Technology |
|----------|------------|
| Runtime | Rust (Tokio async runtime) |
| Framework | Axum 0.7 |
| Storage | JSON flat-file persistence |
| CORS | tower-http |
| Hashing | Blake3 |
| Data | RwLock for concurrency |
| Timestamping | Chrono |

---

# 📌 6. How to Run

### 1️⃣ Install Rust:
```
rustup update
```

### 2️⃣ Build:
```
cargo build
```

### 3️⃣ Run:
```
cargo run
```

Server starts at:

```
http://localhost:8080
```

---

# 📌 7. Ingest Test

```
curl -X POST http://localhost:8080/ingest \
  -H "Content-Type: application/json" \
  -d @event.json
```

---

# 📌 8. Future Goals

### 🚀 1. Production database  
PostgreSQL or ClickHouse.

### 🚀 2. Distributed event stream  
Kafka / NATS.

### 🚀 3. ML-based drift scoring  
Use anomaly detection models.

### 🚀 4. Device fingerprint normalization  
More advanced tokenization.

### 🚀 5. Real-time processing  
WebSockets.

---

# 📌 9. What This Engine Enables

This backend can power:

- SOC dashboards  
- Fraud detection  
- Identity risk scoring  
- Login anomaly alerts  
- Zero-trust verification  

This engine is designed to be extendable and secure, with clean Rust code and reliable APIs.

---

# 🎉 Final Notes

This engine is stable, tested, and ready for further expansion.  
Perfect foundation for security, identity, and behavioral analytics systems.
