mod grid;
mod mqo;
mod orbit_control_ex;
mod physics;
mod sphere;
mod vehicle;

use std::error::Error;

use crate::{orbit_control_ex::OrbitControlEx, physics::PhysicsSet};
use grid::grid_mesh;
use mqo::load_mqo_scale;
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

    let mut physics = PhysicsSet::new();

    let mut vehicle = Vehicle::new(physics.new_body());
    let vehicle_pos = vehicle.pos(&physics.rigid_body_set);

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(-30.0, 10.0, 25.) + vehicle_pos,
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControlEx::builder()
        .target(vehicle_pos)
        .min_distance(0.10)
        .max_distance(100.0)
        .pan_speed(0.01)
        .zoom_speed(0.01)
        .build();

    let resources = [
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
    let models = load_mqo_scale(&mut model_src, None, 1. / 30.0, &|| ())?;
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

    let light = AmbientLight::new(&context, 0.1, Srgba::WHITE);
    let point = PointLight::new(
        &context,
        10.,
        Srgba::WHITE,
        &Vec3::new(10., 0., -10.),
        Attenuation {
            constant: 0.,
            linear: 0.,
            quadratic: 0.,
        },
    );

    let mut follow = true;

    // main loop
    window.render_loop(move |mut frame_input| {
        physics.step();

        vehicle.update(frame_input.elapsed_time * 1e-3, &mut physics.rigid_body_set);
        let transform = vehicle.transform(&physics.rigid_body_set);

        for mesh in &mut meshes {
            mesh.set_transformation(transform);
        }

        let viewport = Viewport {
            x: 0,
            y: 0,
            width: frame_input.viewport.width,
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);
        for e in &frame_input.events {
            if let Event::KeyPress { kind, .. } = e {
                if *kind == Key::F {
                    follow = !follow;
                } else if *kind == Key::R {
                    vehicle.reset(&mut physics.rigid_body_set);
                }
            }
        }
        if follow {
            let new_target = vehicle.pos(&physics.rigid_body_set);
            let target = control.target();
            control.set_target(new_target);
            let cpos = *camera.position();
            let delta = cpos - target;
            let up = *camera.up();
            camera.set_view(new_target + delta, new_target, up);
        }
        control.handle_events(&mut camera, &mut frame_input.events);

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
