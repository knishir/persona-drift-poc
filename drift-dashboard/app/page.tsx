"use client";

import { useEffect, useState } from "react";
import  api  from "@/lib/api";

export default function UserDetailPage({ params }: any) {
  const userId = params.userId;
  const [events, setEvents] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function load() {
      try {
        const res = await api.get("/profiles");
        const count = res.data.profiles[userId];

        setEvents(new Array(count).fill(0)); // placeholder
      } catch (err) {
        console.error(err);
      }

      setLoading(false);
    }
    load();
  }, [userId]);

  if (loading) return <div className="p-10 text-xl">Loading…</div>;

  return (
    <div className="p-10 text-xl space-y-4">
      <h1 className="text-3xl font-bold mb-4">User: {userId}</h1>
      <p>Events recorded: {events.length}</p>
    </div>
  );
}
