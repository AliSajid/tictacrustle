// SPDX-FileCopyrightText: 2022 - 2026 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! # ttserver - HTTP API Server
//!
//! An HTTP API server for the Tic Tac Toe game.

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::sync::RwLock;
use ttrustle_lib::{Board, SquareValue};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// API state shared across routes.
#[derive(Clone)]
pub struct AppState {
    /// The current game board.
    pub board: RwLock<Board>,
    /// The current game status.
    pub status: RwLock<String>,
}

/// Represents a board state for the API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BoardState {
    /// The encoded board state.
    pub encoded: String,
    /// The status of the game.
    pub status: String,
    /// The current winner, if any.
    pub winner: Option<String>,
}

impl BoardState {
    fn new(board: &Board, status: &str, winner: Option<&str>) -> Self {
        Self {
            encoded: Board::encode(board),
            status: status.to_string(),
            winner: winner.map(|s| s.to_string()),
        }
    }
}

/// API response types.
type ApiError = (StatusCode, Json<serde_json::Value>);

/// Error response type.
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ttserver=debug,axum=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting TT Server API...");

    // Create initial game state
    let mut board = Board::new();
    let status = "playing".to_string();

    // Create shared state
    let state = State(AppState {
        board: RwLock::new(board),
        status: RwLock::new(status),
    });

    // Build router
    let router = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/board", get(get_board).post(create_board))
        .route("/move", post(make_move))
        .route("/reset", post(reset))
        .with_state(state);

    // Launch server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn index() -> &'static str {
    r#"
<!DOCTYPE html>
<html>
<head><title>TT Server API</title></head>
<body>
    <h1>TT Server API</h1>
    <p><a href="/health">/health</a> - Health check</p>
    <p><a href="/board">/board</a> - Get/Create board</p>
    <p><a href="/move">/move</a> - Make a move</p>
    <p><a href="/reset">/reset</a> - Reset the game</p>
</body>
</html>
"#
}

async fn health() -> &'static str {
    r#"{"status":"ok"}"#
}

async fn get_board(
    State(state): State<AppState>,
) -> Result<Json<BoardState>, ApiError> {
    let board = state.board.read().await;
    let status = state.status.read().await.clone();
    let winner = board.winner().map(|p| format!("{}", p));

    let response = BoardState::new(&board, &status, winner.as_deref());
    Ok(Json(response))
}

async fn create_board(
    State(state): State<AppState>,
) -> Result<Json<BoardState>, ApiError> {
    let mut board = state.board.write().await;
    let mut status = state.status.write().await.clone();

    // Reset board
    let board = Board::new();
    *board = board;

    *status = "playing".to_string();

    let response = BoardState::new(&board, &status, None);
    Ok(Json(response))
}

async fn make_move(
    State(state): State<AppState>,
    pos: Move,
) -> Result<Json<BoardState>, ApiError> {
    let mut board_state = state.board.write().await;
    let status = state.status.read().await.clone();

    // Check if game is still playing
    if status != "playing" {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Game is over".into() })));
    }

    // Validate position (1-indexed)
    let row = pos.row - 1;
    let col = pos.col - 1;

    if row >= 3 || col >= 3 {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Invalid position".into() })));
    }

    // Check if square is already played
    let square = &board_state.get_square(row + 1, col + 1);
    if square.get_value() != SquareValue::Empty {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Square already played".into() })));
    }

    // Place the move
    let player_symbol = match pos.player.as_str() {
        "X" => SquareValue::X,
        "O" => SquareValue::O,
        _ => return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Invalid player".into() }))),
    };
    board_state.get_square_mut(row + 1, col + 1).set_square_value(player_symbol);

    // Check for winner
    let winner = check_winner(&board_state);

    let response = BoardState::new(&board_state, &status, winner.as_deref());
    Ok(Json(response))
}

async fn reset(
    State(state): State<AppState>,
) -> Result<Json<BoardState>, ApiError> {
    let mut board = state.board.write().await;
    let mut status = state.status.write().await.clone();

    *board = Board::new();
    *status = "playing".to_string();

    let response = BoardState::new(&board, &status, None);
    Ok(Json(response))
}

/// Check for a winner after a move.
fn check_winner(board: &Board) -> Option<String> {
    let squares = board.iter();

    // Check rows
    for row in squares {
        let row_values: Vec<SquareValue> = row.iter().map(|s| s.get_value()).collect();
        if row_values.iter().all(|v| *v != SquareValue::Empty)
            && row_values[0] == row_values[1]
            && row_values[1] == row_values[2]
        {
            return Some(format!("{}", row_values[0]));
        }
    }

    // Check columns
    for col in 0..3 {
        let mut col_values = [board.get_square(1, col + 1), board.get_square(2, col + 1), board.get_square(3, col + 1)];
        let col_values: Vec<SquareValue> = col_values.iter().map(|s| s.get_value()).collect();
        if col_values.iter().all(|v| *v != SquareValue::Empty)
            && col_values[0] == col_values[1]
            && col_values[1] == col_values[2]
        {
            return Some(format!("{}", col_values[0]));
        }
    }

    // Check diagonals
    let diag1 = [board.get_square(1, 1), board.get_square(2, 2), board.get_square(3, 3)];
    let diag1_values: Vec<SquareValue> = diag1.iter().map(|s| s.get_value()).collect();
    if diag1_values.iter().all(|v| *v != SquareValue::Empty)
        && diag1_values[0] == diag1_values[1]
        && diag1_values[1] == diag1_values[2]
    {
        return Some(format!("{}", diag1_values[0]));
    }

    let diag2 = [board.get_square(1, 3), board.get_square(2, 2), board.get_square(3, 1)];
    let diag2_values: Vec<SquareValue> = diag2.iter().map(|s| s.get_value()).collect();
    if diag2_values.iter().all(|v| *v != SquareValue::Empty)
        && diag2_values[0] == diag2_values[1]
        && diag2_values[1] == diag2_values[2]
    {
        return Some(format!("{}", diag2_values[0]));
    }

    None
}

/// Move position for API requests.
#[derive(Deserialize, Debug)]
struct Move {
    /// Row number (1-3).
    row: u8,
    /// Column number (1-3).
    col: u8,
    /// Player making the move.
    player: String,
}

async fn shutdown_signal() {
    tracing::info!("Shutting down...");
}
