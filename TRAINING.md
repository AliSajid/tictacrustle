# Training Guide

This document explains how the `tttraining` crate works and how to use it.

## What is Training?

Training generates the **MENACE memory structure** that gets embedded in each binary. The crate runs simulations where MENACE plays against itself (or random players), adjusting its decision weights based on outcomes.

## How Training Works

### The Simulation Loop

```
┌─────────────────────────────────────────────────┐
│              TRAINING WORKFLOW                   │
├─────────────────────────────────────────────────┤
│                                                 │
│  1. Initialize:                                 │
│     - Create matchboxes for all states          │
│     - Set base weights (e.g., 5 per move)       │
│                                                 │
│  2. Simulate:                                   │
│     - AI plays against random or itself         │
│     - Track outcomes                            │
│     - Update weights                            │
│                                                 │
│  3. Export:                                     │
│     - Write weights to assets/*.txt             │
│     - Format as base64 for embedding            │
│                                                 │
│  4. Verify:                                     │
│     - Check export size                         │
│     - Validate base64 encoding                  │
└─────────────────────────────────────────────────┘
```

### Command-Line Interface

```bash
# Train a new brain
cargo run --bin tttraining -- train \
  --name "my-brain" \
  --iterations 1000000 \
  --output assets/my-brain.txt

# Train with different parameters
cargo run --bin tttraining -- train \
  --name "menace-c-random-vs" \
  --iterations 500000 \
  --vs random  # Play against random
```

### Training Options

```
--name <name>           Name for this brain (max 32 chars)
--iterations <count>    Number of training games (default: 100000)
--vs <opponent>         Opponent: random|ai
--brain <tier>          Opponent tier (0-11, default: random)
--format <variant>      MENACE-C or MENACE-S
--seed <number>         Random seed for reproducibility
--output <path>         Output file path (default: ./assets/*.txt)
--prune                 Remove weights below minimum (0)
--verbose               Show progress during training
```

### Training Progress

```
[████████░░░░░░░░░░] 60% (600000/1000000 games)
  Current state: X-O- -  -X- -X-
  Winning moves: [2, 4, 7]
  Repeating weights...
```

## Generated Assets

Training outputs:

1. **Base weights file**: Raw weight data
2. **Base64 encoded file**: For binary embedding
3. **Training log**: Game outcomes and weight changes

```
assets/
├── {name}.txt              # Raw weights
├── {name}.base64           # Encoded weights
└── {name}.log              # Training log
```

## Verification

After training, verify the output:

```bash
# Check file exists and has content
ls -lh assets/*.base64

# Decode and inspect first few lines
base64 -d assets/my-brain.base64 | head -20

# Run a quick test game
cargo run --bin ttrustle -- play --brain my-brain
```

## Multiple Brains

You can train multiple brains for different use cases:

```bash
# Quick brain (50k games, good for casual play)
cargo run --bin tttraining -- train \
  --name "quick" \
  --iterations 50000

# Standard brain (100k games, default)
cargo run --bin tttraining -- train \
  --name "standard" \
  --iterations 100000

# Full brain (500k games, for serious play)
cargo run --bin tttraining -- train \
  --name "full" \
  --iterations 500000
```

## Integration with Build

The training crate integrates with the build pipeline:

1. **Post-build script**: Runs training after compilation
2. **Assets copy**: Copies generated files to output directory
3. **Embedding**: Bakes weights into binary

## Troubleshooting

### Training Takes Too Long

Reduce iterations or use fewer CPU cores:

```bash
# Limit to 4 threads
RUST_NUM_THREADS=4 cargo run --bin tttraining -- train ...
```

### Memory Issues

Reduce state space:

```bash
# MENACE-C is smaller than MENACE-S
cargo run --bin tttraining -- train --format MENACE-C ...
```

### Weight Distribution

Check if weights are too sparse:

```bash
# Inspect weight distribution
base64 -d assets/my-brain.base64 | awk '{print $1, $2}' | sort -t' ' -k2 -n | head -10
```

## Next Steps

After training:

1. **Test the brain**: Play games against it
2. **Compare brains**: Use different tiers to test capabilities
3. **Export visualization**: Generate graphs of weight evolution
4. **Deploy**: Use in ttserver or embed in ttui
