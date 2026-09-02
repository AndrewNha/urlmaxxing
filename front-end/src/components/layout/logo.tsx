import { Link } from "react-router-dom";
import { cn } from "@/lib/utils";
import { BRAND_NAME } from "@/lib/brand";

export function Logo({ className, to = "/" }: { className?: string; to?: string }) {
  return (
    <Link to={to} className={cn("inline-flex min-h-11 items-center font-semibold tracking-tight", className)} aria-label={`${BRAND_NAME} home`}>
      {BRAND_NAME}
    </Link>
  );
}
