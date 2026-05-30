# Architecture

This document describes the system architecture of Tic-Tac-Rustle.

## System Overview

```
┌───────────────────────────────────────────────────────────────┐
│                    TIC TAC RUSTLE SYSTEM                      │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────┐     ┌─────────┐     ┌─────────┐                  │
│  │ ttweb   │────▶│ ttserver│────▶│ttrustle-│                  │
│  │ (Web)   │     │ (API)   │     │  lib    │                  │
│  └─────────┘     └─────────┘     └─────────┘                  │
│                        ▲           ▲                          │
│                        │           │                          │
│                   ┌────┴─────┐   ┌─┴─────┐                    │
│                   │ tttui    │   │ ttgui │                    │
│                   └──────────┘   └───────┘                    │
│                        │           │                          │
│                   ┌────┴─────┐   ┌───────┐                    │
│                   │tttraining│   │assets │                    │
│                   └──────────┘   └───────┘                    │
│                                                               │
│  All components operate offline: no external dependencies     │
│  Models are embedded at compile time                          │
└───────────────────────────────────────────────────────────────┘
```

## Component Catalog

### Core Crates

| Crate | Path | Role |
|-------|------|------|
| **ttrustle-lib** | `./ttrustle-lib/` | Core game logic, board state, win conditions |
| **ttrustle** | `./ttrustle/` | CLI for simulation and checkpoint generation |
| **tttraining** | `./tttraining/` | Training crate (new) |
| **ttserver** | `./ttserver/` | Stateless Axum API server |
| **ttui** | `./ttui/` | Terminal UI (Ratatui) |
| **ttgui** | `./ttgui/` | Desktop GUI (GTK) |

### Frontend Crates

| Crate | Path | Role |
|-------|------|------|
| **ttweb** | `./ttweb/` | SvelteKit web frontend |

## Crate Responsibilities

### ttrustle-lib (Library)

The core domain logic crate:

- **Board representation**: Ternary state encoding (base-3)
- **Game rules**: Valid states, legal moves, win detection
- **MENACE system**: Matchbox structure, reinforcement learning
- **GameScanner**: Pure domain rules for verification

```rust
pub trait GameScanner {
    fn is_valid_state(&self, state: u32) -> bool;
    fn legal_moves(&self, state: u32) -> Vec<u8>;
    fn outcome(&self, state: u32, move_idx: u8) -> Result<Outcome>;
}
```

### ttrustle (Binary)

The CLI interface crate:

- **Offline simulation**: Run games without network
- **Checkpoint generation**: Generate training artifacts
- **Brain listing**: Show available pre-trained brains

```bash
# List available brains
ttrustle list-brains

# Play against a specific brain
ttrustle play --brain standard

# Generate training checkpoint
ttrustle train-checkpoint --name my-brain
```

### tttraining (Binary - New)

The training crate:

- **Simulation runner**: Run MENACE against itself/random
- **Weight adjustment**: Apply reinforcement after each game
- **Artifact export**: Generate base64-encoded weights
- **Progress tracking**: Show training progress and statistics

```bash
# Train a new brain
tttraining train --name new-brain --iterations 100000

# Train with specific opponent
tttraining train --name new-brain --iterations 100000 --vs ai --brain 10
```

### ttserver (Binary)

The API server crate:

- **Stateless routing**: Each request is independent
- **Model pool**: Pre-loaded weights shared across threads
- **Rate limiting**: `tower-governor` protects against abuse
- **CORS**: Configurable access control

Endpoints:

| Path | Method | Description |
|------|--------|-------------|
| `/api/move` | POST | Get AI's next move |
| `/api/state` | GET | Get current state encoding |
| `/api/brain` | GET | List available brains |
| `/health` | GET | Health check |

### ttui (Binary)

The terminal UI crate:

- **Game display**: Render board using Ratatui
- **Input handling**: Keyboard controls
- **Brain selection**: Choose AI tier
- **Statistics**: Show game history and outcomes

### ttgui (Binary)

The desktop GUI crate:

- **GTK rendering**: Native desktop window
- **Game visualization**: Animated board states
- **Settings**: Configure brain and preferences
- **About**: Credits and documentation links

## Data Flow

### Game Flow

```
┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────────┐
│ ttweb   │────▶│ttserver │────▶│ttrustle │────▶│  assets │
│   UI    │     │  API    │     │  lib    │     │ weights │
└─────────┘     └─────────┘     └─────────┘     └─────────┘
```

### Training Flow

```
┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────────┐
│  ttrust │────▶│  tt     │────▶│tttraini │────▶│  assets │
│         │     │  traini │     │   ng    │     │ weights │
└─────────┘     └─────────┘     └─────────┘     └─────────┘
```

## Compile-Time Pruning

The `build.rs` script prunes the game tree:

```rust
// Build script: reduce from 19,683 to 5,478 states
fn prune_game_tree() {
    let mut states: HashMap<u32, u32> = HashMap::new();
    // Insert legal states only
    for state in legal_states() {
        states.insert(state, 1);
    }
    // Export as lookup table
    write_lookup_table(&states);
}
```

## Security Considerations

### API Layer

- **Payload limits**: 2KB max request body
- **Rate limiting**: 100 requests/minute per IP
- **CORS policy**: Configurable via env vars

### Server Configuration

```toml
# ttserver/Cargo.toml
[dependencies]
tower-governor = "0.6"     # Rate limiting
axum = { version = "0.7", features = ["cors"] }
```

## Offline-First Design

All components work without network:

1. **Baked weights**: No runtime downloads
2. **No external lookups**: Self-contained operation
3. **Deterministic output**: Same input = same output

## State Diagram

```
                    ┌─────────────┐
                    │   Startup   │
                    └──────┬──────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
        ▼                  ▼                  ▼
   ┌─────────┐      ┌─────────┐        ┌─────────┐
   │  Play   │      │ Train   │        │  Serve  │
   └────┬────┘      └────┬────┘        └────┬────┘
        │                │                  │
        └────────────────┼──────────────────┘
                         │
                         ▼
                   ┌─────────┐
                   │  Game   │
                   │  Loop   │
                   └─────────┘
```

## ASCII Diagrams

### Board Representation

```
  8 ┌───┬───┬───┐
    │ A │ B │ C │
  7 ├───┼───┼───┤
    │ D │ E │ F │
  6 ├───┼───┼───┤
    │ G │ H │ I │
  5 └───┴───┴───┘
    1   2   3
```

Ternary encoding (flattened):

```
Index:  0  1  2  3  4  5  6  7  8
Value: X  O  -  -  X  O  -  X  O
State:  1  2  0  0  1  2  0  1  2
Encoded: 120012012 (base 3)
Decimal:   4257 (state index)
```

### Memory Layout

```
┌────────────────────────────────────────┐
│  Matchbox[5478]                        │
│   ┌──────────────────────────────────┐ │
│   │  State 0: [(0,5), (1,5), (2,5)] │ │
│   │  State 1: [(0,6), (1,5), (2,5)] │ │
│   │  ...                             │ │
│   └──────────────────────────────────┘ │
└────────────────────────────────────────┘
```
