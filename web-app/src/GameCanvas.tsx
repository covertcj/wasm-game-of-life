import { FC, useEffect, useRef, useState } from 'react';
import { Universe } from 'wasm-game-of-life';
import { createRenderer, GameOfLifeRenderer } from './canvas';

interface GameCanvasProps {
  universe: Universe;
}

export const GameCanvas: FC<GameCanvasProps> = ({ universe }) => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [renderer, setRenderer] = useState<GameOfLifeRenderer | undefined>();
  const [loopTimer, setLoopTimer] = useState<
    ReturnType<typeof requestAnimationFrame> | undefined
  >();

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) {
      setRenderer(undefined);
      return;
    }

    setRenderer(createRenderer(canvas, universe.width(), universe.height()));
  }, [universe.width(), universe.height(), canvasRef.current]);

  useEffect(() => {
    if (loopTimer) {
      cancelAnimationFrame(loopTimer);
    }

    if (!renderer) {
      return;
    }

    if (renderer) {
      const gameLoop = () => {
        // TODO: this isn't really handled well.  The ticking should be managed
        // by the owner of the universe state, which is the App component
        universe.tick();
        renderer.render(universe);

        setLoopTimer(requestAnimationFrame(gameLoop));
      };

      setLoopTimer(requestAnimationFrame(gameLoop));
    }
  }, [renderer, universe]);

  return (
    <>
      <canvas ref={canvasRef} />
    </>
  );
};
