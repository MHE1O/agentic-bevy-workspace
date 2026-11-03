# Agentic Bevy Workspace Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build and verify a Linux-first, agent-friendly Bevy game workspace with Avian physics, Leafwing input, asset-loader state flow, a reusable template, and an `xtask` CLI.

**Architecture:** A virtual Cargo workspace centralizes compatible dependencies. `engine_core` constructs windowed and headless apps, `_template` owns gameplay as a library plus thin binary, optional diagnostics stay behind `dev`, and a Bevy-free `xtask` handles project operations.

**Tech Stack:** Rust, Cargo, Bevy, Avian 2D, Leafwing Input Manager, `bevy_asset_loader`, `bevy-inspector-egui`, Clap, Serde YAML, `comfy-table`.

## Global Constraints

- Resolve all Bevy ecosystem crates to one compatible Bevy minor release and record exact versions.
- Every game exposes `pub fn app(headless: bool) -> App` and keeps `main.rs` thin.
- Headless execution never initializes Winit or rendering.
- Gameplay reads Leafwing actions, not raw keyboard resources; physics movement changes velocities or forces in `FixedUpdate`, not transforms.
- Required `.cargo/config.toml` settings work without `clang`, `mold`, or `lld`; fast-linker settings remain commented unless installed.
- `dev` enables optional Bevy dynamic linking and diagnostics; release remains portable.
- Only 2D template generation is implemented; `--dimension 3d` returns a clear unsupported-scaffold error.
- This directory is not a Git repository, so verification checkpoints replace commit steps.

---

### Task 1: Compatibility Sandbox

**Files:**
- Create: `compat-check/Cargo.toml`
- Create: `compat-check/src/main.rs`
- Create: `COMPATIBILITY.md`

**Interfaces:**
- Produces: an exact dependency matrix consumed by root `[workspace.dependencies]` and version-matched documentation.

- [ ] **Step 1: Discover current releases and Bevy compatibility**

Run `cargo search` for each required crate, inspect candidate crate metadata, and select a stable compatible release line rather than mixing latest releases blindly.

- [ ] **Step 2: Write the minimal resolver application**

Create `compat-check/src/main.rs`:

```rust
fn main() {}
```

Create `compat-check/Cargo.toml` with a standalone `[package]`, exact `=`-pinned candidate versions, and an empty `[workspace]` table so it is isolated from the parent workspace.

- [ ] **Step 3: Resolve and compile the matrix**

Run: `cargo check --manifest-path compat-check/Cargo.toml`
Expected: `Finished` with no duplicate-Bevy trait errors. If resolution fails, adjust the whole matrix to the documented compatible release line and repeat.

- [ ] **Step 4: Record the evidence**

Write `COMPATIBILITY.md` with selected versions, resolved Bevy version, Rust version, available linker tools, verification commands, and API notes verified from source or official documentation.

### Task 2: Root Workspace and Shared Engine

**Files:**
- Create: `Cargo.toml`
- Create: `.cargo/config.toml`
- Create: `.cargo/aliases.toml` only if aliases cannot live in `config.toml`
- Create: `crates/engine_core/Cargo.toml`
- Create: `crates/engine_core/src/lib.rs`
- Create: `crates/dev_diagnostics/Cargo.toml`
- Create: `crates/dev_diagnostics/src/lib.rs`

**Interfaces:**
- Produces: `engine_core::{bootstrap, GameState}` where `bootstrap(headless: bool) -> App`; `GameState` defaults to `Loading`.
- Produces: `dev_diagnostics::DevDiagnosticsPlugin` implementing Bevy `Plugin`.

- [ ] **Step 1: Add the virtual manifest and portable build configuration**

Define members `crates/*`, `games/*`, and `xtask`; centralize exact dependencies and shared package metadata. Set development optimization in root profiles and define `cargo xtask` in `.cargo/config.toml`. Include commented Linux `clang`/linker flags with switch instructions.

