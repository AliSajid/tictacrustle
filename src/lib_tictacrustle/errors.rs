// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt::{
    Display,
    Error,
    Formatter,
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum GameError {
    SquareAlreadyX,
    SquareAlreadyO,
    SquareNotEmpty,
    SquareAlreadyPlayed,
    InvalidSquare,
    GameAlreadyWon,
    InvalidTriple,
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self {
            Self::SquareAlreadyPlayed => write!(f, "Square already played"),
            Self::InvalidSquare => write!(f, "Invalid square"),
            Self::GameAlreadyWon => write!(f, "Game already won"),
            Self::SquareAlreadyX => write!(f, "Square already X"),
            Self::SquareAlreadyO => write!(f, "Square already O"),
            Self::SquareNotEmpty => write!(f, "Square not empty"),
            Self::InvalidTriple => write!(f, "Invalid triple"),
        }
    }
}
