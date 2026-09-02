import { useCallback, useEffect, useMemo, useState } from "react";
import { AnimatePresence, motion } from "framer-motion";
import { BookmarkPlus, Plus, RefreshCw, Search } from "lucide-react";
import { BookmarkCard } from "@/components/bookmarks/bookmark-card";
import { BookmarkForm } from "@/components/bookmarks/bookmark-form";
import { PageTitle } from "@/components/page-title";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Dialog } from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import { Spinner } from "@/components/ui/spinner";
import { useAuth } from "@/contexts/auth-context";
import { api } from "@/lib/api";
import type { Bookmark, BookmarkInput } from "@/types";

export function BookmarksPage() {
  const { user } = useAuth();
  const [bookmarks, setBookmarks] = useState<Bookmark[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [search, setSearch] = useState("");
  const [editorOpen, setEditorOpen] = useState(false);
  const [selected, setSelected] = useState<Bookmark | null>(null);
  const [deleteTarget, setDeleteTarget] = useState<Bookmark | null>(null);
  const [actionLoading, setActionLoading] = useState(false);
  const [actionError, setActionError] = useState("");
  const [notice, setNotice] = useState("");

  const loadBookmarks = useCallback(async () => {
    setLoading(true);
    setError("");
    try {
      const data = await api<Bookmark[]>("/bookmarks", { authenticated: true });
      setBookmarks(data);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : "Could not load bookmarks.");
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => { void loadBookmarks(); }, [loadBookmarks]);

  const filteredBookmarks = useMemo(() => {
    const query = search.trim().toLocaleLowerCase("en-US");
    if (!query) return bookmarks;
    return bookmarks.filter((bookmark) =>
      bookmark.title.toLocaleLowerCase("en-US").includes(query)
      || bookmark.url.toLocaleLowerCase("en-US").includes(query)
      || bookmark.tags.some((tag) => tag.toLocaleLowerCase("en-US").includes(query)),
    );
  }, [bookmarks, search]);

  function openCreate() {
    setSelected(null);
    setActionError("");
    setEditorOpen(true);
  }

  function openEdit(bookmark: Bookmark) {
    setSelected(bookmark);
    setActionError("");
    setEditorOpen(true);
  }

  function showNotice(message: string) {
    setNotice(message);
    window.setTimeout(() => setNotice(""), 3000);
  }

  async function saveBookmark(input: BookmarkInput) {
    setActionLoading(true);
    setActionError("");
    try {
      if (selected) {
        const updated = await api<Bookmark>(`/bookmarks/${selected.id}`, { method: "PATCH", authenticated: true, body: input });
        setBookmarks((items) => items.map((item) => item.id === updated.id ? updated : item));
        showNotice("Bookmark updated successfully.");
      } else {
        const created = await api<Bookmark>("/bookmarks", { method: "POST", authenticated: true, body: input });
        setBookmarks((items) => [created, ...items]);
        showNotice("Bookmark added successfully.");
      }
      setEditorOpen(false);
    } catch (caught) {
      setActionError(caught instanceof Error ? caught.message : "Could not save the bookmark.");
    } finally {
      setActionLoading(false);
    }
  }

  async function deleteBookmark() {
    if (!deleteTarget) return;
    setActionLoading(true);
    setActionError("");
    try {
      await api<Bookmark>(`/bookmarks/${deleteTarget.id}`, { method: "DELETE", authenticated: true });
      setBookmarks((items) => items.filter((item) => item.id !== deleteTarget.id));
      setDeleteTarget(null);
      showNotice("Bookmark deleted.");
    } catch (caught) {
      setActionError(caught instanceof Error ? caught.message : "Could not delete the bookmark.");
    } finally {
      setActionLoading(false);
    }
  }

  return (
    <section className="flex-1 px-4 py-8 sm:px-6 sm:py-10 lg:px-8">
      <PageTitle title="My bookmarks" />
      <div className="mx-auto w-full max-w-7xl">
        <motion.div initial={{ opacity: 0, y: 12 }} animate={{ opacity: 1, y: 0 }} className="mb-8 flex flex-col justify-between gap-5 sm:flex-row sm:items-end">
          <div className="min-w-0">
            <p
              title={`${user?.username}'s collection`}
              className="mb-2 break-words text-sm text-muted-foreground"
            >
              {user?.username}'s collection
            </p>
            <h1 className="text-3xl font-semibold tracking-tight sm:text-4xl">My bookmarks</h1>
            <p className="mt-2 text-muted-foreground">{bookmarks.length} {bookmarks.length === 1 ? "link saved" : "links saved"}</p>
          </div>
          <Button size="lg" onClick={openCreate} className="w-full sm:w-auto"><Plus className="size-4" /> New bookmark</Button>
        </motion.div>

        {notice && <Alert variant="success" className="mb-5" aria-live="polite">{notice}</Alert>}

        <div className="mb-6 flex gap-2">
          <div className="relative min-w-0 flex-1">
            <Search className="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" aria-hidden="true" />
            <Input value={search} onChange={(event) => setSearch(event.target.value)} className="pl-9" placeholder="Search by title, URL, or tag..." aria-label="Search bookmarks" />
          </div>
          <Button variant="outline" size="icon" onClick={() => void loadBookmarks()} disabled={loading} aria-label="Refresh bookmarks">
            <RefreshCw className={`size-4 ${loading ? "animate-spin" : ""}`} />
          </Button>
        </div>

        {loading ? (
          <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-3" aria-label="Loading bookmarks">
            {Array.from({ length: 6 }).map((_, index) => <Skeleton key={index} className="h-52" />)}
          </div>
        ) : error ? (
          <div className="rounded-xl border border-border bg-card p-8 text-center">
            <Alert className="mx-auto max-w-lg text-left">{error}</Alert>
            <Button variant="outline" className="mt-5" onClick={() => void loadBookmarks()}><RefreshCw className="size-4" /> Try again</Button>
          </div>
        ) : filteredBookmarks.length ? (
          <motion.div layout className="grid min-w-0 gap-4 sm:grid-cols-2 xl:grid-cols-3">
            <AnimatePresence mode="popLayout">
              {filteredBookmarks.map((bookmark) => <BookmarkCard key={bookmark.id} bookmark={bookmark} onEdit={openEdit} onDelete={(item) => { setActionError(""); setDeleteTarget(item); }} />)}
            </AnimatePresence>
          </motion.div>
        ) : (
          <div className="rounded-2xl border border-dashed border-border p-10 text-center sm:p-16">
            <span className="mx-auto flex size-14 items-center justify-center rounded-2xl bg-secondary"><BookmarkPlus className="size-6" /></span>
            <h2 className="mt-5 text-xl font-semibold">{search ? "No results" : "Your collection starts here"}</h2>
            <p className="mx-auto mt-2 max-w-md text-sm text-muted-foreground">{search ? "Try searching for a different title, address, or tag." : "Add your first bookmark and keep your best links always at hand."}</p>
            {!search && <Button className="mt-6" onClick={openCreate}><Plus className="size-4" /> Add your first link</Button>}
          </div>
        )}
      </div>

      <Dialog open={editorOpen} onClose={() => !actionLoading && setEditorOpen(false)} title={selected ? "Edit bookmark" : "New bookmark"} description={selected ? "Update the link's information." : "Save a link to refer to whenever you want."}>
        {actionError && <Alert className="mb-4">{actionError}</Alert>}
        <BookmarkForm bookmark={selected} loading={actionLoading} onSubmit={saveBookmark} onCancel={() => setEditorOpen(false)} />
      </Dialog>

      <Dialog open={Boolean(deleteTarget)} onClose={() => !actionLoading && setDeleteTarget(null)} title="Delete bookmark?" description="This action is permanent and cannot be undone." className="max-w-md">
        {actionError && <Alert className="mb-4">{actionError}</Alert>}
        <p className="mb-6 break-words text-sm">You are about to delete <strong>{deleteTarget?.title}</strong>.</p>
        <div className="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
          <Button variant="outline" onClick={() => setDeleteTarget(null)} disabled={actionLoading}>Cancel</Button>
          <Button variant="destructive" onClick={() => void deleteBookmark()} disabled={actionLoading}>{actionLoading && <Spinner />} Delete permanently</Button>
        </div>
      </Dialog>
    </section>
  );
}
