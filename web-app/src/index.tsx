import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';

import { Universe, initialize } from 'wasm-game-of-life';
import { App } from './App';
import { createRenderer } from './canvas';

const appElement = document.getElementById('game-of-life-app');
if (!appElement) {
  throw new Error('Failed to find the app root in the DOM');
}

const appRoot = createRoot(appElement);

appRoot.render(
  <StrictMode>
    <App />
  </StrictMode>,
);

initialize();
