# Project Overview: Tic-Tac-Rustle

This project implements a historical reinforcement learning system, **MENACE**, using modern Rust and a polyglot monorepo architecture.

## What is MENACE?

**MENACE** (Machine Educable Noughts and Crosses Engine) was created in 1961 by Donald Michie at University of Edinburgh. It was one of the first practical implementations of reinforcement learning.

The original system used:
- 304 matchboxes, one for each possible game state
- Colored beads representing possible moves
- A reinforcement learning algorithm that adjusted bead counts after each game

This project translates that physical system into code:
- A **static matrix** replaces matchboxes (pre-calculated at compile time)
- **Beads** become decision weights stored in memory
- **Training** adjusts weights through simulated games

## Why a Workspace?

The project uses a Cargo workspace to separate concerns. This isn't just technical debt; it serves specific purposes:

### 1. Isolation of Responsibilities

Each crate has a single purpose, making it easier to:
- Test in isolation
- Update independently
- Understand what each piece does

### 2. Parallel Development

Different crates can be worked on simultaneously without conflicts.

### 3. Compile-Time Training

The `tttraining` crate generates decision trees at build time. These are embedded in the final binary, enabling offline operation with zero runtime dependencies.

## Crates Overview

| Crate | Purpose | Type |
|--------|--------|--------|
| **ttrustle-lib** | Core game logic, board state, win conditions | Library |
| **ttrustle** | CLI for offline simulation and checkpoint generation | Binary |
| **ttserver** | Axum API server (read-only, stateless) | Binary |
| **ttgui** | Desktop GUI (GTK-based) | Binary |
| **tttui** | Terminal UI (Ratatui-based) | Binary |
| **tttraining** | Training crate that runs MENACE simulations | Binary |

## Core Concepts

### Ternary Board Representation

The board is a flattened 9-digit base-3 number. Each cell can be:
- 0 (Empty)
- 1 (X)
- 2 (O)

Legal states: 5,478 (pruned from 19,683 theoretical maximum)

### Two MENACE Variants

1. **MENACE-C**: Treats rotational and reflection symmetries as equivalent (like the original)
2. **MENACE-S**: Treats all states as distinct (no symmetry assumptions)

## Quick Start

```bash
# Install mise and toolchain
mise use -t ttrustle

# Run the CLI
cargo run --bin ttrustle -- help

# View available brains
cargo run --bin ttrustle -- list-brains

# Train a new brain
cargo run --bin tttraining
```

## Documentation Index

- [Architecture](./ARCHITECTURE.md) - System design and data flow
- [Workspace](./WORKSPACE.md) - Workspace structure explained
- [Training](./TRAINING.md) - How training works and using the training crate
- [Development](./DEVELOPMENT.md) - Developer onboarding
- [API](./API.md) - API reference for ttserver

## Related Projects

- [MENACE Wikipedia](https://en.wikipedia.org/wiki/MENACE)
- [Axum documentation](https://docs.rs/axum)
- [Ratatui documentation](https://docs.rs/ratatui)
- [SvelteKit frontend](./ttweb/README.md)
