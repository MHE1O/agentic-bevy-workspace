# Bevy 0.20 Conventions

## States and Schedules

`GameState` derives `States` and defaults to `Loading`. Use `OnEnter`/`OnExit` for setup and cleanup, `Update` for UI and visual work, and `FixedUpdate` for physics-driven gameplay.

```rust
use bevy::prelude::*;
use engine_core::GameState;

fn spawn() {}

fn configure(app: &mut App) {
    app.init_state::<GameState>()
        .add_systems(OnEnter(GameState::Playing), spawn);
}
```

Request a transition rather than replacing state resources:

```rust
fn finish_loading(mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::Menu);
}
```

## Required Components

Prefer required components for invariants instead of repeating bundles:

```rust
use bevy::prelude::*;

#[derive(Component)]
#[require(Transform, Visibility)]
struct Actor;
```

Spawning `Actor` also inserts default `Transform` and `Visibility` values. Explicitly supplied components override required defaults.

## Observers

Bevy 0.20 observers use `On<Event>` and run immediately when triggered:

```rust
use bevy::prelude::*;

#[derive(Event)]
struct Jumped;

fn watch(app: &mut App) {
    app.add_observer(|_: On<Jumped>| {
        info!("jumped");
    });
}
```

Use observers for reactive domain events, not routine per-frame movement. Lifecycle observer syntax is `On<Add<Component>>` in 0.20; older `Trigger` and `On<Add, Component>` examples are not compatible.

## Queries

`Single` is appropriate only when exactly one match is an invariant. Use `Query` when a state may legitimately have zero matches. Keep headless tests free of camera, window, and render resources.
