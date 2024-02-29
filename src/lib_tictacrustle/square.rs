// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt::{
    self,
    Display,
    Formatter,
};

use crate::SquareValue;

/// `Square` is a struct that represents a square in a Tic-Tac-Toe game.
///
/// Each square on the Tic-Tac-Toe board is represented by an instance of this struct. The `value`
/// field determines the current state of the square, which can be either
/// [`X`](enum.SquareValue.html#variant.X), [`O`](enum.SquareValue.html#variant.O), or
/// [`Empty`](enum.SquareValue.html#variant.Empty) as defined by the
/// [`SquareValue`](enum.SquareValue.html) enum.
///
/// # Examples
///
/// ```
/// use tictacrustle::{
///     Square,
///     SquareValue,
/// };
///
/// // Create an empty square
/// let empty_square = Square::new();
//# assert!(empty_square.is_empty());
/// // Create a square with an 'X'
/// let mut x_square = Square::new();
/// x_square.set_x();
//# assert!(x_square.is_x());
/// // Create a square with an 'O'
/// let mut o_square = Square::new();
/// assert!(o_square.is_empty());
//# assert_eq!(o_square.value, SquareValue::O);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Square {
    /// The current state of the square.
    /// This is a variant of the `SquareValue` enum.
    ///
    /// It can take one of the following values:
    /// - [`X`](enum.SquareValue.html#variant.X)
    /// - [`O`](enum.SquareValue.html#variant.O)
    /// - [`Empty`](enum.SquareValue.html#variant.Empty)
    value: SquareValue,
}

#[allow(dead_code)]
impl Square {
    /// Creates a new `Square` instance with an `Empty` value.
    ///
    /// This method is a constructor for the `Square` struct. It returns a new `Square` instance
    /// where the `value` field is set to `SquareValue::Empty`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tictacrustle::{
    ///     Square,
    ///     SquareValue,
    /// };
    ///
    /// // Create a new square
    /// let square = Square::new();
    //# assert_eq!(square.value, SquareValue::Empty);
    /// ```
    /// # Returns
    ///
    /// A new `Square` instance with an `Empty` value.
impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[allow(dead_code)]
impl Square {
    pub fn new() -> Square {
        Square {
            value: SquareValue::Empty,
        }
    }

    fn is_empty(&self) -> bool {
        self.value == SquareValue::Empty
    }

    fn is_x(&self) -> bool {
        self.value == SquareValue::X
    }

    fn is_o(&self) -> bool {
        self.value == SquareValue::O
    }

    fn get_value(&self) -> SquareValue {
        self.value
    }

    pub fn set_value(&mut self, value: &str) {
        match value {
            "X" => self.value = SquareValue::X,
            "O" => self.value = SquareValue::O,
            _ => self.value = SquareValue::Empty,
        }
    }

    fn set_x(&mut self) {
        self.value = SquareValue::X;
    }

    fn set_o(&mut self) {
        self.value = SquareValue::O;
    }

    fn set_empty(&mut self) {
        self.value = SquareValue::Empty;
    }
}

impl Default for Square {
    fn default() -> Self {
        Square::new()
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use rstest::{
        fixture,
        rstest,
    };

    use super::*;

    #[fixture]
    fn empty_square() -> Square {
        Square::new()
    }

    #[fixture]
    fn x_square() -> Square {
        let mut square = Square::new();
        square.set_x();
        square
    }

    #[fixture]
    fn o_square() -> Square {
        let mut square = Square::new();
        square.set_o();
        square
    }

    #[test]
    fn test_new() {
        let square = Square::new();
        assert_eq!(square.get_value(), SquareValue::Empty);
    }

        assert_eq!(square.value, SquareValue::Empty);
    }

    #[test]
    fn test_is_empty() {
        let square = Square::new();
        assert!(square.is_empty());
    }

    #[test]
    fn test_is_x() {
        let mut square = Square::new();
        square.set_x();
        assert!(square.is_x());
    }

    #[test]
    fn test_is_o() {
        let mut square = Square::new();
        square.set_o();
        assert!(square.is_o());
    }

    #[test]
    fn test_get_value() {
        let mut square = Square::new();
        square.set_x();
        assert_eq!(square.get_value(), SquareValue::X);
    }

    #[test]
    fn test_set_value() {
        let mut square = Square::new();
        square.set_value("X");
        assert_eq!(square.get_value(), SquareValue::X);
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
    fn test_display() {
        let mut square = Square::new();
        square.set_x();
        assert_eq!(format!("{}", square), " X ");
    }

    #[test]
    fn test_default() {
        let square = Square::default();
        assert_eq!(square.value, SquareValue::Empty);
    }

    #[rstest]
    #[case(empty_square(), empty_square(), true)]
    #[case(x_square(), x_square(), true)]
    #[case(o_square(), o_square(), true)]
    #[case(empty_square(), x_square(), false)]
    #[case(empty_square(), o_square(), false)]
    #[case(x_square(), o_square(), false)]
    fn test_eq(#[case] square1: Square, #[case] square2: Square, #[case] expected: bool) {
        assert_eq!(square1 == square2, expected);
    }
}
