# ttrustle Workspace Structure

This workspace has been converted from a monolithic crate into a modular workspace with individual crates.

## Architecture

```
ttrustle/ (workspace root)
├── Cargo.toml                    # Workspace members, shared dependencies
├── mise.toml                     # Common tools, profiles, env vars
├── WORKSPACE.md                  # This file
├── guide/                        # Documentation
└── [crates]
    ├── ttrustle-lib/
    │   ├── Cargo.toml            # Per-crate: lints
    │   ├── mise.toml             # Per-crate: settings, env vars
    │   └── src/
    │       ├── lib.rs
    │       ├── board.rs
    │       ├── errors.rs
    │       ├── game.rs
    │       ├── player.rs
    │       ├── square.rs
    │       └── square_value.rs
    └── ttrustle/
        ├── Cargo.toml            # Per-crate: lints
        ├── mise.toml             # Per-crate: settings, env vars
        └── src/
            └── main.rs
```

## Workspace Root: `Cargo.toml`

- Defines workspace members
- Shared dependencies via `[workspace.dependencies]`
- Workspace package metadata (edition, authors, etc.)
- Rust version from workspace
- Profiles via `[workspace.metadata.mise]`

## Workspace Root: `mise.toml`

**Common tools** (shared across all crates):
- cargo-* tools (audit, llvm-cov, bacon, etc.)
- Development tools (mdbook, prettier, etc.)
- Environment variables (ANTHROPIC_* variables)

**Workspace profiles** (applied to all crates):
- `profile.dev.opt-level = 1`
- `profile.release.opt-level = "z"` with LTO

## Per-Crate: `Cargo.toml` Settings

### ttrustle-lib (Library crate)

```toml
[package]
name = "ttrustle-lib"
authors.workspace = true
edition.workspace = true
version.workspace = true
rust-version.workspace = true

# Per-crate lints (library-specific)
[lints]
rust.missing_docs = "allow"
```

### ttrustle (Binary crate)

```toml
[package]
name = "ttrustle"
authors.workspace = true
edition.workspace = true
version.workspace = true
rust-version.workspace = true

[[bin]]
name = "ttrustle"
path = "src/main.rs"

# Per-crate lints (binary-specific)
[lints]
rust.missing_docs = "allow"
rust.unused_must_use = "allow"  # Allow ignoring const return values
```

## Per-Crate: `mise.toml` Settings

### ttrustle-lib/mise.toml

```toml
[settings]
rust-version = "1.85.1"

[env."ttrustle-lib"]
RUSTFLAGS = "-A dead_code -Wmissing_docs"
```

**Rationale**: Library crate allows `dead_code` since internals may not be used directly.

### ttrustle/mise.toml

```toml
[settings]
rust-version = "1.85.1"
edition = "2024"

[env."ttrustle"]
RUSTFLAGS = "-D warnings"
```

**Rationale**: Binary crate uses stricter warnings.

## Adding New Crates

1. Create new crate directory (e.g., `tttpub/`)
2. Create `Cargo.toml` with workspace inheritance
3. Create `mise.toml` with per-crate settings
4. Add to root `Cargo.toml` in `members` list
5. Add profile settings to root `mise.toml` `[profiles]` section

### Example: Adding tttpub

```bash
mkdir -p tttpub/src
```

```toml
# tttpub/Cargo.toml
[package]
name = "tttpub"
authors.workspace = true
edition.workspace = true
version.workspace = true
rust-version.workspace = true

[dependencies]
ttrustle-lib.workspace = true
serde.workspace = true

[lints]
rust.missing_docs = "allow"
```

```toml
# tttpub/mise.toml
[settings]
rust-version = "1.85.1"

[env."tttpub"]
RUSTFLAGS = "-D warnings"
```

## Verification Commands

```bash
# Check workspace builds
cargo check --workspace

# Run tests
cargo test --workspace

# Build release
cargo build --workspace --release
```
