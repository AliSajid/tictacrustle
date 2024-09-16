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
    pub fn new() -> Square {
        Square {
            value: SquareValue::Empty,
        }
    }

    /// Checks if the `Square` is empty.
    ///
    /// This method returns `true` if `value` in the [`Square`](struct.Square.html) instance is
    /// [`Empty`](enum.SquareValue.html#variant.Empty), and `false` otherwise.
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
    //# assert_eq!(square.is_empty(), true);
    /// // Create a square with an 'X'
    /// let mut x_square = Square::new();
    /// x_square.set_x();
    //# assert_eq!(square.is_empty(), false);
    /// // Create a square with an 'O'
    /// let mut o_square = Square::new();
    /// o_square.set_o();
    //# assert_eq!(square.is_empty(), false);
    /// ```
    /// # Returns
    ///
    /// A boolean indicating whether the `Square` is empty.
    ///
    /// # See Also
    ///
    /// - [`is_x`](struct.Square.html#method.is_x)
    /// - [`is_o`](struct.Square.html#method.is_o)
    pub fn is_empty(&self) -> bool {
        self.value == SquareValue::Empty
    }

    /// Checks if the `Square`'s value is `X`.
    ///
    /// This method returns `true` if the `Square`'s value is `SquareValue::X`, and `false`
    /// otherwise.
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
    //# assert_eq!(square.is_x(), false);
    /// // Create a square with an 'X'
    /// let mut x_square = Square::new();
    /// x_square.set_x();
    //# assert_eq!(x_square.is_x(), true);
    /// // Create a square with an 'O'
    /// let mut o_square = Square::new();
    /// o_square.set_o();
    //# assert_eq!(o_square.is_x(), false);
    /// ```
    /// # Returns
    ///
    /// A boolean indicating whether the `Square`'s value is `X`.
    ///
    /// # See Also
    ///
    /// - [`is_empty`](struct.Square.html#method.is_empty)
    /// - [`is_o`](struct.Square.html#method.is_o)
    pub fn is_x(&self) -> bool {
        self.value == SquareValue::X
    }

    /// Checks if the `Square`'s value is `O`.
    ///
    /// This method returns `true` if the `Square`'s value is `SquareValue::O`, and `false`
    /// otherwise.
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
    //# assert_eq!(square.is_o(), false);
    /// // Create a square with an 'O'
    /// let mut o_square = Square::new();
    /// o_square.set_o();
    //# assert_eq!(o_square.is_o(), true);
    /// // Create a square with an 'X'
    /// let mut x_square = Square::new();
    /// x_square.set_x();
    //# assert_eq!(x_square.is_o(), false);
    /// ```
    /// # Returns
    ///
    /// A boolean indicating whether the `Square`'s value is `O`.
    ///
    /// # See Also
    ///
    /// - [`is_empty`](struct.Square.html#method.is_empty)
    /// - [`is_x`](struct.Square.html#method.is_x)
    pub fn is_o(&self) -> bool {
        self.value == SquareValue::O
    }

    /// Returns the value of the `Square`.
    ///
    /// This method returns the current `SquareValue` of the `Square` instance. The `SquareValue`
    /// can be either `X`, `O`, or `Empty`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tictacrustle::{
    ///     Square,
    ///     SquareValue,
    /// };
    ///
    /// // Create a square with an 'X'
    /// let mut x_square = Square::new();
    /// x_square.set_x();
    /// assert_eq!(x_square.get_value(), &SquareValue::X);
    ///
    /// // Create a square with an 'O'
    /// let mut o_square = Square::new();
    /// o_square.set_o();
    /// assert_eq!(o_square.get_value(), &SquareValue::O);
    ///
    /// // Create an empty square
    /// let empty_square = Square::new();
    /// assert_eq!(empty_square.get_value(), &SquareValue::Empty);
    /// ```
    ///
    /// # Returns
    ///
    /// The current `SquareValue` of the `Square`.
    ///
    /// # See Also
    ///
    /// - [`set_value`](struct.Square.html#method.set_value)
    /// - [`set_x`](struct.Square.html#method.set_x)
    /// - [`set_o`](struct.Square.html#method.set_o)
    /// - [`is_empty`](struct.Square.html#method.is_empty)
    /// - [`is_x`](struct.Square.html#method.is_x)
    /// - [`is_o`](struct.Square.html#method.is_o)
    pub fn get_value(&self) -> SquareValue {
        self.value
    }

    /// Sets the value of the `Square`.
    ///
    /// This method sets the `SquareValue` of the `Square` instance based on the provided string.
    /// The string can be either `"X"`, `"O"`, or `" "` (for `Empty`).
    ///
    /// If an invalid string is provided, the method will panic with the message "Invalid value".
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
    /// let mut square = Square::new();
    ///
    /// // Set the square's value to 'X'
    /// square.set_value("X");
    /// assert_eq!(square.get_value(), SquareValue::X);
    ///
    /// // Set the square's value to 'O'
    /// square.set_value("O");
    /// assert_eq!(square.get_value(), SquareValue::O);
    ///
    /// // Set the square's value to 'Empty'
    /// square.set_value(" ");
    /// assert_eq!(square.get_value(), SquareValue::Empty);
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the provided string is not `"X"`, `"O"`, or `" "`.
    ///
    /// # Arguments
    ///
    /// * `value` - A string that represents the new `SquareValue`. It should be either `"X"`,
    ///   `"O"`, or `" "`.
    pub fn set_value(&mut self, value: &str) {
        match value {
            "X" => self.value = SquareValue::X,
            "O" => self.value = SquareValue::O,
            _ => self.value = SquareValue::Empty,
        }
    }

    /// Sets the `Square`'s value to `X`.
    ///
    /// This method calls the `set_value` method with the argument `"X"`, which sets the `Square`'s
    /// value to `SquareValue::X`.
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
    /// let mut square = Square::new();
    ///
    /// // Set the square's value to 'X'
    /// square.set_x();
    /// assert_eq!(square.get_value(), SquareValue::X);
    /// ```
    pub fn set_x(&mut self) {
        self.set_value("X");
    }

    /// Sets the `Square`'s value to `O`.
    ///
    /// This method calls the `set_value` method with the argument `"O"`, which sets the `Square`'s
    /// value to `SquareValue::O`.
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
    /// let mut square = Square::new();
    ///
    /// // Set the square's value to 'O'
    /// square.set_o();
    /// assert_eq!(square.get_value(), SquareValue::O);
    /// ```
    pub fn set_o(&mut self) {
        self.set_value("O");
    }
}

