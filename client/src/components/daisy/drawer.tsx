import { useState } from "react";
import { useSpacetime } from "../../providers/spacetime-context";
import VersionDropdown from "../version-dropdown";

export const Drawer = ({ children }: { children: React.ReactNode }) => {
  return (
    <div className="bg-base-100 drawer mx-auto max-w-[100rem] lg:drawer-open">
      <input id="drawer" type="checkbox" className="drawer-toggle"></input>

      <div className="drawer-content">
        <div className="bg-base-100 flex justify-center rounded-sm p-2">
          <a
            href="/docs/upgrade/"
            className="btn btn-soft group btn-sm [width:clamp(20rem,100%,30rem)] rounded-full"
          >
            🎉 daisyUI 5.0 upgrade guide
          </a>
        </div>

        <div className="bg-base-100/90 text-base-content sticky top-0 z-30 flex h-16 w-full [transform:translate3d(0,0,0)] justify-center backdrop-blur transition-shadow duration-100 print:hidden">
          <nav className="navbar w-full">
            <div className="flex flex-1 items-center md:gap-1 lg:gap-2">
              <span
                className="tooltip tooltip-bottom before:text-xs before:content-[attr(data-tip)]"
                data-tip="Menu"
              >
                <label
                  aria-label="Open menu"
                  htmlFor="drawer"
                  className="btn btn-square btn-ghost drawer-button lg:hidden "
                >
                  <svg
                    width="20"
                    height="20"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    className="inline-block h-5 w-5 stroke-current md:h-6 md:w-6"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M4 6h16M4 12h16M4 18h16"
                    ></path>
                  </svg>
                </label>
              </span>
              <div className="flex items-center gap-2 lg:hidden">
                <a className="btn btn-ghost flex-0 gap-1 px-2 md:gap-2">
                  <svg
                    className="h-5 w-5 md:h-6 md:w-6"
                    width="28"
                    height="28"
                    viewBox="0 0 415 415"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <rect
                      x="82.5"
                      y="290"
                      width="250"
                      height="125"
                      rx="62.5"
                      fill="#1AD1A5"
                    ></rect>
                    <circle
                      cx="207.5"
                      cy="135"
                      r="130"
                      fill="black"
                      fill-opacity=".3"
                    ></circle>
                    <circle cx="207.5" cy="135" r="125" fill="white"></circle>
                    <circle cx="207.5" cy="135" r="56" fill="#FF9903"></circle>
                  </svg>
                  <span className="font-title text-base-content text-lg md:text-xl">
                    daisyUI
                  </span>
                </a>

                <VersionDropdown />
              </div>
            </div>
            <div className="flex">
              <div className="hidden flex-none items-center lg:inline-block">
                <a
                  data-sveltekit-preload-data=""
                  href="/components/"
                  className="btn btn-sm btn-ghost drawer-button font-normal"
                >
                  Components
                </a>
                <a
                  data-sveltekit-preload-data=""
                  href="/components/"
                  className="btn btn-sm btn-ghost drawer-button font-normal"
                >
                  Components
                </a>
              </div>
              <div className="dropdown dropdown-end block ">
                <div
                  tabIndex={0}
                  role="button"
                  className="btn btn-sm gap-1 btn-ghost"
                >
                  <div className="bg-base-100 border-base-content/10 grid shrink-0 grid-cols-2 gap-0.5 rounded-md border p-1">
                    <div className="bg-base-content size-1 rounded-full"></div>{" "}
                    <div className="bg-primary size-1 rounded-full"></div>{" "}
                    <div className="bg-secondary size-1 rounded-full"></div>{" "}
                    <div className="bg-accent size-1 rounded-full"></div>
                  </div>{" "}
                  <svg
                    width="12px"
                    height="12px"
                    className="mt-px hidden h-2 w-2 fill-current opacity-60 sm:inline-block"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 2048 2048"
                  >
                    <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
                  </svg>
                </div>
                <div className="dropdown-content bg-base-200 text-base-content rounded-box top-px h-[30.5rem] max-h-[calc(100vh-8.6rem)] overflow-y-auto border border-white/5 shadow-2xl outline-1 outline-black/5 mt-16">
                  <ul className="menu w-56">
                    <li className="menu-title text-xs">Theme</li>
                    <li>
                      <button className="gap-3 px-2" data-set-theme="dark">
                        <div
                          data-theme="dark"
                          className="bg-base-100 grid shrink-0 grid-cols-2 gap-0.5 rounded-md p-1 shadow-sm"
                        >
                          <div className="bg-base-content size-1 rounded-full"></div>{" "}
                          <div className="bg-primary size-1 rounded-full"></div>{" "}
                          <div className="bg-secondary size-1 rounded-full"></div>{" "}
                          <div className="bg-accent size-1 rounded-full"></div>
                        </div>{" "}
                        <div className="w-32 truncate">dark</div>{" "}
                        <svg
                          xmlns="http://www.w3.org/2000/svg"
                          width="16"
                          height="16"
                          viewBox="0 0 24 24"
                          fill="currentColor"
                          className="invisible h-3 w-3 shrink-0"
                        >
                          <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                        </svg>
                      </button>
                    </li>
                  </ul>
                </div>
              </div>
            </div>
          </nav>
        </div>
        <div className="relative max-w-[100vw] px-6 pb-16 xl:pe-2">
          {children}
        </div>
      </div>

      <div className="drawer-side z-40 scroll-smooth scroll-pt-20">
        <label
          htmlFor="drawer"
          className="drawer-overlay"
          aria-label="Close menu"
        ></label>
        <aside className="bg-base-100 min-h-screen w-80">
          <div className="bg-base-100/90 navbar sticky top-0 z-20 hidden items-center gap-2 px-4 py-2 backdrop-blur lg:flex ">
            <a
              href="/"
              aria-current="page"
              aria-label="Homepage"
              className="btn btn-ghost flex-0 px-2"
            >
              <svg
                width="32"
                height="32"
                viewBox="0 0 415 415"
                xmlns="http://www.w3.org/2000/svg"
              >
                <rect
                  x="82.5"
                  y="290"
                  width="250"
                  height="125"
                  rx="62.5"
                  fill="#1AD1A5"
                ></rect>
                <circle
                  cx="207.5"
                  cy="135"
                  r="130"
                  fill="black"
                  fill-opacity=".3"
                ></circle>
                <circle cx="207.5" cy="135" r="125" fill="white"></circle>
                <circle cx="207.5" cy="135" r="56" fill="#FF9903"></circle>
              </svg>{" "}
              <div className="font-title inline-flex text-lg md:text-2xl">
                daisyUII
              </div>
            </a>
            <VersionDropdown />
          </div>
          <div className="h-4"></div>

          <ul className="menu w-full px-4 py-0">
            {/* 50 items */}
            {Array.from({ length: 50 }, (_, i) => (
              <li key={i}>Item {i + 1}</li>
            ))}
          </ul>
        </aside>
      </div>
    </div>
  );
};

