import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import App from "@/App";

function renderAt(path: string, authenticated = false) {
  const fetchMock = vi.fn(async (input: RequestInfo | URL, init?: RequestInit) => {
    const url = String(input);

    if (url.endsWith("/auth/me")) {
      return authenticated
        ? new Response(JSON.stringify({ id: "user-1", username: "andre" }), { status: 200, headers: { "content-type": "application/json" } })
        : new Response(JSON.stringify({ error: "Unauthorized" }), { status: 401, headers: { "content-type": "application/json" } });
    }

    if (url.endsWith("/auth/logout") && init?.method === "POST") {
      return new Response(null, { status: 204 });
    }

    return new Response(JSON.stringify([]), { status: 200, headers: { "content-type": "application/json" } });
  });

  vi.stubGlobal("fetch", fetchMock);
  window.history.replaceState({}, "", path);
  return { ...render(<App />), fetchMock };
}

describe("brand and authenticated navigation", () => {
  it("uses the exact visible brand and public calls to action", async () => {
    renderAt("/");
    expect(screen.getByRole("link", { name: "Urlmaxxing home" })).toHaveTextContent("Urlmaxxing");
    expect(screen.getByRole("link", { name: "Get started" })).toHaveAttribute("href", "/register");
    await waitFor(() => expect(document.title).toBe("Urlmaxxing — save URLs for later"));
  });

  it("uses app navigation everywhere for an authenticated user", async () => {
    renderAt("/", true);
    await waitFor(() => expect(screen.getByRole("link", { name: "Urlmaxxing home" })).toHaveAttribute("href", "/app"));
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
    const { fetchMock } = renderAt("/app", true);
    await screen.findByRole("button", { name: "Sign out" });
    await user.click(screen.getByRole("button", { name: "Use dark theme" }));
    expect(document.documentElement).toHaveClass("dark");
    await user.click(screen.getByRole("button", { name: "Sign out" }));
    await waitFor(() => expect(window.location.pathname).toBe("/login"));
    expect(fetchMock).toHaveBeenCalledWith(
      expect.stringContaining("/auth/logout"),
      expect.objectContaining({ method: "POST", credentials: "include" }),
    );
  });
});

describe("authentication form", () => {
  it("validates fields and toggles password visibility", async () => {
    const user = userEvent.setup();
    renderAt("/login");
    await user.click(await screen.findByRole("button", { name: "Sign in" }));
    expect(screen.getByRole("alert")).toHaveTextContent("Username must be at least 3 characters");
    const password = screen.getByLabelText("Password");
    expect(password).toHaveAttribute("type", "password");
    await user.click(screen.getByRole("button", { name: "Show password" }));
    expect(password).toHaveAttribute("type", "text");
  });
});