- [ ] **Step 2: Implement shared states and bootstrap**

Use `#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]` for `GameState`, with `#[default] Loading`. `bootstrap(false)` adds `DefaultPlugins`; `bootstrap(true)` adds `MinimalPlugins`, `StatesPlugin`, `AssetPlugin`, `InputPlugin`, `TransformPlugin`, and Avian `PhysicsPlugins::default()`, then inserts `Time<Fixed>` and the state as needed by the pinned APIs. Keep game-specific input, loading, menu, and player plugins out of this crate.

- [ ] **Step 3: Implement optional diagnostics**

`DevDiagnosticsPlugin` adds `FrameTimeDiagnosticsPlugin`, `WorldInspectorPlugin`, and Avian `PhysicsDebugPlugin` using the selected versions. Gate this crate's use from games with the `dev` feature.

- [ ] **Step 4: Check shared crates**

Run: `cargo check -p engine_core -p dev_diagnostics`
Expected: both crates compile without warnings or platform display initialization.

### Task 3: `xtask` CLI

**Files:**
- Create: `xtask/Cargo.toml`
- Create: `xtask/src/main.rs`
- Create: `xtask/src/tasks/mod.rs`
- Create: `xtask/src/tasks/new_game.rs`
- Create: `xtask/src/tasks/status.rs`
- Create: `xtask/src/tasks/doc.rs`
- Create: `xtask/src/tasks/verify.rs`
- Test: unit tests adjacent to parsing, validation, and copy logic

**Interfaces:**
- Produces: `cargo xtask new <game_name> [--dimension 2d|3d]`
- Produces: `cargo xtask status`
- Produces: `cargo xtask doc [--open] [--no-deps] [--private]`
- Produces: `cargo xtask verify [--game <name>]`

- [ ] **Step 1: Write failing metadata and validation tests**

Test strict frontmatter parsing with `serde_yaml`, progress bounds `0..=100`, phase and dimension enums, game names matching ASCII `[a-z][a-z0-9_]*`, missing delimiters, and malformed YAML.

- [ ] **Step 2: Implement CLI and metadata parsing**

Use Clap derive subcommands, `anyhow::Context` for actionable failures, and a typed `Scope` structure. Require both opening and closing `---` lines and deserialize only the enclosed YAML.

- [ ] **Step 3: Implement game creation**

Resolve the workspace root from `CARGO_MANIFEST_DIR`, recursively copy `games/_template` without following unrelated paths, reject existing destinations, replace the package name, and update YAML fields through the typed scope serializer. Return an explicit error for 3D.

- [ ] **Step 4: Implement status and Cargo wrappers**

Sort game rows by package/directory name and print aligned columns using `comfy-table`. Build `std::process::Command` argument lists for doc and verify; propagate failed exit statuses. Scoped verification adds `-p <game>` to fmt, Clippy, and test commands.

- [ ] **Step 5: Run CLI tests and compile check**

Run: `cargo test -p xtask && cargo check -p xtask`
Expected: all unit tests pass and the binary compiles without Bevy dependencies.

### Task 4: Template Game and Headless Simulation

**Files:**
- Create: `games/_template/Cargo.toml`
- Create: `games/_template/src/lib.rs`
- Create: `games/_template/src/main.rs`
- Create: `games/_template/src/actions.rs`
- Create: `games/_template/src/loading.rs`
- Create: `games/_template/src/menu.rs`
- Create: `games/_template/src/player.rs`
- Create: `games/_template/tests/headless_sim.rs`
- Create: `games/_template/assets/.gitkeep`
- Create: `games/_template/SCOPE.md`

**Interfaces:**
- Consumes: `engine_core::{bootstrap, GameState}` and optional `DevDiagnosticsPlugin`.
- Produces: `game_template::app(headless: bool) -> App` and `PlayerAction::{Move, Jump, Start}`.

