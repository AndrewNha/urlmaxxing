import { useState, type FormEvent } from "react";
import { Link, useLocation, useNavigate } from "react-router-dom";
import { ArrowRight } from "lucide-react";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Spinner } from "@/components/ui/spinner";
import { PasswordInput } from "@/components/password-input";
import { useAuth } from "@/contexts/auth-context";

export function AuthForm({ mode }: { mode: "login" | "register" }) {
  const isLogin = mode === "login";
  const { login, register } = useAuth();
  const navigate = useNavigate();
  const location = useLocation();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setError("");
    const cleanUsername = username.trim();
    if (cleanUsername.length < 3) {
      setError("Username must be at least 3 characters.");
      return;
    }
    if (password.length < 6) {
      setError("Password must be at least 6 characters.");
      return;
    }
    if (!isLogin && password !== confirmPassword) {
      setError("Passwords do not match.");
      return;
    }

    setLoading(true);
    try {
      const credentials = { username: cleanUsername, password };
      if (isLogin) await login(credentials);
      else await register(credentials);
      const from = (location.state as { from?: string } | null)?.from;
      navigate(from?.startsWith("/") ? from : "/app", { replace: true });
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : "Unable to continue.");
    } finally {
      setLoading(false);
    }
  }

  return (
    <Card className="w-full max-w-md animate-slide-in-blurred-top border-border/80 shadow-xl shadow-black/5 dark:shadow-black/20">
      <CardHeader className="space-y-2 pb-5">
        <CardTitle className="text-2xl">{isLogin ? "Sign in" : "Create an account"}</CardTitle>
        <CardDescription>
          {isLogin ? "View and manage your saved URLs." : "Create an account to start saving URLs."}
        </CardDescription>
      </CardHeader>
      <CardContent>
        <form onSubmit={handleSubmit} className="space-y-4" noValidate>
          {error && <Alert>{error}</Alert>}
          <div className="space-y-2">
            <Label htmlFor="username">Username</Label>
            <Input id="username" name="username" autoComplete="username" value={username} onChange={(event) => setUsername(event.target.value)} placeholder="your_username" required minLength={3} disabled={loading} autoFocus />
          </div>
          <div className="space-y-2">
            <Label htmlFor="password">Password</Label>
            <PasswordInput id="password" name="password" autoComplete={isLogin ? "current-password" : "new-password"} value={password} onChange={(event) => setPassword(event.target.value)} placeholder="At least 6 characters" required minLength={6} disabled={loading} />
          </div>
          {!isLogin && (
            <div className="space-y-2">
              <Label htmlFor="confirm-password">Confirm password</Label>
              <PasswordInput id="confirm-password" name="confirm-password" autoComplete="new-password" value={confirmPassword} onChange={(event) => setConfirmPassword(event.target.value)} placeholder="Enter the password again" required disabled={loading} />
            </div>
          )}
          <Button type="submit" className="w-full" size="lg" disabled={loading}>
            {loading ? <><Spinner />{isLogin ? "Signing in..." : "Creating account..."}</> : <>{isLogin ? "Sign in" : "Create account"}<ArrowRight className="size-4" /></>}
          </Button>
        </form>
        <p className="mt-6 text-center text-sm text-muted-foreground">
          {isLogin ? "Need an account?" : "Already have an account?"}{" "}
          <Link to={isLogin ? "/register" : "/login"} className="font-medium text-foreground underline-offset-4 hover:underline">
            {isLogin ? "Create one" : "Sign in"}
          </Link>
        </p>
      </CardContent>
    </Card>
  );
}
