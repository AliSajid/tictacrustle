// SPDX-FileCopyrightText: 2022 - 2026 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! # tttui - Terminal UI for Tic Tac Toe
//!
//! A terminal-based UI for playing Tic Tac Toe.

use anyhow::Result;
use crossterm::event;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use ttrustle_lib::{Board, Game, Player, Symbol};

fn main() -> Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::terminal::enable_echo()?;

    let mut board = Board::new();
    let mut game = Game::new();
    let mut current_player = Player { symbol: Symbol::X };
    let mut input_pos = (0, 0);

    let mut app = App::new(&mut board, &mut game, current_player, input_pos);

    let mut stdout = std::io::stdout();
    let result = ratatui::ui::run(|| app.ui(&mut stdout))?;

    if result {
        println!("\n\nGame completed!");
    }

    Ok(())
}

#[derive(Default)]
struct App {
    board: Board,
    game: Game,
    current_player: Player,
    input_pos: (usize, usize),
    game_over: bool,
}

impl App {
    fn new(board: &mut Board, game: &mut Game, player: Player, pos: (usize, usize)) -> Self {
        let mut app = Self::default();
        app.board = board.clone();
        app.game = game.clone();
        app.current_player = player;
        app.input_pos = pos;
        app.game_over = game.winner().is_some();
        app
    }

    fn ui(&mut self, area: &mut Area) -> Result<()> {
        // Set terminal size
        let (w, h) = crossterm::terminal::size()?;

        // Create layout
        let rects = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Length(4),  // Title
                Constraint::Min(h - 4), // Main game
                Constraint::Length(2),  // Instructions
            ])
            .split(area);

        // Title
        let title = Paragraph::new(" tttui - Tic Tac Toe Terminal UI ")
            .style(Style::default().add_modifier(Moderator::BOLD))
            .block(Block::default().borders(Borders::ALL));
        title.render(area, Rect::new(0, 0, w, 4))?;

        // Main game area
        let game_area = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Min(10), // Board
                Constraint::Length(25), // Messages
            ])
            .split(rects[1]);

        // Render board
        let board_para = Paragraph::new(&format!("{}", self.board))
            .style(Style::default().fg(Color::Cyan))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Player {}", self.current_player)),
            );
        board_para.render(area, game_area[0]);

        // Messages
        let msg = if self.game_over {
            match self.game.winner() {
                Some(winner) => format!("Winner: {}", winner),
                None => "Draw!".to_string(),
            }
        } else {
            format!("Enter square position (1-3,1-3): {}", self.input_pos.0 + 1)
        };
        let msg_para = Paragraph::new(&msg).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Messages "),
        );
        msg_para.render(area, game_area[1]);

        // Input area
        let input_para = Paragraph::new(&format!("Position: ({}, {})", self.input_pos.0, self.input_pos.1))
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Input "),
            );
        input_para.render(area, rects[2]);

        Ok(())
    }
}
