import { useState } from "react";
import { useSpacetime } from "../../providers/spacetime-context";

export const Drawer = ({ children }: { children: React.ReactNode }) => {
  return (
    <div className="drawer lg:drawer-open h-[calc(100%-3rem)]">
      <input id="my-drawer-2" type="checkbox" className="drawer-toggle" />
      <div className="drawer-content flex flex-col items-center justify-center">
        {children}
      </div>
      <div className="drawer-side h-full">
        <label
          htmlFor="my-drawer-2"
          aria-label="close sidebar"
          className="drawer-overlay"
        ></label>
        <ul className="menu bg-base-200 text-base-content min-h-full w-80 p-4">
          <ProfileSection />
          <DrawerTabs />
        </ul>
      </div>
    </div>
  );
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
const DrawerTabs = () => {
  return (
    <div className="tabs tabs-lift">
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
        Live
      </label>
      <div className="tab-content bg-base-100 border-base-300 p-6">
        Tab content 1
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
