// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{
    Board,
    GameError,
    Player,
    Symbol,
};

#[allow(dead_code)]
pub struct Game {
    player_x: Player,
    player_o: Player,
    board:    Board,
    winner:   Option<Player>,
}

impl Game {
    #[must_use]
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

    #[allow(dead_code)]
    pub fn play(&mut self) -> Result<(), GameError> {
        self.board.get_square_mut(2, 2).set_value("X");
        Ok(())
    }

    #[allow(dead_code)]
    #[must_use]
    pub const fn winner(&self) -> Option<Player> {
        self.winner
    }

    #[must_use]
    pub const fn board(&self) -> &Board {
        &self.board
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
