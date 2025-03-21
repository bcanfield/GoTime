import React, { useState, useEffect } from "react";
import viteLogo from "/vite.svg";
import {
  DbConnection,
  ErrorContext,
  EventContext,
  Game,
  Message,
  User,
} from "./module_bindings";
import { Identity } from "@clockworklabs/spacetimedb-sdk";

export type PrettyMessage = {
  senderName: string;
  text: string;
};

// Define TypeScript types matching backend SpotState.
export type Occupant = "Empty" | "Black" | "White";
export type SpotState = {
  occupant: Occupant;
  move_number: number | null;
  marker: string | null;
};

function useMessages(conn: DbConnection | null): Message[] {
  const [messages, setMessages] = useState<Message[]>([]);
  useEffect(() => {
    if (!conn) return;
    const onInsert = (_ctx: EventContext, message: Message) => {
      setMessages((prev) => [...prev, message]);
    };
    conn.db.message.onInsert(onInsert);
    const onDelete = (_ctx: EventContext, message: Message) => {
      setMessages((prev) =>
        prev.filter(
          (m) =>
            m.text !== message.text &&
            m.sent !== message.sent &&
            m.sender !== message.sender
        )
      );
    };
    conn.db.message.onDelete(onDelete);
    return () => {
      conn.db.message.removeOnInsert(onInsert);
      conn.db.message.removeOnDelete(onDelete);
    };
  }, [conn]);
  return messages;
}

function useGames(conn: DbConnection | null): Game[] {
  const [games, setGames] = useState<Game[]>([]);
  useEffect(() => {
    if (!conn) return;
    const onInsert = (_ctx: EventContext, game: Game) => {
      setGames((prev) => [...prev, game]);
    };
    const onDelete = (_ctx: EventContext, game: Game) => {
      setGames((prev) => prev.filter((g) => g.id !== game.id));
    };
    const onUpdate = (_ctx: EventContext, _oldGame: Game, newGame: Game) => {
      console.log("GAME UPDATED", { newGame });
      setGames((prev) => prev.map((g) => (g.id === newGame.id ? newGame : g)));
    };
    conn.db.game.onInsert(onInsert);
    conn.db.game.onDelete(onDelete);
    conn.db.game.onUpdate(onUpdate);
    return () => {
      conn.db.game.removeOnInsert(onInsert);
      conn.db.game.removeOnDelete(onDelete);
      conn.db.game.removeOnUpdate(onUpdate);
    };
  }, [conn]);
  return games;
}

function useUsers(conn: DbConnection | null): Map<string, User> {
  const [users, setUsers] = useState<Map<string, User>>(new Map());
  useEffect(() => {
    if (!conn) return;
    const onInsert = (_ctx: EventContext, user: User) => {
      setUsers((prev) => new Map(prev.set(user.identity.toHexString(), user)));
    };
    conn.db.user.onInsert(onInsert);
    const onUpdate = (_ctx: EventContext, oldUser: User, newUser: User) => {
      setUsers((prev) => {
        prev.delete(oldUser.identity.toHexString());
        return new Map(prev.set(newUser.identity.toHexString(), newUser));
      });
    };
    conn.db.user.onUpdate(onUpdate);
    const onDelete = (_ctx: EventContext, user: User) => {
      setUsers((prev) => {
        prev.delete(user.identity.toHexString());
        return new Map(prev);
      });
    };
    conn.db.user.onDelete(onDelete);
    return () => {
      conn.db.user.removeOnInsert(onInsert);
      conn.db.user.removeOnUpdate(onUpdate);
      conn.db.user.removeOnDelete(onDelete);
    };
  }, [conn]);
  return users;
}

