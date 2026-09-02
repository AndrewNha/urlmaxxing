import { useEffect } from "react";
import { BRAND_NAME, BRAND_TAGLINE } from "@/lib/brand";

export function PageTitle({ title }: { title?: string }) {
  useEffect(() => {
    document.title = title ? `${title} | ${BRAND_NAME}` : `${BRAND_NAME} — ${BRAND_TAGLINE}`;
    return () => { document.title = BRAND_NAME; };
  }, [title]);
  return null;
}
