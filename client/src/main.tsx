import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App.tsx";
import NewApp from "./NewApp.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <NewApp />
  </StrictMode>
);
