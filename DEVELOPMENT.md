# Developer Guide

This guide helps you get started contributing to Tic-Tac-Rustle.

## Setting Up Your Environment

### Prerequisites

- Rust 1.85.1 or later
- `mise` for toolchain management
- Git for version control

### Quick Setup

```bash
# Clone the repo
git clone https://github.com/AliSajid/tictacrustle.git
cd tictacrustle

# Install toolchain
mise use -t ttrustle

# Build and test
cargo test --workspace
```

### Development Mode

```bash
# Run development server (if applicable)
mise run dev

# Run specific crate
cargo run --bin ttrustle

# Run tests
cargo test
```

## Crate Development

### Working on Core Logic

Edit `ttrustle-lib/`:

```bash
# Edit core library
cd ttrustle-lib/src/

# Add a new board state validation function
fn validate_board_state(state: u32) -> bool {
    // Implementation
}

# Run tests for this crate
cargo test --package ttrustle-lib

# Run clippy
cargo clippy --package ttrustle-lib
```

### Working on CLI

Edit `ttrustle/`:

```bash
# Edit CLI
cd ttrustle/src/

# Run the CLI
cargo run

# Add a new command
fn train_brain() { /* ... */ }
```

### Working on Training

Edit `tttraining/`:

```bash
# Edit training crate
cd tttraining/src/

# Run training
cargo run --bin tttraining -- train --iterations 10000

# Check training logs
ls assets/*.log
```

### Working on Server

Edit `ttserver/`:

```bash
# Edit server
cd ttserver/src/

# Run server
cargo run

# Test API endpoints
curl http://localhost:8080/health
```

## Testing

### Unit Tests

```bash
# Run tests for specific crate
cargo test -p ttrustle-lib

# Run all tests
cargo test --workspace

# Run with coverage
cargo llvm-cov --workspace
```

### Integration Tests

Tests live in `tests/`:

```bash
# Run integration tests
cargo test --test integration

# Run specific test
cargo test --test integration -- game_flow
```

### Benchmarks

```bash
# Run benchmarks
cargo bench --package ttrustle-lib

# Compare with previous run
cargo bench --bench game_scanner
```

## Code Style

### Formatting

```bash
# Format all code
cargo fmt --workspace

# Check formatting
cargo fmt --check
```

### Linting

```bash
# Run clippy
cargo clippy --workspace --all-targets

# Fix clippy warnings
cargo clippy --fix --workspace
```

### Conventional Commits

Commit messages should follow Conventional Commits:

```
feat: add new training iteration parameter
fix: correct board state validation for edge case
docs: update ARCHITECTURE.md with data flow diagram
test: add integration test for game loop
perf: reduce memory usage in board representation
chore: update dependencies to latest versions
```

### Example Commit

```bash
git add ttrustle-lib/src/board.rs
git commit -m "fix: correct board state validation for edge case

- Handle symmetric board states correctly
- Add test case for rotational symmetry
- Update unit tests to cover edge cases"
```

## Building

### Debug Build

```bash
cargo build --workspace
```

### Release Build

```bash
cargo build --workspace --release
```

### Cross-Compilation

```bash
# Build for Linux
RUSTFLAGS="-C target-feature=+sse4.2" cargo build --target x86_64-unknown-linux-gnu

# Build for Windows
cargo build --target x86_64-pc-windows-msvc
```

## Documentation

### Building Docs

```bash
# Build all docs
cargo doc --workspace --no-deps

# Open in browser
open target/doc/ttrustle/
```

### Writing Doc Comments

Use rustdoc-friendly comments:

```rust
/// Validates a board state is legal
///
/// A state is legal if:
/// - It has been reached from a valid initial state
/// - All cells are X, O, or empty
/// - X moves are always >= O moves
///
/// # Examples
///
/// ```
/// assert!(ttrustle_lib::board::is_valid_state(0));
/// ```
pub fn is_valid_state(state: u32) -> bool {
    // ...
}
```

## Debugging

### Logging

Add structured logging:

```rust
use tracing::{info, warn};

fn play_game() {
    info!("Starting game with brain={}", brain_name);
    // ...
    warn!("Brain ran out of moves, using default");
}
```

### Running with Tracing

```bash
# Enable tracing
RUST_LOG=info cargo run --bin ttrustle

# Filter by module
RUST_LOG=ttrustle_lib=debug cargo run --bin ttrustle
```

### Debugging Training

```bash
# Run training with verbose output
cargo run --bin tttraining -- train --iterations 1000 --verbose

# Check weight distribution
base64 -d assets/test.base64 | awk '{print $1, $2}' | sort -k2 -n
```

## Review Process

### Before Submitting

1. **Run tests**: `cargo test --workspace`
2. **Run clippy**: `cargo clippy --workspace`
3. **Format code**: `cargo fmt`
4. **Update docs**: Add rustdoc comments for new public APIs
5. **Check benchmarks**: Ensure performance hasn't regressed

### PR Guidelines

- Keep PRs focused on one change
- Link related issues
- Include relevant tests
- Update documentation as needed

## Common Tasks

### Adding a New Command to CLI

```rust
// In ttrustle/src/main.rs

#[derive(Subcommand)]
pub enum Command {
    #[command(name = "train-brain")]
    TrainBrain {
        /// Brain name
        #[arg(short, long)]
        name: String,
    },
}

fn handle_train_brain(args: TrainBrain) {
    // Implementation
}
```

### Adding a New Trait to Core Library

```rust
// In ttrustle-lib/src/lib.rs

pub trait GameStrategy {
    /// Get the next move for the given state
    fn get_next_move(&self, state: u32) -> u8;
    
    /// Get current win rate
    fn win_rate(&self) -> f64;
}

// Implementation
impl GameStrategy for MenaceBrain {
    fn get_next_move(&self, state: u32) -> u8 {
        // ...
    }
    
    fn win_rate(&self) -> f64 {
        // ...
    }
}
```

### Adding a New Brain Tier

```bash
# Train a new brain
cargo run --bin tttraining -- train \
  --name "tier-12" \
  --iterations 1000000 \
  --format MENACE-C
```

## Troubleshooting

### Build Failures

```bash
# Clean and rebuild
cargo clean
cargo build --workspace

# Check for missing dependencies
mise sync
```

### Test Failures

```bash
# Run specific failing test
cargo test --package ttrustle-lib -- <test_name>

# Update dependencies
mise update
```

### CI Failures

Check GitHub Actions logs at:
https://github.com/AliSajid/tictacrustle/actions

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum documentation](https://docs.rs/axum)
- [Ratatui documentation](https://docs.rs/ratatui)
- [Tracing book](https://tracing.rs/guide/)
- [Conventional Commits](https://www.conventionalcommits.org/)