function App() {
  const [newName, setNewName] = useState("");
  const [settingName, setSettingName] = useState(false);
  const [systemMessage, setSystemMessage] = useState("");
  const [newMessage, setNewMessage] = useState("");
  const [connected, setConnected] = useState<boolean>(false);
  const [identity, setIdentity] = useState<Identity | null>(null);
  const [conn, setConn] = useState<DbConnection | null>(null);
  const [selectedGameId, setSelectedGameId] = useState<bigint | null>(null);

  const onCellClick = (x: number, y: number) => {
    const selectedGame = games.find((g) => g.id === selectedGameId) || null;
    if (conn && selectedGame) {
      conn.reducers.placeStone(selectedGame.id, x, y);
    }
  };

  const onPass = () => {
    const selectedGame = games.find((g) => g.id === selectedGameId) || null;
    if (conn && selectedGame) {
      conn.reducers.passMove(selectedGame.id);
    }
  };

  useEffect(() => {
    const subscribeToQueries = (conn: DbConnection, queries: string[]) => {
      for (const query of queries) {
        conn
          ?.subscriptionBuilder()
          .onApplied(() => {
            console.log("SDK client cache initialized.");
          })
          .onError((err) => {
            console.error("Error subscribing to query:", err);
          })
          .subscribe(query);
      }
    };
    const onConnect = (
      conn: DbConnection,
      identity: Identity,
      token: string
    ) => {
      setIdentity(identity);
      setConnected(true);
      localStorage.setItem("auth_token", token);
      console.log("Connected with identity:", identity.toHexString());
      conn.reducers.onSendMessage(() => console.log("Message sent."));
      conn.reducers.onCreateGame(() => console.log("Game created."));
      subscribeToQueries(conn, [
        "SELECT * FROM message",
        "SELECT * FROM user",
        "SELECT * FROM game",
      ]);
    };
    const onDisconnect = () => {
      console.log("Disconnected");
      setConnected(false);
    };
    const onConnectError = (_ctx: ErrorContext, err: Error) => {
      console.log("Connection error:", err);
    };
    setConn(
      DbConnection.builder()
        .withUri("ws://localhost:3000")
        .withModuleName("quickstart-chat")
        .withToken(localStorage.getItem("auth_token") || "")
        .onConnect(onConnect)
        .onDisconnect(onDisconnect)
        .onConnectError(onConnectError)
        .build()
    );
  }, []);

  useEffect(() => {
    if (!conn) return;
    conn.db.user.onInsert((_ctx, user) => {
      if (user.online) {
        const name = user.name || user.identity.toHexString().substring(0, 8);
        setSystemMessage((prev) => prev + `\n${name} has connected.`);
      }
    });
    conn.db.user.onUpdate((_ctx, oldUser, newUser) => {
      const name =
        newUser.name || newUser.identity.toHexString().substring(0, 8);
      if (!oldUser.online && newUser.online) {
        setSystemMessage((prev) => prev + `\n${name} has connected.`);
      } else if (oldUser.online && !newUser.online) {
        setSystemMessage((prev) => prev + `\n${name} has disconnected.`);
      }
    });
  }, [conn]);

  const messages = useMessages(conn);
  const users = useUsers(conn);
  const games = useGames(conn);
  const selectedGame = games.find((g) => g.id === selectedGameId) || null;

  const prettyMessages: PrettyMessage[] = messages
    .sort((a, b) => (a.sent > b.sent ? 1 : -1))
    .map((message) => ({
      senderName:
        users.get(message.sender.toHexString())?.name ||
        message.sender.toHexString().substring(0, 8),
      text: message.text,
    }));

  if (!conn || !connected || !identity) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-100">
        <h1 className="text-2xl font-bold">Connecting...</h1>
      </div>
    );
  }

  const name =
    users.get(identity.toHexString())?.name ||
    identity.toHexString().substring(0, 8) ||
    "";

  const onSubmitNewName = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setSettingName(false);
    conn.reducers.setName(newName);
  };

  const onMessageSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setNewMessage("");
    conn.reducers.sendMessage(newMessage);
  };

  const createGame = async () => {
    // For demonstration, you can optionally pass a handicap (e.g., 2)
    conn.reducers.createGame(9, 0); // 9x9 game without handicap.
  };

  const joinGame = async (gameId: bigint) => {
    conn.reducers.joinGame(gameId);
  };

  return (
    <div className="min-h-screen bg-gray-100 p-4">
      <h1 className="text-3xl font-bold mb-6 text-center">Online Go MVP</h1>
      <div className="max-w-4xl mx-auto bg-white shadow rounded p-6 space-y-6">
        <div className="flex items-center gap-2">
          <a href="https://vite.dev" target="_blank" rel="noreferrer">
            <img src={viteLogo} className="logo" alt="Vite logo" />
          </a>
          <h1 className="text-4xl font-bold">SpacetimeDB Chat Demo</h1>
        </div>
        {/* Profile Section */}
        <div>
          <h1 className="text-2xl font-bold mb-2">Profile</h1>
          {!settingName ? (
            <>
              <p className="mb-2">{name}</p>
              <button
                onClick={() => {
                  setSettingName(true);
                  setNewName(name);
                }}
                className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
              >
                Edit Name
              </button>
            </>
          ) : (
            <form onSubmit={onSubmitNewName} className="flex space-x-2">
              <input
                type="text"
                aria-label="name input"
                value={newName}
                onChange={(e) => setNewName(e.target.value)}
                className="border rounded px-2 py-1 flex-grow"
              />
              <button
                type="submit"
                className="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
              >
                Submit
              </button>
            </form>
          )}
        </div>

        {/* Messages Section */}
        <div>
          <h1 className="text-2xl font-bold mb-2">Messages</h1>
          {prettyMessages.length < 1 ? (
            <p className="text-gray-500">No messages</p>
          ) : (
            prettyMessages.map((message, key) => (
              <div key={key} className="mb-4 p-4 border rounded">
                <p className="font-bold">{message.senderName}</p>
                <p>{message.text}</p>
              </div>
            ))
          )}
        </div>

        {/* System Section */}
        <div>
          <h1 className="text-2xl font-bold mb-2">System</h1>
          <div className="p-4 bg-gray-50 border rounded whitespace-pre-wrap">
            <p>{systemMessage}</p>
          </div>
        </div>

        {/* New Message Section */}
        <div>
          <form
            onSubmit={onMessageSubmit}
            className="flex flex-col items-center space-y-4 w-full"
          >
            <h3 className="text-xl font-semibold">New Message</h3>
            <textarea
              aria-label="message input"
              value={newMessage}
              onChange={(e) => setNewMessage(e.target.value)}
              className="w-full border rounded p-2"
            ></textarea>
            <button
              type="submit"
              className="px-4 py-2 bg-indigo-500 text-white rounded hover:bg-indigo-600"
            >
              Send
            </button>
          </form>
        </div>
      </div>
      {/* Game Section */}
      <div className="max-w-4xl mx-auto bg-white shadow rounded p-6">
        <h2 className="text-3xl font-bold mb-4 text-center">Available Games</h2>
        <div className="flex justify-end mb-4">
          <button
            onClick={createGame}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            Create Game
          </button>
        </div>
        {games.length < 1 ? (
          <p className="text-gray-500 text-center">No available games.</p>
        ) : (
          <table className="w-full border-collapse">
            <thead>
              <tr>
                <th className="border border-gray-400 p-2">Game ID</th>
                <th className="border border-gray-400 p-2">Board Size</th>
                <th className="border border-gray-400 p-2">Status</th>
                <th className="border border-gray-400 p-2">Action</th>
              </tr>
            </thead>
            <tbody>
              {games.map((game) => (
                <tr key={game.id}>
                  <td className="border border-gray-400 p-2">{game.id}</td>
                  <td className="border border-gray-400 p-2">
                    {game.boardSize}
                  </td>
                  <td className="border border-gray-400 p-2">
                    {game.playerWhite ? "Full" : "Waiting"}
                  </td>
                  <td className="border border-gray-400 p-2">
                    {!game.playerWhite ? (
                      <button
                        onClick={() => joinGame(game.id)}
                        className="px-3 py-1 bg-green-500 text-white rounded hover:bg-green-600"
                      >
                        Join
                      </button>
                    ) : (
                      "N/A"
                    )}
                    <button
                      onClick={() => setSelectedGameId(game.id)}
                      className="px-3 py-1 bg-green-500 text-white rounded hover:bg-green-600 ml-2"
                    >
                      View
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
      {selectedGame && (
        <Board game={selectedGame} onCellClick={onCellClick} onPass={onPass} />
      )}
    </div>
  );
}

export default App;

type BoardProps = {
  game: Game;
  onCellClick: (x: number, y: number) => void;
  onPass: () => void;
};

function Board({ game, onCellClick, onPass }: BoardProps) {
  const size = game.boardSize;
  let board: SpotState[] = [];
  try {
    board = JSON.parse(game.board);
  } catch (e) {
    console.error("Error parsing board:", e);
  }
  const rows = [];
  for (let y = 0; y < size; y++) {
    const cells = [];
    for (let x = 0; x < size; x++) {
      const idx = y * size + x;
      const cell: SpotState = board[idx];
      let display = "";
      if (cell.occupant === "Black") {
        display = "●";
      } else if (cell.occupant === "White") {
        display = "○";
      }
      cells.push(
        <td
          key={x}
          onClick={() => onCellClick(x, y)}
          className="w-10 h-10 border border-gray-400 text-center cursor-pointer hover:bg-gray-200"
        >
          {display}
        </td>
      );
    }
    rows.push(<tr key={y}>{cells}</tr>);
  }
  return (
    <div>
      <p>Turn: {game.turn}</p>
      <table className="border-collapse mt-4">
        <tbody>{rows}</tbody>
      </table>
      {!game.gameOver && (
        <button
          onClick={onPass}
          className="mt-4 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
        >
          Pass
        </button>
      )}
      {game.gameOver && (
        <div className="mt-4 p-4 border rounded bg-yellow-100">
          <h2 className="text-xl font-bold">Game Over</h2>
          <p>
            Final Score: Black: {game.finalScoreBlack || 0} - White:{" "}
            {game.finalScoreWhite || 0}
          </p>
        </div>
      )}
    </div>
  );
}
