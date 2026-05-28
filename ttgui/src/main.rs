// SPDX-FileCopyrightText: 2022 - 2026 Ali Sajid Imami
//
# SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! # ttgui - GUI Tic Tac Toe
//!
//! A graphical user interface for playing Tic Tac Toe using egui.

use eframe::egui;
use ttrustle_lib::{Board, Game, Player, SquareValue, Symbol};

fn main() -> anyhow::Result<()> {
    let app = TtGuiApp::default();

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "ttgui - Tic Tac Toe GUI",
        native_options,
        Box::new(move |_cc| Ok(Box::new(app.clone()))),
    )
}

struct TtGuiApp {
    board: Board,
    game: Game,
    current_player: Player,
    message: String,
    selected_pos: Option<(usize, usize)>,
}

impl Default for TtGuiApp {
    fn default() -> Self {
        Self {
            board: Board::new(),
            game: Game::new(),
            current_player: Player { symbol: Symbol::X },
            message: String::new(),
            selected_pos: None,
        }
    }
}

impl eframe::App for TtGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render title
        let title = egui::Label::new(egui::RichText::new("ttgui - Tic Tac Toe").strong());
        ctx.add_sized(
            egui::Id::from_str("title").unwrap(),
            egui::Layout::centered(egui::Align::CENTER),
            egui::TextStyle::Body,
            title,
        );

        // Render game board
        let board_response = self.render_board(ctx);

        // Show message
        if !self.message.is_empty() {
            let msg = egui::Label::new(egui::RichText::new(&self.message).sense(egui::Sense::none()));
            ctx.add(
                egui::Id::from_str("message").unwrap(),
                msg,
                egui::Response::default(),
            );
        }

        // Show player info
        let player_label = egui::Label::new(
            egui::RichText::new(format!("Next player: {}", self.current_player))
                .small(),
        );
        ctx.add(
            egui::Id::from_str("player").unwrap(),
            player_label,
            egui::Response::default(),
        );

        // Render instructions
        if let Some(pos) = self.selected_pos {
            ctx.add(
                egui::Label::new(
                    egui::RichText::new(format!("Click to place: ({}, {})", pos.0, pos.1)).small(),
                )
                .sense(egui::Sense::click()),
            );
        }

        // Handle response
        match board_response.response {
            Some(res) => {
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(res.rect.size()));
            }
            None => {}
        }
    }
}

impl TtGuiApp {
    fn render_board(&mut self, ctx: &egui::Context) -> egui::Response {
        let board_grid = egui::Grid::new("board").num_columns(3);
        board_grid.spacing([4.0, 4.0]);
        board_grid.response_visible(true);

        for (i, row) in self.board.iter().enumerate() {
            for (j, square) in row.iter().enumerate() {
                let square_id = egui::Id::from_str(&format!(
                    "square_{}_{}",
                    i + 1,
                    j + 1
                ))
                .unwrap();

                let square_value = square.get_value();

                let style = match square_value {
                    SquareValue::X => egui::Style::filled,
                    SquareValue::O => egui::Style::filled,
                    SquareValue::Empty => egui::Style::none,
                };

                let square_text = match square_value {
                    SquareValue::X => " X ",
                    SquareValue::O => " O ",
                    SquareValue::Empty => "   ",
                };

                let square = egui::Label::new(egui::RichText::new(square_text));

                board_grid.cell(|_, _| {
                    // Create response for each cell
                    ctx.add(
                        square_id,
                        square,
                        egui::Response::default(),
                    );
                });
            }
        }

        board_grid.end()
    }
}
