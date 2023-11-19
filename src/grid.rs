use three_d_asset::{Indices, Positions, TriMesh, Vec2, Vec3};

/// Returns a grid mesh
pub(crate) fn grid_mesh(
    cell_count_x: u32,
    cell_count_y: u32,
    cell_size: f32,
    line_width: f32,
) -> TriMesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let mut uvs = vec![];
    let cell_count_x = cell_count_x as i32;
    let cell_count_y = cell_count_y as i32;

    let bound_x = cell_count_x as f32 * cell_size;
    let bound_y = cell_count_y as f32 * cell_size;

    for iy in -cell_count_y..=cell_count_y {
        let start_idx = positions.len() as u16;
        let pos = Vec3::new(bound_x, 0., iy as f32 * cell_size);
        let pos_neg = Vec3::new(-pos.x, pos.y, pos.z);
        positions.push(pos + Vec3::new(0., 0., line_width));
        positions.push(pos + Vec3::new(0., 0., -line_width));
        positions.push(pos_neg + Vec3::new(0., 0., -line_width));
        positions.push(pos_neg + Vec3::new(0., 0., line_width));
        uvs.push(Vec2::new(0., 0.));
        uvs.push(Vec2::new(0., 1.));
        uvs.push(Vec2::new(1., 1.));
        uvs.push(Vec2::new(1., 1.));
        uvs.push(Vec2::new(1., 0.));
        uvs.push(Vec2::new(0., 0.));
        indices.push(start_idx);
        indices.push(start_idx + 1);
        indices.push(start_idx + 2);
        indices.push(start_idx + 2);
        indices.push(start_idx + 3);
        indices.push(start_idx);
    }
    for ix in -cell_count_x..=cell_count_x {
        let start_idx = positions.len() as u16;
        let pos = Vec3::new(ix as f32 * cell_size, 0., bound_y);
        let pos_neg = Vec3::new(pos.x, pos.y, -pos.z);
        positions.push(pos + Vec3::new(line_width, 0., 0.));
        positions.push(pos + Vec3::new(-line_width, 0., 0.));
        positions.push(pos_neg + Vec3::new(-line_width, 0., 0.));
        positions.push(pos_neg + Vec3::new(line_width, 0., 0.));
        uvs.push(Vec2::new(0., 0.));
        uvs.push(Vec2::new(0., 1.));
        uvs.push(Vec2::new(1., 1.));
        uvs.push(Vec2::new(1., 1.));
        uvs.push(Vec2::new(1., 0.));
        uvs.push(Vec2::new(0., 0.));
        indices.push(start_idx);
        indices.push(start_idx + 1);
        indices.push(start_idx + 2);
        indices.push(start_idx + 2);
        indices.push(start_idx + 3);
        indices.push(start_idx);
    }

    TriMesh {
        indices: Indices::U16(indices),
        positions: Positions::F32(positions),
        uvs: Some(uvs),
        ..Default::default()
    }
}
