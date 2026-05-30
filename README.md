<!--
SPDX-FileCopyrightText: 2022 - 2026 Ali Sajid Imami
SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

<!-- badges -->
![GitHub Release](https://img.shields.io/github/v/tag/AliSajid/tictacrustle?label=version&logo=github)
![Continuous integration](https://github.com/AliSajid/tictacrustle/actions/workflows/ci.yaml/badge.svg)
![GitHub issues](https://img.shields.io/github/issues/AliSajid/tictacrustle?label=issues)
![REUSE Compliance](https://img.shields.io/reuse/compliance/github.com%2FAliSajid%2Ftictacrustle)

<!-- description -->

An historical reinforcement learning implementation of **MENACE** (Matchbox Educable Noughts and Crosses Engine) using a polyglot monorepo with Rust crates and SvelteKit frontend.

## What Is This Project?

This project replicates Donald Michie's 1961 **MENACE** system — one of the first practical implementations of reinforcement learning — using modern Rust and a modular workspace architecture.

Instead of physical matchboxes and colored beads, this project uses:
- A **static matrix** pre-calculated at compile time
- **Decision weights** stored in memory (the "beads")
- **Training simulations** that adjust weights through gameplay

## Quick Start

```bash
# Install mise and toolchain
mise use -t ttrustle

# Build the project
cargo build --workspace

# Run the CLI
cargo run --bin ttrustle -- help

# Train a new brain
cargo run --bin tttraining -- train --iterations 100000

# Play against the trained AI
cargo run --bin ttrustle -- play --brain standard
```

## Project Structure

```
ttrustle/
├── Cargo.toml                    # Workspace root
├── mise.toml                     # Common tools and profiles
├── PROJECT_OVERVIEW.md           # High-level overview
├── ARCHITECTURE.md               # System design
├── TRAINING.md                   # Training guide
├── WORKSPACE.md                  # Workspace structure
├── guide/                        # User documentation
│   └── src/
└── [crates]
    ├── ttrustle-lib/             # Core game logic
    ├── ttrustle/                 # CLI binary
    ├── tttraining/               # Training crate
    ├── tttui/                    # Terminal UI
    ├── ttgui/                    # Desktop GUI
    ├── ttserver/                 # API server
    └── ttweb/                    # SvelteKit frontend
```

## Core Features

- **Static, Pre-Calculated Decision Space**: 5,478 legal states pruned at compile time
- **Granular Difficulty Scale**: 11 pre-trained "brains" from random to perfect play
- **Deep Brain Visualization**: Interactive frontend showing decision weights and state trajectories
- **Offline-First**: All weights embedded at compile time, no runtime dependencies

## Crates Overview

| Crate | Purpose |
|-------|--------|
| **ttrustle-lib** | Core game logic, board state, win conditions |
| **ttrustle** | CLI for offline simulation and checkpoint generation |
| **tttraining** | Training crate (runs MENACE simulations, generates weights) |
| **ttserver** | Stateless Axum API server |
| **ttui** | Terminal UI (Ratatui-based) |
| **ttgui** | Desktop GUI (GTK-based) |
| **ttweb** | SvelteKit web frontend |

## Documentation

- [Project Overview](./PROJECT_OVERVIEW.md) - High-level explanation
- [Architecture](./ARCHITECTURE.md) - System design and data flow
- [Training](./TRAINING.md) - How to train and use the training crate
- [MENACE](./MENACE.md) - What MENACE is and how it works
- [Development](./DEVELOPMENT.md) - Developer onboarding guide
- [Guide](./guide/) - User-facing documentation

## How It Works

### The Game

Tic-Tac-Toe played between a human and MENACE AI:

```
  8 ┌───┬───┬───┐
    │ X │ O │ - │
  7 ├───┼───┼───┤
    │ - │ X │ O │
  6 ├───┼───┼───┤
    │ O │ - │ X │
  5 └───┴───┴───┘
```

### The AI

MENACE uses a matchbox-inspired system:
- Each possible game state has a "matchbox" with decision weights
- Weights increase when used successfully
- Over time, the system learns optimal strategy

### Training

The `tttraining` crate runs simulations where MENACE plays against itself (or random), adjusting weights based on outcomes. These weights are embedded in the final binary for offline operation.

## Requirements

- Rust 1.85.1 or later
- `mise` for toolchain management
- For UI crates:
  - TUI: No additional dependencies
  - GUI: GTK installed
  - Web: Node.js 20.x

## License

Dual-licensed under [MIT](LICENSES/MIT.txt) and [Apache-2.0](LICENSES/Apache-2.0.txt).

## Acknowledgements

- Donald Michie and the original MENACE system
- The Rust community for outreach and training
- The [New Rustacean](https://newrustacean.com/) podcast
