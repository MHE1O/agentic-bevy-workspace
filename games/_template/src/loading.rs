use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use engine_core::GameState;

/// Every asset used by gameplay belongs in this collection.
#[derive(AssetCollection, Resource)]
pub struct GameAssets {}

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<GameAssets>(),
        );
    }
}
