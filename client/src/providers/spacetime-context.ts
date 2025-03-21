import { Identity } from "@clockworklabs/spacetimedb-sdk";
import { createContext, useContext } from "react";
import { DbConnection, Game, Message, User } from "../module_bindings";

// Define the context shape
export type SpacetimeContextType = {
  conn: DbConnection | null;
  connected: boolean;
  identity: Identity | null;
  messages: Message[];
  games: Game[];
  users: Map<string, User>;
  systemMessage: string;
};

// Create the context with an initial default value
export const SpacetimeContext = createContext<SpacetimeContextType>({
  conn: null,
  connected: false,
  identity: null,
  messages: [],
  games: [],
  users: new Map(),
  systemMessage: "",
});

// Custom hook for consuming the context
export const useSpacetime = () => useContext(SpacetimeContext);
