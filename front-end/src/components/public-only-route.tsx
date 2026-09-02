import { Navigate, Outlet } from "react-router-dom";
import { useAuth } from "@/contexts/auth-context";

export function PublicOnlyRoute() {
  return useAuth().isAuthenticated ? <Navigate to="/app" replace /> : <Outlet />;
}
