// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt;

/// The Player
#[derive(Debug, Clone, Copy)]
pub struct Player {
    /// The symbol of the player
    pub symbol: Symbol,
}

#[allow(dead_code)]
impl Player {
    /// Create a new player
    #[must_use]
    pub fn new(symbol: &str) -> Self {
        match symbol {
            "X" => Self { symbol: Symbol::X },
            "O" => Self { symbol: Symbol::O },
            _ => panic!("Invalid symbol"),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player {}", self.symbol)
    }
}

/// The Symbol
#[derive(Debug, Clone, Copy)]
pub enum Symbol {
    /// The X symbol
    X,
    /// The O symbol
    O,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}