export const WebDrawer = () => {
  return;
};

const ProfileSection = () => {
  const { users, identity, conn } = useSpacetime();
  const [settingName, setSettingName] = useState(false);
  const [newName, setNewName] = useState("");
  const name = identity
    ? users.get(identity.toHexString())?.name ||
      identity.toHexString().substring(0, 8)
    : "";
  const onSubmitNewName = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setSettingName(false);
    conn?.reducers.setName(newName);
  };
  return (
    <>
      {!settingName ? (
        <div className="flex items-center justify-between">
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
        </div>
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
    </>
  );
};

export type PrettyMessage = {
  senderName: string;
  text: string;
};

const MessageSection = () => {
  const { users, identity, conn, messages } = useSpacetime();
  const [newMessage, setNewMessage] = useState("");

  const onMessageSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setNewMessage("");
    conn?.reducers.sendMessage(newMessage);
  };
  const prettyMessages: PrettyMessage[] = messages
    .sort((a, b) => (a.sent > b.sent ? 1 : -1))
    .map((message) => ({
      senderName:
        users.get(message.sender.toHexString())?.name ||
        message.sender.toHexString().substring(0, 8),
      text: message.text,
    }));

  return (
    <div className="h-full bg-pink-500 space-y-2">
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
  );
};

const TestTabs = () => {
  return (
    <div className="h-full bg-orange-500 flex flex-col ">
      <div className="bg-red-500 w-full h-12">asdf</div>

      <div className="bg-pink-500 flex-auto overflow-auto">
        aasdfasdfasdfasdfasdfasdf
        {/* render 50 fakse messages in a loop */}
        {Array.from({ length: 50 }, (_, i) => (
          <div key={i} className="border-b border-gray-300 px-4 py-2 text-sm">
            Message {i + 1}
          </div>
        ))}
      </div>
    </div>
  );
};
const DrawerTabs = () => {
  return (
    <div className="tabs tabs-lift h-full bg-red-500">
      <label className="tab">
        <input type="radio" name="my_tabs_4" />
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          strokeWidth="1.5"
          stroke="currentColor"
          className="size-4 me-2"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z"
          />
        </svg>
        Chat
      </label>
      <div className="tab-content bg-base-100 border-base-300 p-6">
        <MessageSection />
      </div>

      <label className="tab">
        <input type="radio" name="my_tabs_4" defaultChecked />
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          strokeWidth="1.5"
          stroke="currentColor"
          className="size-4 me-2"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M15.182 15.182a4.5 4.5 0 0 1-6.364 0M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0ZM9.75 9.75c0 .414-.168.75-.375.75S9 10.164 9 9.75 9.168 9 9.375 9s.375.336.375.75Zm-.375 0h.008v.015h-.008V9.75Zm5.625 0c0 .414-.168.75-.375.75s-.375-.336-.375-.75.168-.75.375-.75.375.336.375.75Zm-.375 0h.008v.015h-.008V9.75Z"
          />
        </svg>
        Laugh
      </label>
      <div className="tab-content bg-base-100 border-base-300 p-6">
        Tab content 2
      </div>

      <label className="tab">
        <input type="radio" name="my_tabs_4" />
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          strokeWidth="1.5"
          stroke="currentColor"
          className="size-4 me-2"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"
          />
        </svg>
        Love
      </label>
      <div className="tab-content bg-base-100 border-base-300 p-6">
        Tab content 3
      </div>
    </div>
  );
};

export const DrawerButton = () => {
  return (
    <label
      htmlFor="my-drawer-2"
      className="btn btn-primary drawer-button lg:hidden"
    >
      Open drawer
    </label>
  );
};
