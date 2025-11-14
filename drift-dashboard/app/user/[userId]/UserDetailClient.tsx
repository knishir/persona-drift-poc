// drift-dashboard/app/user/[userId]/UserDetailClient.tsx
"use client";

import { useEffect, useState } from "react";
import api from "@/lib/api";

export default function UserDetailClient({ userId }: { userId: string }) {
  const [loading, setLoading] = useState(true);
  const [profileCount, setProfileCount] = useState<number | null>(null);

  useEffect(() => {
    async function load() {
      try {
        const res = await api.get("/profiles");
        setProfileCount(res.data?.profiles?.[userId] ?? 0);
      } catch (err) {
        console.error("Failed to load profiles:", err);
        setProfileCount(null);
      } finally {
        setLoading(false);
      }
    }
    load();
  }, [userId]);

  if (loading) return <div style={{ padding: 20 }}>Loading user…</div>;

  if (profileCount === null) {
    return (
      <div style={{ padding: 20 }}>
        <h1>User: <i style={{ color: "red" }}>{userId}</i></h1>
        <div style={{ color: "red" }}>User not found or failed to load data</div>
      </div>
    );
  }

  return (
    <div style={{ padding: 20 }}>
      <h1>User: {userId}</h1>
      <div style={{ marginTop: 12 }}>
        <strong>Events recorded:</strong> {profileCount}
      </div>
    </div>
  );
}
