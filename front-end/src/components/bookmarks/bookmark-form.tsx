import { useEffect, useState, type FormEvent } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Spinner } from "@/components/ui/spinner";
import { normalizeUrl, parseTags } from "@/lib/utils";
import type { Bookmark, BookmarkInput } from "@/types";

interface BookmarkFormProps {
  bookmark?: Bookmark | null;
  loading: boolean;
  onSubmit: (input: BookmarkInput) => Promise<void>;
  onCancel: () => void;
}

export function BookmarkForm({ bookmark, loading, onSubmit, onCancel }: BookmarkFormProps) {
  const [title, setTitle] = useState("");
  const [url, setUrl] = useState("");
  const [tags, setTags] = useState("");
  const [error, setError] = useState("");

  useEffect(() => {
    setTitle(bookmark?.title ?? "");
    setUrl(bookmark?.url ?? "");
    setTags(bookmark?.tags.join(", ") ?? "");
    setError("");
  }, [bookmark]);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setError("");
    const normalizedUrl = normalizeUrl(url);
    if (!title.trim()) {
      setError("Enter a title for this URL.");
      return;
    }
    try {
      new URL(normalizedUrl);
    } catch {
      setError("Enter a valid URL.");
      return;
    }
    await onSubmit({ title: title.trim(), url: normalizedUrl, tags: parseTags(tags) });
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      {error && <p role="alert" className="text-sm text-destructive">{error}</p>}
      <div className="space-y-2">
        <Label htmlFor="bookmark-title">Title</Label>
        <Input id="bookmark-title" value={title} onChange={(event) => setTitle(event.target.value)} placeholder="e.g. React documentation" maxLength={120} required disabled={loading} autoFocus />
      </div>
      <div className="space-y-2">
        <Label htmlFor="bookmark-url">URL</Label>
        <Input id="bookmark-url" type="text" inputMode="url" value={url} onChange={(event) => setUrl(event.target.value)} placeholder="https://example.com" required disabled={loading} />
      </div>
      <div className="space-y-2">
        <Label htmlFor="bookmark-tags">Tags</Label>
        <Input id="bookmark-tags" value={tags} onChange={(event) => setTags(event.target.value)} placeholder="work, reading, reference" disabled={loading} />
        <p className="text-xs text-muted-foreground">Separate tags with commas.</p>
      </div>
      <div className="flex flex-col-reverse gap-2 pt-2 sm:flex-row sm:justify-end">
        <Button type="button" variant="outline" onClick={onCancel} disabled={loading}>Cancel</Button>
        <Button type="submit" disabled={loading}>{loading && <Spinner />}{bookmark ? "Save changes" : "Save URL"}</Button>
      </div>
    </form>
  );
}
