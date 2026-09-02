import { Github } from "lucide-react";
import { BRAND_NAME } from "@/lib/brand";

export function Footer() {
  return (
    <footer className="border-t border-border">
      <div className="mx-auto flex w-full max-w-7xl flex-col items-center justify-between gap-3 px-4 py-6 text-sm text-muted-foreground sm:flex-row sm:px-6 lg:px-8">
        <p>© {new Date().getFullYear()} {BRAND_NAME}. Save URLs for later.</p>
        <a href="https://github.com/AndrewNha" target="_blank" rel="noreferrer" className="inline-flex min-h-11 items-center gap-2 transition-colors hover:text-foreground focus-visible:rounded-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring" aria-label={`${BRAND_NAME} on GitHub`}>
          <Github className="size-4" /> github.com/AndrewNha
        </a>
      </div>
    </footer>
  );
}
