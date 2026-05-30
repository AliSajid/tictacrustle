# Playing

This guide shows you how to play Tic-Tac-Toe against the MENACE AI.

## Using the CLI

### List Available Brains

```bash
cargo run --bin ttrustle -- list-brains
```

Output:
```
Available brains:
  random        - Completely random play
  quick         - Quick brain (50k games)
  standard      - Standard brain (100k games)
  expert        - Expert brain (500k games)
```

### Play a Game

```bash
# Play against random
cargo run --bin ttrustle -- play --brain random

# Play against standard
cargo run --bin ttrustle -- play --brain standard

# Play against expert
cargo run --bin ttrustle -- play --brain expert
```

### Command Options

```bash
cargo run --bin ttrustle -- play \
  --brain <name>           # Brain to play against
  --format <variant>       # MENACE-C or MENACE-S
  --moves <n>              # Number of moves (play until game ends)
  --verbose                # Show move weights and decisions
```

### Game Flow

```
┌─────────────────────────────────┐
│  1. CLI shows empty board       │
├─────────────────────────────────┤
│  2. You make your first move    │
├─────────────────────────────────┤
│  3. AI responds                 │
├─────────────────────────────────┤
│  4. Game continues...           │
├─────────────────────────────────┤
│  5. Game ends or you type 'q'  │
└─────────────────────────────────┘
```

## Using the API Server

### Start the Server

```bash
cargo run --bin ttserver
```

Server starts on port 8080 (configurable via `RUST_SERVER_PORT`).

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

Response:
```json
{
  "move": 7,
  "position": 3,
  "row": 1,
  "column": 2,
  "board_after": [
    ["", "X", ""],
    ["", "", "X"],
    ["", "O", ""]
  ]
}
```

### Streaming Game

Play a complete game by making sequential requests:

```bash
# Initialize board
curl -X POST http://localhost:8080/api/move \
  -d '{"board":[["","",""],["","",""],["","",""]], "brain":"standard"}'

# Make move 1
curl -X POST http://localhost:8080/api/move \
  -d '{"board":[["X","",""],["","",""],["","",""]], "brain":"standard"}'

# Make move 2
curl -X POST http://localhost:8080/api/move \
  -d '{"board":[["X","",""],["O","",""],["","",""]], "brain":"standard"}'
```

## Keyboard Controls

When playing via CLI:

- `q` - Quit game
- `r` - Replay last game
- `?` - Show help

## Visualizing the Game

```
Move 1:
┌───┬───┬───┐
│   │   │   │
├───┼───┼───┤
│   │   │   │
├───┼───┼───┤
│ X │   │   │
└───┴───┴───┘

AI chooses position 4
```

## Tips

1. **Try different brains**: Compare random vs standard vs expert
2. **Watch the pattern**: Notice how the AI blocks you
3. **Set up traps**: Can you create a fork (two winning threats)?
4. **Play multiple games**: See how consistent each brain is

## Troubleshooting

### Brain Not Found

```
error: Brain "my-brain" not found
Available brains: [random, standard, expert]
```

Solution: Train your own brain or use a pre-trained one.

### Server Not Running

```
Connection refused
```

Solution: Start the server first:
```bash
cargo run --bin ttserver
```

### Invalid Board Format

```
error: Invalid board state
```

Solution: Ensure board is a 3x3 grid with `"X"`, `"O"`, or `""`.

## Next Steps

- Read [The AI](./04-the-ai.md) to understand the AI
- Try [Training](./06-training.md) your own brain
- Explore [Advanced](./07-advanced.md) tips
