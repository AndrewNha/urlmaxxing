import "@testing-library/jest-dom/vitest";
import { afterEach, vi } from "vitest";
import { cleanup } from "@testing-library/react";

const storage = new Map<string, string>();
const storageMock: Storage = {
  get length() { return storage.size; },
  clear: () => storage.clear(),
  getItem: (key) => storage.get(key) ?? null,
  key: (index) => [...storage.keys()][index] ?? null,
  removeItem: (key) => { storage.delete(key); },
  setItem: (key, value) => { storage.set(key, String(value)); },
};
Object.defineProperty(window, "localStorage", { configurable: true, value: storageMock });
Object.defineProperty(globalThis, "localStorage", { configurable: true, value: storageMock });

Object.defineProperty(window, "matchMedia", {
  writable: true,
  value: (query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    addListener: vi.fn(),
    removeListener: vi.fn(),
    dispatchEvent: vi.fn(),
  }),
});

class IntersectionObserverMock implements IntersectionObserver {
  readonly root = null;
  readonly rootMargin = "0px";
  readonly thresholds = [0];
  disconnect() {}
  observe() {}
  takeRecords() { return []; }
  unobserve() {}
}
Object.defineProperty(window, "IntersectionObserver", { configurable: true, value: IntersectionObserverMock });
Object.defineProperty(globalThis, "IntersectionObserver", { configurable: true, value: IntersectionObserverMock });

afterEach(() => {
  cleanup();
  localStorage.clear();
  vi.restoreAllMocks();
});
