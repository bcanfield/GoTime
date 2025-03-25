export type Occupant = "Empty" | "Black" | "White";
export type SpotState = {
  occupant: Occupant;
  move_number: number | null;
  marker: string | null;
  scoring_owner: Occupant | null;
  scoring_explanation: string | null;
  playable: boolean;
};
