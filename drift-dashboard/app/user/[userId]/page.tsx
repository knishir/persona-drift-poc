// drift-dashboard/app/user/[userId]/page.tsx
import UserDetailClient from "./UserDetailClient";

export default async function UserDetailPage({ params }: any) {
  // Next.js 16: params is a Promise, so await it
  const { userId } = await params;
  return <UserDetailClient userId={userId} />;
}
