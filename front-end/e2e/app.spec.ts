import { expect, test, type Page } from "@playwright/test";

const initialBookmark = {
  id: "bookmark-1",
  user_id: "user-1",
  title: "React documentation",
  url: "https://react.dev",
  tags: ["docs", "react"],
  created_at: "2026-01-02T12:00:00.000Z",
};

async function mockApi(page: Page) {
  let bookmarks = [initialBookmark];
  await page.route("http://localhost:3000/**", async (route) => {
    const request = route.request();
    const path = new URL(request.url()).pathname;
    if (path === "/auth/login") {
      return route.fulfill({ json: { token: "test-token", user: { id: "user-1", username: "andre" } } });
    }
    if (path === "/users") return route.fulfill({ json: { id: "user-1", username: "andre" } });
    if (path === "/bookmarks" && request.method() === "GET") return route.fulfill({ json: bookmarks });
    if (path === "/bookmarks" && request.method() === "POST") {
      const created = { id: "bookmark-2", user_id: "user-1", created_at: "2026-01-03T12:00:00.000Z", ...request.postDataJSON() };
      bookmarks = [created, ...bookmarks];
      return route.fulfill({ status: 201, json: created });
    }
    if (path.startsWith("/bookmarks/") && request.method() === "PATCH") {
      const updated = { ...bookmarks[0], ...request.postDataJSON() };
      bookmarks = bookmarks.map((item) => item.id === updated.id ? updated : item);
      return route.fulfill({ json: updated });
    }
    if (path.startsWith("/bookmarks/") && request.method() === "DELETE") {
      bookmarks = bookmarks.filter((item) => !path.endsWith(item.id));
      return route.fulfill({ json: {} });
    }
    return route.fulfill({ status: 404, json: {} });
  });
}

async function authenticate(page: Page) {
  await page.addInitScript(() => {
    localStorage.setItem("urlmaxxing:token", "test-token");
    localStorage.setItem("urlmaxxing:user", JSON.stringify({ id: "user-1", username: "andre" }));
  });
}

async function expectResponsive(page: Page) {
  const measurements = await page.evaluate(() => ({
    viewport: document.documentElement.clientWidth,
    documentWidth: document.documentElement.scrollWidth,
    controlsOutside: [...document.querySelectorAll<HTMLElement>("button, a, input")].filter((element) => {
      const rect = element.getBoundingClientRect();
      return rect.right > document.documentElement.clientWidth + 1 || rect.left < -1;
    }).length,
  }));
  expect(measurements.documentWidth).toBeLessThanOrEqual(measurements.viewport + 1);
  expect(measurements.controlsOutside).toBe(0);
}

test.beforeEach(async ({ page }) => mockApi(page));

test("home, auth validation, login, theme, and logout", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("link", { name: "Urlmaxxing home" })).toBeVisible();
  await expectResponsive(page);
  await page.getByRole("link", { name: "I have an account" }).click();
  await page.getByRole("button", { name: "Sign in" }).click();
  await expect(page.getByRole("alert")).toContainText("Username must be at least 3 characters");
  await page.getByLabel("Username").fill("andre");
  await page.getByLabel("Password").fill("secret1");
  await page.getByRole("button", { name: "Show password" }).click();
  await page.getByRole("button", { name: "Sign in", exact: true }).click();
  await expect(page).toHaveURL(/\/app$/);
  await page.getByRole("button", { name: "Use dark theme" }).click();
  await expect(page.locator("html")).toHaveClass(/dark/);
  await page.getByRole("button", { name: "Sign out" }).click();
  await expect(page).toHaveURL(/\/login$/);
});

test("bookmark search and CRUD dialogs", async ({ page }) => {
  await authenticate(page);
  await page.goto("/app");
  await expect(page.getByText("React documentation")).toBeVisible();
  await expectResponsive(page);
  await page.getByLabel("Search bookmarks").fill("missing");
  await expect(page.getByRole("heading", { name: "No results" })).toBeVisible();
  await page.getByLabel("Search bookmarks").fill("");
  await page.getByRole("button", { name: "New bookmark" }).click();
  await expect(page.getByRole("dialog", { name: "New bookmark" })).toBeVisible();
  await page.getByLabel("Title").fill("MDN");
  await page.getByLabel("URL").fill("developer.mozilla.org");
  await page.getByLabel("Tags").fill("docs, web");
  await page.getByRole("button", { name: "Save URL" }).click();
  await expect(page.getByText("Bookmark added successfully.")).toBeVisible();
  await page.getByRole("button", { name: "Edit MDN" }).click();
  await page.getByLabel("Title").fill("MDN Web Docs");
  await page.getByRole("button", { name: "Save changes" }).click();
  await expect(page.getByText("Bookmark updated successfully.")).toBeVisible();
  await page.getByRole("button", { name: "Delete MDN Web Docs" }).click();
  await page.getByRole("button", { name: "Delete permanently" }).click();
  await expect(page.getByText("Bookmark deleted.")).toBeVisible();
});

test("protected route, retry, and 404 remain usable", async ({ page }) => {
  await page.goto("/app");
  await expect(page).toHaveURL(/\/login$/);
  await page.goto("/does-not-exist");
  await expect(page.getByRole("heading", { name: "Page not found" })).toBeVisible();
  await expectResponsive(page);
});

test("visual baselines for home and app", async ({ page }, testInfo) => {
  const visualViewport = testInfo.project.name.endsWith("375x667") || testInfo.project.name.endsWith("1440x900");
  test.skip(!visualViewport, "Zen mobile/desktop baselines only");
  for (const theme of ["light", "dark"] as const) {
    await page.addInitScript((selectedTheme) => localStorage.setItem("urlmaxxing:theme", selectedTheme), theme);
    await page.goto("/");
    await expect(page).toHaveScreenshot(`home-${theme}.png`, { fullPage: true, animations: "disabled" });
    await authenticate(page);
    await page.goto("/app");
    await expect(page.getByText("React documentation")).toBeVisible();
    await expect(page).toHaveScreenshot(`app-${theme}.png`, { fullPage: true, animations: "disabled" });
  }
});