- [ ] **Step 1: Write the failing headless integration test**

Boot `app(true)`, assert initial `GameState::Loading`, drive the state to `Playing`, press/release semantic actions, advance fixed time, and assert that a dynamic player's transform changes through Avian velocity rather than direct transform writes. Include a no-display-server test command.

- [ ] **Step 2: Define semantic actions and asset loading**

Derive `Actionlike` for `PlayerAction`; bind Move to a virtual horizontal axis or D-pad, Jump and Start to buttons, and include gamepad bindings. Add `InputManagerPlugin<PlayerAction>` and a default `InputMap`. Define a resource deriving `AssetCollection`; configure `bevy_asset_loader` for `GameState::Loading` and transition to `Menu` after collection completion.

- [ ] **Step 3: Implement menu and player systems**

Menu starts on the semantic Start action. Spawn a camera and simple menu only in windowed mode. Spawn a dynamic player with an Avian capsule/circle collider, `LinearVelocity`, and the input bundle. In `FixedUpdate`, query `ActionState<PlayerAction>`, set horizontal velocity, and apply jump velocity when pressed. Do not mutate player `Transform` in movement systems.

- [ ] **Step 4: Assemble library and binary**

`app(headless)` starts from `bootstrap(headless)`, initializes `GameState`, and adds input, loading, menu, and player plugins. `main.rs` calls `game_template::app(false).run()`. Optional `dev` enables `bevy/dynamic_linking` and diagnostics without affecting release defaults.

- [ ] **Step 5: Add strict project scope**

Write the exact YAML fields and Markdown milestone/session-log sections from the mission, with dimension `2d` and a current headless-verification update.

- [ ] **Step 6: Run template checks**

Run: `cargo test -p game_template --test headless_sim`
Expected: the headless simulation passes without X11 or Wayland.

### Task 5: Agent Knowledge Base

**Files:**
- Create: `AGENTS.md`
- Create: `.agent/AGENTS.md`
- Create: `.agent/ARCHITECTURE.md`
- Create: `.agent/BEVY_CONVENTIONS.md`
- Create: `.agent/PHYSICS_AVIAN.md`
- Create: `.agent/INPUT_LEAFWING.md`
- Create: `.agent/ASSET_PIPELINE.md`

**Interfaces:**
- Produces: concise operational and version-matched API guidance for humans and coding agents.

- [ ] **Step 1: Write operational rules and architecture**

Include scope read/update rules, crate boundaries, plugin ownership, headless verification, and the physics/input/asset golden rules. Root `AGENTS.md` points to `.agent/AGENTS.md` and target scope files.

- [ ] **Step 2: Write API-specific guides**

Document states and schedules, required components and observers where supported, Avian bodies/layers/events/spatial queries, Leafwing bindings and headless mocking, and asset collections/state transitions. Compile snippets mentally against the exact selected APIs and avoid examples from incompatible releases.

### Task 6: Full Acceptance

**Files:**
- Modify only files implicated by verification failures.

**Interfaces:**
- Consumes: every prior task's public command and crate.

- [ ] **Step 1: Format and verify the workspace**

Run: `cargo xtask verify`
Expected: formatting, warning-free Clippy, and all tests pass.

- [ ] **Step 2: Verify scoped operation and metadata**

Run: `cargo xtask verify --game game_template && cargo xtask status`
Expected: scoped checks pass and the table lists Template Prototype, Prototype, valid progress, current objective, and blockers.

- [ ] **Step 3: Verify creation and cleanup behavior**

Run `cargo xtask new smoke_game --dimension 2d`, verify package/scope changes and `cargo check -p smoke_game`, then remove only the generated smoke-test directory. Confirm 3D and duplicate-name requests fail without modifying the template.

- [ ] **Step 4: Check documentation build**

Run: `cargo xtask doc --no-deps --private`
Expected: workspace documentation builds successfully; do not use `--open` in headless automation.
