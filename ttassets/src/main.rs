// SPDX-FileCopyrightText: 2022 - 2026 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! # ttassets - Terminal UI for Tic Tac Toe
//!
//! A terminal-based UI for playing Tic Tac Toe.

use anyhow::Result;
#[expect(unused_imports)]
use ttrustle_lib::{Board, Game, Player, Symbol};

fn main() -> Result<()> {
    println!("Generating trinary encodings of valid Tic-Tac-Toe states");

    generate_symmetric_states();
    generate_asymmetric_states();

    Ok(())
}

fn generate_symmetric_states() -> usize {
    todo!()
}

fn generate_asymmetric_states() -> usize {
    todo!()
}
