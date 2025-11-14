"use client";

import { useEffect, useState } from "react";
import  api  from "@/lib/api";

export default function Home() {
  const [profiles, setProfiles] = useState<any>({});
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function load() {
      try {
        const res = await api.get("/profiles");
        setProfiles(res.data.profiles);
      } catch (error) {
        console.error("Failed loading profiles", error);
      }
      setLoading(false);
    }
    load();
  }, []);

  if (loading) return <div className="p-10 text-xl">Loading users…</div>;

  return (
    <div className="p-10 text-xl space-y-4">
      <h1 className="text-3xl font-bold mb-5">Users</h1>

      {Object.keys(profiles).length === 0 && <div>No users found</div>}

      {Object.entries(profiles).map(([user, count]: any) => (
        <a
          href={`/user/${user}`}
          key={user}
          className="block p-4 border rounded-lg bg-gray-100 hover:bg-gray-200 transition"
        >
          <div className="font-bold">{user}</div>
          <div className="text-sm text-gray-700">{count} events</div>
        </a>
      ))}
    </div>
  );
}
