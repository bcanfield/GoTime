import { createFileRoute } from "@tanstack/react-router";
import Games from "../components/spacetime-components/games";
import GameCreationFormModal from "../components/spacetime-components/create-game-modal";
import { useState } from "react";

export const Route = createFileRoute("/")({
  component: HomeComponent,
});

function HomeComponent() {
  const [showModal, setShowModal] = useState(false);

  return (
    <div>
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">Welcome to Go Lobby</h1>
        <button className="btn btn-primary" onClick={() => setShowModal(true)}>
          + Create Game
        </button>
      </div>
      <Games />
      <GameCreationFormModal
        open={showModal}
        onClose={() => setShowModal(false)}
      />
    </div>
  );
}
