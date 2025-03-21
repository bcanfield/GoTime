import { useEffect, useMemo, useRef, useState } from "react";
import { useSpacetime } from "../providers/spacetime-context";

export default function ChatBox() {
  const { users, conn, messages } = useSpacetime();
  const [newMessage, setNewMessage] = useState("");

  const messagesEndRef = useRef<HTMLDivElement>(null);

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

  const onMessageSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setNewMessage("");
    conn?.reducers.sendMessage(newMessage);
  };

  const [showScrollButton, setShowScrollButton] = useState(false);
  const messagesContainerRef = useRef<HTMLDivElement>(null);

  const isNearBottom = () => {
    const el = messagesContainerRef.current;
    return el ? el.scrollHeight - el.scrollTop - el.clientHeight < 100 : false;
  };

  useEffect(() => {
    const el = messagesContainerRef.current;
    if (el && isNearBottom()) {
      el.scrollTop = el.scrollHeight;
      setShowScrollButton(false);
    } else {
      setShowScrollButton(true);
    }
  }, [messages]);

  const handleScroll = () => {
    if (isNearBottom()) {
      setShowScrollButton(false);
    }
  };

  return (
    <div className="max-w-md mx-auto   rounded-xl shadow bg-base-100 h-[500px] flex flex-col">
      <div
        ref={messagesContainerRef}
        onScroll={handleScroll}
        className="flex-1 overflow-y-auto p-4 space-y-2 overscroll-contain"
      >
        {prettyMessages.map((msg, key) => (
          <div
            key={key}
            // TODO add logic to check if current user
            className={`chat ${msg.senderName === "user" ? "chat-end" : "chat-start"}`}
          >
            <div className="chat-header">
              {msg.senderName}
              <time className="opacity-50">
                {msg.timestamp.toDate().toDateString()}
              </time>
            </div>
            <div className="chat-bubble">{msg.text}</div>
            <div className="chat-footer opacity-50">Delivered</div>
            {/* <div className="chat-bubble">{msg.text}</div> */}
          </div>
        ))}
        <div ref={messagesEndRef} />
      </div>

      {showScrollButton && (
        <button
          className="btn btn-sm btn-primary absolute bottom-20 right-4 shadow"
          onClick={() => {
            messagesContainerRef.current?.scrollTo({
              top: messagesContainerRef.current.scrollHeight,
            });
            setShowScrollButton(false);
          }}
        >
          Scroll to bottom
        </button>
      )}

      <form
        onSubmit={onMessageSubmit}
        className="sticky bottom-0 p-4 bg-base-100 border-t border-base-300 flex gap-2 items-center"
      >
        <input
          type="text"
          className="input input-bordered input-sm input-primary w-full"
          placeholder="Type your message"
          value={newMessage}
          onChange={(e) => setNewMessage(e.target.value)}
        />

        <button className="btn btn-primary btn-sm" type="submit">
          Send
        </button>
      </form>
    </div>
  );
}
