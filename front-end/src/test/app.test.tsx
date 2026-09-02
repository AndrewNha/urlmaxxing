import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import App from "@/App";
import { TOKEN_KEY, USER_KEY } from "@/lib/storage";

function renderAt(path: string, authenticated = false) {
  if (authenticated) {
    localStorage.setItem(TOKEN_KEY, "test-token");
    localStorage.setItem(USER_KEY, JSON.stringify({ id: "user-1", username: "andre" }));
  }
  window.history.replaceState({}, "", path);
  return render(<App />);
}

describe("brand and authenticated navigation", () => {
  it("uses the exact visible brand and public calls to action", async () => {
    renderAt("/");
    expect(screen.getByRole("link", { name: "Urlmaxxing home" })).toHaveTextContent("Urlmaxxing");
    expect(screen.getByRole("link", { name: "Get started" })).toHaveAttribute("href", "/register");
    await waitFor(() => expect(document.title).toBe("Urlmaxxing — save URLs for later"));
  });

  it("uses app navigation everywhere for an authenticated user", async () => {
    vi.stubGlobal("fetch", vi.fn().mockResolvedValue(new Response(JSON.stringify([]), { status: 200, headers: { "content-type": "application/json" } })));
    renderAt("/", true);
    expect(screen.getByRole("link", { name: "Urlmaxxing home" })).toHaveAttribute("href", "/app");
    expect(screen.getByRole("link", { name: "View my bookmarks" })).toHaveAttribute("href", "/app");
    expect(screen.queryByRole("link", { name: "Get started" })).not.toBeInTheDocument();
  });

  it("protects the app and preserves the intended route", async () => {
    renderAt("/app");
    expect(await screen.findByRole("heading", { name: "Sign in" })).toBeInTheDocument();
    expect(window.location.pathname).toBe("/login");
  });

  it("toggles theme and signs out with announced controls", async () => {
    const user = userEvent.setup();
    vi.stubGlobal("fetch", vi.fn().mockResolvedValue(new Response(JSON.stringify([]), { status: 200, headers: { "content-type": "application/json" } })));
    renderAt("/app", true);
    await user.click(screen.getByRole("button", { name: "Use dark theme" }));
    expect(document.documentElement).toHaveClass("dark");
    await user.click(screen.getByRole("button", { name: "Sign out" }));
    expect(window.location.pathname).toBe("/login");
    expect(localStorage.getItem(TOKEN_KEY)).toBeNull();
  });
});

describe("authentication form", () => {
  it("validates fields and toggles password visibility", async () => {
    const user = userEvent.setup();
    renderAt("/login");
    await user.click(screen.getByRole("button", { name: "Sign in" }));
    expect(screen.getByRole("alert")).toHaveTextContent("Username must be at least 3 characters");
    const password = screen.getByLabelText("Password");
    expect(password).toHaveAttribute("type", "password");
    await user.click(screen.getByRole("button", { name: "Show password" }));
    expect(password).toHaveAttribute("type", "text");
  });
});
