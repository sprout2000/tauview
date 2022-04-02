import React from 'react';
import { createRoot } from 'react-dom/client';

import { setLocales } from './setLocales';

import { App } from './components/App';
import './index.scss';

const locale =
  (window.navigator.languages && window.navigator.languages[0]) ||
  window.navigator.language;

setLocales(locale);

const container = document.getElementById('root');
const root = container && createRoot(container);

root?.render(<App />);
