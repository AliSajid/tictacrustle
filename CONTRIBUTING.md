<!--
SPDX-FileCopyrightText: 2022 - 2026 Ali Sajid Imami
SPDX-License-Identifier: Apache-2.0
-->

<!-- omit in toc -->
# Contributing

Thank you for your interest in contributing to Tic-Tac-Rustle! This guide is for quick reference; see [DEVELOPMENT.md](./DEVELOPMENT.md) for detailed onboarding.

## How to Help

- **Report bugs**: File an issue with reproduction steps
- **Suggest features**: Open an issue describing the enhancement
- **Improve docs**: Update documentation or add examples
- **Contribute code**: Submit pull requests after reviewing guidelines

## Reporting Bugs

Before filing a bug report:

1. **Check version**: Make sure you're using the latest release
2. **Search issues**: Check if the issue already exists
3. **Gather information**:
   - Stack trace or error message
   - OS and platform
   - Rust and Cargo versions
   - Steps to reproduce

File an issue at: https://github.com/AliSajid/tictacrustle/issues/new?template=bug_report.md

## Suggesting Enhancements

Enhancement suggestions should:

1. **Describe current behavior**: What happens now
2. **Describe expected behavior**: What you want instead
3. **Explain the value**: Why this is useful
4. **Consider alternatives**: What else you've tried

## Your First Contribution

### Finding Issues

- **Good first issue**: Look for issues tagged `good first issue`
- **Documentation**: Fix typos, improve clarity
- **Bug fixes**: Issues marked `bug` with reproduction steps

### Setting Up Development

```bash
# Clone the repository
git clone https://github.com/AliSajid/tictacrustle.git
cd tictacrustle

# Install toolchain
mise use -t ttrustle

# Run tests
cargo test --workspace
```

### Making Changes

1. **Create a branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**: Edit code, add tests, update docs

3. **Run tests and clippy**:
   ```bash
   cargo test --workspace
   cargo clippy --workspace --all-targets
   cargo fmt --check
   ```

4. **Commit with conventional commits**:
   ```bash
   git commit -m "feat: add your feature

   - Brief description of what changed
   - References to related issues or PRs
   "
   ```

5. **Push and create PR**:
   ```bash
   git push origin feature/your-feature-name
   ```

## Code Style

### Formatting

We use `rustfmt` for consistent formatting:

```bash
cargo fmt
```

### Linting

Run `clippy` before submitting:

```bash
cargo clippy --workspace --all-targets
```

### Documentation

All public APIs should have rustdoc comments. See [DEVELOPMENT.md](./DEVELOPMENT.md) for examples.

## Commit Messages

Use Conventional Commits:

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `test:` Test changes
- `chore:` Maintenance
- `refactor:` Code changes without feature changes

Example:

```
fix: correct board state validation for edge case

- Handle symmetric board states correctly
- Add test case for rotational symmetry
- Update unit tests to cover edge cases

Closes #123
```

## Pull Request Guidelines

- Keep PRs focused on one change
- Link related issues
- Include relevant tests
- Update documentation as needed
- Add rustdoc comments for new public APIs

## Review Process

1. Submit your PR
2. A maintainer will review
3. Address feedback iteratively
4. Merge once approved

## Code of Conduct

Please read and follow our [Code of Conduct](./CODE_OF_CONDUCT.md).

## License

By contributing, you agree that your submissions will be licensed under the project's dual MIT/Apache-2.0 license.

## Questions?

- **Open an issue**: For questions about the project
- **Check documentation**: See the [guide](./guide/) for user-facing docs
- **Read DEVELOPMENT.md**: For detailed developer guidelines
