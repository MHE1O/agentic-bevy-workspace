use bevy::prelude::*;
use engine_core::{GameState, bootstrap};

#[test]
fn headless_bootstrap_initializes_state_and_updates() {
    let mut app = bootstrap(true);

    assert_eq!(
        app.world().resource::<State<GameState>>().get(),
        &GameState::Loading
    );
    assert!(app.world().contains_resource::<Time<Fixed>>());

    app.update();
}
