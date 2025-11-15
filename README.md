# Persona Drift POC  
### Rust Engine + Next.js Dashboard  
A complete end-to-end system for detecting persona drift using a Rust backend (Axum) and a Next.js 14 dashboard.

---

## 🚀 Project Summary

This project is a **full working POC** that:

- Collects login/device/browser events  
- Stores them persistently  
- Computes drift indicators  
- Generates fingerprint & stability scoring  
- Displays everything in a beautiful UI dashboard  

The system helps security teams detect unusual shifts in user behaviour.

---

# 🏗 Architecture

persona-drift-poc/
│
├── rust-engine/ → Rust Axum backend server
│ ├── src/main.rs
│ ├── store.json
│ └── Cargo.toml
│
└── drift-dashboard/ → Next.js frontend dashboard
├── app/
├── lib/api.ts
└── package.json

---

# ⚙️ Rust Backend (Axum)

### ✔ Event Model

```json
{
  "user_id": "alice",
  "kind": "login",
  "meta": {
    "browser": "chrome",
    "os": "windows",
    "ip": "12.34.56.78",
    "device": "laptop"
  }
}
Stored with timestamp and kept in store.json.

🧵 Endpoints
POST /ingest
Stores event, timestamps it, persists to disk.

GET /drift/:user_id
Computes persona drift using changes across:

IP class

Browser

OS

Device type

Response example:

json
Copy code
{
  "events": 10,
  "drift_score": 40,
  "reasons": ["Multiple IPs", "Multiple Browsers"]
}
GET /fingerprint/:user_id
Returns:

Stable fingerprint (blake3 hash)

Stability score

Latest event tokens

GET /timeline/:user_id
Returns all user events sorted by time.

GET /profiles
Returns count of events per user.

GET /health
Simple server check.

🔐 Persistence
The backend loads data from:

pgsql
Copy code
store.json
And saves automatically after every ingestion.

🔄 CORS
Configured using:

rust
Copy code
let cors = CorsLayer::permissive();
Compatible with Axum 0.7 + tower-http 0.5.

🎨 Next.js 14 Dashboard
The dashboard shows:

✔ User drift analysis
✔ Fingerprint stability
✔ Timeline of events
✔ Total events count
✔ Latest device/browser/ip/os
Everything comes from:

bash
Copy code
/drift/:userId
/fingerprint/:userId
/timeline/:userId
/profiles
📍 Important Frontend Files
lib/api.ts
Axios instance for backend communication.

app/user/[userId]/UserDetailClient.tsx
Main UI for visualizing:

Drift score

Fingerprint

Timeline

Stats

🧪 Testing Instructions
1. Start backend
arduino
Copy code
cd rust-engine
cargo run
2. Send sample event
nginx
Copy code
curl -X POST http://localhost:8080/ingest ^
-H "Content-Type: application/json" ^
-d "@event.json"
3. Start frontend
arduino
Copy code
cd drift-dashboard
npm install
npm run dev
Visit:

👉 http://localhost:3000/user/alice

🌱 Future Goals
Move from JSON → PostgreSQL / SQLite

Add advanced anomaly scoring

Add charts & visualizations

Add WebSocket real-time monitoring

Add role-based admin panel

Deploy Rust backend on Fly.io

Deploy Next.js dashboard on Vercel

Add mobile-friendly dashboard

Add heatmap of risk factors

🛠 Technology Stack
Backend (Rust)
Axum 0.7

Tokio

tower-http

Blake3

serde / serde_json

Frontend (Next.js)
Next.js 14 App Router

React Server Components + Client Components

Axios

TailwindCSS (optional)

🎯 What This Project Can Be Used For
Detecting account sharing

Fraud detection

Device fingerprinting

Security anomaly detection

Login pattern monitoring

Behavioural identity verification

Enterprise SIEM enrichment

📦 How Others Can Use This Repo
Clone the repo

Run the Rust engine

Run the Next.js dashboard

POST events

Immediately see user behaviour patterns visualized

Great as a base for:

SOC tools

SIEM systems

User monitoring solutions

Identity analytics

Research & security labs

🧭 Maintainer Notes
The project persists data locally for simplicity

Any backend restart will reload existing history

Frontend expects Rust engine running on localhost:8080

✅ End of Document
This .md includes everything needed for contributors, future upgrades, and developers exploring the project.
