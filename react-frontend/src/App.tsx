import { FC, useState } from 'react';
import { Universe } from 'game_of_life';
import { GameCanvas } from './GameCanvas';

export const App: FC<{}> = () => {
  const [universe, setUniverse] = useState(() => Universe.new());
  const [paused, setPaused] = useState(false);

  return (
    <>
      <div>
        <button onClick={() => setUniverse(Universe.new())}>
          Reset Universe
        </button>
        <button onClick={() => setPaused(!paused)}>
          {paused ? 'Resume' : 'Pause'}
        </button>
      </div>
      <GameCanvas universe={universe} paused={paused} />
    </>
  );
};
