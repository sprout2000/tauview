import { render } from '@testing-library/react';

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

  mockIPC((cmd, args) => {
    if (cmd === 'add') {
      return (args.a as number) + (args.b as number);
    }
  });
  mockWindows('main');

  const { getCurrent } = await import('@tauri-apps/api/window');
  expect(getCurrent()).toHaveProperty('label', 'main');

  render(<App />);
});
