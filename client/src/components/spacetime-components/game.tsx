import React, { useEffect, useMemo, useState } from "react";
import { Occupant } from "../../App";
import { useSpacetime } from "../../providers/spacetime-context";
import { useGame } from "../../hooks/use-game";
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

  const game = useMemo(() => {
    console.log("Games:", games);
    const foundGame = games.find((game) => game.id === gameId);
    console.log("Found game:", foundGame);
    return foundGame;
  }, [games, gameId]);

  // Subscribe to the specific game.
  const gameSubscription = useGame(conn, gameId);
  const [selectedCell, setSelectedCell] = useState<{
    x: number;
    y: number;
  } | null>(null);

  useEffect(() => {
    console.log({ gameSubscription });
  }, [gameSubscription]);

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

  // Determine if it's the current user's turn.
  console.log({ turn, playerBlack, playerWhite, currentUser });
  const isPlayersTurn =
    currentUser &&
    !gameOver &&
    ((turn === "B" && getUserName(currentUser) === getUserName(playerBlack)) ||
      (playerWhite &&
        turn === "W" &&
        getUserName(currentUser) === getUserName(playerWhite)));

  // Handle cell selection. Only allow selecting empty cells when it's the user's turn.
  const handleCellClick = (x: number, y: number) => {
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

  const handleCancelSelection = () => {
    setSelectedCell(null);
  };

  // Render board rows. Each cell represents an intersection.
  const rows = [];
  for (let y = 0; y < boardSize; y++) {
    const cells = [];
    for (let x = 0; x < boardSize; x++) {
      const idx = y * boardSize + x;
      const cell: SpotState = parsedBoard[idx];
      let display = "";
      if (cell.occupant === "Black") display = "●";
      else if (cell.occupant === "White") display = "○";

      // Allow interaction only if the cell is empty and it is the player's turn.
      const selectable = isPlayersTurn && cell.occupant === "Empty";
      // Highlight the cell if it's selected.
      const isSelected =
        selectedCell && selectedCell.x === x && selectedCell.y === y;

      cells.push(
        <td
          key={x}
          onClick={() => handleCellClick(x, y)}
          className={`w-10 h-10 border border-gray-400 text-center cursor-pointer 
                      ${selectable ? "hover:bg-green-200" : "cursor-not-allowed"} 
                      ${isSelected ? "bg-green-400" : ""}`}
        >
          {display}
        </td>
      );
    }
    rows.push(<tr key={y}>{cells}</tr>);
  }

  const onPass = () => {
    conn?.reducers.passMove(gameId);
  };

  console.log({ playerBlack, currentUser });

  return (
    <div>
      <div className="mb-4">
        <p>
          <strong>Player Black:</strong> {getUserName(playerBlack)}
        </p>
        <p>
          <strong>Player White:</strong>{" "}
          {playerWhite ? getUserName(playerWhite) : "Waiting..."}
        </p>
        <p>
          <strong>Current Turn:</strong> {turn}
        </p>
        {!playerWhite && (
          <button
            onClick={() => joinGame(game.id)}
            className={clsx("btn btn-primary", {
              "btn-disabled":
                !currentUser ||
                getUserName(playerBlack) === getUserName(currentUser),
            })}
          >
            Join
          </button>
        )}
        {!isPlayersTurn && (
          <p className="text-red-600">It&apos;s not your turn.</p>
        )}
      </div>

      <table className="border-collapse mt-4">
        <tbody>{rows}</tbody>
      </table>

      {selectedCell && (
        <div className="mt-4">
          <p>
            Confirm move at intersection ({selectedCell.x}, {selectedCell.y})?
          </p>
          <button
            onClick={handleConfirmMove}
            className="mr-2 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            Confirm
          </button>
          <button
            onClick={handleCancelSelection}
            className="px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600"
          >
            Cancel
          </button>
        </div>
      )}

      {!game.gameOver && (
        <button
          onClick={onPass}
          className={clsx("btn btn-warning", {
            "btn-disabled": !isPlayersTurn,
          })}
        >
          Pass
        </button>
      )}

      {game.gameOver && (
        <div className="mt-4 p-4 border rounded bg-secondary text-secondary-content">
          <h2 className="text-xl font-bold">Game Over</h2>
          <p>
            Final Score: Black: {game.finalScoreBlack || 0} - White:{" "}
            {game.finalScoreWhite || 0}
          </p>
        </div>
      )}
    </div>
  );
};

export default GameBoard;
