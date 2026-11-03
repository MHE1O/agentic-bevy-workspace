# Bevy Asset Loader 0.28

## Collection Ownership

Define every asset handle in `loading.rs`; gameplay systems consume the inserted resource rather than calling `AssetServer::load` with ad-hoc paths.

```rust
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    // Add real handles here, for example:
    // #[asset(path = "player.png")]
    // pub player: Handle<Image>,
}
```

The template intentionally starts with an empty collection so headless tests need no files or render asset types.

## State Configuration

```rust
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use engine_core::GameState;

fn configure(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::Menu)
            .load_collection::<GameAssets>(),
    );
}
```

`LoadingState::new` selects the loading state, `load_collection` registers the collection, and `continue_to_state` selects `Menu` after all handles load. The collection is inserted as a resource before gameplay accesses it.

## Rules

- Keep asset paths relative to the game's `assets/` directory.
- Add new handles to `GameAssets`, not local system parameters.
- Use `OnEnter(GameState::Playing)` for setup that depends on loaded resources.
- Keep the empty-collection path working headlessly; asset integration tests may mock or use lightweight asset types without Winit.
- Configure an explicit failure state before shipping assets that must not fail silently.
