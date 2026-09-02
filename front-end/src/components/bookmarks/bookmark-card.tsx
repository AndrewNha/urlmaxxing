import { useState } from "react";
import { motion } from "framer-motion";
import { Check, Copy, ExternalLink, Link2, Pencil, Trash2 } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { displayHostname, normalizeUrl } from "@/lib/utils";
import type { Bookmark } from "@/types";

interface BookmarkCardProps {
  bookmark: Bookmark;
  onEdit: (bookmark: Bookmark) => void;
  onDelete: (bookmark: Bookmark) => void;
}

export function BookmarkCard({ bookmark, onEdit, onDelete }: BookmarkCardProps) {
  const [copied, setCopied] = useState(false);
  const [copyError, setCopyError] = useState(false);

  async function copyUrl() {
    try {
      await navigator.clipboard.writeText(bookmark.url);
      setCopyError(false);
      setCopied(true);
      window.setTimeout(() => setCopied(false), 1800);
    } catch {
      setCopied(false);
      setCopyError(true);
      window.setTimeout(() => setCopyError(false), 3000);
    }
  }

  return (
    <motion.article layout initial={{ opacity: 0, y: 14 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, scale: 0.96 }} transition={{ duration: 0.22 }} className="min-w-0">
      <Card className="group h-full min-w-0 overflow-hidden transition duration-200 hover:-translate-y-0.5 hover:border-foreground/25 hover:shadow-lg">
        <CardContent className="flex h-full min-w-0 flex-col p-5">
          <div className="mb-4 flex min-w-0 items-start gap-3">
            <div className="flex size-10 shrink-0 items-center justify-center rounded-lg bg-secondary"><Link2 className="size-4" /></div>
            <div className="min-w-0 flex-1">
              <h2 className="truncate font-semibold" title={bookmark.title}>{bookmark.title}</h2>
              <p className="mt-0.5 truncate text-sm text-muted-foreground" title={bookmark.url}>{displayHostname(bookmark.url)}</p>
            </div>
            <Button asChild variant="ghost" size="icon" className="-mr-2 -mt-2 shrink-0" aria-label={`Open ${bookmark.title}`}>
              <a href={normalizeUrl(bookmark.url)} target="_blank" rel="noreferrer"><ExternalLink className="size-4" /></a>
            </Button>
          </div>
          <div className="mb-5 flex min-h-6 flex-wrap gap-1.5">
            {bookmark.tags.length ? bookmark.tags.map((tag) => <Badge key={tag} className="truncate">{tag}</Badge>) : <span className="text-xs text-muted-foreground">No tags</span>}
          </div>
          <div className="mt-auto flex items-center justify-between gap-2 border-t border-border pt-4">
            <time className="min-w-0 truncate text-xs text-muted-foreground" dateTime={bookmark.created_at}>
              {new Intl.DateTimeFormat("en-US", { day: "2-digit", month: "short", year: "numeric" }).format(new Date(bookmark.created_at))}
            </time>
            <div className="flex shrink-0 gap-1">
              <Button variant="ghost" size="icon" onClick={copyUrl} aria-label={copied ? "URL copied" : "Copy URL"} title={copied ? "Copied!" : "Copy URL"}>
                {copied ? <Check className="size-4" /> : <Copy className="size-4" />}
              </Button>
              <Button variant="ghost" size="icon" onClick={() => onEdit(bookmark)} aria-label={`Edit ${bookmark.title}`}><Pencil className="size-4" /></Button>
              <Button variant="ghost" size="icon" className="hover:bg-destructive/10 hover:text-destructive" onClick={() => onDelete(bookmark)} aria-label={`Delete ${bookmark.title}`}><Trash2 className="size-4" /></Button>
            </div>
          </div>
          <span className="sr-only" role="status" aria-live="polite">{copied ? "URL copied to clipboard." : copyError ? "Could not copy URL." : ""}</span>
        </CardContent>
      </Card>
    </motion.article>
  );
}
