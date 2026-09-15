import { createContext, useCallback, useContext, useEffect, useMemo, useState, type ReactNode } from "react";
import { api } from "@/lib/api";
import type { AuthCredentials, LoginResponse, User } from "@/types";

interface AuthContextValue {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (credentials: AuthCredentials) => Promise<void>;
  register: (credentials: AuthCredentials) => Promise<void>;
  logout: () => Promise<void>;
  updateUsername: (username: string) => Promise<void>;
  changePassword: (currentPassword: string, newPassword: string) => Promise<void>;
  deleteAccount: (currentPassword: string) => Promise<void>;
}

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  const logout = useCallback(async () => {
    await api<void>("/auth/logout", { method: "POST" }).catch(() => undefined);
    setUser(null);
  }, []);

  const login = useCallback(async (credentials: AuthCredentials) => {
    const response = await api<LoginResponse>("/auth/login", { method: "POST", body: credentials });
    setUser(response.user);
  }, []);

  const register = useCallback(async (credentials: AuthCredentials) => {
    await api<User>("/users", { method: "POST", body: credentials });
    await login(credentials);
  }, [login]);

  const updateUsername = useCallback(async (username: string) => {
    if (!user) throw new Error("You must be signed in to update your account.");
    const updatedUser = await api<User>(`/users/${user.id}`, {
      method: "PUT",
      authenticated: true,
      body: { username },
    });
    setUser(updatedUser);
  }, [user]);

  const changePassword = useCallback(async (currentPassword: string, newPassword: string) => {
    if (!user) throw new Error("You must be signed in to update your password.");
    await api<User>(`/users/${user.id}/password`, {
      method: "PATCH",
      authenticated: true,
      body: { current_password: currentPassword, new_password: newPassword },
    });
    setUser(null);
  }, [user]);

  const deleteAccount = useCallback(async (currentPassword: string) => {
    if (!user) throw new Error("You must be signed in to delete your account.");
    await api<User>(`/users/${user.id}`, {
      method: "DELETE",
      authenticated: true,
      body: { current_password: currentPassword },
    });
    setUser(null);
  }, [user]);

  useEffect(() => {
    let active = true;

    api<User>("/auth/me")
      .then((currentUser) => {
        if (active) setUser(currentUser);
      })
      .catch(() => {
        if (active) setUser(null);
      })
      .finally(() => {
        if (active) setIsLoading(false);
      });

    return () => {
      active = false;
    };
  }, []);

  useEffect(() => {
    window.addEventListener("urlmaxxing:unauthorized", logout);
    return () => window.removeEventListener("urlmaxxing:unauthorized", logout);
  }, [logout]);

  const value = useMemo(
    () => ({ user, isAuthenticated: Boolean(user), isLoading, login, register, logout, updateUsername, changePassword, deleteAccount }),
    [user, isLoading, login, register, logout, updateUsername, changePassword, deleteAccount],
  );
  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) throw new Error("useAuth must be used within AuthProvider");
  return context;
}
