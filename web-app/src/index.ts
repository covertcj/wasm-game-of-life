import { Universe, initialize } from 'wasm-game-of-life';
import { createRenderer } from './canvas';

initialize();

const universe = Universe.new();
const renderer = createRenderer(universe.width(), universe.height());

const gameLoop = () => {
  universe.tick();

  renderer.render(universe);

  requestAnimationFrame(gameLoop);
};

requestAnimationFrame(gameLoop);
