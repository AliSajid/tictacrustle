<!--
SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami

SPDX-License-Identifier: CC0-1.0
-->

# Tic-Tac-Rustle - A Tic Tac Toe Game with MENACE

![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/AliSajid/ttt_menace?include_prereleases)
![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/AliSajid/ttt_menace)
[![Continuous integration](https://github.com/AliSajid/ttt_menace/actions/workflows/ci.yaml/badge.svg)](https://github.com/AliSajid/ttt_menace/actions/workflows/ci.yaml)
[![Contribute with Gitpod](https://img.shields.io/badge/Contribute%20with-Gitpod-908a85?logo=gitpod)](https://gitpod.io/#https://github.com/AliSajid/ttt_menace)


![GitHub issues](https://img.shields.io/github/issues/AliSajid/ttt_menace)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/AliSajid/ttt_menace)
![GitHub (Pre-)Release Date](https://img.shields.io/github/release-date-pre/AliSajid/ttt_menace)
![GitHub commits since latest release (by SemVer including pre-releases)](https://img.shields.io/github/commits-since/AliSajid/ttt_menace/latest?include_prereleases&sort=semver)
![GitHub contributors](https://img.shields.io/github/contributors/AliSajid/ttt_menace)

This is a toy project to create a Tic Tac Toe game that can be played by any two players. The ultimate intention here is to create the game in such
a way that it can be played by a human and a computer. The computer will be using the MENACE algorithm to learn how to play the game.

## Builds

| Platform | Rust Version |Status |
| -------- | ------ | ------ |
| Linux    | stable <br/> beta <br/> nightly <br/> MSRV (1.64.0) | ![Ubuntu x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-stable.json) <br/> ![Ubuntu x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-beta.json) <br/> ![Ubuntu x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-nightly.json) <br/> ![Ubuntu x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-msrv.json) |
| Windows  | stable <br/> beta <br/> nightly <br/> MSRV (1.64.0) | ![macos x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-stable.json) <br/> ![macos x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-beta.json) <br/> ![macos x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-nightly.json) <br/> ![macos x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-msrv.json) |
| macos    | stable <br/> beta <br/> nightly <br/> MSRV (1.64.0) | ![Windows x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-stable.json) <br/> ![Windows x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-beta.json) <br/> ![Windows x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-nightly.json) <br/> ![Windows x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-msrv.json) |

## [MENACE](https://en.wikipedia.org/wiki/MENACE)

Machine Educable Noughts and Crosses Engine (MENACE) is one of the first implementations of a machine learning system. It was developed by
Donald Michie in 1961. The original system was developed using a stack of matchboxes over a period of time and was called Matchbox Educable
Noughts and Crosses Engine (MENACE).

This was one of the first systems to use reinforcement learning to learn how to play a game and the first to prove that
a machine could learn how to play a game without being explicitly programmed to do so.

The classical MENACE system consisted of 304 matchboxes. Each matchbox represented a possible state of the game. Each matchbox had a
number of beads inside, with the number and color of beads representing the next move to be made. The system would play a game of Tic Tac Toe
and keep track of the moves made throughout the game. Afterwards, depending on the result of the game (win, lose, draw), the system would either be "rewarded" or "punished" by adding or removing beads from the matchboxes. The system would then play another game and repeat the process.

More information on MENACE can be found [here](https://en.wikipedia.org/wiki/MENACE).

## Project Structure

This project uses Cargo's workspace feature to organize the project into multiple crates. The following is a brief description of each crate:

- `tttm`: This is a binary crate tasked with actually running the game. This crate is planned to host the player interactions with the GUI or TUI, as it progresses.
- `lib_tictactoe_menace`: This is a library crate that contains the core logic of the game. This crate is responsible for the game logic, the game state, and the game rules.
- `tictactoe_menace_player`: This is a library crate that serves two purposes. First, it provides a common interface for the different types of players that can be _plugged into_ the game. Second, it provides a basic implementation for a human player.
- `tictactoe_menace_c`: This is a library crate that implements the MENACE-C system. The implementation of this system is based on the interface defined in `tictactoe_menace_player`.
- `tictactoe_menace_s`: This is a library crate that implements the MENACE-S system. The implementation of this system is based on the interface defined in `tictactoe_menace_player`.

## MENACE Implementation

Since MENACE was originally designed to not require a computer, we have to make certain adjustments in adapting the Matchbox-based system to a computer-based system. We will follow the following principles in our implementation:

- The matchboxes will be represented by a 2D array of `u8` values. Each value will represent the number of beads in the matchbox.
- We will use a HashMap to store the state of the game. The key will be a string representation of the board state and the value will be the index of the matchbox in the 2D array.

Additionally, the original MENACE implementation used a manually curated list of possible game states that treated the rotationally symmetrical board states as equivalent. Since we are not constrained by the number of virtual matchboxes, we will be implementing the MENACE system in two flavors:

1. MENACE-C: This will be the classic MENACE system that treats the rotationally symmetrical board states as equivalent.
2. MENACE-S: This will be the MENACE system that treats the rotationally symmetrical board states as distinct.

## Roadmap

The project is currently in the early stages of development. The following is a list of features that will be implemented in the future:

- [ ] [Implement the base game logic](https://github.com/AliSajid/ttt_menace/milestone/1).
- [ ] [Implement the human player](https://github.com/AliSajid/ttt_menace/milestone/2).
- [ ] [Implement the MENACE-C system](https://github.com/AliSajid/ttt_menace/milestone/3).
- [ ] [Implement the MENACE-S system](https://github.com/AliSajid/ttt_menace/milestone/4).
- [ ] [Implement the CLI](https://github.com/AliSajid/ttt_menace/milestone/5).
- [ ] [Add a TUI](https://github.com/AliSajid/ttt_menace/milestone/6). (Likely using [tui-rs](https://github.com/fdehau/tui-rs)).
- [ ] [Add a GUI](https://github.com/AliSajid/ttt_menace/milestone/7). (Implementation details TBD)

## Contributing

Contributions to the project are welcome. Please see the [Contributing Guidelines](CONTRIBUTING.md) for more information.

This project is Gitpod-enabled. You can use Gitpod to contribute to the project without having to install any dependencies on your local machine. Simply click on the button below to start a Gitpod workspace.

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/AliSajid/ttt_menace)

## License

This project is dual-licensed under the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Acknowledgements

This project would not be possible without the efforts of the Rust Community for outreach and training.

I would specifically mention the following people and projects:

- Chris Krycho and the [New Rustacean](https://newrustacean.com/) Podcast.
- Bogdan Pshonyak and the [Let's Get Rusty](https://www.youtube.com/c/letsgetrusty) YouTube Channel.
- Tris Oaten (NAMTAO) and his [No Boilerplate](https://www.youtube.com/c/NoBoilerplate) YouTube Channel.
- My loving family for their support and encouragement.
