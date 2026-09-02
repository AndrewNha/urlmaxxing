import { useState } from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { Dialog } from "@/components/ui/dialog";

function DialogHarness() {
  const [open, setOpen] = useState(false);
  return <><button onClick={() => setOpen(true)}>Open editor</button><Dialog open={open} onClose={() => setOpen(false)} title="Editor" description="Edit an item"><input aria-label="Title" autoFocus /><button>Save</button></Dialog></>;
}

it("moves, traps, and restores focus and closes with Escape", async () => {
  const user = userEvent.setup();
  render(<DialogHarness />);
  const opener = screen.getByRole("button", { name: "Open editor" });
  await user.click(opener);
  await waitFor(() => expect(screen.getByLabelText("Title")).toHaveFocus());
  expect(screen.getByRole("dialog")).toHaveAccessibleName("Editor");
  await user.keyboard("{Escape}");
  await waitFor(() => expect(screen.queryByRole("dialog")).not.toBeInTheDocument());
  await waitFor(() => expect(opener).toHaveFocus());
});
