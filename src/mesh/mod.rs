use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

pub mod polygons;

pub enum Shape {
    Hex
}

pub fn generate_mesh(shape: Shape) -> Mesh {
    let mut positions = vec![];
    let mut uvs = vec![];
    let mut normals = vec![];
    let mut indices = vec![];

    match shape {
        Shape::Hex => polygons::generate_hex(&mut positions, &mut uvs, &mut normals, &mut indices),
    }

    // Create the mesh with the new asset usage
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

