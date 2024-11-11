use bevy::prelude::*;
use hextile::HexTilePlugin;
use camera::CameraPlugin;

pub mod mesh;
pub mod hextile;
pub mod camera;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            HexTilePlugin,
            CameraPlugin
        ))
        .add_systems(Startup, spawn_scene)
        .run();
}

// Step 2: Use the loaded assets in a separate system
fn spawn_scene(
    mut commands: Commands,
) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::IDENTITY
            .looking_at(-Vec3::Y, Vec3::Y),
        ..default()
    });
}