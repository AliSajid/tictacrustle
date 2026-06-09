// SPDX-FileCopyrightText: 2022 - 2026 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt;

/// The possible values for a square on the board.
///
/// This enum represents the state of a single square: empty, X, or O.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SquareValue {
    /// An empty square with no piece.
    Empty,
    /// A square occupied by X.
    X,
    /// A square occupied by O.
    O,
}

impl fmt::Display for SquareValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SquareValue::X => write!(f, " X "),
            SquareValue::O => write!(f, " O "),
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
}
