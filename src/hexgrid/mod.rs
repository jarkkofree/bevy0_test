use bevy::prelude::*;

use std::collections::HashMap;
use rand::Rng;

use crate::mesh;
use crate::hexgrid::coord::*;

mod coord;

// https://www.redblobgames.com/grids/hexagons/

pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(HexGrid::default())
            .add_systems(Startup, generate_hex_grid);
    }
}

#[derive(Resource)]
pub struct HexGrid {
    pub coord_to_entity: HashMap<(i32, i32), Entity>,
    pub radius: i32,
    max_materials: usize,
    pub hex_size: f32
}

impl Default for HexGrid {
    fn default() -> Self {
        HexGrid {
            coord_to_entity: HashMap::new(),
            hex_size: 1.0,
            
            // cells = 1+3r(r+1)
            // radius: 21, cells: 1387
            // radius: 70, cells: 14,911, similar to CK3
            // radius: 500, cells: 751,501, probably shouldn't go above this
            radius: 70,

            // reuse materials after this
            max_materials: 14911,
        }
    }
}

fn axial_to_world(q: i32, r: i32, size: f32) -> (f32, f32) {
    let x = size * (3.0_f32.sqrt() * q as f32 + 3.0_f32.sqrt() / 2.0 * r as f32);
    let z = size * (3.0 / 2.0 * r as f32);
    (x, z)
}

fn generate_hex_grid(
    mut commands: Commands,
    mut hex_grid: ResMut<HexGrid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let shape = mesh::Shape::Hex;
    let hex_mesh = mesh::generate_mesh(shape);
    let mesh_handle = meshes.add(hex_mesh);

    let mut rng = rand::thread_rng();
    let mut material_pool: Vec<Handle<StandardMaterial>> = Vec::new();

    for q in -hex_grid.radius..=hex_grid.radius {
        let r1 = (-hex_grid.radius).max(-q - hex_grid.radius);
        let r2 = hex_grid.radius.min(-q + hex_grid.radius);
        for r in r1..=r2 {
            let (x, z) = axial_to_world(q, r, hex_grid.hex_size);

            // Check if we have reached the max number of unique materials
            let material_handle = if material_pool.len() < hex_grid.max_materials {
                // Generate a new random color
                let red = rng.gen::<f32>();
                let green = rng.gen::<f32>();
                let blue = rng.gen::<f32>();
                
                let base_color = Color::srgb(red, green, blue);
                let material = StandardMaterial {
                    base_color,
                    ..default()
                };
                let handle = materials.add(material);
                material_pool.push(handle.clone());
                handle
            } else {
                // Reuse an existing material from the pool
                let index = rng.gen_range(0..material_pool.len());
                material_pool[index].clone()
            };

            let coord = Coord {
                q,
                r,
            };

            let e = commands
                .spawn(PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: material_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, 0.0, z),
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(coord)
                .id();

            hex_grid.coord_to_entity.insert((q, r), e);
        }
    }
}