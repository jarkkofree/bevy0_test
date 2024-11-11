pub fn generate_hex(
    positions: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    normals: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
) {
    // Center vertex at origin
    positions.push([0.0, 0.0, 0.0]);
    uvs.push([0.5, 0.5]);
    normals.push([0.0, 1.0, 0.0]); // Normals pointing up along Y-axis

    let angle_offset = std::f32::consts::PI / 6.0; // 30 degrees offset for flat-topped hexagon

    // Generate the 6 vertices of the hexagon
    for i in 0..6 {
        let angle = std::f32::consts::PI / 3.0 * i as f32 + angle_offset;
        let x = angle.cos();
        let z = angle.sin(); // Use 'z' instead of 'y' for positions
        positions.push([x, 0.0, z]);
        uvs.push([(x + 1.0) / 2.0, (z + 1.0) / 2.0]); // UV mapping
        normals.push([0.0, 1.0, 0.0]); // Normals pointing up along Y-axis
    }

    // Define the indices for the triangles
    for i in 1..7 {
        indices.extend_from_slice(&[
            0,
            if i == 6 { 1 } else { (i + 1) as u32 },
            i as u32,
        ]);
    }
}