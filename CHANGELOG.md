## Changelog

All notable changes to Tic-Tac-Rustle will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.0-next.3] - [Unreleased]

### Added

- `tttraining` crate for training simulations and weight generation
- `PROJECT_OVERVIEW.md` for high-level project explanation
- `MENACE.md` for historical context and implementation details
- `TRAINING.md` for training workflow documentation
- `API.md` for API reference documentation
- `DEVELOPMENT.md` for developer onboarding guide

### Changed

- Restructured from monolithic crate to modular workspace
- Updated `README.md` with current project state
- Updated `ARCHITECTURE.md` with complete crate catalog
- Updated `WORKSPACE.md` with workspace structure and adding new crates guide
- Updated `CONTRIBUTING.md` with contribution guidelines
- Updated `ROADMAP.md` with current milestones and timeline

### Fixed

- `README.md` - outdated badges and description
- `CONTRIBUTING.md` - removed reference to "Gainful Key"

---

## [1.0.0-next.2] (2026-05-28)

### Added

- `ttgui` desktop GUI crate (GTK-based)
- `ttstatic` crate for static asset bundling
- REUSE compliance for license headers

### Changed

- Restructured project into Cargo workspace
- Created `ttrustle-lib` as core library crate
- Created `ttrustle` as CLI binary crate
- Created `ttserver` as API server crate
- Created `ttui` as terminal UI crate

### Fixed

- Cleanup settings and vale rules
- Clear warnings in CI scripts

---

## [1.0.0-next.1] (2024-02-26)

### Added

- Basic board representation for showing game state
- Game player types (human, random, AI)
- Game error types with proper Display implementations
- Game play functionality

### Changed

- Reorganized crate structure
- Moved from monolithic to binary structure

### Fixed

- Allow dead code for unimplemented parts
- Fix print warnings
- Fix player ambiguity bug

---

## [0.1.0] (Initial Release)

### Added

- Core game logic implementation
- Board state representation
- Game rules and win detection
- MENACE system foundation

---

## Migration Guide

### From Monolithic to Workspace

If you're migrating from the previous monolithic version:

1. **Update dependencies**: Point to workspace members
2. **Update paths**: `ttrustle/` -> use `ttrustle-lib` for core logic
3. **Update imports**: Add `use ttrustle_lib::*;` for core types

### Before Migration

```toml
# Old monolithic Cargo.toml
[package]
name = "ttrustle"
version = "1.0.0-next.1"
```

### After Migration

```toml
# Workspace root Cargo.toml
[workspace]
members = ["ttrustle-lib", "ttrustle", "tttraining", ...]
```

---

## [Unreleased] - Known Issues

- `ttgui` GUI still in development
- `ttweb` integration with `ttstatic` pending
- Some documentation sections incomplete
