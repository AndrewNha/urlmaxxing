import { createContext, useCallback, useContext, useEffect, useMemo, useState, type ReactNode } from "react";
import { api } from "@/lib/api";
import { getStoredUser, TOKEN_KEY, USER_KEY } from "@/lib/storage";
import type { AuthCredentials, LoginResponse, User } from "@/types";

interface AuthContextValue {
  user: User | null;
  isAuthenticated: boolean;
  login: (credentials: AuthCredentials) => Promise<void>;
  register: (credentials: AuthCredentials) => Promise<void>;
  logout: () => void;
}

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(getStoredUser);

  const logout = useCallback(() => {
    localStorage.removeItem(TOKEN_KEY);
    localStorage.removeItem(USER_KEY);
    setUser(null);
  }, []);

  const login = useCallback(async (credentials: AuthCredentials) => {
    const response = await api<LoginResponse>("/auth/login", { method: "POST", body: credentials });
    localStorage.setItem(TOKEN_KEY, response.token);
    localStorage.setItem(USER_KEY, JSON.stringify(response.user));
    setUser(response.user);
  }, []);

  const register = useCallback(async (credentials: AuthCredentials) => {
    await api<User>("/users", { method: "POST", body: credentials });
    await login(credentials);
  }, [login]);

  useEffect(() => {
    window.addEventListener("urlmaxxing:unauthorized", logout);
    return () => window.removeEventListener("urlmaxxing:unauthorized", logout);
  }, [logout]);

  const value = useMemo(() => ({ user, isAuthenticated: Boolean(user), login, register, logout }), [user, login, register, logout]);
  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) throw new Error("useAuth must be used within AuthProvider");
  return context;
}
