// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt;

/// Enum `SquareValue` represents the state of a square in a Tic-Tac-Toe game.
///
/// A square on the Tic-Tac-Toe board can be in one of three states: it can be empty, or it can
/// contain an 'X' or an 'O'. This enum provides a type-safe way to represent these states in the
/// game logic.
///
/// # Examples
///
/// ```
/// use tictacrustle::SquareValue;
///
/// // Create a square with an 'X'
/// let x_square = SquareValue::X;
/// assert_eq!(x_square, SquareValue::X);
///
/// // Create a square with an 'O'
/// let o_square = SquareValue::O;
/// assert_eq!(o_square, SquareValue::O);
///
/// // Create an empty square
/// let empty_square = SquareValue::Empty;
/// assert_eq!(empty_square, SquareValue::Empty);
/// ```
///
/// The `SquareValue` enum implements the `Display` trait, allowing it to be easily printed for
/// debugging or user interface purposes.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SquareValue {
    /// An empty square, which neither player has marked.
    Empty,
    /// A square marked by the 'X' player.
    X,
    /// A square marked by the 'O' player.
    O,
}

/// `SquareValue` is an enum that represents the possible values of a square in a Tic-Tac-Toe game.
/// It can be either `X`, `O`, or `Empty` if the square is not yet filled.
///
/// This `impl` block provides a way to display the `SquareValue` enum.
impl fmt::Display for SquareValue {
    /// Formats the `SquareValue` for display.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a `Formatter` instance.
    ///
    /// # Returns
    ///
    /// This function returns a `fmt::Result` which is an alias for `Result<(), Error>`.
    /// It returns `Ok` if the operation was successful and `Err` otherwise.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // If the `SquareValue` is `X`, it will display " X ".
            SquareValue::X => write!(f, " X "),
            // If the `SquareValue` is `O`, it will display " O ".
            SquareValue::O => write!(f, " O "),
            // If the `SquareValue` is `Empty`, it will display "   ".
            SquareValue::Empty => write!(f, "   "),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", SquareValue::X), " X ");
        assert_eq!(format!("{}", SquareValue::O), " O ");
        assert_eq!(format!("{}", SquareValue::Empty), "   ");
    }

    #[test]
    fn test_eq() {
        assert_eq!(SquareValue::X, SquareValue::X);
        assert_eq!(SquareValue::O, SquareValue::O);
        assert_eq!(SquareValue::Empty, SquareValue::Empty);
    }
}
