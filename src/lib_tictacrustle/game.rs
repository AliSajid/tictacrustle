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
    ///
    /// `player_x` is always assigned the `Symbol::X`, and `player_o` is assigned `Symbol::O`.
    /// The board is initialized to its default empty state, and no winner is set.
    ///
    /// # Returns
    ///
    /// A new `Game` ready to be played.
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

    /// Plays a hardcoded move to the center of the board (`(2, 2)`).
    ///
    /// This is a stub method and should be expanded to allow interactive gameplay,
    /// validation, turn management, and win detection.
    ///
    /// # Errors
    ///
    /// Returns a `GameError` if the move is invalid or the game is already over.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the move was successful.
    #[allow(dead_code)]
    pub const fn play(&mut self) -> Result<(), GameError> {
        self.board.get_square_mut(2, 2).set_x();
        Ok(())
    }

    /// Returns the winner of the game, if any.
    ///
    /// # Returns
    ///
    /// An `Option<Player>` which is `Some(player)` if a winner has been determined,
    /// or `None` if the game is still ongoing or ended in a draw.
    #[allow(dead_code)]
    #[must_use]
    pub fn winner(&self) -> Option<Player> {
        self.winner.clone()
    }

    /// Returns an immutable reference to the current state of the game board.
    ///
    /// Useful for displaying the board or analyzing its contents.
    ///
    /// # Returns
    ///
    /// A reference to the `Board` instance used in the game.
    #[must_use]
    pub const fn board(&self) -> &Board {
        &self.board
    }
}

impl Default for Game {
    /// Creates a default game instance by calling [`Game::new`].
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {

    use crate::{
        Game,
        SquareValue,
    };

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

    #[test]
    fn play_sets_center_square() {
        let mut game = Game::new();

        game.play().unwrap();

        let center = game.board().get_square(2, 2).get_value();
        assert_eq!(center, SquareValue::X);
    }

    #[test]
    fn winner_starts_none() {
        let game = Game::new();
        assert!(game.winner().is_none());
    }

    #[test]
    fn play_returns_ok() {
        let mut game = Game::new();
        let result = game.play();
        assert!(result.is_ok());
    }
}
