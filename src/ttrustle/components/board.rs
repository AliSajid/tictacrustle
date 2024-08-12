// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use super::square::Square;

#[derive(Debug, PartialEq, Eq)]

pub struct Board {
    squares: [[Square; 3]; 3],
}

#[allow(dead_code)]
impl Board {
    pub fn new() -> Board {
        Board {
            squares: [[Square::new(); 3]; 3],
        }
    }

    fn get_row(&self, row: usize) -> [Square; 3] {
        [
            self.get_square(row, 0),
            self.get_square(row, 1),
            self.get_square(row, 2),
        ]
    }

    fn get_column(&self, column: usize) -> [Square; 3] {
        [
            self.get_square(0, column),
            self.get_square(1, column),
            self.get_square(2, column),
        ]
    }

    fn get_diagonal(&self, diagonal: usize) -> [Square; 3] {
        match diagonal {
            0 => [
                self.get_square(0, 0),
                self.get_square(1, 1),
                self.get_square(2, 2),
            ],
            1 => [
                self.get_square(0, 2),
                self.get_square(1, 1),
                self.get_square(2, 0),
            ],
            _ => panic!("Invalid diagonal"),
        }
    }

    pub fn get_square(&self, row: usize, column: usize) -> Square {
        self.squares[row][column]
    }

    pub fn print(&self) {
        for row in 0..3 {
            println!(
                "{}|{}|{}",
                self.get_square(row, 0),
                self.get_square(row, 1),
                self.get_square(row, 2)
            );
            if row < 2 {
                println!("-----------");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row() {
        let board = Board::new();
        assert_eq!(
            board.get_row(0),
            [Square::new(), Square::new(), Square::new()]
        );
        assert_eq!(
            board.get_row(1),
            [Square::new(), Square::new(), Square::new()]
        );
        assert_eq!(
            board.get_row(2),
            [Square::new(), Square::new(), Square::new()]
        );
    }

    #[test]
    fn test_get_column() {
        let board = Board::new();
        assert_eq!(
            board.get_column(0),
            [Square::new(), Square::new(), Square::new()]
        );
        assert_eq!(
            board.get_column(1),
            [Square::new(), Square::new(), Square::new()]
        );
        assert_eq!(
            board.get_column(2),
            [Square::new(), Square::new(), Square::new()]
        );
    }

    #[test]
    fn test_get_diagonal() {
        let board = Board::new();
        assert_eq!(
            board.get_diagonal(0),
            [Square::new(), Square::new(), Square::new()]
        );
        assert_eq!(
            board.get_diagonal(1),
            [Square::new(), Square::new(), Square::new()]
        );
    }
}
