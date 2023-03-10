import { render } from '@testing-library/react';
import { ToolBar } from './ToolBar';

test('render ToolBar component', () => {
  const mockFn = vi.fn();
  render(
    <ToolBar
      onOpen={mockFn}
      onNext={mockFn}
      onPrev={mockFn}
      onRemove={mockFn}
      onToggleGrid={mockFn}
    />
  );
});
