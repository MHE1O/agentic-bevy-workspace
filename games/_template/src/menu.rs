use bevy::prelude::*;
use engine_core::GameState;
use leafwing_input_manager::prelude::ActionState;

use crate::actions::{PlayerAction, PlayerInput};

pub struct MenuPlugin {
    pub headless: bool,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, start_game.run_if(in_state(GameState::Menu)));
        if !self.headless {
            app.add_systems(OnEnter(GameState::Menu), spawn_menu)
                .add_systems(OnExit(GameState::Menu), despawn_menu);
        }
    }
}

#[derive(Component)]
struct MenuRoot;

fn start_game(
    actions: Single<&ActionState<PlayerAction>, With<PlayerInput>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if actions.just_pressed(&PlayerAction::Start) {
        next_state.set(GameState::Playing);
    }
}

fn spawn_menu(mut commands: Commands) {
    commands.spawn((
        Text::new("Press Space or Enter to play"),
        TextFont {
            font_size: bevy::text::FontSize::Px(32.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(45.0),
            left: Val::Percent(25.0),
            ..default()
        },
        MenuRoot,
    ));
}

fn despawn_menu(mut commands: Commands, menu: Single<Entity, With<MenuRoot>>) {
    commands.entity(*menu).despawn();
}