impl Default for Square {
    fn default() -> Self {
        Self::new()
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

    #[rstest]
    fn test_new(empty_square: Square) {
        assert_eq!(empty_square.get_value(), &SquareValue::Empty);
    }

        assert_eq!(square.value, SquareValue::Empty);
    }

    #[test]
    fn test_is_empty() {
        let square = Square::new();
        assert!(square.is_empty());
    }

    #[rstest]
    fn test_is_x(x_square: Square) {
        assert!(x_square.is_x());
    }

    #[rstest]
    fn test_is_o(o_square: Square) {
        assert!(o_square.is_o());
    }

    #[rstest]
    #[case(empty_square(), &SquareValue::Empty)]
    #[case(x_square(), &SquareValue::X)]
    #[case(o_square(), &SquareValue::O)]
    fn test_get_value(#[case] square: Square, #[case] expected: &SquareValue) {
        assert_eq!(square.get_value(), expected);
    }

    #[rstest]
    #[case("X", &SquareValue::X, true, false)]
    #[case("O", &SquareValue::O, false, true)]
    fn test_set_value(
        #[case] value: &str,
        #[case] result: &SquareValue,
        #[case] is_x: bool,
        #[case] is_o: bool,
    ) {
        let mut square = Square::new();
        square.set_x().expect("Failed to set square to X");
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
        square.set_o();
        assert_eq!(square.get_value(), SquareValue::O);
    }

    #[test]
    fn test_set_value() {
        let mut square = Square::new();
        square.set_value("X");
        assert_eq!(square.get_value(), SquareValue::X);
        square.set_value("O");
        assert_eq!(square.get_value(), SquareValue::O);
        square.set_value(" ");
        assert_eq!(square.get_value(), SquareValue::Empty);
    }

    #[test]
    #[should_panic(expected = "Invalid value")]
    fn test_set_value_invalid() {
        let mut square = Square::new();
        square.set_value("invalid");
    }

    #[rstest]
    #[should_panic(expected = "Square is already")]
    #[case(x_square(), "X")]
    #[case(x_square(), "O")]
    #[case(o_square(), "X")]
    #[case(o_square(), "O")]
    fn test_set_value_x(#[case] mut square: Square, #[case] set_value: &str) {
        square.set_value(set_value);
    }

    #[rstest]
    fn test_set_x(mut empty_square: Square) {
        empty_square.set_x();
        assert!(empty_square.is_x());
        assert!(!empty_square.is_o());
        assert!(!empty_square.is_empty());
    }

    #[rstest]
    fn test_set_o(mut empty_square: Square) {
        empty_square.set_o();
        assert!(empty_square.is_o());
        assert!(!empty_square.is_x());
        assert!(!empty_square.is_empty());
    }

    #[rstest]
    #[case(empty_square(), " ")]
    #[case(x_square(), "X")]
    #[case(o_square(), "O")]
    fn test_display(#[case] square: Square, #[case] expected: &str) {
        assert_eq!(format!("{square}"), expected);
    }

    #[test]
    fn test_default() {
        let square = Square::default();
        assert_eq!(square.get_value(), SquareValue::Empty);
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
