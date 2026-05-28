// SPDX-FileCopyrightText: 2022 - 2026 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt::{Display, Error, Formatter};

/// Errors that can occur during game play.
#[derive(Debug)]
pub enum GameError {
    /// Attempting to play on a square that already has a piece.
    SquareAlreadyPlayed,
    /// Attempting to access an invalid square coordinate.
    InvalidSquare,
    /// The game has already ended (win or draw).
    GameAlreadyWon,
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self {
            GameError::SquareAlreadyPlayed => write!(f, "Square already played"),
            GameError::InvalidSquare => write!(f, "Invalid square"),
            GameError::GameAlreadyWon => write!(f, "Game already won"),
        }
    }
}
