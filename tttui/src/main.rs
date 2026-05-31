// SPDX-FileCopyrightText: 2022 - 2026 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! # tttui - Terminal UI for Tic Tac Toe
//!
//! A terminal-based UI for playing Tic Tac Toe.

use anyhow::Result;
#[expect(unused_imports)]
use crossterm::event;
#[expect(unused_imports)]
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
#[expect(unused_imports)]
use ttrustle_lib::{Board, Game, Player, Symbol};

fn main() -> Result<()> {
    // crossterm::terminal::disable_raw_mode()?;
    // crossterm::terminal::enable_echo()?;
    //
    // let mut board = Board::new();
    // let mut game = Game::new();
    // let mut current_player = Player { symbol: Symbol::X };
    // let mut input_pos = (0, 0);
    //
    // let mut app = App::new(&mut board, &mut game, current_player, input_pos);
    //
    // let mut stdout = std::io::stdout();
    // let result = ratatui::ui::run(|| app.ui(&mut stdout))?;
    //
    // if result {
    //     println!("\n\nGame completed!");
    // }
    //
    Ok(())
}
