import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { BookmarkForm } from "@/components/bookmarks/bookmark-form";
import { parseTags } from "@/lib/utils";

it("normalizes URLs and deduplicates tags", async () => {
  const user = userEvent.setup();
  const onSubmit = vi.fn().mockResolvedValue(undefined);
  render(<BookmarkForm loading={false} onSubmit={onSubmit} onCancel={vi.fn()} />);
  await user.type(screen.getByLabelText("Title"), "React docs");
  await user.type(screen.getByLabelText("URL"), "react.dev");
  await user.type(screen.getByLabelText("Tags"), "docs, react, docs");
  await user.click(screen.getByRole("button", { name: "Save URL" }));
  expect(onSubmit).toHaveBeenCalledWith({ title: "React docs", url: "https://react.dev", tags: ["docs", "react"] });
  expect(parseTags("a, a, b")).toEqual(["a", "b"]);
});

it("reports an invalid bookmark URL", async () => {
  const user = userEvent.setup();
  render(<BookmarkForm loading={false} onSubmit={vi.fn()} onCancel={vi.fn()} />);
  await user.type(screen.getByLabelText("Title"), "Broken");
  await user.type(screen.getByLabelText("URL"), "://");
  await user.click(screen.getByRole("button", { name: "Save URL" }));
  expect(screen.getByRole("alert")).toHaveTextContent("Enter a valid URL");
});
