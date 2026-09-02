import { AuthForm } from "@/components/auth/auth-form";
import { PageTitle } from "@/components/page-title";

export function LoginPage() {
  return (
    <section className="relative flex flex-1 items-center justify-center overflow-hidden px-4 py-12 sm:px-6">
      <PageTitle title="Sign in" />
      <div className="grid-fade absolute inset-0 -z-10 opacity-70" />
      <AuthForm mode="login" />
    </section>
  );
}
