// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt::Display;

use crate::Square;

#[derive(Debug)]
pub struct Board {
    squares: [Square; 9],
}

impl Board {
    #[must_use]
    pub fn new() -> Self {
        Self {
            squares: [Square::default(); 9],
        }
    }

    #[must_use]
    pub const fn get_square(&self, row: usize, col: usize) -> &Square {
        let index = Self::translate_coordinates(row, col);
        &self.squares[index]
    }

    pub fn get_square_mut(&mut self, row: usize, col: usize) -> &mut Square {
        let index = Self::translate_coordinates(row, col);
        &mut self.squares[index]
    }

    #[must_use]
    pub const fn get_row(&self, row: usize) -> [&Square; 3] {
        [
            self.get_square(row, 1),
            self.get_square(row, 2),
            self.get_square(row, 3),
        ]
    }

    #[must_use]
    pub const fn get_column(&self, col: usize) -> [&Square; 3] {
        [
            self.get_square(1, col),
            self.get_square(2, col),
            self.get_square(3, col),
        ]
    }

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

    const fn translate_coordinates(row: usize, col: usize) -> usize {
        let row_index = (row - 1) * 3;
        let col_index = col - 1;
        row_index + col_index
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Board {
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
