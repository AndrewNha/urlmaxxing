import { useState, type FormEvent } from "react";
import { motion } from "framer-motion";
import { KeyRound, Save, Trash2, UserRound } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { PasswordInput } from "@/components/password-input";
import { PageTitle } from "@/components/page-title";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Dialog } from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Spinner } from "@/components/ui/spinner";
import { useAuth } from "@/contexts/auth-context";

function passwordByteLength(password: string) {
  return new TextEncoder().encode(password).length;
}

export function AccountPage() {
  const { user, updateUsername, changePassword, deleteAccount } = useAuth();
  const navigate = useNavigate();

  const [username, setUsername] = useState(user?.username ?? "");
  const [usernameLoading, setUsernameLoading] = useState(false);
  const [usernameError, setUsernameError] = useState("");
  const [usernameNotice, setUsernameNotice] = useState("");

  const [currentPassword, setCurrentPassword] = useState("");
  const [newPassword, setNewPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [passwordLoading, setPasswordLoading] = useState(false);
  const [passwordError, setPasswordError] = useState("");

  const [deleteOpen, setDeleteOpen] = useState(false);
  const [deletePassword, setDeletePassword] = useState("");
  const [deleteLoading, setDeleteLoading] = useState(false);
  const [deleteError, setDeleteError] = useState("");

  async function handleUsernameSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setUsernameError("");
    setUsernameNotice("");

    const cleanUsername = username.trim().toLowerCase();
    if (cleanUsername.length < 3 || cleanUsername.length > 30) {
      setUsernameError("Username must be between 3 and 30 characters.");
      return;
    }
    if (!/^[a-z0-9_]+$/.test(cleanUsername)) {
      setUsernameError("Username may contain only letters, numbers, and underscores.");
      return;
    }
    if (cleanUsername === user?.username) {
      setUsernameError("Enter a different username.");
      return;
    }

    setUsernameLoading(true);
    try {
      await updateUsername(cleanUsername);
      setUsername(cleanUsername);
      setUsernameNotice("Username updated successfully.");
    } catch (caught) {
      setUsernameError(caught instanceof Error ? caught.message : "Could not update your username.");
    } finally {
      setUsernameLoading(false);
    }
  }

  async function handlePasswordSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setPasswordError("");

    if (!currentPassword) {
      setPasswordError("Enter your current password.");
      return;
    }
    if (newPassword.length < 8) {
      setPasswordError("New password must be at least 8 characters.");
      return;
    }
    if (passwordByteLength(newPassword) > 30) {
      setPasswordError("New password must be at most 30 bytes.");
      return;
    }
    if (currentPassword === newPassword) {
      setPasswordError("New password must be different from your current password.");
      return;
    }
    if (newPassword !== confirmPassword) {
      setPasswordError("New passwords do not match.");
      return;
    }

    setPasswordLoading(true);
    try {
      await changePassword(currentPassword, newPassword);
      navigate("/login", {
        replace: true,
        state: { notice: "Password updated. Sign in again." },
      });
    } catch (caught) {
      setPasswordError(caught instanceof Error ? caught.message : "Could not update your password.");
      setPasswordLoading(false);
    }
  }

  function openDeleteDialog() {
    setDeletePassword("");
    setDeleteError("");
    setDeleteOpen(true);
  }

  function closeDeleteDialog() {
    if (!deleteLoading) setDeleteOpen(false);
  }

  async function handleDelete(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setDeleteError("");
    if (!deletePassword) {
      setDeleteError("Enter your current password.");
      return;
    }

    setDeleteLoading(true);
    try {
      await deleteAccount(deletePassword);
      navigate("/login", {
        replace: true,
        state: { notice: "Your account has been deleted." },
      });
    } catch (caught) {
      setDeleteError(caught instanceof Error ? caught.message : "Could not delete your account.");
      setDeleteLoading(false);
    }
  }

  return (
    <section className="flex-1 px-4 py-8 sm:px-6 sm:py-10 lg:px-8">
      <PageTitle title="Account settings" />
      <motion.div
        initial={{ opacity: 0, y: 12 }}
        animate={{ opacity: 1, y: 0 }}
        className="mx-auto w-full max-w-3xl"
      >
        <div className="mb-8">
          <p className="mb-2 text-sm text-muted-foreground">Signed in as {user?.username}</p>
          <h1 className="text-3xl font-semibold tracking-tight sm:text-4xl">Account settings</h1>
          <p className="mt-2 text-muted-foreground">Manage your profile, password, and account.</p>
        </div>

        <div className="space-y-5">
          <Card>
            <CardHeader>
              <div className="mb-2 flex size-10 items-center justify-center rounded-lg bg-secondary">
                <UserRound className="size-5" aria-hidden="true" />
              </div>
              <CardTitle>Username</CardTitle>
              <CardDescription>Change the name shown throughout the application.</CardDescription>
            </CardHeader>
            <CardContent>
              <form onSubmit={handleUsernameSubmit} className="space-y-4" noValidate>
                {usernameNotice && <Alert variant="success">{usernameNotice}</Alert>}
                {usernameError && <Alert>{usernameError}</Alert>}
                <div className="space-y-2">
                  <Label htmlFor="account-username">Username</Label>
                  <Input
                    id="account-username"
                    name="username"
                    autoComplete="username"
                    value={username}
                    onChange={(event) => setUsername(event.target.value)}
                    required
                    minLength={3}
                    maxLength={30}
                    disabled={usernameLoading}
                  />
                  <p className="text-xs text-muted-foreground">Use 3–30 letters, numbers, or underscores.</p>
                </div>
                <Button type="submit" disabled={usernameLoading}>
                  {usernameLoading ? <><Spinner />Saving...</> : <><Save className="size-4" />Save username</>}
                </Button>
              </form>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <div className="mb-2 flex size-10 items-center justify-center rounded-lg bg-secondary">
                <KeyRound className="size-5" aria-hidden="true" />
              </div>
              <CardTitle>Password</CardTitle>
              <CardDescription>Changing your password signs you out on every device.</CardDescription>
            </CardHeader>
            <CardContent>
              <form onSubmit={handlePasswordSubmit} className="space-y-4" noValidate>
                {passwordError && <Alert>{passwordError}</Alert>}
                <div className="space-y-2">
                  <Label htmlFor="current-password">Current password</Label>
                  <PasswordInput id="current-password" name="current-password" autoComplete="current-password" value={currentPassword} onChange={(event) => setCurrentPassword(event.target.value)} required disabled={passwordLoading} />
                </div>
                <div className="grid gap-4 sm:grid-cols-2">
                  <div className="space-y-2">
                    <Label htmlFor="new-password">New password</Label>
                    <PasswordInput id="new-password" name="new-password" autoComplete="new-password" value={newPassword} onChange={(event) => setNewPassword(event.target.value)} required minLength={8} disabled={passwordLoading} />
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="confirm-new-password">Confirm new password</Label>
                    <PasswordInput id="confirm-new-password" name="confirm-new-password" autoComplete="new-password" value={confirmPassword} onChange={(event) => setConfirmPassword(event.target.value)} required minLength={8} disabled={passwordLoading} />
                  </div>
                </div>
                <Button type="submit" disabled={passwordLoading}>
                  {passwordLoading ? <><Spinner />Updating...</> : <><KeyRound className="size-4" />Update password</>}
                </Button>
              </form>
            </CardContent>
          </Card>

          <Card className="border-destructive/40">
            <CardHeader>
              <CardTitle className="text-destructive">Danger zone</CardTitle>
              <CardDescription>Deleting your account permanently removes your bookmarks and cannot be undone.</CardDescription>
            </CardHeader>
            <CardContent>
              <Button type="button" variant="destructive" onClick={openDeleteDialog}>
                <Trash2 className="size-4" />Delete account
              </Button>
            </CardContent>
          </Card>
        </div>
      </motion.div>

      <Dialog
        open={deleteOpen}
        onClose={closeDeleteDialog}
        title="Delete your account?"
        description="Your account and all saved bookmarks will be permanently deleted."
        className="max-w-md"
      >
        <form onSubmit={handleDelete} className="space-y-4" noValidate>
          {deleteError && <Alert>{deleteError}</Alert>}
          <div className="space-y-2">
            <Label htmlFor="delete-password">Current password</Label>
            <PasswordInput id="delete-password" name="delete-password" autoComplete="current-password" value={deletePassword} onChange={(event) => setDeletePassword(event.target.value)} required disabled={deleteLoading} autoFocus />
            <p className="text-xs text-muted-foreground">Enter your password to confirm this permanent action.</p>
          </div>
          <div className="flex flex-col-reverse gap-2 pt-2 sm:flex-row sm:justify-end">
            <Button type="button" variant="outline" onClick={closeDeleteDialog} disabled={deleteLoading}>Cancel</Button>
            <Button type="submit" variant="destructive" disabled={deleteLoading}>
              {deleteLoading ? <><Spinner />Deleting...</> : <><Trash2 className="size-4" />Delete permanently</>}
            </Button>
          </div>
        </form>
      </Dialog>
    </section>
  );
}
