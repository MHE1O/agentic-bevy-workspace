use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Debug, PartialEq, Eq, Hash, Clone, Copy, Reflect)]
pub enum PlayerAction {
    #[actionlike(Axis)]
    Move,
    Jump,
    Start,
}

#[derive(Component)]
pub struct PlayerInput;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Startup, spawn_input);
    }
}

fn spawn_input(mut commands: Commands) {
    commands.spawn((PlayerInput, default_input_map()));
}

fn default_input_map() -> InputMap<PlayerAction> {
    let mut input_map = InputMap::default();
    input_map.insert_axis(PlayerAction::Move, VirtualAxis::ad());
    input_map.insert_axis(PlayerAction::Move, VirtualAxis::horizontal_arrow_keys());
    input_map.insert(PlayerAction::Jump, KeyCode::Space);
    input_map.insert(PlayerAction::Start, KeyCode::Space);
    input_map.insert(PlayerAction::Start, KeyCode::Enter);

    #[cfg(feature = "gamepad")]
    {
        input_map.insert_axis(PlayerAction::Move, VirtualAxis::dpad_x());
        input_map.insert_axis(PlayerAction::Move, GamepadControlAxis::LEFT_X);
        input_map.insert(PlayerAction::Jump, GamepadButton::South);
        input_map.insert(PlayerAction::Start, GamepadButton::Start);
    }

    input_map
}
