<!--
SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
<<<<<<< HEAD
||||||| parent of dfd8585 (build: update the project structure)
=======

SPDX-License-Identifier: CC0-1.0
-->
>>>>>>> dfd8585 (build: update the project structure)

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# Tic-Tac-Rustle - A Tic-Tac-Toe Game with MENACE

![GitHub Release (w/pre-release)](https://img.shields.io/github/v/release/AliSajid/tictacrustle?include_prereleases&logo=semantic-release)
![GitHub Release](https://img.shields.io/github/v/release/AliSajid/tictacrustle?logo=semantic-release)
[![Continuous integration](https://github.com/AliSajid/tictacrustle/actions/workflows/ci.yaml/badge.svg)](https://github.com/AliSajid/tictacrustle/actions/workflows/ci.yaml)
[![Contribute with Gitpod](https://img.shields.io/badge/Contribute%20with-Gitpod-908a85?logo=gitpod)](https://gitpod.io/#https://github.com/AliSajid/tictacrustle)
![GitHub issues](https://img.shields.io/github/issues/AliSajid/tictacrustle)
![REUSE Compliance](https://img.shields.io/reuse/compliance/github.com%2FAliSajid%2Ftictacrustle)

This project develops a Tic-Tac-Toe game in Rust with MENACE AI. It offers a server-client setup where users can play against a cloud-hosted MENACE AI or deploy their own locally.

## Builds

<<<<<<< HEAD
|         | Stable | Beta | Nightly | MSRV (1.72.0) |
| ------- | ------ | ---- | ------- | ---- |
| Linux   | ![Ubuntu x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-stable.json) | ![Ubuntu x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-beta.json) | ![Ubuntu x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-nightly.json) | ![Ubuntu x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-msrv.json) |
| Windows | ![Windows x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-stable.json) | ![Windows x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-beta.json) | ![Windows x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-nightly.json) | ![Windows x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-msrv.json) |
| macos   | ![macos x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-stable.json) | ![macos x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-beta.json) | ![macos x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-nightly.json) | ![macos x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-msrv.json) |
||||||| parent of c197bdc (docs: update docs for compliance with vale)
| Platform | Rust Version |Status |
| -------- | ------ | ------ |
| Linux    | stable <br/> beta <br/> nightly <br/> MSRV (1.64.0) | ![Ubuntu x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-stable.json) <br/> ![Ubuntu x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-beta.json) <br/> ![Ubuntu x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-nightly.json) <br/> ![Ubuntu x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-msrv.json) |
| Windows  | stable <br/> beta <br/> nightly <br/> MSRV (1.64.0) | ![macos x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-stable.json) <br/> ![macos x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-beta.json) <br/> ![macos x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-nightly.json) <br/> ![macos x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-msrv.json) |
| macOS    | stable <br/> beta <br/> nightly <br/> MSRV (1.64.0) | ![Windows x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-stable.json) <br/> ![Windows x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-beta.json) <br/> ![Windows x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-nightly.json) <br/> ![Windows x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-msrv.json) |
=======
| Platform | Rust Version |Status |
| -------- | ------ | ------ |
| Linux    | stable <br/> beta <br/> nightly <br/> MSRV (1.64.0) | ![Ubuntu x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-stable.json) <br/> ![Ubuntu x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-beta.json) <br/> ![Ubuntu x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-nightly.json) <br/> ![Ubuntu x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/ubuntu-msrv.json) |
| Windows  | stable <br/> beta <br/> nightly <br/> MSRV (1.64.0) | ![macos x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-stable.json) <br/> ![macos x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-beta.json) <br/> ![macos x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-nightly.json) <br/> ![macos x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/windows-msrv.json) |
| macos    | stable <br/> beta <br/> nightly <br/> MSRV (1.64.0) | ![Windows x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-stable.json) <br/> ![Windows x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-beta.json) <br/> ![Windows x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-nightly.json) <br/> ![Windows x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/185618f862b98debb8b034c34e83173d/raw/macos-msrv.json) |
>>>>>>> c197bdc (docs: update docs for compliance with vale)

## [MENACE](https://en.wikipedia.org/wiki/MENACE)

Machine Educable Naughts and Crosses Engine (MENACE) is one of the first implementations of a machine learning system. Donald Michie developed it in 1961 while working at University of Edinburgh. The original system used a stack of matchboxes labeled with possible game states, along with a reinforcement learning algorithm, to learn the optimal strategy over a certain number of games. Michie called this system Matchbox Educable Naughts and Crosses Engine (MENACE).

This was one of the first systems to use reinforcement learning to learn how to play a game and the first to prove that a machine could learn how to play a game without being explicitly programmed to do so.

The classical MENACE system consisted of 304 matchboxes. Each matchbox represented a possible state of the game. Each matchbox had up to nine colored beads inside, with the number and color of beads representing the next move on the 3 X 3 board. The player would make the first move, and then draw a random bead from the matchbox matching the state of the game. This represents the move that MENACE _has chosen_ to make. The process continues until the player or MENACE wins the game. If MENACE wins, the player returns the beads to the matchbox, along with extra beads for the winning move. If the player wins, the player does not return the beads to the matchbox. This process repeats until MENACE achieves the optimal strategy.

More information on MENACE is available [here](https://en.wikipedia.org/wiki/MENACE).

## Project Structure

This project has three parts:

- `lib_tictacrustle`: This is the library crate that contains the core logic of the game. This crate manages the game logic, the game state, and the game rules. This crate is also responsible for the MENACE system.
- `ttrustle`: This is a binary crate tasked with actually running the game. This crate hosts the player interactions with the GUI[^1] and TUI[^2], as it progresses.
- `ttserver`: This is a binary crate that hosts the MENACE AI. This crate handles running the MENACE system and providing an API for the `ttrustle` binary to interact with.

## MENACE Implementation

Since MENACE predates both the internet and consumer computers, the original implementation was purely matchbox-based. In translating that system to a modern incarnation, we adhere to the following principles:

- A database replaces the matchboxes used in the original implementation.
- The game runs as a client-server system that has independent clients and servers.
- The server along with the database and API is usable both locally and in the cloud.

The original MENACE implementation used a manually curated list of possible game states that treated the rotationally symmetrical board states as the same. Since this implementation is not constrained by the number of virtual matchboxes, we build the MENACE system in two flavors:

1. MENACE-C: This is the classic MENACE system that treats the rotationally symmetrical board states as the same.
2. MENACE-S: This is the MENACE system that treats the rotationally symmetrical board states as distinct.

## Roadmap

The project is in its initial stages of development. The following list includes features that we plan to add in the future:

- [ ] [Add the base game logic](https://github.com/AliSajid/tictacrustle/milestone/1).
- [ ] [Add the human player](https://github.com/AliSajid/tictacrustle/milestone/2).
- [ ] [Add the MENACE-C system](https://github.com/AliSajid/tictacrustle/milestone/3).
- [ ] [Add the MENACE-S system](https://github.com/AliSajid/tictacrustle/milestone/4).
- [ ] [Add the Command-line Interface (CLI)](https://github.com/AliSajid/tictacrustle/milestone/5).
- [ ] [Add a Terminal User Interface (TUI)](https://github.com/AliSajid/tictacrustle/milestone/6).
- [ ] [Add a Graphical User Interface (GUI)](https://github.com/AliSajid/tictacrustle/milestone/7).

## Contributing

Contributions to the project are welcome. Please see the [Contributing Guidelines](CONTRIBUTING.md) for more information.

This project is Gitpod-enabled. You can use Gitpod to contribute to the project without having to install any dependencies on your local machine. You can click the button below to start a Gitpod workspace with a complete development environment.

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/AliSajid/tictacrustle)

## License

This project is dual-licensed under the [MIT License](LICENSES/MIT.txt) and the [Apache License (Version 2.0)](LICENSES/Apache-2.0.txt). You may choose to use this project under either license, at your discretion. Other, insignificant files are under the [CC0 License](LICENSES/CC0-1.0.txt). Please see the [LICENSES](LICENSES) directory for more information.

This project is REUSE compliant. You can find more information about REUSE [here](https://reuse.software/).

## Code of Conduct

<!-- vale write-good.Passive = NO -->
<!-- vale Google.Passive = NO -->
This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, You are expected to uphold this code.
<!-- vale Google.Passive = YES -->
<!-- vale write-good.Passive = YES -->

## Acknowledgements

This project would not be possible without the efforts of the Rust Community for outreach and training.

Specific people and projects worth mentioning:

- Chris Krycho and the [New Rustacean](https://newrustacean.com/) Podcast.
- Bogdan Pshonyak and the [Let's Get Rusty](https://www.youtube.com/c/letsgetrusty) YouTube Channel.
- Tris Oaten (NAMTAO) and the [No Boilerplate](https://www.youtube.com/c/NoBoilerplate) YouTube Channel.
- My loving family for their support and encouragement.


[^1]: Graphical User Interface
[^2]: Terminal User Interface
