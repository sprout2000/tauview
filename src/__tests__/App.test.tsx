/* eslint-disable @typescript-eslint/ban-ts-comment */
import { render } from '@testing-library/react';
import { App } from '../App';

import { randomFillSync } from 'crypto';

import { mockWindows, mockIPC } from '@tauri-apps/api/mocks';

beforeAll(() => {
  //@ts-ignore
  window.crypto = {
    // @ts-ignore
    getRandomValues: function (buffer) {
      // @ts-ignore
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

  mockIPC((cmd, args) => {
    if (cmd === 'add') {
      return (args.a as number) + (args.b as number);
    }
  });
  mockWindows('main');

  // const spy = vi.spyOn(window, '__TAURI_IPC__');
  const { getCurrent } = await import('@tauri-apps/api/window');
  expect(getCurrent()).toHaveProperty('label', 'main');

  render(<App />);
});
