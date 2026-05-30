# Overview

Welcome to Tic-Tac-Rustle!

## What Is This Project?

Tic-Tac-Rustle is a modern implementation of **MENACE**, one of the first practical implementations of reinforcement learning. Originally created in 1961 by Donald Michie at University of Edinburgh, MENACE used physical matchboxes and colored beads to learn how to play Tic-Tac-Toe.

## How It Works

The original MENACE system:
- Used 304 matchboxes (one for each possible game state)
- Each matchbox contained colored beads representing possible moves
- After each game, the system adjusted bead counts based on outcomes
- Over time, the system learned optimal play

Our implementation:
- Uses a **static matrix** pre-calculated at compile time
- Stores **decision weights** in memory (the "beads")
- **Training simulations** adjust weights through gameplay
- **Embeds** trained weights in binaries for offline operation

## Project Structure

```
ttrustle/
├── ttrustle-lib/      # Core game logic
├── ttrustle/          # CLI binary
├── tttraining/        # Training crate (NEW)
├── ttserver/          # API server
├── ttui/              # Terminal UI
├── ttgui/             # Desktop GUI
└── ttweb/             # Web frontend
```

## Getting Started

### Installation

```bash
# Install mise and toolchain
mise use -t ttrustle

# Build the project
cargo build --workspace
```

### First Game

```bash
# List available brains
cargo run --bin ttrustle -- list-brains

# Play against standard brain
cargo run --bin ttrustle -- play --brain standard
```

## Brain Tiers

Pre-trained brains represent different evolution stages:

| Tier | Name | Description | Win Rate |
|------|------|-------------|----------|
| 0 | random | Completely random play | 0% |
| 1-3 | Early | Learning the basics | Variable |
| 4-6 | Intermediate | Tactical awareness | ~55% |
| 7-8 | Advanced | Strategic play | ~65% |
| 9-10 | Near-optimal | Advanced strategy | ~85% |
| 11 | Perfect | Unbeatable play | 100% |

## Documentation

- [Architecture](../../ARCHITECTURE.md) - System design
- [Training](../../TRAINING.md) - Training guide
- [API](../../API.md) - API reference
- [DEVELOPMENT](../../DEVELOPMENT.md) - Developer guide

## License

Dual-licensed under [MIT](../../LICENSES/MIT.txt) and [Apache-2.0](../../LICENSES/Apache-2.0.txt).
