# Workspace Structure

This workspace has been restructured from a monolithic crate into a modular Cargo workspace with individual crates.

## Structure

```
ttrustle/ (workspace root)
├── Cargo.toml                    # Workspace members, shared dependencies
├── mise.toml                     # Common tools, profiles, env vars
├── WORKSPACE.md                  # This file
├── PROJECT_OVERVIEW.md           # High-level overview (new)
├── ARCHITECTURE.md               # System architecture (updated)
├── TRAINING.md                   # Training guide (new)
├── DEVELOPMENT.md                # Developer guide (new)
├── guide/                        # User-facing documentation
│   └── src/
│       ├── SUMMARY.md
│       └── chapter_*.md
└── [crates]
    ├── ttrustle-lib/
    │   ├── Cargo.toml
    │   ├── mise.toml
    │   └── src/
    ├── ttrustle/
    │   ├── Cargo.toml
    │   ├── mise.toml
    │   └── src/
    ├── tttraining/               # Training crate (new)
    │   ├── Cargo.toml
    │   ├── mise.toml
    │   └── src/
    ├── tttui/
    │   ├── Cargo.toml
    │   └── src/
    ├── ttgui/
    │   ├── Cargo.toml
    │   └── src/
    ├── ttserver/
    │   ├── Cargo.toml
    │   └── src/
    └── ttweb/                    # SvelteKit frontend
       ├── package.json
       └── app/
```

## Why This Structure?

### 1. Separation of Concerns

Each crate has a single, well-defined responsibility:

| Crate | Responsibility |
|-------|---------------|
| `ttrustle-lib` | Core game logic and domain rules |
| `ttrustle` | CLI interface for offline operations |
| `tttraining` | Training simulations and weight generation |
| `ttserver` | API server (stateless, rate-limited) |
| `ttui` | Terminal UI (Ratatui-based) |
| `ttgui` | Desktop GUI (GTK-based) |

### 2. Independent Development

You can work on crates in parallel without conflicts:

```bash
# Work on core logic (won't affect UI crates)
cargo edit -p ttrustle-lib

# Work on CLI (doesn't touch server)
cargo edit -p ttrustle

# Work on training separately
cargo edit -p tttraining
```

### 3. Compile-Time Training

The `tttraining` crate runs simulations and generates weights that get embedded in binaries:

```
1. Run tttraining with specific iterations
2. Generate weight files in assets/
3. Build binaries embed weights
4. Run with zero dependencies
```

### 4. Profile Optimization

Different crates have different optimization needs:

```toml
# Library: allow dead_code for internal APIs
[profile.dev]
rustflags = ["-A dead_code"]

# Binary: strict warnings
[profile.dev]
rustflags = ["-D warnings"]

# Release: optimized for all crates
[profile.release]
opt-level = 3
lto = "thin"
```

## Workspace Root: `Cargo.toml`

```toml
[workspace]
resolver = "2"
members = ["ttrustle-lib", "ttrustle", "tttraining", "ttui", "ttgui", "ttserver"]
# ttweb is outside workspace (Node.js project)

[workspace.package]
authors = ["Ali Sajid Imami"]
categories = ["game", "ai", "tictactoe", "menace"]
description = "Tic Tac Toe game with MENACE AI"
edition = "2024"
license = "MIT OR Apache-2.0"
rust-version = "1.85.1"
version = "1.0.0-next.2"

[workspace.dependencies]
# Shared dependencies
anyhow = { version = "1.0.80", features = ["backtrace"] }
color-eyre = "0.6.5"
rand = "0.9.4"
serde = { version = "1.0.228", features = ["derive"] }
```

## Workspace Root: `mise.toml`

**Common tools** (shared across all crates):

```toml
[settings]
rust-version = "1.85.1"

[env."global"]
RUSTFLAGS = "-D warnings"
CLIPPY = "warn"

[tools]
cargo-audit = "latest"
cargo-llvm-cov = "latest"
cargo-nextest = "latest"
```

**Workspace profiles**:

```toml
[profiles]
dev.opt-level = 1
release.opt-level = "z"
release.lto = true
release.strip = true
```

## Per-Crate Settings

### ttrustle-lib (Library Crate)

```toml
[package]
name = "ttrustle-lib"

[lints]
rust.missing_docs = "allow"  # Library internals may not be used directly
```

