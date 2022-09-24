import { Universe, initialize } from 'wasm-game-of-life';

initialize();

const canvas = document.getElementById('game-of-life-canvas');
if (!canvas) {
  throw new Error("Couldn't get element to render the app into");
}

const universe = Universe.new();

const renderLoop = () => {
  canvas.textContent = universe.render();
  universe.tick();

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
