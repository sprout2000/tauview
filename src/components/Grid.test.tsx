import { render } from '@testing-library/react';
import { Grid } from './Grid';

test('render Grid component', () => {
  const mockFn = vi.fn();
  render(
    <Grid url="" imgList={[]} onClickBlank={mockFn} onClickThumb={mockFn} />
  );
});
