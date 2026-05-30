# MENACE Implementation

This document explains the MENACE system used in Tic-Tac-Rustle.

## Original MENACE

The original MENACE system (1961) used physical matchboxes:

```
┌─────────────────────────────────────────────────────────┐
│                    ORIGINAL MENACE (1961)                │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  304 matchboxes, one for each game state                 │
│                                                           │
│  Each matchbox contains colored beads:                    │
│  - Each bead represents a possible move                   │
│  - Color encodes the move (1=X, 2=O)                     │
│                                                           │
│  After each game:                                         │
│  - AI wins: Add 2 beads to winning moves, 1 to others     │
│  - AI loses: Remove beads from losing moves               │
│                                                           │
│  Over time, the system learns optimal strategy           │
└─────────────────────────────────────────────────────────┘
```

## Our Implementation

### Static Matchbox Matrix

Instead of physical matchboxes, we use a **pre-calculated matrix**:

```rust
// Each entry maps a board state to possible moves and their weights
type Matchbox = HashMap<u32, Vec<(Move, u32)>>; // state -> [(move, weight)]

// 5,478 legal board states, each mapped to 1-8 possible moves
const MATCHBOXES: [u32; 5478]; // Compact representation
```

### Decision Weights ("Beads")

Each move has a **decision weight** that increases with successful use:

```
move_weight = base_weight + reinforcement_bonus
```

#### Reinforcement Rules

| Outcome | Reinforcement |
|---------|--------------|
| AI wins | +2 beads to winning move, +1 to others |
| Draw | +1 to all moves |
| AI loses | -1 from losing move (minimum 0) |

### Two Flavors

**MENACE-C (Classic)**:
- Treats rotationally equivalent states as identical
- Reduces state space from 19,683 to 5,478
- Matches original Michie's approach

**MENACE-S (Symmetry)**:
- Treats all states as distinct
- Larger state space but more explicit
- Useful for comparing with classical approaches

## Training Workflow

The `tttraining` crate runs simulations:

```
1. Initialize all matchboxes with base weights
2. Run N games (AI vs AI or AI vs random)
3. Apply reinforcement after each game
4. Export final weights to assets/
5. Build binaries embed weights
```

## Brain Tiers

Pre-trained "brains" represent different evolution stages:

| Tier | Description | Win Rate |
|------|-------------|----------|
| 0 | Random | 0% |
| 1-3 | Early learning | Variable |
| 4-6 | Intermediate | ~55% |
| 7-8 | Advanced | ~65% |
| 9-10 | Near-optimal | ~85% |
| 11 | Perfect play | 100% |

## Why This Matters

This implementation:
- **Educational**: Demonstrates RL principles in a accessible way
- **Historical**: Honors Michie's original vision
- **Modern**: Uses efficient data structures
- **Offline**: Works without internet
- **Extensible**: Easy to add new variants
