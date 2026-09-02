import type { User } from "@/types";

export const TOKEN_KEY = "urlmaxxing:token";
export const USER_KEY = "urlmaxxing:user";
export const THEME_KEY = "urlmaxxing:theme";

export function getStoredUser(): User | null {
  try {
    const value = localStorage.getItem(USER_KEY);
    return value ? (JSON.parse(value) as User) : null;
  } catch {
    localStorage.removeItem(USER_KEY);
    return null;
  }
}
