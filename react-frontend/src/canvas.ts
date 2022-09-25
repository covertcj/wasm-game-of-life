import { Cell, Universe } from 'game_of_life';
import { memory } from 'game_of_life/game_of_life_bg.wasm';

// TODO: this should all be moveable to a rust webgpu library, bevy maybe?

const CELL_BORDER_SIZE = 1;
const CELL_SIZE = 10;
const GRID_COLOR = '#888';
const DEAD_COLOR = '#FFF';
const ALIVE_COLOR = '#000';

const getCanvasElement = () => {
  const canvases = document.getElementsByTagName('canvas');
  for (let i = 0; i < canvases.length; i++) {
    const canvas = canvases[i];
    if (canvas.id === 'game-of-life-canvas') {
      return canvas;
    }
  }

  return null;
};

export interface GameOfLifeRenderer {
  render(universe: Universe): void;
}

const drawGrid = (
  ctx: CanvasRenderingContext2D,
  width: number,
  height: number,
): void => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // draw vertical lines
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + CELL_BORDER_SIZE) + CELL_BORDER_SIZE, 0);
    ctx.lineTo(
      i * (CELL_SIZE + CELL_BORDER_SIZE) + CELL_BORDER_SIZE,
      (CELL_SIZE + CELL_BORDER_SIZE) * height + CELL_BORDER_SIZE,
    );
  }

  // draw horizontal lines
  for (let i = 0; i <= height; i++) {
    ctx.moveTo(0, i * (CELL_SIZE + CELL_BORDER_SIZE) + CELL_BORDER_SIZE);
    ctx.lineTo(
      (CELL_SIZE + CELL_BORDER_SIZE) * width + CELL_BORDER_SIZE,
      i * (CELL_SIZE + CELL_BORDER_SIZE) + CELL_BORDER_SIZE,
    );
  }

  ctx.stroke();
};

const drawCells = (ctx: CanvasRenderingContext2D, universe: Universe): void => {
  const width = universe.width();
  const height = universe.height();

  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const idx = y * width + x;

      ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;
      ctx.fillRect(
        x * (CELL_SIZE + CELL_BORDER_SIZE) + CELL_BORDER_SIZE,
        y * (CELL_SIZE + CELL_BORDER_SIZE) + CELL_BORDER_SIZE,
        CELL_SIZE,
        CELL_SIZE,
      );
    }
  }

  ctx.stroke();
};

export const createRenderer = (
  canvas: HTMLCanvasElement,
  width: number,
  height: number,
): GameOfLifeRenderer => {
  canvas.width = width * (CELL_SIZE + CELL_BORDER_SIZE) + CELL_BORDER_SIZE;
  canvas.height = height * (CELL_SIZE + CELL_BORDER_SIZE) + CELL_BORDER_SIZE;

  const ctx = canvas.getContext('2d');
  if (!ctx) {
    throw Error("Couldn't get canvas rendering context");
  }

  return {
    render: (universe: Universe) => {
      drawGrid(ctx, width, height);
      drawCells(ctx, universe);
    },
  };
};
