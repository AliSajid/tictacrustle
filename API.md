# API Reference

This document describes the `ttserver` API endpoints and data structures.

## Overview

The `ttserver` crate provides a stateless REST API for playing games against the MENACE AI. All requests are processed independently with no session state.

## Base URL

```
http://localhost:8080
```

Configure via `RUST_SERVER_PORT` environment variable.

## Endpoints

### Health Check

**GET /health**

Returns server health status.

**Response:**
```json
{
  "status": "ok",
  "timestamp": 1706123456
}
```

### Available Brains

**GET /api/brain**

Lists all available pre-trained brains.

**Response:**
```json
[
  {
    "name": "random",
    "tier": 0,
    "description": "Completely random play",
    "win_rate": 0.0
  },
  {
    "name": "standard",
    "tier": 4,
    "description": "Standard brain (medium difficulty)",
    "win_rate": 0.55
  },
  {
    "name": "expert",
    "tier": 9,
    "description": "Expert brain (advanced difficulty)",
    "win_rate": 0.85
  }
]
```

### Get AI Move

**POST /api/move**

Get the AI's next move for the current game state.

**Request:**
```json
{
  "board": [
    ["", "X", ""],
    ["", "", ""],
    ["", "O", ""]
  ],
  "brain": "standard"
}
```

Or use state encoding:
```json
{
  "state": 4257,
  "brain": "standard"
}
```

**Response:**
```json
{
  "move": 7,
  "position": 3,
  "row": 1,
  "column": 2,
  "board_after": [
    ["", "X", "X"],
    ["", "", ""],
    ["", "O", ""]
  ]
}
```

**Error Responses:**

400 Bad Request:
```json
{
  "error": "Invalid board state",
  "message": "Board must have 3x3 grid with X, O, or empty cells"
}
```

404 Not Found:
```json
{
  "error": "Brain not found",
  "available_brains": ["random", "standard", "expert"]
}
```

500 Internal Server Error:
```json
{
  "error": "Game over",
  "message": "Game has already ended"
}
```

### Get State Encoding

**POST /api/encode**

Convert a board state to ternary encoding.

**Request:**
```json
{
  "board": [
    ["X", "O", ""],
    ["", "", ""],
    ["", "O", "X"]
  ]
}
```

**Response:**
```json
{
  "state": 1234,
  "ternary": "120012011",
  "hash": "a3f7b2c1"
}
```

## Data Structures

### Board Representation

A 3x3 board represented as a 2D array:

```typescript
type Board = string[][];
```

Each cell contains:
- `"X"` for X marker
- `"O"` for O marker
- `""` for empty

### Move Response

```typescript
interface MoveResponse {
  move: number;          // 0-8 (flattened index)
  position: number;      // 0-8
  row: number;           // 0-2
  column: number;        // 0-2
  board_after: Board;    // Board state after this move
}
```

### Brain Info

```typescript
interface BrainInfo {
  name: string;
  tier: number;
  description: string;
  win_rate: number;      // 0.0-1.0
}
```

## Error Codes

| Code | Meaning |
|------|--------|
| 200 | Success |
| 400 | Invalid request (bad board format, unknown brain) |
| 404 | Resource not found |
| 500 | Internal server error |

## Rate Limiting

The API uses `tower-governor` for rate limiting:

- **Default**: 100 requests/minute per IP
- **Token bucket algorithm**
- Configurable via `RUST_RATE_LIMIT` env var

## CORS

CORS is enabled by default for development. Configure via:

```toml
# In ttserver/Cargo.toml
tower-cors = { version = "0.3", features = ["tracing"] }
```

## Authentication

The current implementation has no authentication. For production:

1. Add API key verification
2. Use reverse proxy (Cloudflare/Nginx)
3. Configure CORS headers

## Streaming Games

For real-time game streaming, make sequential `/api/move` requests:

```bash
# Start new game
curl -X POST http://localhost:8080/api/move \
  -H "Content-Type: application/json" \
  -d '{"board":[["","",""],["","",""],["","",""]], "brain":"standard"}'

# Make move
curl -X POST http://localhost:8080/api/move \
  -H "Content-Type: application/json" \
  -d '{"board":[["X","",""],["","",""],["","",""]], "brain":"standard"}'
```

## CLI Usage

Use the API via CLI:

```bash
# Play against AI
cargo run --bin ttrustle -- play --server http://localhost:8080

# Make a move programmatically
cargo run --bin ttrustle -- move \
  --server http://localhost:8080 \
  --board '["X", "", ""]' \
  --brain standard
```

## Examples

### Python

```python
import requests

# Get AI move
response = requests.post(
    "http://localhost:8080/api/move",
    json={
        "board": [
            ["X", "", ""],
            ["", "O", ""],
            ["", "", ""]
        ],
        "brain": "standard"
    }
)
print(response.json())
```

### JavaScript

```javascript
const response = await fetch(
  "http://localhost:8080/api/move",
  {
    method: "POST",
    headers: {"Content-Type": "application/json"},
    body: JSON.stringify({
      board: [
        ["X", "", ""],
        ["", "O", ""],
        ["", "", ""]
      ],
      brain: "standard"
    })
  }
);
const move = await response.json();
console.log(move);
```

### cURL

```bash
curl -X POST http://localhost:8080/api/move \
  -H "Content-Type: application/json" \
  -d '{"board":[["X","",""],["O","",""],["","",""]], "brain":"standard"}'
```

## SDKs

No official SDKs yet. The API is designed to be language-agnostic using JSON/HTTP.

## Future Endpoints

- `POST /api/game/initialize` - Start a new game
- `GET /api/game/{id}/state` - Get current game state
- `POST /api/game/{id}/move` - Make a move in game {id}
- `GET /api/stats` - Get aggregate statistics

## Reference Implementation

See `ttserver/src/main.rs` for the reference implementation.
