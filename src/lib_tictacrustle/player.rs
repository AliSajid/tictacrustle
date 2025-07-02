// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt;

/// Represents a player in the game of Tic Tac Toe.
///
/// Each player is identified by a `Symbol`—either `X` or `O`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    /// The symbol associated with the player (`X` or `O`).
    pub symbol: Symbol,
}

#[allow(dead_code)]
impl Player {
    /// Creates a new `Player` with the specified symbol.
    ///
    /// # Arguments
    ///
    /// * `symbol` - A string representing the player's symbol. Must be `"X"` or `"O"`.
    ///
    /// # Returns
    ///
    /// A `Player` with the corresponding `Symbol`.
    ///
    /// # Panics
    ///
    /// Panics if the provided symbol is not `"X"` or `"O"`.
    ///
    /// # Examples
    ///
    /// ```
    /// let player_x = Player::new("X");
    /// assert_eq!(player_x.symbol, Symbol::X);
    /// ```
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
    /// Formats the player as a string for display.
    ///
    /// # Examples
    ///
    /// ```
    /// let player = Player::new("X");
    /// println!("{}", player); // Outputs: Player X
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player {}", self.symbol)
    }
}

/// Represents a Tic Tac Toe symbol used by a player.
///
/// The symbol is either `X` or `O`, and is used to mark moves on the board.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    /// The `X` symbol, typically used by the first player.
    X,
    /// The `O` symbol, typically used by the second player.
    O,
}

impl fmt::Display for Symbol {
    /// Formats the symbol as either "X" or "O".
    ///
    /// # Examples
    ///
    /// ```
    /// let sym = Symbol::X;
    /// println!("{}", sym); // Outputs: X
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}
