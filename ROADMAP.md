# Project Roadmap & Release Milestones

This document marks our current progression trajectory and outlines what remains before reaching a stable production release.

---

## Where We Are Right Now (Current Status)

* [ ] **Project Establishment**: Initial setup is done
* [ ] **Automation Setup**: Automation for testing and building are in place

---

## Upcoming Milestones

### Phase 1: Infrastructure Building

* [ ] **Monorepo Architecture Setup:** `mise` workflow orchestrated and verified for both Node.js and Rust environments.
* [ ] **Core Domain Engine Finished:** Ternary calculation routines, array mapping filters, and win/loss logic locked down inside `lib_tictacrustle`.
* [ ] **Compile-Time Pruning Logic:** Automated pruning in `build.rs` reducing the search tree down from 19,683 states down to the legal 5,478 elements.

### Phase 2: The Training CLI & Artifact Pipeline (`traintrustle`)

* [ ] Implement the automated simulation loop runner.
* [ ] Integrate the step-by-step analytics tracker to export delta metrics into `ttweb/static/learning_history.json`.
* [ ] Build the Base64 ASCII output encoder for GitHub-friendly version control.

### Phase 3: The High-Performance Network Server (`ttserver`)

* [ ] Establish the Axum asynchronous routing skeleton.
* [ ] Implement macro compilation boundaries utilizing `include_str!` to bind all 11 milestone brains into static thread-safe memory arrays.
* [ ] Integrate request size constraints, `tower-governor` rate limiters, and CORS access control lists.

### Phase 4: Desktop Interface (`tttui`)

* [ ] Implement a lightweight local terminal wrapper leveraging the `ratatui` crate using the exact same embedded asset arrays for local offline matches.

### Phase 5: Frontend Visualization Studio (`ttweb`)

* [ ] Set up the core interactive SvelteKit Tic-Tac-Toe arena board.
* [ ] Integrate an Evolution Slider tied to the 11 evolutionary checkpoints.
* [ ] Build graph visualization dashboards using D3.js/LayerCake to display weight curves and decision trajectories utilizing the generated log history.

---

## Anticipated Full Release Target

We're tracking towards a highly stable, completely self-contained deployment release. 

* **Alpha Release (Internal Integration testing):** Expected mid-horizon (4–6 weeks out), featuring complete local server routing connectivity to the baseline SvelteKit views.
* **Beta Release (Open-source optimization phase):** Expected 8–10 weeks out, allowing public testing, performance validation, and security fuzzing of the API routes.
* **V1.0.0 Full Release:** Target completion within **12 weeks**. This final build will provide a single-executable compilation profile where `ttserver` serves the pre-compiled static SPA frontend assets natively on a single port for seamless zero-dependency deployment.

