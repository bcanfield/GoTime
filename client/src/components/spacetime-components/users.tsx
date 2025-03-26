import { useMemo } from "react";
import { useSpacetime } from "../..//providers/spacetime-context";

const Users = () => {
  const { users } = useSpacetime();

  const usersArray = useMemo(
    () =>
      Array.from(users, ([id, user]) => ({
        id,
        name: user.name ?? user.identity.toHexString().substring(0, 8),
      })),
    [users]
  );

  return (
    <ul>
      {usersArray.map((user) => (
        <li key={user.id}>
          <p className="font-bold w-full ">{user.name}</p>
        </li>
      ))}
    </ul>
  );
};

export default Users;
