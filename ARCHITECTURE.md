# tic tac rustle Architecture

`t tic tac rustle` is a polyglot monorepo that packs a multi-crate **Cargo Workspace** for backend logic and an independent **SvelteKit** app for the frontend. Everything talks to each other cleanly with strict type contracts.

## Structure

```text
tic_tac_rustle/
├── mise.toml                    # Orchestration and task dependencies
├── Cargo.toml                   # Root Cargo Workspace definition
├── assets/                      # Pre-trained model weights (baked in)
├── tttui/                       # Terminal-based UI (Rust + TUI crate)
├── ttgui/                       # Desktop GUI (GTK-based frontend)
├── ttserver/                    # Fast Axum API server (read-only, stateless)
├── ttweb/                       # SvelteKit/Vite web frontend
├── ttrustle-lib/                # Shared library crate (core engine logic)
└── ttrustle/                    # CLI for offline simulation + checkpoint generation
```

## Core Systems

### Ternary Board Representation
The board is a flattened 9-digit base-3 number stored in a u16 register. Each cell can be:

- 0 (Empty)
- 1 (X)
- 2 (O)

Maximum theoretical states: 19,683 (3⁹).  
Legal achievable states: 5,478 (pruned by compile-time rules in `build.rs`).

### Strict Architectural Contracts (`ttrustle-lib`)
Two traits govern the engine:

- **GameScanner**: Encapsulates pure, stateless domain rules (valid states, legal moves, outcomes). Designed to be verifiable with property-based tests.
- **EducableEngine**: Defines interfaces for move selection, reinforcement backpropagation, and binary blob serialization for model export.

### Offline Compilation Pipeline
`traintrustle` runs offline simulations to train the AI across 11 evolutionary epochs. The trained models are written to `/assets/*.txt` in base64 format. `ttserver` and `ttui` compile these into binary during build time.

## Dataflow Diagram

```
┌────────────────────────────────────────────────────────────────────────────────┐
│                        TIC TAC RUSTLE ECOSYSTEM                                 │
├────────────────────────────────────────────────────────────────────────────────┤
│                                                                                  │
│  ┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐              │
│  │  ttweb   │────▶│ ttserver │────▶│ttrustle- │────▶│  assets  │              │
│  │ (Web)    │     │ (Axum)   │     │   lib     │     │ (Weights) │              │
│  └──────────┘     │          │     │   Engine  │     │ (Baked-in)│              │
│                   └──────────┘     └──────────┘     └──────────┘              │
│                        ▲               ▲                                      │
│                        │               │                                       │
│                   ┌────┴─────┐  ┌──────┴──────┐                                │
│                   │ tttui    │  │  ttgui      │                                │
│                   │ (TUI)    │  │  (GTK GUI)  │                                │
│                   └──────────┘  └─────────────┘                                │
│                                                                                  │
│  All components are offline-first: no external lookups, no network dependencies│
│  Models are baked into binaries at compile time                                  │
└────────────────────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

- **ttweb**: Browser-based frontend (SvelteKit) for web clients
- **tttui**: Terminal-based UI (Rust TUI crate) for CLI users
- **ttgui**: Desktop graphical UI (GTK-based) for advanced users
- **ttserver**: Stateless Axum API that routes requests to `ttrustle-lib`
- **tttrustle-lib**: Core engine library with ternary encoding, GameScanner, and EducableEngine traits
- **assets**: Model weights embedded in binaries (no runtime network calls)

## Offline-First Data Integrity

`t tic tac rustle` operates in an **offline-first** architecture:

1. **Baked-in Weights**: All trained model weights are compiled directly into the binary at build time. No external model files or runtime downloads are required.

2. **No Network Lookups**: The engine never performs HTTP requests to fetch models or configuration. Every component is self-contained.

3. **Air-Gappable Deployment**: Deploy the binary to any machine and it works immediately, even on isolated networks.

4. **Deterministic Behavior**: With all weights embedded, the engine produces identical outputs across different machines and times.

## Stateless Server Architecture

The **ttserver** component maintains a strictly stateless design:

- **No Session State**: No user sessions or game state are persisted on the server
- **Shared Model Pool**: Model weights are loaded once at startup and shared across threads
- **Read-Only API**: The server never modifies weights or configuration at runtime
- **Scale-Friendly**: Add more server instances behind a load balancer without coordination overhead
- **Fast Startup**: On each restart, only the embedded weights need to be deserialized

## API & Network Contract
The API between `ttweb` and `ttserver` is stateless, avoiding session-locking overhead.

**Move Request (POST /api/move)**  
JSON body:
```json
{
  "current_state": 163,
  "brain_tier": 4
}
```

**Move Response (200 OK)**  
JSON body:
```json
{
  "next_move": 4
}
```

## Security Hardening
- **Edge Layer**: Everything is behind a reverse proxy (Cloudflare/Nginx) to scrub volumetric DDoS attacks and handle TLS termination.
- **Payload Isolation**: Axum's body-parser middleware limits requests to a maximum of 2KB to prevent oversized payloads.
- **Rate Limiting**: `tower-governor` throttles requests by IP using token-bucket algorithms.
- **Client Integrity**: SvelteKit enforces strict CSP to minimize XSS vulnerabilities.
