<!--
SPDX-FileCopyrightText: 2022 - 2026 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# Tic-Tac-Rustle

<!-- badges -->
![GitHub Release (w/pre-release)](https://img.shields.io/github/v/release/AliSajid/tictacrustle?include_prereleases&logo=semantic-release)
![GitHub Release](https://img.shields.io/github/v/release/AliSajid/tictacrustle?logo=semantic-release)
[![Continuous integration](https://github.com/AliSajid/tictacrustle/actions/workflows/ci.yaml/badge.svg)](https://github.com/AliSajid/tictacrustle/actions/workflows/ci.yaml)
![GitHub issues](https://img.shields.io/github/issues/AliSajid/tictacrustle)
![REUSE Compliance](https://img.shields.io/reuse/compliance/github.com%2FAliSajid%2Ftictacrustle)

<!-- description -->
An ambitious historical tribute and modern engineering exercise replicating Donald Michie's 1961 **MENACE** (Matchbox Educable Noughts and Crosses Engine) using a high-performance, decoupled polyglot monorepo architecture built in Rust and SvelteKit.

## Project Vision

`tic tac rustle` explores the bridge between historical reinforcement learning models and modern software patterns. Instead of training an AI live or relying on standard minimax brute-forcing during runtime, this project implements a highly optimized, read-only compile-time matrix of the entire valid Tic-Tac-Toe game tree.

Our goal is to build an unbeatable core engine alongside a series of chronological "evolutionary snapshots" (epochs) that demonstrate how the system physically alters its decision weights ("beads") as it learns. This entire experience is wrapped in a beautiful web visualization interface allowing users to peek straight into the AI's "brain" as they play.

## What to Expect

* **Static, Pre-Calculated Decision Space:** The entire legal board space (5,478 states) is evaluated, pruned of impossible permutations, and mapped to ternary values at compile time.
* **Granular Difficulty Scale:** 11 distinct pre-trained brains (from completely random to flawless tactical mastery) to test human capabilities across various stages of AI evolution.
* **Deep Brain Visualization:** An interactive web frontend that parses training analytics to display real-time decision weight graphs, heatmaps, and state trajectory lines.
* **Zero-Overhead Server Core:** A read-only API backend with lookups running at native memory speeds, fortified against application-layer threats.

## Project Structure

This project has three parts:

* `lib_tictacrustle`: This is the library crate that contains the core logic of the game. This crate manages the game logic, the game state, and the game rules. This crate is also responsible for the MENACE system.
* `ttrustle`: This is a binary crate tasked with actually running the game. This crate hosts the player interactions with the GUI[^1] and TUI[^2], as it progresses.
* `ttserver`: This is a binary crate that hosts the MENACE AI. This crate handles running the MENACE system and providing an API for the `ttrustle` binary to interact with.

## Monorepo Environment Setup

This project uses `mise` for polyglot toolchain management and task orchestration.

### Prerequisites

Ensure you have `mise` installed on your machine.

### Quick Start

To spin up the entire development environment (both the Rust API backend and the SvelteKit frontend concurrently), run:

```bash
mise run dev
```

## MENACE

Machine Educable Naughts and Crosses Engine (MENACE) is one of the first implementations of a machine learning system. Donald Michie developed it in 1961 while working at University of Edinburgh. The original system used a stack of matchboxes labeled with possible game states, along with a reinforcement learning algorithm, to learn the optimal strategy over a certain number of games. Michie called this system Matchbox Educable Naughts and Crosses Engine (MENACE).

This was one of the first systems to use reinforcement learning to learn how to play a game and the first to prove that a machine could learn how to play a game without being explicitly programmed to do so.

The classical MENACE system consisted of 304 matchboxes. Each matchbox represented a possible state of the game. Each matchbox had up to nine colored beads inside, with the number and color of beads representing the next move on the 3 X 3 board. The player would make the first move, and then draw a random bead from the matchbox matching the state of the game. This represents the move that MENACE _has chosen_ to make. The process continues until the player or MENACE wins the game. If MENACE wins, the player returns the beads to the matchbox, along with extra beads for the winning move. If the player wins, the player does not return the beads to the matchbox. This process repeats until MENACE achieves the optimal strategy.

[More information on MENACE is available here](https://en.wikipedia.org/wiki/MENACE).

## MENACE Implementation

Since MENACE predates both the internet and consumer computers, the original implementation was purely matchbox-based. In translating that system to a modern incarnation, we adhere to the following principles:

* A static compile-time matrix replaces the matchboxes used in the original implementation.
* The game runs as a client-server system that has independent clients and servers.
* The server along with the database and API is usable both locally and in the cloud.

The original MENACE implementation used a manually curated list of possible game states that treated the rotational and reflection symmetries in board states as identical. Since this implementation is not constrained by the number of virtual matchboxes, we build the MENACE system in two flavors:

1. **MENACE-C**: This is the classic MENACE system that treats the rotational and reflective symmetries in board states as equivalent.
2. **MENACE-S**: This is the MENACE system that treats the rotational and reflective symmetries in board states as distinct.

## Roadmap

The project is in its initial stages of development. The following list includes features that we plan to add in the future:

* [ ] Add the base game logic
* [ ] Add the human player
* [ ] Add the MENACE-C system
* [ ] Add the MENACE-S system
* [ ] Add the Command-line Interface (CLI)
* [ ] Add a Terminal User Interface (TUI)
* [ ] Add a Graphical User Interface (GUI)

## Contributing

Contributions to the project are welcome. Please see the [Contributing Guidelines](CONTRIBUTING.md) for more information.

## License

This project is dual-licensed under the [MIT License](LICENSES/MIT.txt) and the [Apache License (Version 2.0)](LICENSES/Apache-2.0.txt). You may choose to use this project under either license, at your discretion. Other, insignificant files are under the [CC0 License](LICENSES/CC0-1.0.txt). Please see the [LICENSES](LICENSES) directory for more information.

This project is REUSE compliant. [You can find more information about REUSE here](https://reuse.software/).

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, You are expected to uphold this code.

## Acknowledgements

This project would not be possible without the efforts of the Rust Community for outreach and training.

Specific people and projects worth mentioning:

* Chris Krycho and the [New Rustacean](https://newrustacean.com/) Podcast.
* Bogdan Pshonyak and the [Let's Get Rusty](https://www.youtube.com/c/letsgetrusty) YouTube Channel.
* Tris Oaten (NAMTAO) and the [No Boilerplate](https://www.youtube.com/c/NoBoilerplate) YouTube Channel.
* My loving family for their support and encouragement.

[^1]: Graphical User Interface

[^2]: Terminal User Interface
