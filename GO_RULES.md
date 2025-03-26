# How to Play Go – Official Resources & Our Implementation

For a complete, official introduction to the game of Go, we recommend going to one of the more official sources online.
---

## Our Implementation Overview

Our system is designed to closely follow the official rules of Go while providing additional insights for modern gameplay. Below is an overview of the main rules and scoring logic that we have implemented.

### 1. Basic Gameplay

- **Board and Stones:**  
  Go is played on a grid (commonly 19×19, though 9×9 and 13×13 are popular for beginners). Players use Black and White stones, placing one stone per turn.

- **Alternating Moves:**  
  Black traditionally starts the game. If handicap stones are used, Black is given extra stones before play begins, and White then makes the first move.

- **No Repositioning:**  
  Once placed, stones remain on the board until they are captured.

### 2. Groups, Liberties, and Capturing

- **Groups (Chains):**  
  Stones that are adjacent horizontally or vertically form a group.

- **Liberties:**  
  The liberties of a stone (or group) are its adjacent empty intersections. Our system uses flood-fill algorithms to identify groups and count their liberties.

- **Capturing:**  
  When a group loses all its liberties, it is captured and removed from the board. Our implementation marks removed stones (e.g., with a marker like "removed") so the final state is clear.

### 3. Ko and Move Legality

- **Ko Rule:**  
  A move that would recreate a previous board state is forbidden to prevent endless capture-and-recapture cycles (known as "ko fights").

- **Suicide Rule:**  
  A stone cannot be played if it would result in its own group having no liberties, unless it simultaneously captures enemy stones.

- **Playability:**  
  We determine whether each empty spot is legally playable by simulating a move there. A spot is flagged as unplayable if:
  - It is already occupied,
  - Placing a stone there would result in a group with no liberties (without capturing enemy stones),
  - Or if it would violate the ko rule.

### 4. Scoring

We support two main scoring systems:

#### Area Scoring (Chinese Style)
- **Calculation:**  
  **Score = (Number of stones on board) + (Number of empty intersections completely enclosed by that color)**
  
  *Example:* If Black has 100 stones and encloses 50 empty spots, Black’s score is 150. White receives additional bonus points (komi) to compensate for Black’s first-move advantage.

#### Territory Scoring (Japanese/Korean Style)
- **Calculation:**  
  **Score = (Number of empty intersections completely enclosed by a player's stones) + (Captured stones, if tracked)**
  
  *Note:* Our implementation focuses on the enclosed empty regions. Only regions that do not touch the board edge and are bordered by a single color are counted.

#### Komi
- **Komi:**  
  To balance the game, White receives extra points (typically 6.5) in every game.

### 5. Annotations for UI

Our backend enriches the board state with detailed annotations, so the client UI can clearly display:
- **Scoring Details:**  
  Each empty spot is annotated with which player (if any) “owns” the point and a textual explanation (e.g., "Cell enclosed by Black" or "Neutral").
  
- **Playability Information:**  
  Each spot includes a flag indicating whether a move can be legally played there. This is determined by simulating moves at every empty cell and checking for violations of the rules (e.g., suicide or ko).

The annotated board is returned to the client so that the UI can visually disable unplayable spots and display scoring insights in real time.

---

## Summary

Our system:
- **Identifies groups and counts liberties** using flood-fill algorithms.
- **Automatically removes dead stones** to reflect the final, live state of the board.
- **Computes scores** using either area or territory scoring methods, with komi applied to White.
- **Annotates each board spot** with scoring and playability metadata, so players can see how every point is derived.

For further details on the official rules of Go, please refer to the resources above.