import { createFileRoute } from "@tanstack/react-router";
import GameBoard from "../../components/spacetime-components/game";

export const Route = createFileRoute("/game/$gameId")({
  component: RouteComponent,
});

function RouteComponent() {
  const { gameId } = Route.useParams();

  return (
    <div>
      <GameBoard gameId={BigInt(gameId)} />
    </div>
  );
}
