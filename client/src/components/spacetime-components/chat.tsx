import { useMemo, useState } from "react";
import { useSpacetime } from "../..//providers/spacetime-context";
import { Timestamp } from "@clockworklabs/spacetimedb-sdk";

export type PrettyMessage = {
  senderName: string;
  text: string;
  timestamp: Timestamp;
};

const Chat = () => {
  const { users, conn, messages } = useSpacetime();
  const [newMessage, setNewMessage] = useState("");

  const onMessageSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setNewMessage("");
    conn?.reducers.sendMessage(newMessage);
  };

  const prettyMessages = useMemo(
    () =>
      messages
        .sort((a, b) => (a.sent > b.sent ? 1 : -1))
        .map((message) => ({
          senderName:
            users.get(message.sender.toHexString())?.name ||
            message.sender.toHexString().substring(0, 8),
          text: message.text,
          timestamp: message.sent,
        })),
    [messages, users]
  );

  return (
    <div className=" h-full flex flex-col bg-pink-500/10  space-y-2">
      {prettyMessages.length < 1 ? (
        <p className="text-gray-500">No messages</p>
      ) : (
        prettyMessages.map((message, key) => (
          <div className="chat chat-start" key={key}>
            <div className="chat-header">
              {message.senderName}
              <time className="text-xs opacity-50">
                {message.timestamp.toDate().toDateString()}
              </time>
            </div>
            <div className="chat-bubble">{message.text}</div>
          </div>
        ))
      )}
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
  );
};

export default Chat;
