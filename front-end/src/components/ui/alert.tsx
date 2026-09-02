import type { HTMLAttributes } from "react";
import { CircleAlert, CircleCheck } from "lucide-react";
import { cn } from "@/lib/utils";

interface AlertProps extends HTMLAttributes<HTMLDivElement> {
  variant?: "error" | "success";
}

export function Alert({ className, variant = "error", children, ...props }: AlertProps) {
  const Icon = variant === "success" ? CircleCheck : CircleAlert;
  return (
    <div
      role={variant === "error" ? "alert" : "status"}
      className={cn(
        "flex items-start gap-2 rounded-lg border px-3 py-2.5 text-sm",
        variant === "error" ? "border-destructive/30 bg-destructive/10 text-foreground" : "border-border bg-secondary text-foreground",
        className,
      )}
      {...props}
    >
      <Icon className="mt-0.5 size-4 shrink-0" aria-hidden="true" />
      <div>{children}</div>
    </div>
  );
}
