# Compatibility Matrix

Verified on 2026-11-02 with `rustc 1.99.0` and `cargo 1.99.0`.

| Crate | Exact version | Bevy line |
| --- | --- | --- |
| `bevy` | `0.20.1` | `0.20.1` |
| `avian2d` | `0.8.0` | `0.20.1` |
| `leafwing-input-manager` | `0.22.0` | `0.20.1` |
| `bevy_asset_loader` | `0.28.1` | `0.20.1` |
| `bevy-inspector-egui` | `0.38.0` | `0.20.1` |

## Evidence

`cargo check --manifest-path compat-check/Cargo.toml` completed successfully with one resolved Bevy version. The final sandbox used the same reduced feature set as the workspace: Bevy's custom 2D/UI profile, Avian without default debug/picking/scene integrations, Leafwing keyboard/mouse/asset support, and the inspector's 2D-only integration.

The first all-default-features attempt failed because this machine lacks the `libudev` development package. It also revealed that mixing Bevy `0.18.1` with current Avian `0.8.0` and Leafwing `0.22.0` resolves two Bevy lines. The workspace therefore uses Bevy `0.20.1` throughout.

## Linux Tools

- Rust: installed and working.
- `clang`, `mold`, `lld`, and `ld.lld`: not found on `PATH` during the pre-flight check.
- Fast-linker settings are commented opt-ins in `.cargo/config.toml`; the default build uses the system Rust linker driver.

## Feature Decisions

- Default builds omit Bevy audio and `bevy_gilrs`, avoiding host audio/input development packages in CI.
- Physical gamepad support is available through the template's optional `gamepad` feature, which enables `bevy/bevy_gilrs` and `leafwing-input-manager/gamepad`; Linux hosts then need the appropriate `libudev` development package.
- Avian's debug plugin is enabled only through `dev_diagnostics`, not in headless/default game builds.
- Bevy dynamic linking is enabled only by the template's `dev` feature.

## Verified API Notes

- Leafwing `0.22` stores `ActionState<A>` and `InputMap<A>` as entity components; gameplay queries them with `Query`, and tests can mutate `ActionState` directly with `press`/`release`.
- Avian uses `PhysicsPlugins::default()`, `RigidBody`, `Collider`, `LinearVelocity`, `CollisionLayers`, and `PhysicsDebugPlugin` from its prelude.
- Asset loading uses `LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu).load_collection::<GameAssets>()` with `App::add_loading_state`.
- Inspector setup uses `EguiPlugin::default()` and `WorldInspectorPlugin::new()` when diagnostics are enabled.
- Bevy states use the `States` derive and `App::init_state`; headless tests manually call `App::update` and control fixed time rather than starting Winit.
