import { ArrowLeft, SearchX } from "lucide-react";
import { Link } from "react-router-dom";
import { PageTitle } from "@/components/page-title";
import { Button } from "@/components/ui/button";
import { useAuth } from "@/contexts/auth-context";

export function NotFoundPage() {
  const { isAuthenticated } = useAuth();
  return (
    <section className="flex flex-1 items-center justify-center px-4 py-20 text-center">
      <PageTitle title="Page not found" />
      <div className="animate-slide-in-blurred-top max-w-lg">
        <span className="mx-auto flex size-16 items-center justify-center rounded-2xl bg-secondary"><SearchX className="size-7" /></span>
        <p className="mt-6 text-sm font-semibold uppercase tracking-[0.25em] text-muted-foreground">Error 404</p>
        <h1 className="mt-3 text-4xl font-semibold tracking-tight sm:text-5xl">Page not found</h1>
        <p className="mt-4 text-muted-foreground">The address may have changed or may not exist.</p>
        <Button asChild className="mt-8"><Link to={isAuthenticated ? "/app" : "/"}><ArrowLeft className="size-4" /> {isAuthenticated ? "Back to bookmarks" : "Back to home"}</Link></Button>
      </div>
    </section>
  );
}
