# The AI

This document explains how the MENACE AI works in Tic-Tac-Rustle.

## What Is MENACE?

MENACE (Machine Educable Noughts and Crosses Engine) is a reinforcement learning system that learns by playing games.

### Original MENACE (1961)

Donald Michie's original implementation used:
- **304 matchboxes**: One for each possible game state
- **Colored beads**: Each bead represented a possible move
- **Learning**: After each game, adjust bead counts based on outcome

### Our Implementation

We translate this physical system into code:
- **Matchbox** -> HashMap mapping states to moves
- **Beads** -> Decision weights (u32 counter)
- **Learning** -> Reinforcement after each game

## How MENACE Learns

### Initialization

Each matchbox starts with base weights:

```rust
type Matchbox = HashMap<u32, Vec<(Move, u32)>>;

const INITIAL_WEIGHTS: u32 = 5;  // Base weight per move
```

### Game Loop

```
1. Player makes a move
2. AI draws a weighted random move
3. Game continues until someone wins
4. Adjust weights based on outcome
```

### Weight Adjustment

| Outcome | Adjustment |
|---------|-----------|
| AI wins | +2 to winning move, +1 to others |
| Draw | +1 to all moves |
| AI loses | -1 from losing move (min 0) |

### Learning Over Time

```
Games: 1          10,000      100,000     1,000,000
Weights: 5,5,5    50,60,70     80,100,120  200,250,300
Strategy: Random  Learning     Tactical     Mastered
```

## Brain Tiers

Pre-trained brains represent different stages of learning:

```
Tier 0:   Random (no learning)
Tier 1-3:  Early learning, still making mistakes
Tier 4-6:  Tactical, blocks opponent, creates opportunities
Tier 7-8:  Strategic, plans ahead, recognizes patterns
Tier 9-10: Near-optimal, rarely loses
Tier 11:  Perfect play (unbeatable)
```

## MENACE-C vs MENACE-S

### MENACE-C (Classic)

- Treats rotationally equivalent states as identical
- Uses fewer matchboxes (5,478 vs 19,683)
- Matches original Michie's approach
- Smaller memory footprint

### MENACE-S (Distinct)

- Treats all states as distinct
- Larger state space
- More explicit representation
- Useful for comparing with classical approaches

## Memory Layout

```
┌─────────────────────────────────────┐
│  Matchbox[5478]                    │
│   ┌──────────────────────────────┐ │
│   │  State 0: [(0,5), (1,5), (2,5)]││
│   │  State 1: [(0,6), (1,5), (2,5)]││
│   │  State 2: [(1,6), (2,5), (3,4)]││
│   │  ...                         ││
│   └──────────────────────────────┘ │
└─────────────────────────────────────┘
```

Each entry maps a state to its possible moves and weights.

## Weight Visualization

```
State: X-O- -  -X- -X-
Moves: [A, B, C, D, E]
Weights: [5, 8, 6, 4, 7]

AI chooses: B (weight 8, highest)
```

## Why MENACE Is Special

1. **No explicit programming**: The system learned through experience
2. **Interpretable**: You can inspect the weights
3. **Historical**: One of the first RL systems
4. **Educational**: Clear mapping to physical implementation
5. **Offline**: Works without network after training

## Limitations

- **No exploration beyond training**: Weights are static after training
- **No online learning**: Can't learn during gameplay
- **No generalization**: Specific to 3x3 grid
- **No multi-agent coordination**: Each brain is independent

## Advanced Usage

### Inspecting Weights

```bash
# Decode and inspect weight distribution
base64 -d assets/standard.base64 | awk '{print $1, $2}' | \
  sort -t' ' -k2 -n | head -10
```

### Custom Training

```bash
# Train with custom parameters
cargo run --bin tttraining -- train \
  --name my-brain \
  --iterations 100000 \
  --format MENACE-C
```

### Weight Analysis

See [TRAINING.md](../../TRAINING.md) for weight analysis techniques.
