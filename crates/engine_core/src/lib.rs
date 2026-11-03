//! Shared application bootstrap and lifecycle states.

use avian2d::prelude::PhysicsPlugins;
use bevy::{
    asset::AssetPlugin, input::InputPlugin, prelude::*, state::app::StatesPlugin,
    transform::TransformPlugin,
};

/// Top-level lifecycle shared by every game.
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Playing,
}

/// Construct a windowed game app or a display-free simulation app.
pub fn bootstrap(headless: bool) -> App {
    let mut app = App::new();

    if headless {
        app.add_plugins((
            MinimalPlugins,
            StatesPlugin,
            AssetPlugin::default(),
            InputPlugin,
            TransformPlugin,
            PhysicsPlugins::default(),
        ));
    } else {
        app.add_plugins((DefaultPlugins, PhysicsPlugins::default()));
    }

    app.init_state::<GameState>();
    app
}
