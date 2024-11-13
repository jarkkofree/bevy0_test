use bevy::prelude::*;
use hextile::HexTilePlugin;
use camera::CameraPlugin;
use lighting::LightingPlugin;

pub mod mesh;
pub mod hextile;
pub mod camera;
pub mod lighting;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            HexTilePlugin,
            CameraPlugin,
            LightingPlugin
        ))
        .run();
}

