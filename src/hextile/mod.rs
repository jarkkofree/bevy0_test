use bevy::prelude::*;
use std::collections::HashMap;
use crate::mesh;
use rand::Rng;

// https://www.redblobgames.com/grids/hexagons/

pub struct HexTilePlugin;

impl Plugin for HexTilePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(HexMap::default())
            .add_systems(Startup, generate_hex_grid);
    }
}

#[derive(Component)]
pub struct HexTile {
    pub id: usize,
    pub q: i32,
    pub r: i32,
    default_color: Handle<StandardMaterial>,
    current_color: Handle<StandardMaterial>
}

#[derive(Resource)]
pub struct HexMap {
    pub coord_to_entity: HashMap<(i32, i32), Entity>,
    pub id_to_coord: HashMap<usize, (i32, i32)>,
    pub radius: i32,
    max_materials: usize,
    pub hex_size: f32
}

impl Default for HexMap {
    fn default() -> Self {
        HexMap {
            coord_to_entity: HashMap::new(),
            id_to_coord: HashMap::new(),
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
    mut hex_map: ResMut<HexMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut id_counter = 0;
    let shape = mesh::Shape::Hex;
    let hex_mesh = mesh::generate_mesh(shape);
    let mesh_handle = meshes.add(hex_mesh);

    let mut rng = rand::thread_rng();
    let mut material_pool: Vec<Handle<StandardMaterial>> = Vec::new();

    for q in -hex_map.radius..=hex_map.radius {
        let r1 = (-hex_map.radius).max(-q - hex_map.radius);
        let r2 = hex_map.radius.min(-q + hex_map.radius);
        for r in r1..=r2 {
            let (x, z) = axial_to_world(q, r, hex_map.hex_size);

            // Check if we have reached the max number of unique materials
            let material_handle = if material_pool.len() < hex_map.max_materials {
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

            let hex_tile = HexTile {
                id: id_counter,
                q,
                r,
                default_color: material_handle.clone(),
                current_color: material_handle.clone(),
            };

            let entity = commands
                .spawn(PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: material_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, 0.0, z),
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(hex_tile)
                .id();

            hex_map.coord_to_entity.insert((q, r), entity);
            hex_map.id_to_coord.insert(id_counter, (q, r));

            id_counter += 1;
        }
    }
}