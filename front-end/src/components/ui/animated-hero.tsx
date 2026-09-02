import { useEffect, useMemo, useState } from "react";
import { motion } from "framer-motion";
import { MoveRight } from "lucide-react";
import { Link } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { useAuth } from "@/contexts/auth-context";

function Hero() {
  const { isAuthenticated } = useAuth();
  const [titleNumber, setTitleNumber] = useState(0);
  const titles = useMemo(
    () => ["organized", "searchable", "private", "within reach", "yours"],
    [],
  );

  useEffect(() => {
    const timeoutId = window.setTimeout(() => {
      setTitleNumber((current) => (current === titles.length - 1 ? 0 : current + 1));
    }, 2000);

    return () => window.clearTimeout(timeoutId);
  }, [titleNumber, titles]);

  return (
    <section className="relative isolate flex min-h-[calc(100dvh-4rem)] w-full items-center overflow-hidden border-b border-border px-4 py-20 sm:px-6 lg:px-8">
      <div className="grid-fade absolute inset-0 -z-20" />
      <div className="absolute left-1/2 top-16 -z-10 h-72 w-72 -translate-x-1/2 rounded-full bg-foreground/[0.04] blur-3xl sm:h-[28rem] sm:w-[28rem]" />
      <div className="container mx-auto">
        <div className="flex flex-col items-center justify-center gap-8 text-center">
          <div className="flex max-w-3xl flex-col gap-5">
            <h1 className="slide-in-blurred-top text-balance text-5xl font-semibold tracking-[-0.045em] sm:text-6xl md:text-7xl">
              <span>Your links, always</span>
              <span className="relative flex h-[1.25em] w-full justify-center overflow-hidden pt-1 text-muted-foreground">
                {titles.map((title, index) => (
                  <motion.span
                    key={title}
                    className="absolute font-semibold"
                    initial={{ opacity: 0, y: -100 }}
                    transition={{ type: "spring", stiffness: 50 }}
                    animate={
                      titleNumber === index
                        ? { y: 0, opacity: 1 }
                        : { y: titleNumber > index ? -150 : 150, opacity: 0 }
                    }
                  >
                    {title}
                  </motion.span>
                ))}
              </span>
            </h1>
            <p className="mx-auto max-w-2xl text-balance text-lg leading-relaxed tracking-tight text-muted-foreground md:text-xl">
              Save what matters, organize it your way, and find it again in
              seconds. Your corner of the web, without the clutter.
            </p>
          </div>
          <div className="flex w-full max-w-sm flex-col gap-3 sm:w-auto sm:max-w-none sm:flex-row">
            {isAuthenticated ? (
              <Button size="lg" className="gap-4" asChild><Link to="/app">View my bookmarks <MoveRight className="size-4" /></Link></Button>
            ) : (
              <>
                <Button size="lg" className="gap-4" asChild><Link to="/register">Get started <MoveRight className="size-4" /></Link></Button>
                <Button size="lg" variant="outline" asChild><Link to="/login">I have an account</Link></Button>
              </>
            )}
          </div>
        </div>
      </div>
    </section>
  );
}

export { Hero };
