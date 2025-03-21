import { useState } from "react";
import { useSpacetime } from "../../providers/spacetime-context";

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
        <div className="flex items-center   space-x-2 ">
          <p className="  input">{name}</p>
          <button
            onClick={() => {
              setSettingName(true);
              setNewName(name);
            }}
            className="btn-sm btn btn-primary"
          >
            Edit Name
          </button>
        </div>
      ) : (
        <form
          onSubmit={onSubmitNewName}
          className="flex space-x-2 items-center"
        >
          <input
            type="text"
            aria-label="name input"
            value={newName}
            onChange={(e) => setNewName(e.target.value)}
            className="input"
          />
          <button type="submit" className="btn-sm btn btn-primary">
            Submit
          </button>
        </form>
      )}
    </>
  );
};

export default ProfileSection;
