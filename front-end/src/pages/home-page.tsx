import { motion } from "framer-motion";
import { LockKeyhole, Sparkles, Zap } from "lucide-react";
import { PageTitle } from "@/components/page-title";
import { Hero } from "@/components/ui/animated-hero";
import { Card } from "@/components/ui/card";

const features = [
  {
    icon: Zap,
    title: "Save in seconds",
    text: "Add a title, URL, and optional tags.",
  },
  {
    icon: LockKeyhole,
    title: "Private collection",
    text: "Your saved URLs stay linked to your account.",
  },
  {
    icon: Sparkles,
    title: "Easy to find",
    text: "Search saved URLs by title, address, or tag.",
  },
];

export function HomePage() {
  return (
    <>
      <PageTitle />
      <Hero />
      <section className="px-4 py-20 sm:px-6 lg:px-8">
        <div className="mx-auto max-w-7xl">
          <div className="mx-auto mb-12 max-w-2xl text-center">
            <p className="text-sm font-medium uppercase tracking-[0.2em] text-muted-foreground">
              Built for bookmarks
            </p>
            <h2 className="mt-3 text-balance text-3xl font-semibold tracking-tight sm:text-4xl">
              Save URLs. Find them when you need them.
            </h2>
          </div>
          <div className="grid gap-4 md:grid-cols-3">
            {features.map(({ icon: Icon, title, text }, index) => (
              <motion.div
                key={title}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true, margin: "-60px" }}
                transition={{ delay: index * 0.08 }}
              >
                <Card className="h-full p-6 transition-transform duration-200 hover:-translate-y-1">
                  <span className="mb-5 flex size-11 items-center justify-center rounded-xl bg-foreground text-background">
                    <Icon className="size-5" />
                  </span>
                  <h3 className="font-semibold">{title}</h3>
                  <p className="mt-2 text-sm leading-relaxed text-muted-foreground">{text}</p>
                </Card>
              </motion.div>
            ))}
          </div>
        </div>
      </section>
    </>
  );
}
