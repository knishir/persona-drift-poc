2. NEXTJS_DASHBOARD.md
(full file inside one code block — COPY AS IS)
# Persona Drift Detector – Next.js Dashboard (v16)

This dashboard visualizes user identity patterns, event histories, and drift data coming from the Rust backend. It is built using Next.js 16 + Axios + React Hooks.

---

## 🧠 What the Dashboard Does

### 1. Lists all users  
Fetches data from:


GET /profiles

Shows:
- user_id  
- total event count  
- link to user detail page  

### 2. Shows individual user details  
From:
- `/drift/{user}`
- `/fingerprint/{user}`
- `/timeline/{user}` (future)

Currently displays:
- drift score (when enabled)
- event count
- behavioral fingerprint (future)
- timeline data (future)

### 3. Provides clean UI for identity analytics  
The dashboard is meant to look like an:
- Identity Security Console  
- SIEM mini-dashboard  
- Drift Monitoring Panel  

---

## 🔧 Tech Stack

- **Next.js 16 (App Router)**
- **React 18 Hooks**
- **Axios** for API calls
- **Tailwind CSS (optional future)**
- **MCP-ready structure** (for future AI integrations)

---

## 📂 Folder Structure



drift-dashboard/
│── app/
│ ├── page.tsx (home page: user list)
│ └── user/[userId]/
│ ├── page.tsx (server: reads URL params)
│ └── UserDetailClient.tsx (client logic + API calls)
│── lib/
│ └── api.ts (axios instance)
│── .env.local
│── package.json


---

## ⚙️ API Configuration

### `lib/api.ts`
A single Axios instance:


import axios from "axios";

export const api = axios.create({
baseURL: process.env.NEXT_PUBLIC_API_URL,
timeout: 5000,
});


### `.env.local`


NEXT_PUBLIC_API_URL=http://localhost:8080


---

## 🖥 Home Page (`page.tsx`)

- Fetches `/profiles`
- Renders a list
- Each user links to:


/user/{userId}


---

## 🧩 User Detail Page

### `page.tsx` (server component)
Extracts:


params.userId

and passes to the client component.

### `UserDetailClient.tsx` (client component)
Does all API calls:
- `/drift/{user}`
- `/fingerprint/{user}`
- `/timeline/{user}`

Renders:
- drift score  
- fingerprint  
- tokens (future)  
- event timeline (future)  

---

## 🧪 Testing the Dashboard

1. Start backend:


cargo run


2. Start dashboard:


npm run dev


3. Open:


http://localhost:3000


4. Click a user:


http://localhost:3000/user/alice


---

## 🛠 How to Use This (Developers / Companies)

This dashboard can be used to build:
- Identity threat detection console  
- Behavioral analytics visualization  
- Device trust score UI  
- SIEM identity extension  
- SOC operator dashboard  
- Real-world fraud detection dashboards  

---

## 🚀 Future Enhancements

- Drift heatmaps  
- Timeline charts  
- Fingerprint stability view  
- Compare two users  
- Alert system (red/yellow/green)  
- MCP-powered auto analysis  
- Suggest actions via AI  
- Exportable reports  
- Role-based access  
- Event streaming dashboards  

---

## ✔️ Summary

This Next.js app serves as a clean, modern UI for the Rust backend behavioral identity engine.  
It provides a strong foundation to grow into a full identity-drift monitoring and security product.
