# Agentic Bevy Workspace Design

## Goal

Create a Linux-first Cargo virtual workspace for small 2D Bevy games, with reproducible dependency versions, semantic input, ECS physics, explicit state transitions, headless integration tests, and agent-oriented project tooling.

## Dependency and Toolchain Gate

Before implementation, resolve and compile one exact Bevy-compatible release line for `bevy`, `avian2d`, `leafwing-input-manager`, `bevy_asset_loader`, and `bevy-inspector-egui`. Record the selected versions and relevant APIs in the workspace documentation. Use Cargo workspace dependencies so every crate resolves the same Bevy release.

The `.cargo/config.toml` must work on a stock Linux Rust installation. Compiler and linker flags for `clang` plus `mold` or `lld` will be documented, commented opt-ins unless the corresponding tools are installed. Development optimization and optional Bevy dynamic linking are workspace-level concerns; release builds must not require dynamic linking.

## Architecture

- `crates/engine_core`: owns `GameState` (`Loading`, `Menu`, `Playing`) and shared app construction. Windowed apps use `DefaultPlugins`; headless apps use `MinimalPlugins` plus only the asset, input, state, transform, and physics support required by simulation tests. No window or renderer is initialized headlessly.
- `crates/dev_diagnostics`: optional development diagnostics, including inspector, frame-rate display, and Avian debug rendering. It is excluded from default game builds and enabled through the game's `dev` feature.
- `games/_template`: a library-first 2D game exposing `pub fn app(headless: bool) -> App`, plus a thin binary. Modules own actions, asset loading, menu transitions, and the physics-driven player.
- `xtask`: a lightweight CLI with no Bevy dependency. It creates games from `_template`, reports parsed scope metadata, wraps documentation commands, and runs formatting, linting, and tests.
- `.agent`: version-matched operational guidance. The root `AGENTS.md` directs agents to the detailed knowledge base.

## Runtime Data Flow

Keyboard or gamepad input updates Leafwing `ActionState<PlayerAction>`. Fixed-schedule player systems consume that resource and modify Avian velocities rather than transforms. Avian updates physics and collision data. State transitions follow `Loading -> Menu -> Playing`; loading collections own asset handles, menu systems handle the start action, and state entry/exit systems manage setup and cleanup. The headless test inserts mocked actions, advances fixed time, and observes physics results without a display server.

## Project Metadata and CLI

Every `games/<name>/SCOPE.md` has strict YAML frontmatter with `title`, `phase`, `progress`, `dimension`, `current_objective`, and `blockers`. `xtask status` parses that frontmatter with a YAML parser and renders an aligned table. `xtask new` rejects invalid or existing game names, copies the template recursively, updates package and frontmatter metadata, and rejects unsupported 3D scaffolding explicitly rather than silently emitting a mislabeled 2D game.

`xtask verify` scopes all three Cargo operations to `-p <name>` when requested. `xtask doc` maps `--private` to Cargo's `--document-private-items` and forwards `--open` and `--no-deps`. Child-process failures return nonzero exits with actionable context.

## Error Handling

The CLI uses structured errors for invalid names, malformed metadata, I/O failures, and failed Cargo commands. Game startup has one deterministic plugin path per mode. Headless tests avoid asset files and platform windowing dependencies. No required build setting assumes an uninstalled linker.

## Testing and Acceptance

- Unit-test CLI name validation, frontmatter parsing, and template copying where practical.
- Compile `xtask` independently before adding the game.
- Boot the template headlessly, mock movement and jump input, advance physics, and assert semantic action and physical motion behavior.
- Run `cargo xtask verify`, `cargo xtask status`, and a scoped verification.
- Confirm formatting, warning-free Clippy checks, tests, and documentation commands succeed with the locked dependency matrix.
