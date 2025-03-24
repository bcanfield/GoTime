import React, { useMemo, useState } from "react";
import { Occupant } from "../../App";
import { useSpacetime } from "../../providers/spacetime-context";
import clsx from "clsx";

// todo: import this from the module bindings
export type SpotState = {
  occupant: Occupant;
  move_number: number | null;
  marker: string | null;
};

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

  // Render board rows. Each cell contains an intersection point.
  const rows = [];
  for (let y = 0; y < boardSize; y++) {
    const cells = [];
    for (let x = 0; x < boardSize; x++) {
      const idx = y * boardSize + x;
      const cell: SpotState = parsedBoard[idx];
      // Determine what to display:
      // If a stone is placed, render it; otherwise, render a small dot.
      let content;
      console.log({ cell });
      if (cell.occupant === "Black") {
        content = <span className="text-black text-xl">●</span>;
      } else if (cell.occupant === "White") {
        content = <span className="text-white text-xl">○</span>;
      } else {
        // A small gray dot to denote an intersection.
        content = <span className="block w-1 h-1 bg-gray-500 rounded-full" />;
      }

      // Check if this intersection is selectable and/or selected.
      const selectable = isPlayersTurn && cell.occupant === "Empty";
      const isSelected =
        selectedCell && selectedCell.x === x && selectedCell.y === y;

      cells.push(
        <td key={x} className="w-10 h-10 relative">
          {/* The intersection element */}
          <div
            onClick={() => handleIntersectionClick(x, y)}
            className={clsx(
              "absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 flex items-center justify-center rounded-full",
              "w-6 h-6",
              {
                "cursor-pointer hover:bg-green-200": selectable,
                "cursor-not-allowed": !selectable,
                "bg-green-400": isSelected,
              }
            )}
          >
            {content}
          </div>
        </td>
      );
    }
    rows.push(<tr key={y}>{cells}</tr>);
  }

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
    <div className="max-w-md space-y-4">
      <div className="flex justify-between w-full">
        <div
          className={`flex flex-col items-center gap-1.5   ${
            playerBlacksTurn ? "font-bold text-primary" : "text-neutral"
          }`}
        >
          <div className={`badge badge-lg badge-neutral`}>Black</div>
          <div className="text-sm">{getUserName(playerBlack)}</div>
          {playerBlacksTurn && (
            <span className="badge badge-primary">Your Turn</span>
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
            <span className="badge badge-primary">Your Turn</span>
          )}
        </div>
      </div>

      <div className="flex items-center w-full justify-center">
        <table className="border-collapse">
          <tbody>{rows}</tbody>
        </table>
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
