# Getting Started

This guide walks you through setting up Tic-Tac-Rustle and playing your first game.

## Prerequisites

- Rust 1.85.1 or later
- `mise` for toolchain management
- Git for version control (optional, for cloning)

## Installation

### Step 1: Install Mise

```bash
# Install mise (if not already installed)
curl --proto '=https' --tlsv1.2 -LsSf https://mise.run | sh

# Install ttrustle toolchain
mise use -t ttrustle
```

### Step 2: Clone the Repository

```bash
git clone https://github.com/AliSajid/tictacrustle.git
cd ttrustle
```

### Step 3: Build the Project

```bash
# Build all crates (including training artifacts)
cargo build --workspace --release

# Or use the convenience script
mise run build
```

### Step 4: Verify Installation

```bash
# Check available brains
cargo run --bin ttrustle -- list-brains
```

## Quick Play

```bash
# Play against random (easiest)
cargo run --bin ttrustle -- play --brain random

# Play against standard (medium)
cargo run --bin ttrustle -- play --brain standard

# Play against expert (hard)
cargo run --bin ttrustle -- play --brain expert
```

## Using the API Server

### Start the Server

```bash
cargo run --bin ttserver
```

### Make a Move

```bash
curl -X POST http://localhost:8080/api/move \
  -H "Content-Type: application/json" \
  -d '{
    "board": [
      ["", "X", ""],
      ["", "", ""],
      ["", "O", ""]
    ],
    "brain": "standard"
  }'
```

## Next Steps

- Read [The AI](./04-the-ai.md) to understand how MENACE works
- Try [Training](./06-training.md) your own brain
- Explore [Advanced](./07-advanced.md) tips and tricks

## Troubleshooting

### Build Fails with Missing Dependencies

```bash
# Sync toolchain
mise sync

# Install missing dependencies
mise install
```

### Training Takes Too Long

Reduce iterations or use fewer CPU cores:

```bash
RUST_NUM_THREADS=2 cargo run --bin tttraining -- train --iterations 50000
```

### API Returns Errors

- Check that the server is running: `curl http://localhost:8080/health`
- Verify board format: 3x3 grid with X, O, or empty strings
- Check available brains: `curl http://localhost:8080/api/brain`

## Keyboard Shortcuts

When playing via CLI:

- `q` - Quit game
- `?` - Show help
- `r` - Replay last game
