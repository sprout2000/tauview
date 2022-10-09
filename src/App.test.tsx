import '@testing-library/jest-dom';
import userEvent from '@testing-library/user-event';
import { render, screen, fireEvent } from '@testing-library/react';

import { randomFillSync } from 'node:crypto';
import { mockIPC, mockWindows } from '@tauri-apps/api/mocks';

import { App } from './App';

beforeAll(() => {
  window.crypto = {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    getRandomValues: (buffer: NodeJS.ArrayBufferView) => {
      return randomFillSync(buffer);
    },
  };
});

test('render App component', async () => {
  window.ResizeObserver =
    window.ResizeObserver ||
    vi.fn().mockImplementation(() => ({
      observe: vi.fn(),
      disconnect: vi.fn(),
    }));

  mockWindows('main');

  mockIPC((cmd) => {
    if (
      cmd === 'get_entries' ||
      cmd === 'open_dialog' ||
      cmd === 'move_to_trash'
    ) {
      vi.fn();
    }
  });
  const spy = vi.spyOn(window, '__TAURI_IPC__');

  const { getCurrent } = await import('@tauri-apps/api/window');
  expect(getCurrent()).toHaveProperty('label', 'main');

  render(<App />);

  await userEvent.click(screen.getByTestId('open-button'));
  expect(spy).toHaveBeenCalled();

  await userEvent.click(screen.getByTestId('prev-button'));
  await userEvent.click(screen.getByTestId('next-button'));

  await userEvent.click(screen.getByTestId('trash-button'));
  expect(spy).toHaveBeenCalled();

  fireEvent.keyDown(screen.getByTestId('container'), { key: 'J' });
});
