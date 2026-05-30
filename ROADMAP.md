# Roadmap & Release Milestones

This document outlines the current progression and future plans for Tic-Tac-Rustle.

---

## Current Status

**Version:** 1.0.0-next.2

**Architecture:** Workspace with modular crates

**Features Implemented:**

- [x] Monorepo setup with `mise` workflow
- [x] Core game logic in `ttrustle-lib`
- [x] CLI in `ttrustle`
- [x] Training crate `tttraining` (NEW)
- [x] API server `ttserver`
- [x] Terminal UI `ttui`
- [x] Desktop GUI `ttgui` (in progress)
- [x] Web frontend `ttweb` (SvelteKit)
- [x] REUSE compliance for licenses

**Pending:**

- [ ] Complete GUI `ttgui`
- [ ] Static asset bundler `ttstatic`
- [ ] CI/CD pipeline for training artifacts
- [ ] Documentation completion

---

## Upcoming Milestones

### Phase 1: Infrastructure (Current)

- [x] Workspace structure finalized
- [x] Training crate implemented
- [ ] CI/CD for training artifacts
- [ ] Automated asset generation pipeline

**ETA:** In progress (current sprint)

### Phase 2: Frontend Integration

- [ ] `ttstatic` crate for static asset bundling
- [ ] SvelteKit SPA bundled into `ttweb`
- [ ] Zero-dependency deployment profile
- [ ] Production API integration

**ETA:** 4-6 weeks

### Phase 3: GUI Completion

- [ ] Complete `ttgui` with GTK bindings
- [ ] Settings and preferences panel
- [ ] Game visualization with animations
- [ ] Statistics and history tracking

**ETA:** 6-8 weeks

### Phase 4: Documentation

- [ ] User guide completion
- [ ] API reference documentation
- [ ] Example projects and tutorials
- [ ] Migration guides

**ETA:** Ongoing

### Phase 5: Beta Release

- [ ] All major features implemented
- [ ] Performance benchmarks documented
- [ ] Security audit completed
- [ ] Public testing enabled

**ETA:** 10-12 weeks from project start

### Phase 6: V1.0.0 Release

- [ ] Single-executable deployment profile
- [ ] Complete documentation suite
- [ ] Release notes and changelog
- [ ] Community onboarding complete

**ETA:** 12 weeks from project start

---

## Feature Backlog

### Core Engine

- [ ] MENACE-S variant implementation
- [ ] Custom loss functions
- [ ] Multi-agent simulation mode
- [ ] Statistical analysis export

### Training Pipeline

- [ ] Auto-training daemon
- [ ] Cloud-based distributed training
- [ ] Training progress visualization
- [ ] Model comparison tools

### UI Enhancements

- [ ] Undo/redo support
- [ ] Game history tracking
- [ ] Statistics dashboard
- [ ] Custom brain editor

### Server Features

- [ ] WebSocket support for real-time games
- [ ] Game replay endpoints
- [ ] Admin dashboard
- [ ] Usage analytics

### Developer Tools

- [ ] Benchmark suite
- [ ] Debug visualization tools
- [ ] Test fixtures library
- [ ] Example templates

---

## Release Timeline

| Phase | Target | Deliverables |
|-------|--------|----|
| Alpha | Mid-sprint | Internal integration |
| Beta | 8-10 weeks | Public testing |
| V1.0 | 12 weeks | Full release |

---

## Changelog

See [CHANGELOG.md](./CHANGELOG.md) for version history.

---

## How to Contribute

Looking for issues to work on? Check these tags:

- `help wanted` - Good for beginners
- `good first issue` - Simple tasks
- `enhancement` - Feature requests
- `bug` - Issues to fix

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

---

## Architecture Evolution

The project has evolved through several phases:

1. **Monolithic** (2022-2024): Single crate, hard to maintain
2. **Workspace** (2024-present): Modular crates, clear separation

The workspace split enables:
- Parallel development
- Independent testing
- Clear responsibility boundaries

See [WORKSPACE.md](./WORKSPACE.md) for details.

---

## Community

- **GitHub**: https://github.com/AliSajid/tictacrustle
- **Issues**: https://github.com/AliSajid/tictacrustle/issues
- **Documentation**: https://docs.rs/tictacrustle
