import { Outlet } from "react-router-dom";
import { Footer } from "@/components/layout/footer";
import { Header } from "@/components/layout/header";

export function AppLayout() {
  return (
    <div className="flex min-h-dvh w-full flex-col overflow-x-hidden">
      <Header />
      <main className="flex flex-1 flex-col"><Outlet /></main>
      <Footer />
    </div>
  );
}
