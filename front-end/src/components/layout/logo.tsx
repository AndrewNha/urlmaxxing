import { Bookmark } from "lucide-react";
import { Link } from "react-router-dom";
import { cn } from "@/lib/utils";
import { BRAND_NAME } from "@/lib/brand";

export function Logo({ className, to = "/" }: { className?: string; to?: string }) {
  return (
    <Link to={to} className={cn("inline-flex min-h-11 items-center gap-2 font-semibold tracking-tight", className)} aria-label={`${BRAND_NAME} home`}>
      <span className="flex size-8 items-center justify-center rounded-lg bg-foreground text-background">
        <Bookmark className="size-4 fill-current" />
      </span>
      <span>{BRAND_NAME}</span>
    </Link>
  );
}
