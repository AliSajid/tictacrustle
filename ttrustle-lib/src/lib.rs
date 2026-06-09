// SPDX-FileCopyrightText: 2022 - 2026 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! # ttrustle-lib
//!
//! The core library for Tic Tac Toe gameplay.
//!
//! This library provides the game logic including:
//! - Board management with a 3x3 grid
//! - Player management (X and O)
//! - Game state tracking
//! - Square value enumeration
//!
//! # Example
//!
//! ```rust
//! use ttrustle_lib::{Board, Player, Symbol, Square};
//!
//! let mut board = Board::new();
//! println!("{}", board);
//! ```

mod board;
mod game;
mod player;
mod square;
mod square_value;

pub use board::Board;
pub use game::Game;
pub use player::{Player, Symbol};
pub use square::Square;
pub use square_value::SquareValue;
