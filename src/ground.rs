use crate::{
    perlin_noise::{gen_terms, perlin_noise_pixel},
    xor128::Xor128,
};
use three_d::*;
use three_d_asset::{Texture2D, TriMesh};

pub(crate) fn gen_ground(context: &Context) -> Result<Gm<Mesh, PhysicalMaterial>, String> {
    let tex_size = 256;
    let bits = 8;
    let mut rng = Xor128::new(332324);
    let perlin_terms = gen_terms(&mut rng, bits);
    let mut texture_data = vec![[0.; 3]; tex_size * tex_size];
    for yi in 0..tex_size {
        for xi in 0..tex_size {
            let inten = perlin_noise_pixel(xi as f64, yi as f64, bits, &perlin_terms) as f32;
            let pixel = &mut texture_data[xi + yi * tex_size];
            pixel[0] = inten * 0.5 + 0.5;
            pixel[1] = inten * 0.3 + 0.3;
            pixel[2] = inten * 0.1 + 0.1;
        }
    }

    let mut texture = Texture2D::default();
    texture.width = tex_size as u32;
    texture.height = tex_size as u32;
    texture.data = TextureData::RgbF32(texture_data);

    let ground = TriMesh::square();
    let mut ground_obj = Gm::new(
        Mesh::new(&context, &ground),
        PhysicalMaterial::new(
            &context,
            &CpuMaterial {
                roughness: 0.6,
                metallic: 0.,
                lighting_model: LightingModel::Cook(
                    NormalDistributionFunction::TrowbridgeReitzGGX,
                    GeometryFunction::SmithSchlickGGX,
                ),
                albedo_texture: Some(texture),
                ..Default::default()
            },
        ),
    );
    ground_obj.material.render_states.cull = Cull::Back;
    ground_obj.set_transformation(Mat4::from_scale(500.) * Mat4::from_angle_x(Deg(-90.)));

    Ok(ground_obj)
}
