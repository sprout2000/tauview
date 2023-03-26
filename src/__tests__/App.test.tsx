import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";

import { randomFillSync } from "node:crypto";
import { mockIPC, mockWindows } from "@tauri-apps/api/mocks";

import { App } from "../App";

beforeAll(() => {
  window.crypto = {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    getRandomValues: (buffer: NodeJS.ArrayBufferView) => {
      return randomFillSync(buffer);
    },
  };

  window.ResizeObserver =
    window.ResizeObserver ||
    vi.fn().mockImplementation(() => ({
      disconnect: vi.fn(),
      observe: vi.fn(),
    }));
});

test("render App component", async () => {
  mockIPC((cmd) => {
    if (cmd == "get_entries") {
      return [] as string[];
    }
  });

  mockWindows("main");
  const { getCurrent } = await import("@tauri-apps/api/window");
  expect(getCurrent()).toHaveProperty("label", "main");

  render(<App />);

  const spy = vi.spyOn(window, "__TAURI_IPC__");
  await userEvent.click(screen.getByLabelText("open-button"));
  expect(spy).toHaveBeenCalledTimes(1);
});
