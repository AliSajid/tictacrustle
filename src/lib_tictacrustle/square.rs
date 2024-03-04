// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt::{
    self,
    Display,
    Formatter,
};

use anyhow::{
    Error,
    Result,
};

use super::square_value::SquareValue;

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
    #[must_use]
    pub const fn new() -> Self {
        Self {
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
    //# assert!(square.is_empty());
    /// // Create a square with an 'X'
    /// let mut x_square = Square::new();
    /// x_square.set_x();
    //# assert!(!square.is_empty());
    /// // Create a square with an 'O'
    /// let mut o_square = Square::new();
    /// o_square.set_o();
    //# assert!(!square.is_empty());
    /// ```
    /// # Returns
    ///
    /// A boolean indicating whether the `Square` is empty.
    ///
    /// # See Also
    ///
    /// - [`is_x`](struct.Square.html#method.is_x)
    /// - [`is_o`](struct.Square.html#method.is_o)
    #[must_use]
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
    //# assert!(!square.is_x());
    /// // Create a square with an 'X'
    /// let mut x_square = Square::new();
    /// x_square.set_x();
    //# assert!(x_square.is_x());
    /// // Create a square with an 'O'
    /// let mut o_square = Square::new();
    /// o_square.set_o();
    //# assert!(!o_square.is_x());
    /// ```
    /// # Returns
    ///
    /// A boolean indicating whether the `Square`'s value is `X`.
    ///
    /// # See Also
    ///
    /// - [`is_empty`](struct.Square.html#method.is_empty)
    /// - [`is_o`](struct.Square.html#method.is_o)
    #[must_use]
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
    //# assert_eq!(o_square.is_o());
    /// // Create a square with an 'X'
    /// let mut x_square = Square::new();
    /// x_square.set_x();
    //# assert!(!x_square.is_o());
    /// ```
    /// # Returns
    ///
    /// A boolean indicating whether the `Square`'s value is `O`.
    ///
    /// # See Also
    ///
    /// - [`is_empty`](struct.Square.html#method.is_empty)
    /// - [`is_x`](struct.Square.html#method.is_x)
    #[must_use]
    pub fn is_o(&self) -> bool {
        self.value == SquareValue::O
    }

    /// Sets the `Square`'s value to `X`.
    ///
    /// This method calls the `set_value` method with the argument `"X"`, which sets the `Square`'s
    /// value to `SquareValue::X`.
    ///
    /// # Errors
    ///
    /// This method returns an error if the `Square` is not empty.
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
    //# square.set_x().expect("Failed to set square to X");
    /// // Create a square with an 'X'
    /// let mut x_square = Square::new();
    /// let res = x_square.set_x();
    /// assert_eq!(x_square.get_value(), SquareValue::X);
    /// assert!(res.is_ok());
    /// // Create a square with an 'O'
    /// let res = x_square.set_o();
    /// assert!(x_square.set_x().is_err());
    /// ```
    pub fn set_x(&mut self) -> Result<()> {
        match self.value {
            SquareValue::Empty => {
                self.value = SquareValue::X;
                Ok(())
            }
            SquareValue::O => Err(Error::msg("Square is not empty")),
            SquareValue::X => Err(Error::msg("Square is already X")),
        }
    }

    /// Sets the `Square`'s value to `O`.
    ///
    /// This method calls the `set_value` method with the argument `"O"`, which sets the `Square`'s
    /// value to `SquareValue::O`.
    /// # Errors
    ///
    /// This method returns an error if the `Square` is not empty.
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
    /// let mut o_square = Square::new();
    /// let res = o_square.set_o();
    /// assert_eq!(o_square.get_value(), SquareValue::O);
    /// assert!(res.is_ok());
    /// // Value cannot be changed now
    /// let res = o_square.set_x();
    /// assert!(res.is_err());
    /// ```
    pub fn set_o(&mut self) -> Result<()> {
        match self.value {
            SquareValue::Empty => {
                self.value = SquareValue::O;
                Ok(())
            }
            SquareValue::X => Err(Error::msg("Square is not empty")),
            SquareValue::O => Err(Error::msg("Square is already O")),
        }
    }

    /// Gets the `Square`'s value.
    ///
    /// This method returns the `Square`'s value as a `SquareValue` enum.
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
    /// assert_eq!(square.get_value(), SquareValue::Empty);
    /// // Create a square with an 'X'
    /// let mut x_square = Square::new();
    /// x_square.set_x();
    /// assert_eq!(x_square.get_value(), SquareValue::X);
    /// // Create a square with an 'O'
    /// let mut o_square = Square::new();
    /// o_square.set_o();
    /// assert_eq!(o_square.get_value(), SquareValue::O);
    /// ```
    /// # Returns
    ///
    /// The `Square`'s value as a `SquareValue` enum.
    #[must_use]
    pub const fn get_value(&self) -> SquareValue {
        self.value
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
    fn square_x() -> Square {
        let mut square = Square::new();
        square.set_x().expect("Failed to set X");
        square
    }

    #[fixture]
    fn square_o() -> Square {
        let mut square = Square::new();
        square.set_o().expect("Failed to set O");
        square
    }

    #[test]
    fn test_new() {
        let square = Square::new();
        assert!(square.is_empty());
    }

    #[test]
    fn test_is_empty() {
        let square = Square::new();
        assert!(square.is_empty());
    }

    #[test]
    fn test_is_x() {
        let mut square = Square::new();
        square.set_x().expect("Failed to set square to X");
        assert!(square.is_x());
    }

    #[test]
    fn test_set_x() {
        let mut square = Square::new();
        let res = square.set_x();
        assert!(square.is_x());
        assert!(!square.is_o());
        assert!(!square.is_empty());
        assert!(res.is_ok());
        let res = square.set_o();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Square is not empty");
        let res: Result<()> = square.set_x();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Square is already X");
    }

    #[test]
    fn test_set_o() {
        let mut square = Square::new();
        square.set_o().expect("Failed to set square to O");
        assert!(square.is_o());
        assert!(!square.is_x());
        assert!(!square.is_empty());
        let res = square.set_x();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Square is not empty");
        let res: Result<()> = square.set_o();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Square is already O");
    }

    #[test]
    fn test_display() {
        let square = Square::new();
        assert_eq!(format!("{}", square), "   ");
        let mut square_x = Square::new();
        square_x.set_x().expect("Failed to set square to X");
        assert_eq!(format!("{}", square_x), " X ");
        let mut square_o = Square::new();
        square_o.set_o().expect("Failed to set square to O");
        assert_eq!(format!("{}", square_o), " O ");
    }

    #[test]
    fn test_default() {
        let square = Square::default();
        assert!(square.is_empty());
        assert!(!square.is_x());
        assert!(!square.is_o());
    }

    #[rstest]
    #[case(empty_square(), empty_square(), true)]
    #[case(square_x(), square_x(), true)]
    #[case(square_o(), square_o(), true)]
    #[case(empty_square(), square_x(), false)]
    #[case(empty_square(), square_o(), false)]
    #[case(square_x(), square_o(), false)]
    fn test_eq(#[case] square1: Square, #[case] square2: Square, #[case] expected: bool) {
        assert_eq!(square1 == square2, expected);
    }
}
