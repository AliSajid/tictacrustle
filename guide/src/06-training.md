# Training

This guide explains how to train your own MENACE brain.

## What Is Training?

Training runs simulations where MENACE plays against itself (or random players), adjusting its decision weights based on outcomes. The trained weights are embedded in the final binary.

## Basic Training

```bash
# Train a new brain
cargo run --bin tttraining -- train \
  --name my-brain \
  --iterations 100000

# Train with different opponent
cargo run --bin tttraining -- train \
  --name my-brain \
  --iterations 100000 \
  --vs random  # Play against random
```

## Command Options

```bash
cargo run --bin tttraining -- train \
  --name <name>            # Brain name (max 32 chars)
  --iterations <count>     # Number of games (default: 100000)
  --vs <opponent>          # Opponent: random|ai
  --brain <tier>           # Opponent tier (0-11)
  --format <variant>       # MENACE-C or MENACE-S
  --seed <number>          # Random seed
  --output <path>          # Output file path
  --prune                  # Remove low weights
  --verbose                # Show progress
```

## Training Workflow

```
┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐
│  Start  │───▶│  Simulate│───▶│ Adjust  │───▶│ Export  │
│         │    │  Games  │    │Weights  │    │  Assets │
└─────────┘    └─────────┘    └─────────┘    └─────────┘
```

### Step 1: Initialize

Create matchboxes with base weights:

```
Each state: 5 beads for each possible move
```

### Step 2: Simulate

Run N games:

```
For each game:
  1. AI plays against opponent
  2. Track outcomes
  3. Apply reinforcement
```

### Step 3: Adjust Weights

```
AI wins: +2 to winning move, +1 to others
Draw: +1 to all moves
AI loses: -1 from losing move (min 0)
```

### Step 4: Export

```
Write weights to assets/
Format as base64 for embedding
```

## Output Files

```
assets/
├── my-brain.txt          # Raw weights
├── my-brain.base64       # Encoded weights
└── my-brain.log          # Training log
```

## Training Options

### Quick Brain (Casual Play)

```bash
cargo run --bin tttraining -- train \
  --name quick \
  --iterations 50000
```

### Standard Brain (Default)

```bash
cargo run --bin tttraining -- train \
  --name standard \
  --iterations 100000
```

### Full Brain (Serious Play)

```bash
cargo run --bin tttraining -- train \
  --name full \
  --iterations 500000
```

### Training Against AI

```bash
cargo run --bin tttraining -- train \
  --name trained-brain \
  --iterations 100000 \
  --vs ai \
  --brain 10  # Play against tier-10 brain
```

## Verifying Training

```bash
# Check output exists
ls -lh assets/*.base64

# Inspect weight distribution
base64 -d assets/my-brain.base64 | awk '{print $1, $2}' | \
  sort -t' ' -k2 -n | head -10

# Test with CLI
cargo run --bin ttrustle -- play --brain my-brain
```

## Progress Output

```
[██████████░░░░░░░░] 75% (750000/1000000 games)
  Current state: X-O- -  -X- -X-
  Winning moves: [2, 4, 7]
  Repeating weights...
```

## Tips

1. **Start small**: Train with 10k iterations first
2. **Watch progress**: Use `--verbose` to see output
3. **Check logs**: Review `my-brain.log` for issues
4. **Compare brains**: Use different iteration counts

## Troubleshooting

### Training Too Slow

```bash
# Limit threads
RUST_NUM_THREADS=4 cargo run --bin tttraining -- train ...
```

### Memory Issues

```bash
# Use MENACE-C (smaller state space)
cargo run --bin tttraining -- train --format MENACE-C ...
```

### Weight Distribution

Check if weights are too sparse:

```bash
# Inspect weight distribution
base64 -d assets/my-brain.base64 | \
  awk '{print $1, $2}' | sort -t' ' -k2 -n | head -10
```

## Next Steps

After training:

1. Test the brain with various opponents
2. Compare with pre-trained brains
3. Export visualization of weight evolution
4. Deploy in ttserver or embed in ttui
