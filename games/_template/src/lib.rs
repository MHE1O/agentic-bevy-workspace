//! Reusable game plugin and headless-capable application entrypoint.

pub mod actions;
pub mod loading;
pub mod menu;
pub mod player;

use bevy::prelude::*;
use engine_core::bootstrap;

use crate::{
    actions::ActionsPlugin, loading::LoadingPlugin, menu::MenuPlugin, player::PlayerPlugin,
};

/// Build the game with windowed rendering or display-free simulation plugins.
pub fn app(headless: bool) -> App {
    let mut app = bootstrap(headless);
    app.add_plugins((
        ActionsPlugin,
        LoadingPlugin,
        MenuPlugin { headless },
        PlayerPlugin { headless },
    ));

    if !headless {
        app.add_systems(Startup, spawn_camera);
    }

    #[cfg(feature = "dev")]
    if !headless {
        app.add_plugins(dev_diagnostics::DevDiagnosticsPlugin);
    }

    app
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