```toml
[ttrustle-lib]
RUSTFLAGS = "-A dead_code -Wmissing_docs"
```

**Rationale**: Library allows `dead_code` since internal types may not be used by callers.

### ttrustle (Binary Crate)

```toml
[package]
name = "ttrustle"

[[bin]]
name = "ttrustle"
path = "src/main.rs"

[lints]
rust.missing_docs = "allow"
rust.unused_must_use = "allow"  # Allow ignoring const return values
```

```toml
[ttrustle]
RUSTFLAGS = "-D warnings"
```

**Rationale**: Binary crate uses stricter warnings for production code.

### tttraining (Training Crate - New)

```toml
[package]
name = "tttraining"

[lints]
rust.missing_docs = "allow"
```

```toml
[tttraining]
RUSTFLAGS = "-D warnings"
```

**Rationale**: Training crate is utility-focused; performance and correctness matter most.

### ttserver (Server Crate)

```toml
[package]
name = "ttserver"

[lints]
rust.missing_docs = "deny"  # API docs are important for maintainers
```

```toml
[ttserver]
RUSTFLAGS = "-D warnings -W clippy::dbg_macro"
```

**Rationale**: Server crate is production code; strict docs ensure API maintainability.

## Adding New Crates

Follow these steps to add a new crate:

1. **Create crate directory**:

```bash
mkdir -p new-crate/src
```

2. **Create `Cargo.toml`**:

```toml
[package]
name = "new-crate"
authors.workspace = true
edition.workspace = true
version.workspace = true
rust-version.workspace = true

[dependencies]
ttrustle-lib.workspace = true

[lints]
rust.missing_docs = "allow"
```

3. **Create `mise.toml`**:

```toml
[settings]
rust-version = "1.85.1"

[env."new-crate"]
RUSTFLAGS = "-D warnings"
```

4. **Add to root `Cargo.toml`**:

```toml
[workspace]
members = ["new-crate"]
```

5. **Add profile to root `mise.toml`**:

```toml
[profiles]
# Add new-crate profile here if needed
```

## Verification Commands

```bash
# Check workspace builds
cargo check --workspace

# Run tests (each crate in isolation)
cargo test --workspace

# Build release binaries
cargo build --workspace --release

# Check for clippy warnings
cargo clippy --workspace --all-targets

# Run benchmarks
cargo benchmark --workspace
```

## Crate Communication

Crates communicate via well-defined interfaces:

```rust
// Core library exports traits for other crates to implement
pub trait GameScanner { /* ... */ }
pub trait EducableEngine { /* ... */ }

// Server loads library and implements service traits
use ttrustle_lib::GameScanner;
// ...

// CLI uses library for simulation
use ttrustle_lib::GameScanner;
// ...
```

## Build Order

When building the workspace, this order is recommended:

```
1. ttrustle-lib    # Core logic first
2. tttraining      # Generate weights
3. ttrustle        # CLI depends on lib + assets
4. ttserver        # Server depends on lib
5. tttui           # TUI depends on lib
6. ttgui           # GUI depends on lib
```

The workspace handles dependencies automatically, but understanding this order helps when adding new crates.

## Directory Conventions

| Directory | Purpose |
|-----------|---------|
| `src/` | Source code |
| `assets/` | Pre-trained weights (generated by training) |
| `tests/` | Integration tests |
| `benches/` | Benchmarks |
| `examples/` | Usage examples |

## Files to Keep vs. Remove

After restructuring, decide which files to keep:

### Keep

- `README.md` - High-level overview and quick start
- `ARCHITECTURE.md` - System design and data flow
- `TRAINING.md` - Training guide (new)
- `PROJECT_OVERVIEW.md` - High-level overview (new)
- `WORKSPACE.md` - Workspace structure (updated)
- `guide/` - User-facing documentation

### Consider Removing or Moving

- `CONTRIBUTING.md` - Generic template, customize or move to `DEVELOPMENT.md`
- `ROADMAP.md` - Keep for planning, or move to wiki
- `LICENSES/report` - REUSE compliance, keep in repo root

### Create

- `DEVELOPMENT.md` - Developer onboarding guide (includes CONTRIBUTING.md content)
- `API.md` - API reference documentation
- `guide/src/` - Expand with user-facing content
