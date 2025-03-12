import React, { useRef, useEffect, useState } from "react";
import { DbConnection, User, EventContext } from "../module_bindings";

const FIELD_WIDTH = 800;
const FIELD_HEIGHT = 600;
const PLAYER_RADIUS = 20;

type Props = {
  conn: DbConnection;
  myIdentity: User["identity"];
};

const GameField: React.FC<Props> = ({ conn, myIdentity }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [users, setUsers] = useState<Map<string, User>>(new Map());

  // Subscribe to user table updates.
  useEffect(() => {
    if (!conn) return;
    const onInsert = (_ctx: EventContext, user: User) => {
      setUsers((prev) => new Map(prev.set(user.identity.toHexString(), user)));
    };
    const onUpdate = (_ctx: EventContext, oldUser: User, newUser: User) => {
      setUsers((prev) => {
        prev.delete(oldUser.identity.toHexString());
        return new Map(prev.set(newUser.identity.toHexString(), newUser));
      });
    };
    const onDelete = (_ctx: EventContext, user: User) => {
      setUsers((prev) => {
        prev.delete(user.identity.toHexString());
        return new Map(prev);
      });
    };

    conn.db.user.onInsert(onInsert);
    conn.db.user.onUpdate(onUpdate);
    conn.db.user.onDelete(onDelete);

    return () => {
      conn.db.user.removeOnInsert(onInsert);
      conn.db.user.removeOnUpdate(onUpdate);
      conn.db.user.removeOnDelete(onDelete);
    };
  }, [conn]);

  // Render users on the canvas.
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // Clear the canvas.
    ctx.clearRect(0, 0, FIELD_WIDTH, FIELD_HEIGHT);

    // Draw the game field border.
    ctx.strokeStyle = "#000";
    ctx.strokeRect(0, 0, FIELD_WIDTH, FIELD_HEIGHT);
    // Draw each user as a circle.
    users.forEach((user) => {
      ctx.beginPath();
      ctx.arc(user.posX, user.posY, PLAYER_RADIUS, 0, Math.PI * 2);
      // Color self differently.
      if (user.identity.toHexString() === myIdentity.toHexString()) {
        ctx.fillStyle = "#3498db"; // Blue.
      } else {
        ctx.fillStyle = "#e74c3c"; // Red.
      }
      ctx.fill();
      ctx.stroke();
    });
  }, [users, myIdentity]);

  // Handle keyboard input for movement.
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      let deltaX = 0;
      let deltaY = 0;
      const speed = 5; // Adjust speed as needed.

      if (e.key === "ArrowUp") {
        deltaY = -speed;
      } else if (e.key === "ArrowDown") {
        deltaY = speed;
      } else if (e.key === "ArrowLeft") {
        deltaX = -speed;
      } else if (e.key === "ArrowRight") {
        deltaX = speed;
      } else {
        return;
      }

      // Call the move_user reducer.
      conn.reducers.moveUser(deltaX, deltaY);
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [conn]);

  return <canvas ref={canvasRef} width={FIELD_WIDTH} height={FIELD_HEIGHT} />;
};

export default GameField;
