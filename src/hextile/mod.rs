use bevy::prelude::*;
use std::collections::HashMap;
use crate::mesh;
use rand::Rng;

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

#[derive(Resource, Default)]
pub struct HexMap {
    pub coord_to_entity: HashMap<(i32, i32), Entity>,
    pub id_to_coord: HashMap<usize, (i32, i32)>,
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
    let radius = 21;
    let mut id_counter = 0;
    let hex_size = 1.0;

    let shape = mesh::Shape::Hex;
    let hex_mesh = mesh::generate_mesh(shape);
    let mesh_handle = meshes.add(hex_mesh);

    let mut rng = rand::thread_rng();

    for q in -radius..=radius {
        let r1 = (-radius).max(-q - radius);
        let r2 = radius.min(-q + radius);
        for r in r1..=r2 {
            let (x, z) = axial_to_world(q, r, hex_size);

            let red = rng.gen::<f32>();
            let green = rng.gen::<f32>();
            let blue = rng.gen::<f32>();
    
            let base_color = Color::srgb(red, green, blue);
            let material = StandardMaterial {
                base_color,
                ..default()
            };
            let material_handle = materials.add(material);

            let hex_tile = HexTile {
                id: id_counter,
                q,
                r,
                default_color: material_handle.clone(),
                current_color: material_handle.clone()
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