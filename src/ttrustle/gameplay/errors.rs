// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt::{Display, Error, Formatter};

#[allow(dead_code)]
#[derive(Debug)]
pub enum GameError {
    SquareAlreadyPlayed,
    InvalidSquare,
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
