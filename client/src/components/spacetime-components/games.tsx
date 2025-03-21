import { Link } from "@tanstack/react-router";
import { useSpacetime } from "../..//providers/spacetime-context";

const Games = () => {
  const { games, getUserName } = useSpacetime();

  const onSelect = (gameId: bigint) => {
    console.log("Selected game ID:", gameId);
  };
  const onCreateGame = () => {
    console.log("Create game button clicked");
  };

  return (
    <div className="grid gap-4">
      {games.length === 0 && (
        <div className="text-base-content/70">
          <p>No games yet. Click "Create Game" to start your first match!</p>
        </div>
      )}

      {games.map((game) => (
        <Link
          to="/game/$gameId"
          params={{ gameId: game.id.toString() }}
          key={game.id.toString()}
          className="card bg-base-100 shadow hover:shadow-lg transition cursor-pointer"
        >
          <div
            key={game.id.toString()}
            className="card bg-base-100 shadow hover:shadow-lg transition cursor-pointer"
            onClick={() => onSelect?.(game.id)}
          >
            <div className="card-body">
              <h2 className="card-title">
                {getUserName(game.playerBlack)} vs{" "}
                {game.playerWhite
                  ? getUserName(game.playerWhite)
                  : "Waiting..."}
              </h2>
              <p>
                Board size: {game.boardSize}x{game.boardSize}
              </p>
              <p>Turn: {game.turn}</p>
              <p>Passes: {game.passes}</p>
              {game.gameOver ? (
                <p className="text-success font-bold">
                  Game Over – Score: {game.finalScoreBlack?.toString() ?? 0}{" "}
                  (Black) / {game.finalScoreWhite?.toString() ?? 0} (White)
                </p>
              ) : (
                <p className="text-info">In Progress</p>
              )}
            </div>
          </div>
        </Link>
      ))}
    </div>
  );
};

export default Games;
