// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::Board;

pub enum Symbol {
    X,
    O,
}

pub struct Player {
    symbol: Symbol,
}

#[allow(dead_code)]
pub struct Game {
    player_x: Player,
    player_o: Player,
    board:    Board,
    winner:   Option<Player>,
}

impl Game {
    pub fn new() -> Self {
        let current_player = Player { symbol: Symbol::X };
        let other_player = Player { symbol: Symbol::O };
        Self {
            player_x: current_player,
            player_o: other_player,
            board:    Board::new(),
            winner:   None,
        }
    }

    // #[allow(dead_code)]
    // pub fn play(&mut self) -> Result<(), Error> {
    //     self.board.get_square(2, 2).set_x();
    //     Ok(())
    // }

    // #[allow(dead_code)]
    // pub fn winner(&self) -> Option<Player> {
    //     &self.winner
    // }

    pub fn board(&self) -> &Board {
        &self.board
    }
}
