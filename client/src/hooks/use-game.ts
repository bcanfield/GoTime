import { useState, useEffect } from "react";
import { DbConnection, EventContext, Game } from "../module_bindings";

export function useGame(conn: DbConnection | null, gameId: bigint) {
  const [game, setGame] = useState<Game | undefined>(undefined);

  useEffect(() => {
    if (!conn) return;

    // Build a query to subscribe to updates for the specific game.
    // The exact query format might vary depending on your backend.
    const query = `SELECT * FROM game WHERE id = ${gameId}`;

    // Subscribe to the game query.
    conn
      .subscriptionBuilder()
      .onApplied(() => {
        console.log(`Subscription for game ${gameId} applied.`);
      })
      .onError((err) => {
        console.error("Error subscribing to game:", err);
      })
      .subscribe(query);

    // Handler to update game state on insert or update.
    const handleGameEvent = (_ctx: EventContext, newGame: Game) => {
      console.log("Game event:", newGame);
      if (newGame.id === gameId) {
        setGame(newGame);
      }
    };

    // Handler to clear game state when the game is deleted.
    const handleGameDelete = (_ctx: EventContext, deletedGame: Game) => {
      if (deletedGame.id === gameId) {
        setGame(undefined);
      }
    };

    // Listen for events on the game table.
    conn.db.game.onInsert(handleGameEvent);
    conn.db.game.onUpdate(handleGameEvent);
    conn.db.game.onDelete(handleGameDelete);

    // Clean up the subscriptions and event listeners on unmount.
    return () => {
      conn.db.game.removeOnInsert(handleGameEvent);
      conn.db.game.removeOnUpdate(handleGameEvent);
      conn.db.game.removeOnDelete(handleGameDelete);
    };
  }, [conn, gameId]);

  return game;
}
