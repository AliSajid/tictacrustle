// SPDX-FileCopyrightText: 2022 - 2026 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::cmp::{Eq, PartialEq};
use std::fmt::{self, Display, Formatter};

use crate::square_value::SquareValue;
use crate::Symbol;

/// A single square on the Tic Tac Toe board.
///
/// Each square can be empty, contain an X, or contain an O.
#[derive(Debug, Clone, Copy)]
pub struct Square {
    value: SquareValue,
}

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Square {}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Square {
    /// Creates a new empty square.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            value: SquareValue::Empty,
        }
    }

    /// Creates a new square with the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The square value (Empty, X, or O).
    #[must_use]
    pub const fn with_value(value: SquareValue) -> Self {
        Self { value }
    }

    /// Returns true if the square is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self.value, SquareValue::Empty)
    }

    /// Returns the value of this square.
    #[must_use]
    pub const fn get_value(&self) -> SquareValue {
        self.value
    }

    /// Sets the square to contain X.
    #[must_use]
    pub const fn set_x(&mut self) -> &mut Self {
        self.value = SquareValue::X;
        self
    }

    /// Sets the square to contain O.
    #[must_use]
    pub const fn set_o(&mut self) -> &mut Self {
        self.value = SquareValue::O;
        self
    }

    /// Sets the square to empty.
    #[must_use]
    pub const fn set_empty(&mut self) -> &mut Self {
        self.value = SquareValue::Empty;
        self
    }

    /// Sets the square with the given symbol.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Either `Symbol::X` or `Symbol::O`.
    pub fn set_value(&mut self, symbol: Symbol) {
        match symbol {
            Symbol::X => self.value = SquareValue::X,
            Symbol::O => self.value = SquareValue::O,
        }
    }

    /// Sets the square with the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The square value (Empty, X, or O).
    pub fn set_square_value(&mut self, value: SquareValue) {
        self.value = value;
    }
}

impl Default for Square {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let square = Square::new();
        assert_eq!(square.value, SquareValue::Empty);
    }

    #[test]
    fn test_is_empty() {
        let square = Square::new();
        assert!(square.is_empty());
    }

    #[test]
    fn test_set_x() {
        let mut square = Square::new();
        square.set_x();
        assert_eq!(square.get_value(), SquareValue::X);
    }

    #[test]
    fn test_set_o() {
        let mut square = Square::new();
        square.set_o();
        assert_eq!(square.get_value(), SquareValue::O);
    }

    #[test]
    fn test_set_empty() {
        let mut square = Square::new();
        square.set_x();
        square.set_empty();
        assert_eq!(square.get_value(), SquareValue::Empty);
    }

    #[test]
    fn test_default() {
        let square = Square::default();
        assert_eq!(square.value, SquareValue::Empty);
    }
}
