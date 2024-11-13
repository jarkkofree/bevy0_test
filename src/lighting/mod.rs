use bevy::prelude::*;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_scene);
    }
}

fn spawn_scene(
    mut commands: Commands,
) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(50., 50., 50.)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}