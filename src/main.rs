mod grid;
mod mqo;
mod orbit_control_ex;
mod sphere;
mod vehicle;

use std::error::Error;

use crate::orbit_control_ex::OrbitControlEx;
use grid::grid_mesh;
use mqo::{load_mqo, load_mqo_scale};
use sphere::uv_sphere;
use three_d::*;
use vehicle::Vehicle;

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    run().await?;
    Ok(())
}

pub async fn run<'src>() -> Result<(), Box<dyn Error>> {
    let window = Window::new(WindowSettings {
        title: "Rusty-space".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(-3.0, 1.0, 2.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControlEx::builder()
        .target(*camera.target())
        .min_distance(0.10)
        .max_distance(100.0)
        .pan_speed(0.02)
        .zoom_speed(0.01)
        .build();

    let mut resources = [
        "assets/F15.mqo",
        "assets/skybox_evening/front.jpg",
        "assets/skybox_evening/back.jpg",
        "assets/skybox_evening/left.jpg",
        "assets/skybox_evening/right.jpg",
        "assets/skybox_evening/top.jpg",
    ];

    // for texture in &mut resources {
    //     *texture = format!("assets/{}", texture);
    // }

    let mut loaded = three_d_asset::io::load_async(&resources).await.unwrap();

    let top_tex = loaded.deserialize("top.jpg").unwrap();
    let right_tex = loaded.deserialize("right.jpg").unwrap();
    let left_tex = loaded.deserialize("left.jpg").unwrap();
    let front_tex = loaded.deserialize("front.jpg").unwrap();
    let back_tex = loaded.deserialize("back.jpg").unwrap();
    let skybox = Skybox::new(
        &context, &right_tex, &left_tex, &top_tex, &top_tex, &front_tex, &back_tex,
    );

    // let ident = Matrix4::identity();

    let mut model_src = loaded.get("F15.mqo")?;
    let models = load_mqo_scale(&mut model_src, None, 0.01, &|| ())?;
    // let models = vec![uv_sphere(10)];
    let mut meshes: Vec<_> = models
        .iter()
        .take(1)
        .map(|model| {
            let mut obj = Gm::new(
                Mesh::new(&context, model),
                PhysicalMaterial::new(
                    &context,
                    &CpuMaterial {
                        roughness: 0.6,
                        metallic: 0.6,
                        lighting_model: LightingModel::Cook(
                            NormalDistributionFunction::TrowbridgeReitzGGX,
                            GeometryFunction::SmithSchlickGGX,
                        ),
                        ..Default::default()
                    },
                ),
            );
            obj.material.render_states.cull = Cull::Back;
            obj
        })
        .collect();

    let grid = grid_mesh(10, 10, 10., 0.1);
    let grid_obj = Gm::new(
        Mesh::new(&context, &grid),
        ColorMaterial::new(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 0,
                    g: 255,
                    b: 0,
                    a: 200,
                },
                ..Default::default()
            },
        ),
    );

    let light = AmbientLight::new(&context, 0.1, Color::WHITE);
    let point = PointLight::new(
        &context,
        10.,
        Color::WHITE,
        &Vec3::new(10., 0., -10.),
        Attenuation {
            constant: 0.,
            linear: 0.,
            quadratic: 0.,
        },
    );

    let mut vehicle = Vehicle::new();
    vehicle.velo.z = 1.;
    vehicle.avelo.y = 0.1;

    // main loop
    window.render_loop(move |mut frame_input| {
        let viewport = Viewport {
            x: 0,
            y: 0,
            width: frame_input.viewport.width,
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        vehicle.update(frame_input.elapsed_time * 1e-3);
        let transform = vehicle.transform();

        for mesh in &mut meshes {
            mesh.set_transformation(transform);
        }

        frame_input
            .screen()
            .clear(ClearState::default())
            .render(&camera, &[&skybox], &[])
            .render(&camera, &meshes, &[&light, &point])
            .render(&camera, &[&grid_obj], &[]);

        FrameOutput::default()
    });

    Ok(())
}
