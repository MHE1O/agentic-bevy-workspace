//! Optional windowed diagnostics for development builds.

use avian2d::prelude::PhysicsDebugPlugin;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

/// Inspector, FPS overlay, and physics debug rendering.
pub struct DevDiagnosticsPlugin;

impl Plugin for DevDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            FrameTimeDiagnosticsPlugin::default(),
            PhysicsDebugPlugin,
        ))
        .add_systems(Startup, spawn_fps_overlay)
        .add_systems(Update, update_fps_overlay);
    }
}

#[derive(Component)]
struct FpsOverlay;

fn spawn_fps_overlay(mut commands: Commands) {
    commands.spawn((
        Text::new("FPS: --"),
        TextFont {
            font_size: bevy::text::FontSize::Px(18.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            left: Val::Px(8.0),
            ..default()
        },
        FpsOverlay,
    ));
}

fn update_fps_overlay(
    diagnostics: Res<DiagnosticsStore>,
    mut overlay: Single<&mut Text, With<FpsOverlay>>,
) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.smoothed())
        .map_or_else(|| "--".to_owned(), |value| format!("{value:.1}"));
    overlay.0 = format!("FPS: {fps}");
}
