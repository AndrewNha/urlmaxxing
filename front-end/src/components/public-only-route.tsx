import { Navigate, Outlet } from "react-router-dom";
import { useAuth } from "@/contexts/auth-context";
import { Spinner } from "@/components/ui/spinner";

export function PublicOnlyRoute() {
  const { isAuthenticated, isLoading } = useAuth();

  if (isLoading) {
    return <div className="flex flex-1 items-center justify-center" role="status"><Spinner className="size-6" /><span className="sr-only">Checking session...</span></div>;
  }

  return isAuthenticated ? <Navigate to="/app" replace /> : <Outlet />;
}
