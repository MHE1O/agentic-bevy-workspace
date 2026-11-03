use std::time::Duration;

use avian2d::prelude::LinearVelocity;
use bevy::{prelude::*, time::TimeUpdateStrategy};
use engine_core::GameState;
use game_template::{
    actions::{PlayerAction, PlayerInput},
    app,
    player::Player,
};
use leafwing_input_manager::{prelude::ActionState, user_input::Buttonlike};

fn player_entity(world: &mut World) -> Entity {
    let mut query = world.query_filtered::<Entity, With<Player>>();
    query.single(world).expect("one player")
}

fn input_entity(world: &mut World) -> Entity {
    let mut query = world.query_filtered::<Entity, With<PlayerInput>>();
    query.single(world).expect("one semantic input entity")
}

#[test]
fn semantic_input_drives_avian_physics_headlessly() {
    let mut app = app(true);
    app.finish();
    app.cleanup();
    assert_eq!(
        app.world().resource::<State<GameState>>().get(),
        &GameState::Loading
    );
    app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
        1.0 / 60.0,
    )));

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Playing);
    app.update();
    app.update();

    let player = player_entity(app.world_mut());
    let input = input_entity(app.world_mut());
    let initial_x = app
        .world()
        .get::<Transform>(player)
        .expect("transform")
        .translation
        .x;

    KeyCode::KeyD.press(app.world_mut());
    app.update();
    let observed_action = app
        .world()
        .get::<ActionState<PlayerAction>>(input)
        .expect("action state")
        .value(&PlayerAction::Move);
    let observed_velocity = app
        .world()
        .get::<LinearVelocity>(player)
        .expect("linear velocity")
        .x;
    assert_eq!(
        (observed_action, observed_velocity),
        (1.0, 160.0),
        "semantic action must reach the fixed-update controller"
    );
    for _ in 0..11 {
        app.update();
    }

    let moved_x = app
        .world()
        .get::<Transform>(player)
        .expect("transform")
        .translation
        .x;
    assert!(
        moved_x > initial_x + 0.1,
        "player did not move: {initial_x} -> {moved_x}"
    );

    KeyCode::Space.press(app.world_mut());
    app.update();
    let vertical_velocity = app
        .world()
        .get::<LinearVelocity>(player)
        .expect("linear velocity")
        .y;
    assert!(
        vertical_velocity > 0.0,
        "jump velocity was {vertical_velocity}"
    );

    KeyCode::Space.release(app.world_mut());
    KeyCode::KeyD.release(app.world_mut());
    app.update();
    let horizontal_velocity = app
        .world()
        .get::<LinearVelocity>(player)
        .expect("linear velocity")
        .x;
    assert_eq!(horizontal_velocity, 0.0);
}
