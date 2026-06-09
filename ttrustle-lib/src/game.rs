// SPDX-FileCopyrightText: 2022 - 2026 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{Board, Player, Symbol};

/// Represents a complete game of Tic Tac Toe between two players.
///
/// The `Game` struct manages two players, the game board, and the game state,
/// including which player (if any) has won.
#[allow(dead_code)]
pub struct Game {
    /// Player using the 'X' symbol.
    player_x: Player,

    /// Player using the 'O' symbol.
    player_o: Player,

    /// The current state of the 3x3 game board.
    board: Board,

    /// The winner of the game, if one exists.
    winner: Option<Player>,
}

impl Game {
    /// Creates a new `Game` instance with two players and an empty board.
    #[must_use]
    pub fn new() -> Self {
        let current_player = Player { symbol: Symbol::X };
        let other_player = Player { symbol: Symbol::O };
        Self {
            player_x: current_player,
            player_o: other_player,
            board: Board::new(),
            winner: None,
        }
    }

    /// Plays a move to the center of the board.
    ///
    /// This is a stub method and should be expanded for interactive gameplay.
    pub const fn play(&mut self) -> Result<(), ()> {
        let center = self.board.get_square_mut(2, 2);
        let _ = center.set_x();
        Ok(())
    }

    /// Returns the winner of the game, if any.
    #[must_use]
    pub fn winner(&self) -> Option<Player> {
        self.winner.clone()
    }

    /// Returns a reference to the game board.
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

#[cfg(test)]
mod tests {
    use crate::{Game, SquareValue};

    #[test]
    fn game_initializes_correctly() {
        let game = Game::new();

        // Check that board is empty
        for row in 1..=3 {
            for col in 1..=3 {
                assert_eq!(
                    game.board().get_square(row, col).get_value(),
                    SquareValue::Empty
                );
            }
        }

        // Check that there is no winner
        assert_eq!(game.winner(), None);
    }

    #[test]
    fn default_game_is_equivalent_to_new() {
        let game1 = Game::new();
        let game2 = Game::default();

        assert_eq!(game1.board(), game2.board());
        assert_eq!(game1.winner(), game2.winner());
    }
}
