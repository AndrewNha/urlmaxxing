import { Bookmark, LogOut, Moon, Sun } from "lucide-react";
import { Link, useNavigate } from "react-router-dom";
import { Logo } from "@/components/layout/logo";
import { Button } from "@/components/ui/button";
import { useAuth } from "@/contexts/auth-context";
import { useTheme } from "@/contexts/theme-context";

export function Header() {
  const { theme, toggleTheme } = useTheme();
  const { user, isAuthenticated, logout } = useAuth();
  const navigate = useNavigate();

  function handleLogout() {
    logout();
    navigate("/login", { replace: true });
  }

  return (
    <header className="sticky top-0 z-40 border-b border-border/70 bg-background/85 backdrop-blur-xl">
      <div className="mx-auto flex h-16 w-full max-w-7xl items-center justify-between gap-3 px-4 sm:px-6 lg:px-8">
        <Logo to={isAuthenticated ? "/app" : "/"} />
        <nav className="flex min-w-0 items-center gap-1 sm:gap-2" aria-label="Main navigation">
          {isAuthenticated ? (
            <>
              <span
                title={`Signed in as ${user?.username}`}
                className="hidden max-w-48 truncate text-sm text-muted-foreground sm:block"
              >
                Signed in as <strong className="font-medium text-foreground">{user?.username}</strong>
              </span>
              <Button asChild variant="ghost" size="sm" className="px-2 sm:px-3">
                <Link to="/app"><Bookmark className="size-4" /><span className="hidden xs:inline sm:inline">Bookmarks</span></Link>
              </Button>
              <Button variant="ghost" size="icon" onClick={handleLogout} aria-label="Sign out">
                <LogOut className="size-4" />
              </Button>
            </>
          ) : (
            <>
              <Button asChild variant="ghost" size="sm" className="hidden sm:inline-flex"><Link to="/login">Sign in</Link></Button>
              <Button asChild size="sm" className="px-2.5 sm:px-3"><Link to="/register"><span className="sm:hidden">Join</span><span className="hidden sm:inline">Create account</span></Link></Button>
            </>
          )}
          <Button variant="ghost" size="icon" onClick={toggleTheme} aria-label={theme === "dark" ? "Use light theme" : "Use dark theme"}>
            {theme === "dark" ? <Sun className="size-4" /> : <Moon className="size-4" />}
          </Button>
        </nav>
      </div>
    </header>
  );
}
