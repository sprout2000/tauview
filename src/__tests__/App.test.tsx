import '@testing-library/jest-dom';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import { randomFillSync } from 'crypto';
import { mockWindows, mockIPC } from '@tauri-apps/api/mocks';

import { App } from '../App';

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
    jest.fn().mockImplementation(() => ({
      disconnect: jest.fn(),
      observe: jest.fn(),
    }));

  mockWindows('main');

  mockIPC((cmd) => {
    if (cmd === 'open_dialog') {
      jest.fn();
    }
  });
  const spy = jest.spyOn(window, '__TAURI_IPC__');

  const { getCurrent } = await import('@tauri-apps/api/window');
  expect(getCurrent()).toHaveProperty('label', 'main');

  render(<App />);

  await userEvent.click(screen.getByTestId('open-button'));
  expect(spy).toHaveBeenCalled();
});
