import React, { useMemo, useState } from "react";
import { useSpacetime } from "../../providers/spacetime-context";
import clsx from "clsx";
import { SpotState } from "../../lib/types";

type Props = {
  gameId: bigint;
};

const GameBoard: React.FC<Props> = ({ gameId }) => {
  // Get connection and currentUser from the provider.
  const { conn, identity: currentUser, getUserName, games } = useSpacetime();

  const game = useMemo(
    () => games.find((game) => game.id === gameId),
    [games, gameId]
  );

  // Subscribe to the specific game.
  const [selectedCell, setSelectedCell] = useState<{
    x: number;
    y: number;
  } | null>(null);

  if (!game) {
    return <div>Loading...</div>;
  }

  const { board, boardSize, playerBlack, playerWhite, gameOver, turn } = game;

  const joinGame = async (gameId: bigint) => {
    conn?.reducers.joinGame(gameId);
  };

  let parsedBoard: SpotState[] = [];
  try {
    parsedBoard = JSON.parse(board);
    console.log({ parsedBoard });
  } catch (e) {
    console.error("Error parsing board:", e);
  }

  // based on the current turn - get the corresponding player
  const currentTurnPlayer = gameOver
    ? undefined
    : turn === "B"
      ? playerBlack
      : playerWhite;

  // Handle intersection (cell) selection. Only allow selecting empty intersections when it's the user's turn.
  const handleIntersectionClick = (x: number, y: number) => {
    const idx = y * boardSize + x;
    const cell = parsedBoard[idx];
    if (!isPlayersTurn || cell.occupant !== "Empty") return;
    setSelectedCell({ x, y });
  };

  // Confirm the move and send it to the backend.
  const handleConfirmMove = () => {
    if (selectedCell) {
      conn?.reducers.placeStone(gameId, selectedCell.x, selectedCell.y);
      setSelectedCell(null);
    }
  };

  const isPlayersTurn =
    currentTurnPlayer &&
    playerWhite &&
    currentUser &&
    getUserName(currentTurnPlayer) === getUserName(currentUser);

  const onPass = () => {
    conn?.reducers.passMove(gameId);
  };

  const playerBlacksTurn =
    currentTurnPlayer &&
    playerWhite &&
    getUserName(currentTurnPlayer) === getUserName(playerBlack);
  const playerWhitesTurn =
    currentTurnPlayer &&
    playerWhite &&
    getUserName(currentTurnPlayer) === getUserName(playerWhite);

  return (
    <div className="space-y-4 w-full max-w-xl mx-auto">
      <div className="flex justify-between w-full">
        <div
          className={`flex flex-col items-center gap-1.5   ${
            playerBlacksTurn ? "font-bold text-primary" : "text-neutral"
          }`}
        >
          <div className={`badge badge-lg badge-neutral`}>Black</div>
          <div className="text-sm">{getUserName(playerBlack)}</div>
          {playerBlacksTurn && (
            <span className="badge badge-primary">
              {isPlayersTurn ? "Your" : `Opponent's`} Turn
            </span>
          )}
        </div>

        <div
          className={`flex flex-col items-center gap-1.5 ${
            playerWhitesTurn ? "font-bold text-primary" : "text-neutral"
          }`}
        >
          <div className={`badge badge-lg badge-secondary`}>White</div>
          <div className="text-sm">
            {playerWhite ? getUserName(playerWhite) : "Waiting..."}
          </div>
          {playerWhitesTurn && (
            <span className="badge badge-primary">
              {isPlayersTurn ? "Your" : `Opponent's`} Turn
            </span>
          )}
        </div>
      </div>

      <div className="relative aspect-square w-full">
        <div
          className="grid absolute inset-0"
          style={{
            gridTemplateColumns: `repeat(${boardSize}, 1fr)`,
            gridTemplateRows: `repeat(${boardSize}, 1fr)`,
          }}
        >
          {parsedBoard.map((cell, idx) => {
            const x = idx % boardSize;
            const y = Math.floor(idx / boardSize);
            const isSelected = selectedCell?.x === x && selectedCell?.y === y;
            const isPlayable = isPlayersTurn && cell.playable;

            let content;
            if (cell.occupant === "Black") {
              content = <div className="w-4 h-4 bg-black rounded-full" />;
            } else if (cell.occupant === "White") {
              content = (
                <div className="w-4 h-4 bg-white rounded-full border border-gray-800" />
              );
            } else {
              content = (
                <div className="w-[2px] h-[2px] bg-gray-500 rounded-full" />
              );
            }

            return (
              <div
                key={`${x}-${y}`}
                onClick={() => isPlayable && handleIntersectionClick(x, y)}
                className={clsx(
                  "flex items-center justify-center aspect-square min-w-0 min-h-0",
                  {
                    "cursor-pointer hover:bg-green-200": isPlayable,
                    "cursor-not-allowed": !isPlayable,
                    "bg-green-400": isSelected,
                  }
                )}
              >
                {content}
              </div>
            );
          })}
        </div>
      </div>
      {!playerWhite ? (
        <button
          onClick={() => joinGame(game.id)}
          className={clsx("btn btn-primary w-full", {
            "btn-disabled":
              !currentUser ||
              getUserName(playerBlack) === getUserName(currentUser),
          })}
        >
          Join
        </button>
      ) : game.gameOver ? (
        <div className=" p-4 border rounded bg-secondary text-secondary-content">
          <h2 className="text-xl font-bold">Game Over</h2>
          <p>
            Final Score: Black: {game.finalScoreBlack || 0} - White:{" "}
            {game.finalScoreWhite || 0}
          </p>
        </div>
      ) : (
        <div className="flex gap-4 justify-between">
          <button
            className={clsx("btn btn-secondary", {
              "btn-disabled": !isPlayersTurn,
            })}
            onClick={onPass}
          >
            Pass
          </button>
          <button
            onClick={handleConfirmMove}
            className={clsx("btn btn-primary", {
              "btn-disabled": !selectedCell,
            })}
          >
            Confirm Move
          </button>
        </div>
      )}
    </div>
  );
};

export default GameBoard;
