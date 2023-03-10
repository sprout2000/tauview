import { render } from '@testing-library/react';
import { View } from './View';

test('render View component', () => {
  window.ResizeObserver =
    window.ResizeObserver ||
    vi.fn().mockImplementation(() => ({
      disconnect: vi.fn(),
      observe: vi.fn(),
    }));

  render(<View url="" />);
});
