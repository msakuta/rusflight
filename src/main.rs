mod grid;
mod ground;
mod mqo;
mod orbit_control_ex;
mod perlin_noise;
mod physics;
mod sphere;
mod ui;
mod vehicle;
mod xor128;

use std::{cell::RefCell, error::Error, rc::Rc};

use crate::{orbit_control_ex::OrbitControlEx, physics::PhysicsSet};
use grid::grid_mesh;
use ground::gen_ground;
use mqo::load_mqo_scale;
use three_d::*;
use ui::Ui;
use vehicle::Vehicle;

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    run().await?;
    Ok(())
}

pub async fn run<'src>() -> Result<(), Box<dyn Error>> {
    let window = Window::new(WindowSettings {
        title: "Rusflight".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let ground_width = 1000. * 10.;
    let mut physics = PhysicsSet::new(ground_width);

    let vehicle = Vehicle::new(physics.new_body());
    let vehicle_pos = vehicle.pos(&physics.rigid_body_set);
    let vehicle = Rc::new(RefCell::new(vehicle));
    let vehicle2 = vehicle.clone();
    physics.register_collision(move |e| vehicle2.borrow_mut().collide(e));

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(-30.0, 10.0, 25.) + vehicle_pos,
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10000.0,
    );
    let mut control = OrbitControlEx::builder()
        .target(vehicle_pos)
        .min_distance(0.10)
        .max_distance(1000.0)
        .pan_speed(0.01)
        .zoom_speed(0.01)
        .build();

    let mut ui = Ui::new(&window, &context);

    let resources = [
        "assets/F15.mqo",
        "assets/skybox_evening/front.jpg",
        "assets/skybox_evening/back.jpg",
        "assets/skybox_evening/left.jpg",
        "assets/skybox_evening/right.jpg",
        "assets/skybox_evening/top.jpg",
    ];

    let mut loaded = three_d_asset::io::load_async(&resources).await.unwrap();

    let top_tex = loaded.deserialize("top.jpg").unwrap();
    let right_tex = loaded.deserialize("right.jpg").unwrap();
    let left_tex = loaded.deserialize("left.jpg").unwrap();
    let front_tex = loaded.deserialize("front.jpg").unwrap();
    let back_tex = loaded.deserialize("back.jpg").unwrap();
    let skybox = Skybox::new(
        &context, &right_tex, &left_tex, &top_tex, &top_tex, &front_tex, &back_tex,
    );

    let model_src = loaded.get("F15.mqo")?;
    let mut meshes = Vehicle::load_model(&model_src, &context)?;

    let grid = grid_mesh(50, 50, 10., 0.1);
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

    let ground_obj = gen_ground(&context)?;

    let light = AmbientLight::new(&context, 0.1, Srgba::WHITE);
    let mut dir_light =
        DirectionalLight::new(&context, 1., Srgba::WHITE, &Vec3::new(-1., -0.5, 1.));

    let mut follow = true;

    // main loop
    window.render_loop(move |mut frame_input| {
        physics.step();

        let transform;
        {
            let mut vehicle = vehicle.borrow_mut();
            vehicle.update(
                frame_input.elapsed_time * 1e-3,
                &mut physics.rigid_body_set,
                &frame_input,
            );
            ui.update_thrust(vehicle.thrust);
            ui.update_aileron(vehicle.aileron);
            ui.update_elevator(vehicle.elevator);
            ui.update_rudder(vehicle.rudder);
            ui.update_has_contact(vehicle.touching_ground);
            transform = vehicle.transform(&physics.rigid_body_set);
        }

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
                    vehicle.borrow_mut().reset(&mut physics.rigid_body_set);
                }
            }
        }
        if follow {
            let new_target = vehicle.borrow().pos(&physics.rigid_body_set);
            let target = control.target();
            control.set_target(new_target);
            let cpos = *camera.position();
            let delta = cpos - target;
            let up = *camera.up();
            camera.set_view(new_target + delta, new_target, up);
        }
        control.handle_events(&mut camera, &mut frame_input.events);

        let render_target = frame_input.screen();

        dir_light.generate_shadow_map(256, &meshes);

        render_target
            .clear(ClearState::default())
            .render(&camera, &[&skybox], &[])
            .render(&camera, &meshes, &[&light, &dir_light])
            .render(&camera, &[&grid_obj], &[])
            .render(&camera, &[&ground_obj], &[&light, &dir_light]);

        ui.render(&render_target);

        FrameOutput::default()
    });

    Ok(())
}
