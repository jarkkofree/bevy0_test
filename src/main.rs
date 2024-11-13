use bevy::prelude::*;
use hexgrid::HexGridPlugin;
use camera::CameraPlugin;
use lighting::LightingPlugin;

pub mod mesh;
pub mod hexgrid;
pub mod camera;
pub mod lighting;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            HexGridPlugin,
            CameraPlugin,
            LightingPlugin
        ))
        .run();
}

