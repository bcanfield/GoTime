import React, { useEffect, useState } from "react";
import viteLogo from "/vite.svg";
import {
  DbConnection,
  ErrorContext,
  EventContext,
  Message,
  User,
} from "./module_bindings";
import { Identity } from "@clockworklabs/spacetimedb-sdk";

export type PrettyMessage = {
  senderName: string;
  text: string;
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

  useEffect(() => {
    const subscribeToQueries = (conn: DbConnection, queries: string[]) => {
      for (const query of queries) {
        conn
          ?.subscriptionBuilder()
          .onApplied(() => {
            console.log("SDK client cache initialized.");
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
      console.log(
        "Connected to SpacetimeDB with identity:",
        identity.toHexString()
      );
      conn.reducers.onSendMessage(() => {
        console.log("Message sent.");
      });

      subscribeToQueries(conn, ["SELECT * FROM message", "SELECT * FROM user"]);
    };

    const onDisconnect = () => {
      console.log("Disconnected from SpacetimeDB");
      setConnected(false);
    };

    const onConnectError = (_ctx: ErrorContext, err: Error) => {
      console.log("Error connecting to SpacetimeDB:", err);
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
      if (oldUser.online === false && newUser.online === true) {
        setSystemMessage((prev) => prev + `\n${name} has connected.`);
      } else if (oldUser.online === true && newUser.online === false) {
        setSystemMessage((prev) => prev + `\n${name} has disconnected.`);
      }
    });
  }, [conn]);

  const messages = useMessages(conn);
  const users = useUsers(conn);

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
    users.get(identity?.toHexString())?.name ||
    identity?.toHexString().substring(0, 8) ||
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

  return (
    <div className="min-h-screen bg-gray-100 p-4">
      <div className="max-w-4xl mx-auto bg-white shadow rounded p-6 space-y-6">
        <div className="flex items-center gap-2">
          <a href="https://vite.dev" target="_blank">
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
    </div>
  );
}

export default App;
