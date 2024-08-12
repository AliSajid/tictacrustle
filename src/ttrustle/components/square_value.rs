// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SquareValue {
    Empty,
    X,
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

    #[test]
    fn test_eq() {
        assert_eq!(SquareValue::X, SquareValue::X);
        assert_eq!(SquareValue::O, SquareValue::O);
        assert_eq!(SquareValue::Empty, SquareValue::Empty);
    }
}
