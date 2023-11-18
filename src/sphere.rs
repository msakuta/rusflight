use three_d_asset::{Indices, Positions, TriMesh, Vec2, Vec3};

///
/// Returns a sphere mesh with radius 1 and center in `(0, 0, 0)` with UV mapping as longitude and latitude.
///
pub(crate) fn uv_sphere(angle_subdivisions: u32) -> TriMesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = vec![];

    positions.push(Vec3::new(0.0, 0.0, 1.0));
    normals.push(Vec3::new(0.0, 0.0, 1.0));
    uvs.push(Vec2::new(0., 0.));

    for j in 0..angle_subdivisions * 2 {
        let j1 = (j + 1) % (angle_subdivisions * 2);
        indices.push(0);
        indices.push((1 + j) as u16);
        indices.push((1 + j1) as u16);
    }

    for i in 0..angle_subdivisions - 1 {
        let i_wrap = (i + 1) as f32 / angle_subdivisions as f32;
        let theta = std::f32::consts::PI * (i + 1) as f32 / angle_subdivisions as f32;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let i0 = 1 + i * angle_subdivisions * 2;
        let i1 = 1 + (i + 1) * angle_subdivisions * 2;

        for j in 0..=angle_subdivisions * 2 {
            let j_wrap = j as f32 / (angle_subdivisions * 2) as f32;
            let phi = std::f32::consts::PI * j as f32 / angle_subdivisions as f32;
            let x = sin_theta * phi.cos();
            let y = sin_theta * phi.sin();
            let z = cos_theta;
            positions.push(Vec3::new(x, y, z));
            normals.push(Vec3::new(x, y, z));
            uvs.push(Vec2::new(j_wrap, i_wrap));

            if i != angle_subdivisions - 2 {
                let j1 = j + 1;
                indices.push((i0 + j) as u16);
                indices.push((i1 + j1) as u16);
                indices.push((i0 + j1) as u16);
                indices.push((i1 + j1) as u16);
                indices.push((i0 + j) as u16);
                indices.push((i1 + j) as u16);
            }
        }
    }
    positions.push(Vec3::new(0.0, 0.0, -1.0));
    normals.push(Vec3::new(0.0, 0.0, -1.0));
    uvs.push(Vec2::new(0., 0.));

    let i = 1 + (angle_subdivisions - 2) * angle_subdivisions * 2;
    for j in 0..angle_subdivisions * 2 {
        let j1 = (j + 1) % (angle_subdivisions * 2);
        indices.push((i + j) as u16);
        indices.push(((angle_subdivisions - 1) * angle_subdivisions * 2 + 1) as u16);
        indices.push((i + j1) as u16);
    }

    three_d_asset::geometry::TriMesh {
        indices: Indices::U16(indices),
        positions: Positions::F32(positions),
        normals: Some(normals),
        uvs: Some(uvs),
        ..Default::default()
    }
}
