// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tictacrustle::Board;

fn main() {
    let mut board = Board::default();
    println!("{board}");
    board.get_square_mut(2, 2).set_x();
    println!("{board}");
    board.get_square_mut(1, 1).set_o();
    println!("{board}");
}
