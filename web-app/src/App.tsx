import { FC, useState } from 'react';
import { Universe } from 'wasm-game-of-life';
import { GameCanvas } from './GameCanvas';

export const App: FC<{}> = () => {
  const [universe, setUniverse] = useState(() => Universe.new());

  return (
    <>
      <button onClick={() => setUniverse(Universe.new())}>
        Reset Universe
      </button>
      <GameCanvas universe={universe} />
    </>
  );
};
