# Avian2D 0.8

## Bodies and Motion

- `RigidBody::Dynamic`: simulated by forces, contacts, and velocity.
- `RigidBody::Kinematic`: controlled by gameplay without dynamic force response.
- `RigidBody::Static`: immovable level geometry.

```rust
use avian2d::prelude::*;
use bevy::prelude::*;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        RigidBody::Dynamic,
        Collider::capsule(0.35, 0.9),
        LinearVelocity::ZERO,
        Transform::default(),
    ));
}
```

Move physical entities through velocity or forces in `FixedUpdate`:

```rust
fn move_player(mut velocity: Single<&mut LinearVelocity, With<crate::player::Player>>) {
    velocity.x = 160.0;
}
```

Initial placement with `Transform` is valid; direct per-frame physical movement is not.

## Collision Layers and Sensors

```rust
use avian2d::prelude::*;

#[derive(PhysicsLayer, Default)]
enum GameLayer {
    #[default]
    Default,
    Player,
    Ground,
    Goal,
}

fn layers() -> CollisionLayers {
    CollisionLayers::new(GameLayer::Player, [GameLayer::Ground, GameLayer::Goal])
}
```

Add `Sensor` to a collider for overlap events without a physical collision response.

## Collision Messages

Bevy 0.20 uses `MessageReader` for Avian collision messages:

```rust
use avian2d::prelude::*;
use bevy::{ecs::message::MessageReader, prelude::*};

fn collisions(mut messages: MessageReader<CollisionStart>) {
    for CollisionStart { collider1, collider2 } in messages.read() {
        info!("{collider1} touched {collider2}");
    }
}
```

## Spatial Queries

`SpatialQuery` is a system parameter. A 2D ray uses `Dir2`:

```rust
use avian2d::prelude::*;
use bevy::prelude::*;

fn ground_check(spatial: SpatialQuery) {
    let filter = SpatialQueryFilter::default();
    let hit = spatial.cast_ray(Vec2::ZERO, Dir2::NEG_Y, 2.0, true, &filter);
    if let Some(hit) = hit {
        info!("ground distance: {}", hit.distance);
    }
}
```

## Schedules and Debug Rendering

`PhysicsPlugins::default()` runs `PhysicsSchedule` in `FixedPostUpdate`. For deterministic manual simulation, advance `Time<Physics>` and run `PhysicsSchedule`, or use Bevy fixed-time advancement. `PhysicsDebugPlugin` belongs only in windowed `dev` builds.
