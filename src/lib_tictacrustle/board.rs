// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt::Display;

use crate::Square;

/// Represents the 3x3 grid for a Tic Tac Toe game.
///
/// Internally, the board stores 9 `Square` values in a flat array.
/// Squares are accessed using 1-based row and column coordinates (1 to 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    squares: [Square; 9],
}

impl Board {
    /// Creates a new `Board` with all squares initialized to their default state.
    ///
    /// # Returns
    ///
    /// A fresh `Board` ready for a new game.
    #[must_use]
    pub fn new() -> Self {
        Self {
            squares: [Square::default(); 9],
        }
    }

    /// Retrieves an immutable reference to a square at the specified row and column.
    ///
    /// # Arguments
    ///
    /// * `row` - Row number (1-based, valid values: 1 to 3).
    /// * `col` - Column number (1-based, valid values: 1 to 3).
    ///
    /// # Panics
    ///
    /// Panics if row or column are outside the range 1..=3.
    #[must_use]
    pub const fn get_square(&self, row: usize, col: usize) -> &Square {
        let index = Self::translate_coordinates(row, col);
        &self.squares[index]
    }

    /// Retrieves a mutable reference to a square at the specified row and column.
    ///
    /// # Arguments
    ///
    /// * `row` - Row number (1-based, valid values: 1 to 3).
    /// * `col` - Column number (1-based, valid values: 1 to 3).
    ///
    /// # Panics
    ///
    /// Panics if row or column are outside the range 1..=3.
    pub const fn get_square_mut(&mut self, row: usize, col: usize) -> &mut Square {
        let index = Self::translate_coordinates(row, col);
        &mut self.squares[index]
    }

    /// Retrieves an array of references to the squares in the specified row.
    ///
    /// # Arguments
    ///
    /// * `row` - Row number (1-based, valid values: 1 to 3).
    ///
    /// # Returns
    ///
    /// A fixed-size array containing references to the 3 squares in the row.
    ///
    /// # Panics
    ///
    /// Panics if row is outside the range 1..=3.
    #[must_use]
    pub const fn get_row(&self, row: usize) -> [&Square; 3] {
        [
            self.get_square(row, 1),
            self.get_square(row, 2),
            self.get_square(row, 3),
        ]
    }

    /// Retrieves an array of references to the squares in the specified column.
    ///
    /// # Arguments
    ///
    /// * `col` - Column number (1-based, valid values: 1 to 3).
    ///
    /// # Returns
    ///
    /// A fixed-size array containing references to the 3 squares in the column.
    ///
    /// # Panics
    ///
    /// Panics if column is outside the range 1..=3.
    #[must_use]
    pub const fn get_column(&self, col: usize) -> [&Square; 3] {
        [
            self.get_square(1, col),
            self.get_square(2, col),
            self.get_square(3, col),
        ]
    }

    /// Retrieves an array of references to the squares on one of the two diagonals.
    ///
    /// # Arguments
    ///
    /// * `diagonal` - A string specifying which diagonal to return.
    ///   - `"l"` for the left-to-right diagonal (`(1,1) -> (3,3)`).
    ///   - `"r"` for the right-to-left diagonal (`(1,3) -> (3,1)`).
    ///
    /// # Returns
    ///
    /// A fixed-size array containing references to the 3 squares in the specified diagonal.
    ///
    /// # Panics
    ///
    /// Panics if `diagonal` is not `"l"` or `"r"`.
    #[must_use]
    pub fn get_diagonal(&self, diagonal: &str) -> [&Square; 3] {
        match diagonal {
            "l" => [
                self.get_square(1, 1),
                self.get_square(2, 2),
                self.get_square(3, 3),
            ],
            "r" => [
                self.get_square(1, 3),
                self.get_square(2, 2),
                self.get_square(3, 1),
            ],
            _ => panic!("Invalid diagonal"),
        }
    }

    /// Converts 1-based `(row, col)` coordinates to a 0-based index in the internal square array.
    ///
    /// # Arguments
    ///
    /// * `row` - Row number (1-based).
    /// * `col` - Column number (1-based).
    ///
    /// # Returns
    ///
    /// A 0-based index into the `squares` array.
    ///
    /// # Panics
    ///
    /// Panics if row or column are zero.
    const fn translate_coordinates(row: usize, col: usize) -> usize {
        let row_index = (row - 1) * 3;
        let col_index = col - 1;
        row_index + col_index
    }
}

impl Default for Board {
    /// Creates a default board by calling [`Board::new`].
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Board {
    /// Formats the board for display with grid lines separating rows.
    ///
    /// Squares are rendered in a traditional Tic Tac Toe layout:
    ///
    /// ```text
    /// X|O|X
    /// -----------
    /// O|X|O
    /// -----------
    /// X| |X
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}|{}|{}\n-----------\n{}|{}|{}\n-----------\n{}|{}|{}",
            self.get_square(1, 1),
            self.get_square(1, 2),
            self.get_square(1, 3),
            self.get_square(2, 1),
            self.get_square(2, 2),
            self.get_square(2, 3),
            self.get_square(3, 1),
            self.get_square(3, 2),
            self.get_square(3, 3)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row() {
        let board = Board::default();
        assert_eq!(board.get_row(1), [&Square::default(); 3]);
        assert_eq!(board.get_row(2), [&Square::default(); 3]);
        assert_eq!(board.get_row(3), [&Square::default(); 3]);
    }

    #[test]
    fn test_get_column() {
        let board = Board::default();
        assert_eq!(board.get_column(1), [&Square::default(); 3]);
        assert_eq!(board.get_column(2), [&Square::default(); 3]);
        assert_eq!(board.get_column(3), [&Square::default(); 3]);
    }

    #[test]
    fn test_get_diagonal() {
        let board = Board::default();
        assert_eq!(board.get_diagonal("l"), [&Square::default(); 3]);
        assert_eq!(board.get_diagonal("r"), [&Square::default(); 3]);
    }
}
