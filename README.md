# 🚀 Persona Drift — Full System Overview  
### *(Rust Backend + Next.js Frontend)*

This document provides a **complete technical overview** of the entire project, including:
- The Rust Behavioral Engine  
- The Next.js Analytics Dashboard  
- What the system does  
- How it works internally  
- What features it currently provides  
- Future goals  
- How anyone can use or extend it  

---

# 📌 1. What This Entire System Does

This project is a **lightweight behavioral intelligence platform** combining Rust + Next.js.

It provides:

### ✔ Behavioral Drift Detection  
Detects sudden changes in:
- IP  
- Browser  
- OS  
- Device type  

Output: **drift score + drift reasons**

---

### ✔ Device Fingerprinting  
Creates stable fingerprints using:
- OS  
- Browser  
- Device  
- IP  

Output: **current fingerprint + stability score**

---

### ✔ Timeline Reconstruction  
Builds a **chronological timeline** of events for every user.

---

### ✔ Frontend Security Dashboard  
Provides analysts with:
- User list  
- Event count  
- Drift score  
- Fingerprint stability  
- Full behavior timeline  

---

# 🧠 2. How the System Works (High Level Flow)

+----------------------------+
| Next.js Dashboard |
| (User analytics frontend) |
+-------------+--------------+
|
| Axios API Calls
▼
+----------------------------+
| Rust Behavior Engine |
| - Event ingestion |
| - Fingerprint creation |
| - Drift detection |
| - Timeline reconstruction |
+-------------+--------------+
|
| JSON read/write
▼
+----------------------------+
| store.json |
+----------------------------+


The **backend processes behavior**, while the **frontend visualizes it**.

---

# ⚙️ 3. Rust Backend (Behavior Engine)

The Rust engine is responsible for:

### ✔ Event Ingestion (`POST /ingest`)
Accepts JSON events containing:
- user_id  
- kind (login, action, etc.)  
- metadata  
- timestamp  

Automatically adds timestamp if missing.

---

### ✔ Persistence  
Everything is saved into:

store.json


Rust loads it on startup and writes back after every change.

---

### ✔ Drift Detection (`GET /drift/:id`)
Evaluates:
- Multiple different IPs  
- Changing OS  
- Browser switching  
- Changing device types  

Returns:
- drift score (0–100)  
- drift reasons (string list)  
- raw event metadata  

---

### ✔ Fingerprint Stability (`GET /fingerprint/:id`)
Tracks stability of:
- IP  
- OS  
- Browser  
- Device type  

Returns:
- fingerprint hash  
- stability score  
- stability history  
- tokenized metadata  

---

### ✔ Timeline (`GET /timeline/:id`)
Sorts events by timestamp.

---

### ✔ Profiles (`GET /profiles`)
Returns:
- all user IDs  
- count of events per user  

---

### ⭐ Technologies Used (Rust)

- **Axum 0.7** — web server  
- **Tokio** — async runtime  
- **Serde** — JSON parsing  
- **tower-http** — CORS  
- **RwLock + HashMap** — state store  
- **JSON file persistence**  

---

# 🌐 4. Next.js Frontend (User Dashboard)

The frontend handles:

### ✔ User List Page  
Fetches `/profiles` and displays:
- user ID  
- events count  
- link to detailed analytics  

---

### ✔ Detailed User Page (`/user/[id]`)
Fetches:
- `/drift/:id`
- `/fingerprint/:id`
- `/timeline/:id`

Displays:
- **drift score**  
- **drift reasons**  
- **stability score**  
- **fingerprint tokens**  
- **timeline in order**  

---

### ✔ Technology Used (Frontend)

- **Next.js 14 (App Router)**  
- **React Client Components**  
- **Axios for API**  
- **Dynamic routing**  
- **Tailwind-ready structure**  
- Clean security dashboard layout  

---

# 🔍 5. What This Proves / Demonstrates

This project proves that:

### ✔ Behavioral identity can be tracked without cookies  
### ✔ A user fingerprint can be built from metadata  
### ✔ Drift can be measured by comparing events  
### ✔ Behavior timelines help analysts identify anomalies  
### ✔ Rust is extremely fast at log ingestion & analysis  
### ✔ Next.js can provide a SOC-grade UI with minimal effort  

This forms the **core of a real behavioral security engine**.

---

# 🎯 6. Future Goals

These can be added later:

### Backend
- GeoIP scoring  
- ML anomaly detection  
- Real-time alerts  
- Redis/Postgres persistence  
- Risk scoring engine  

### Frontend
- Timeline chart UI  
- Drift trend graph  
- Fingerprint comparison UI  
- Admin roles & login  

---

# 📦 7. How Anyone Can Use This

### 🔧 Start Backend
cd rust-engine
cargo run



### 🖥 Start Frontend
cd drift-dashboard
npm install
npm run dev

makefile
Copy code

Visit:

http://localhost:3000



---

# 🏁 8. Final Summary

This repository includes:

### ✔ Rust backend  
- Drift detection  
- Fingerprinting  
- Timeline generation  
- JSON persistence  
- Clean REST API  

### ✔ Next.js frontend  
- SOC-style dashboard  
- User analytics  
- Timeline visualization  

This project acts as the **foundation for identity analytics, fraud detection, or a custom SIEM module**.

---

# 🎉 End of Overview
