# Advanced

This document covers advanced topics and tips for using Tic-Tac-Rustle.

## Weight Analysis

### Inspecting Weights

```bash
# Decode weights
base64 -d assets/my-brain.base64

# Format for analysis
base64 -d assets/my-brain.base64 | \
  awk '{print $1, $2}' | sort -t' ' -k2 -n
```

### Visualizing Distribution

```bash
# Show top 10 lowest weights
base64 -d assets/my-brain.base64 | \
  awk '{print $1, $2}' | sort -t' ' -k2 -n | head -10

# Show top 10 highest weights
base64 -d assets/my-brain.base64 | \
  awk '{print $1, $2}' | sort -t' ' -k2 -rn | head -10
```

## Training Strategies

### Progressive Training

Train incrementally and combine results:

```bash
# Phase 1: Random training
cargo run --bin tttraining -- train --name phase1 --iterations 50000

# Phase 2: AI training
cargo run --bin tttraining -- train --name phase2 --iterations 50000 \
  --vs ai --brain 5

# Combine weights (advanced, see code)
```

### Multi-Brain Comparison

```bash
# Train multiple brains
cargo run --bin tttraining -- train --name random --iterations 10000
cargo run --bin tttraining -- train --name standard --iterations 100000
cargo run --bin tttraining -- train --name expert --iterations 500000

# Compare win rates
ttrustle --list-brains
```

## Server Configuration

### Environment Variables

```bash
export RUST_SERVER_PORT=8080
export RUST_RATE_LIMIT=100  # requests/minute
export RUST_CORS_ORIGIN="*"
```

### Production Settings

```bash
# Disable CORS in production
export RUST_CORS_ORIGIN=""

# Enable logging
export RUST_LOG=info

# Enable metrics
export RUST_METRICS_ENABLED=true
```

## Benchmarking

```bash
# Run benchmarks
cargo bench --package ttrustle-lib

# Compare performance
cargo bench --bench game_scanner

# Benchmark training
cargo bench --bench training_loop
```

## Custom Brains

### Modifying Weights

```rust
// Edit weights directly
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut file = File::open("assets/my-brain.base64").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    // Parse and modify
    // Re-encode and save
}
```

### Creating Custom Brains

1. Start with a pre-trained brain
2. Adjust specific weights
3. Export and test

## Multi-Agent Simulations

Train multiple agents and observe emergent behavior:

```bash
# Train agents with different seeds
cargo run --bin tttraining -- train --name agent1 --iterations 100000 --seed 42
cargo run --bin tttraining -- train --name agent2 --iterations 100000 --seed 123
cargo run --bin tttraining -- train --name agent3 --iterations 100000 --seed 456
```

## Performance Tips

### Memory Optimization

```bash
# Use MENACE-C (smaller)
--format MENACE-C

# Enable pruning
--prune
```

### CPU Optimization

```bash
# Limit threads
RUST_NUM_THREADS=4

# Use SIMD
RUSTFLAGS="-C target-feature=+sse4.2"
```

### Disk I/O

```bash
# Use SSD for assets directory
# Enable async I/O where possible
```

## Security Considerations

### API Rate Limiting

```toml
# In ttserver/Cargo.toml
tower-governor = "0.6"
```

### Request Validation

All requests are validated:
- Board must be 3x3 grid
- Cells must be X, O, or empty
- Brain must exist
- Game must be active

### CORS Configuration

```toml
# Development
tower-cors = { version = "0.3" }

# Production
# Configure specific origins
```

## Debugging

### Enable Logging

```bash
RUST_LOG=debug cargo run --bin ttrustle
```

### Trace Module Execution

```bash
RUST_LOG=trace cargo run --bin ttrustle
```

### Inspect State

```rust
use ttrustle_lib::board::Board;

let board = Board::new();
println!("{:?}", board.state);  // Debug output
```

## References

- [MENACE Wikipedia](https://en.wikipedia.org/wiki/MENACE)
- [Original Michie paper](https://www.iet.org.uk/content/9780863414658)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum documentation](https://docs.rs/axum)
