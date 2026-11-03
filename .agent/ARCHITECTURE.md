# Workspace Architecture

## Crate Boundaries

- `crates/engine_core`: `bootstrap(headless: bool) -> App`, shared `GameState`, and Avian plugin setup. No gameplay.
- `crates/dev_diagnostics`: windowed inspector, FPS text, and physics debug rendering. Games include it only through `dev`.
- `games/<name>`: library-first game code. `src/lib.rs` exposes `pub fn app(headless: bool) -> App`; `src/main.rs` only runs `app(false)`.
- `xtask`: Clap CLI, scope parsing, filesystem operations, and Cargo wrappers. It deliberately does not depend on Bevy.

## Template Modules

- `actions.rs`: semantic action enum, input map, and global `PlayerInput` entity.
- `loading.rs`: `GameAssets` and `LoadingState` configuration.
- `menu.rs`: semantic start transition and window-only menu lifecycle.
- `player.rs`: Avian bodies and fixed-schedule velocity control; visuals are separate from simulation.

## Headless Contract

`bootstrap(true)` adds `MinimalPlugins`, states, assets, input, transforms, and `PhysicsPlugins`. It never adds Winit or rendering. Tests must call:

```rust
let mut app = game_template::app(true);
app.finish();
app.cleanup();
app.update();
```

`finish()` and `cleanup()` are required because plugins such as Avian initialize resources in their lifecycle hooks. Use `TimeUpdateStrategy::ManualDuration` for deterministic fixed-step updates.

## Dependency Flow

Games depend on shared crates; shared crates never depend on games. `xtask` operates through paths and subprocesses. Root `[workspace.dependencies]` is the only ecosystem version source. New games copied from `_template` are discovered by the root `games/*` member glob.
