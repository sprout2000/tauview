import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';

import { App } from './components/App';
import { setLocales } from './setLocales';

import './index.scss';

const locale =
  (window.navigator.languages && window.navigator.languages[0]) ||
  window.navigator.language;

setLocales(locale);

const container = document.getElementById('root');
const root = createRoot(container as Element);

root.render(
  <StrictMode>
    <App />
  </StrictMode>
);
