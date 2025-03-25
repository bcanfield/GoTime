import { Link } from "@tanstack/react-router";
import { useSpacetime } from "../..//providers/spacetime-context";
import { SpotState } from "../../lib/types";

const Games = () => {
  const { games, getUserName } = useSpacetime();

  return (
    <div className="grid gap-4">
      {games.length === 0 && (
        <div className="text-base-content/70">
          <p>No games yet. Click "Create Game" to start your first match!</p>
        </div>
      )}

      <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
        {games.map((game) => (
          <Link
            to="/game/$gameId"
            params={{ gameId: game.id.toString() }}
            key={game.id.toString()}
            className="card bg-base-100  card-border cursor-pointer hover:bg-base-200"
          >
            <div key={game.id} className="flex items-center gap-4 p-2 ">
              <MiniBoard board={game.board} boardSize={game.boardSize} />
              <div>
                <div className="text-sm font-bold">
                  Game #{game.id.toString()}
                </div>
                <div className="text-xs text-gray-500">
                  {getUserName(game.playerBlack)} vs{" "}
                  {game.playerWhite
                    ? getUserName(game.playerWhite)
                    : "Waiting for player..."}
                </div>
              </div>
            </div>
          </Link>
        ))}
      </div>
    </div>
  );
};

type MiniBoardProps = {
  board: string; // serialized JSON
  boardSize: number;
};

const MiniBoard: React.FC<MiniBoardProps> = ({ board, boardSize }) => {
  let parsedBoard: SpotState[] = [];
  try {
    parsedBoard = JSON.parse(board);
  } catch (e) {
    console.error("Failed to parse board JSON", e);
    return null;
  }

  return (
    <div className="w-24 aspect-square">
      <div
        className="grid w-full h-full"
        style={{
          gridTemplateColumns: `repeat(${boardSize}, 1fr)`,
          gridTemplateRows: `repeat(${boardSize}, 1fr)`,
        }}
      >
        {parsedBoard.map((cell, idx) => {
          let content;
          if (cell.occupant === "Black") {
            content = <div className="w-[6px] h-[6px] bg-black rounded-full" />;
          } else if (cell.occupant === "White") {
            content = (
              <div className="w-[6px] h-[6px] bg-white rounded-full border" />
            );
          } else {
            content = (
              <div className="w-[2px] h-[2px] bg-gray-500 rounded-full" />
            );
          }

          return (
            <div
              key={idx}
              className="flex items-center justify-center text-center leading-none"
            >
              {content}
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default Games;
