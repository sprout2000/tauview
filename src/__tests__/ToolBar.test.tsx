import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";

import { ToolBar } from "../ToolBar";

test("render ToolBar component", async () => {
  const mockFn = vi.fn();
  render(
    <ToolBar
      onOpen={mockFn}
      onNext={mockFn}
      onPrev={mockFn}
      onRemove={mockFn}
      onToggleGrid={mockFn}
    />,
  );

  await userEvent.click(screen.getByLabelText("open-button"));
  expect(mockFn).toHaveBeenCalledTimes(1);
  await userEvent.click(screen.getByLabelText("grid-button"));
  expect(mockFn).toHaveBeenCalledTimes(2);
  await userEvent.click(screen.getByLabelText("prev-button"));
  expect(mockFn).toHaveBeenCalledTimes(3);
  await userEvent.click(screen.getByLabelText("next-button"));
  expect(mockFn).toHaveBeenCalledTimes(4);
  await userEvent.click(screen.getByLabelText("trash-button"));
  expect(mockFn).toHaveBeenCalledTimes(5);
});
