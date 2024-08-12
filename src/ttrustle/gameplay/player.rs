// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub symbol: Symbol,
}

#[allow(dead_code)]
impl Player {
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

#[derive(Debug, Clone, Copy)]
pub enum Symbol {
    X,
    O,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::X => write!(f, "X"),
            Symbol::O => write!(f, "O"),
        }
    }
}
