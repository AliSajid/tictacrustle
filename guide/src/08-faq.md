# FAQ

Frequently asked questions about Tic-Tac-Rustle.

## General Questions

### What is MENACE?

MENACE (Machine Educable Noughts and Crosses Engine) is one of the first practical implementations of reinforcement learning, created in 1961 by Donald Michie. It used physical matchboxes and colored beads to learn Tic-Tac-Toe.

### Why use Rust?

Rust provides:
- Memory safety without garbage collection
- Zero-cost abstractions
- Compile-time performance
- Strong type system for correctness

### Is this truly reinforcement learning?

Yes. The system learns through experience:
- Starts with random weights
- Adjusts weights after each game
- Becomes better over time
- No explicit rules for "how to win"

### Can I play against it online?

The project is offline-first by design. Weights are embedded at compile time for privacy and speed.

## Technical Questions

### What is the difference between MENACE-C and MENACE-S?

- **MENACE-C**: Classic approach with symmetry (rotational equivalence)
- **MENACE-S**: Treats all states as distinct

MENACE-C uses 5,478 states vs 19,683 for MENACE-S.

### Why does it say "next.2" in the version?

This indicates a pre-release version (nightly build). The project is still in active development.

### What Rust version do I need?

Rust 1.85.1 or later. This version supports the edition used in the project.

### Can I run this on Windows?

Yes. All crates build on Windows, Linux, and macOS.

### What about GUI support?

The `ttgui` crate provides a GTK-based desktop GUI. Requires GTK installed.

### Is there a mobile version?

Not yet. The project focuses on desktop and CLI platforms.

## Gameplay Questions

### How good is the standard brain?

The standard brain (100k training iterations):
- Win rate: ~55%
- Blocks opponent's threats
- Creates tactical opportunities
- Not unbeatable, but challenging

### Can I beat the expert brain?

The expert brain (500k iterations):
- Win rate: ~85%
- Very difficult to beat
- Only a perfect player (tier 11) can win consistently

### Does it remember past games?

The memory is embedded in the binary at compile time. It doesn't learn during gameplay.

### Can I train my own brain?

Yes! Use the `tttraining` crate:
```bash
cargo run --bin tttraining -- train --name my-brain --iterations 100000
```

## API Questions

### What ports does the server use?

Default: 8080. Configurable via `RUST_SERVER_PORT`.

### How do I make a move?

```bash
curl -X POST http://localhost:8080/api/move \
  -H "Content-Type: application/json" \
  -d '{"board":[["X", "", ""],["", "", ""],["", "", ""]], "brain":"standard"}'
```

### Is the API authenticated?

No authentication in current version. Add API key verification for production.

### What languages support the API?

The API uses JSON/HTTP, so any language can interact with it.

## Development Questions

### How do I contribute?

1. Fork the repository
2. Create a branch
3. Make changes
4. Run tests
5. Submit a PR

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for details.

### Where should I file bugs?

GitHub issues at https://github.com/AliSajid/tictacrustle/issues

### Can I request a feature?

Yes, file an enhancement suggestion issue.

### How do I report a security issue?

Contact the maintainer directly. Do not file a public issue.

## Training Questions

### How long does training take?

Depends on iterations and CPU:
- 10k iterations: ~1 minute (4-core)
- 100k iterations: ~10 minutes (4-core)
- 500k iterations: ~50 minutes (4-core)

### Can I train on CPU?

Yes, but training is faster on GPU if available.

### What if training crashes?

Training is designed to be resilient. Check logs at `assets/*.log`.

### How do I share a trained brain?

Transfer the `.base64` file to another system and rebuild.

## Documentation Questions

### Where can I find more documentation?

- [PROJECT_OVERVIEW.md](../../PROJECT_OVERVIEW.md) - High-level overview
- [ARCHITECTURE.md](../../ARCHITECTURE.md) - System design
- [TRAINING.md](../../TRAINING.md) - Training guide
- [API.md](../../API.md) - API reference

### Is there video documentation?

Not yet. The project focuses on text-based documentation.

### Where is the changelog?

See [CHANGELOG.md](../../CHANGELOG.md) for version history.

## License Questions

### What license is this project under?

Dual-licensed under MIT and Apache-2.0.

### Can I use this in my project?

Yes, under either MIT or Apache-2.0 license.

### Can I modify the code?

Yes, modifications are encouraged.

## Other Questions

### Is this a serious research project?

It's both educational and research-oriented. It honors historical RL while using modern techniques.

### Where can I learn more about RL?

- [Reinforcement Learning textbook](https://www.incompleteideas.net/tilc/)
- [Deep RL course](https://rll.berkeley.edu/deeprlcourse/)
- [RLlib](https://docs.ray.io/en/latest/rllib/)

### Can I cite this project?

Yes! BibTeX:

```bibtex
@misc{tictacrustle,
  title = {Tic Tac Toe with MENACE AI},
  author = {Imami, Ali Sajid},
  year = {2026},
  publisher = {GitHub},
  journal = {GitHub Repository},
  url = {https://github.com/AliSajid/tictacrustle}
}
```

### Contact

For questions, open an issue or contact the maintainer.
