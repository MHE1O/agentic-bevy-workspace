use avian2d::prelude::*;
use bevy::prelude::*;
use engine_core::GameState;
use leafwing_input_manager::prelude::ActionState;

use crate::actions::{PlayerAction, PlayerInput};

const MOVE_SPEED: f32 = 160.0;
const JUMP_SPEED: f32 = 260.0;

pub struct PlayerPlugin {
    pub headless: bool,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        if self.headless {
            app.add_systems(OnEnter(GameState::Playing), spawn_physics);
        } else {
            app.add_systems(
                OnEnter(GameState::Playing),
                (spawn_physics, add_visuals).chain(),
            );
        }
        app.add_systems(
            FixedUpdate,
            control_player.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct Ground;

fn spawn_physics(mut commands: Commands) {
    commands.spawn((
        Player,
        RigidBody::Dynamic,
        Collider::capsule(0.35, 0.9),
        LinearVelocity::ZERO,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        Ground,
        RigidBody::Static,
        Collider::rectangle(800.0, 40.0),
        Transform::from_xyz(0.0, -100.0, 0.0),
    ));
}

fn add_visuals(
    mut commands: Commands,
    player: Single<Entity, With<Player>>,
    ground: Single<Entity, With<Ground>>,
) {
    commands.entity(*player).insert(Sprite::from_color(
        Color::srgb(0.2, 0.7, 1.0),
        Vec2::new(40.0, 70.0),
    ));
    commands.entity(*ground).insert(Sprite::from_color(
        Color::srgb(0.25, 0.8, 0.35),
        Vec2::new(800.0, 40.0),
    ));
}

fn control_player(
    actions: Single<&ActionState<PlayerAction>, With<PlayerInput>>,
    mut velocity: Single<&mut LinearVelocity, With<Player>>,
) {
    velocity.x = actions.value(&PlayerAction::Move) * MOVE_SPEED;
    if actions.pressed(&PlayerAction::Jump) {
        velocity.y = JUMP_SPEED;
    }
}
