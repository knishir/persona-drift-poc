# 🌐 Persona Drift Dashboard (Next.js) — Full Documentation

The **Persona Drift Dashboard** is a Next.js frontend that visualizes user behavioral analytics from the Rust Engine.  
It provides dashboards for drift, fingerprints, timeline, and user statistics.

---

# 📌 1. What This Dashboard Does

The dashboard performs **four main functions**:

### ✔ 1. Lists all users and event counts  
From the `/profiles` endpoint.

### ✔ 2. Shows detailed analytics for each user  
- Drift score  
- Drift reasons  
- Stability score  
- Fingerprint tokens  

### ✔ 3. Displays event timeline  
Sorted events with metadata.

### ✔ 4. Provides a clean UI for SOC/security analysts  
Designed for clarity and investigation.

---

# 📌 2. Technology Stack

| Component | Technology |
|----------|------------|
| Framework | Next.js 16 (App Router) |
| Client | React 18 |
| HTTP | Axios |
| Styling | Tailwind (optional) |
| Runtime | Node 20+ |
| API | Rust Engine (configurable via `.env.local`) |

---

# 📌 3. Project Structure

```
drift-dashboard/
│
├── app/
│   ├── page.tsx                 # Lists all users
│   └── user/
│       └── [userId]/
│           ├── page.tsx         # Server wrapper
│           └── UserDetailClient.tsx
│
├── lib/
│   └── api.ts                   # Axios instance
│
└── .env.local
```

### Why split Server + Client?

To avoid Next.js 16 “async params” errors  
→ `page.tsx` passes params  
→ `UserDetailClient.tsx` performs all API calls and UI rendering.

---

# 📌 4. API Configuration

### File: `.env.local`

```
NEXT_PUBLIC_ENGINE_URL=http://localhost:8080
```

### File: `lib/api.ts`

```ts
import axios from "axios";

export const api = axios.create({
  baseURL: process.env.NEXT_PUBLIC_ENGINE_URL,
});
```

---

# 📌 5. Pages Explained

## 🏠 Home Page — `/`

Fetches:

```
GET /profiles
```

Displays:

- All users  
- Event count  
- Clickable links to `/user/[userId]`

---

## 👤 User Page — `/user/[userId]`

Core component: `UserDetailClient.tsx`

Fetches:

### ✔ `/drift/:id`
- Drift score  
- Drift reasons  

### ✔ `/fingerprint/:id`
- Tokens  
- Stability score  
- Finger history  

### ✔ `/timeline/:id`
- Sorted event list  

### UI rendered:

- User identity  
- Total events  
- Drift summary  
- Fingerprint tokens  
- Timeline table  

---

# 📌 6. How the Dashboard Works (Full Flow)

### 1️⃣ User clicks a username  
→ Navigates to `/user/alice`

### 2️⃣ The client component runs:

```ts
api.get(`/drift/alice`)
api.get(`/fingerprint/alice`)
api.get(`/timeline/alice`)
```

### 3️⃣ State updates  
UI shows:

- Drift score gauge  
- Token chips  
- Timeline  
- Stability percentage  

### 4️⃣ If user not found  
Graceful error is shown on screen.

---

# 📌 7. Example Output

### Home

```
alice — 3 events
bob — 2 events
```

---

### User Page

```
User: alice
Events recorded: 3

Drift Score: 45
Reasons:
- Multiple IPs
- Browser changed
- New OS

Fingerprint:
- browser:chrome
- os:linux
- dev_type:desktop

Timeline:
[login, change browser, login from new IP]
```

---

# 📌 8. Features Completed

### ✔ Fully dynamic routing  
### ✔ Complete API integration  
### ✔ Drift + fingerprint + timeline visualizations  
### ✔ Zero config `.env.local` setup  
### ✔ No Next.js errors (fixed async params)  
### ✔ Working local dev environment  

---

# 📌 9. Future Roadmap

### 🚀 1. Dark mode UI  
### 🚀 2. Charts for drift score history  
### 🚀 3. SOC analyst dashboard  
### 🚀 4. Role-based authentication  
### 🚀 5. Event search and filtering  
### 🚀 6. Realtime updates via WebSockets  
### 🚀 7. Merge user history comparisons  

---

# 📌 10. How To Run

### 1️⃣ Install dependencies:

```
npm install
```

### 2️⃣ Configure API engine URL:

```
NEXT_PUBLIC_ENGINE_URL=http://localhost:8080
```

### 3️⃣ Start development:

```
npm run dev
```

Open:

```
http://localhost:3000
```

---

# 📌 11. What This Dashboard Enables

This system is ideal for:

- Security operations  
- Incident response  
- Fraud detection  
- Behavioral analysis  
- Account compromise tracking  

This frontend + backend combination forms a **complete risk analytics system**.

---

# 🎉 Final Notes

This dashboard is fully functional, extendable, and tightly integrated with the Rust engine.  
Future developers can easily build additional UI features, charts, and investigative tools.
