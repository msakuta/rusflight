use std::error::Error;

use rapier3d::{
    na::{Rotation3, UnitQuaternion, Vector3},
    prelude::*,
};
use three_d::{
    ColorMaterial, Context, CpuMaterial, Cull, Event, FrameInput, Gm, Key, Mesh, PhysicalMaterial,
};
use three_d_asset::{
    Deg, GeometryFunction, InnerSpace, LightingModel, Mat4, NormalDistributionFunction, Quat,
    Srgba, TriMesh, Vec3, Zero,
};

use crate::mqo::load_mqo_scale;

pub(crate) struct Vehicle {
    pub body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
    pub thrust: f32,
    thrust_increase: bool,
    thrust_decrease: bool,
    pub aileron: f32,
    aileron_increase: bool,
    aileron_decrease: bool,
    pub elevator: f32,
    elevator_increase: bool,
    elevator_decrease: bool,
    pub rudder: f32,
    rudder_increase: bool,
    rudder_decrease: bool,
    pub touching_ground: bool,
    wings: Vec<Wing>,
}

impl Vehicle {
    pub fn new((body_handle, collider_handle): (RigidBodyHandle, ColliderHandle)) -> Self {
        use std::f32::consts::PI;
        let wings = vec![
            Wing {
                name: "MainRight".to_string(),
                pos: Vector::new(4., 1., 0.0),
                aero: MAIN_WING_TENSOR,
                control: Control::Aileron,
                sensitivity: -0.05 * PI,
                axis: Vector::new(1., 0., 0.),
                force: Vector::zero(),
            },
            Wing {
                name: "MainLeft".to_string(),
                pos: Vector::new(-4., 1., 0.0),
                aero: MAIN_WING_TENSOR,
                control: Control::Aileron,
                sensitivity: 0.05 * PI,
                axis: Vector::new(1., 0., 0.),
                force: Vector::zero(),
            },
            Wing {
                name: "TailRight".to_string(),
                pos: Vector::new(3., 0.0, 8.),
                aero: TAIL_WING_TENSOR,
                control: Control::Elevator,
                sensitivity: -0.1 * PI,
                axis: Vector::new(1., 0., 0.),
                force: Vector::zero(),
            },
            Wing {
                name: "TailLeft".to_string(),
                pos: Vector::new(-3., 0.0, 8.),
                aero: TAIL_WING_TENSOR,
                control: Control::Elevator,
                sensitivity: -0.1 * PI,
                axis: Vector::new(1., 0., 0.),
                force: Vector::zero(),
            },
            Wing {
                name: "VerticalLeft".to_string(),
                pos: Vector::new(2., 2., 7.),
                aero: RUDDER_TENSOR,
                control: Control::Rudder,
                sensitivity: -0.15 * PI,
                axis: Vector::new(0., 1., 0.),
                force: Vector::zero(),
            },
            Wing {
                name: "VerticalRight".to_string(),
                pos: Vector::new(-2., 2., 7.),
                aero: RUDDER_TENSOR,
                control: Control::Rudder,
                sensitivity: -0.15 * PI,
                axis: Vector::new(0., 1., 0.),
                force: Vector::zero(),
            },
        ];

        Self {
            body_handle,
            collider_handle,
            thrust: 0.,
            thrust_increase: false,
            thrust_decrease: false,
            aileron: 0.,
            aileron_increase: false,
            aileron_decrease: false,
            elevator: 0.,
            elevator_increase: false,
            elevator_decrease: false,
            rudder: 0.,
            rudder_increase: false,
            rudder_decrease: false,
            touching_ground: false,
            wings,
        }
    }

    pub fn update(
        &mut self,
        delta_time: f64,
        rigid_body_set: &mut RigidBodySet,
        frame_input: &FrameInput,
    ) {
        let body = &mut rigid_body_set[self.body_handle];
        for e in &frame_input.events {
            match e {
                Event::KeyPress { kind: Key::Q, .. } => {
                    self.thrust_increase = true;
                }
                Event::KeyRelease { kind: Key::Q, .. } => {
                    self.thrust_increase = false;
                }
                Event::KeyPress { kind: Key::Z, .. } => {
                    self.thrust_decrease = true;
                }
                Event::KeyRelease { kind: Key::Z, .. } => {
                    self.thrust_decrease = false;
                }
                Event::KeyPress { kind: Key::A, .. } => {
                    self.aileron_increase = true;
                }
                Event::KeyRelease { kind: Key::A, .. } => {
                    self.aileron_increase = false;
                }
                Event::KeyPress { kind: Key::D, .. } => {
                    self.aileron_decrease = true;
                }
                Event::KeyRelease { kind: Key::D, .. } => {
                    self.aileron_decrease = false;
                }
                Event::KeyPress { kind: Key::W, .. } => {
                    self.elevator_increase = true;
                }
                Event::KeyRelease { kind: Key::W, .. } => {
                    self.elevator_increase = false;
                }
                Event::KeyPress { kind: Key::S, .. } => {
                    self.elevator_decrease = true;
                }
                Event::KeyRelease { kind: Key::S, .. } => {
                    self.elevator_decrease = false;
                }
                Event::KeyPress { kind: Key::X, .. } => {
                    self.rudder_increase = true;
                }
                Event::KeyRelease { kind: Key::X, .. } => {
                    self.rudder_increase = false;
                }
                Event::KeyPress { kind: Key::C, .. } => {
                    self.rudder_decrease = true;
                }
                Event::KeyRelease { kind: Key::C, .. } => {
                    self.rudder_decrease = false;
                }
                _ => {}
            }
        }
        if delta_time == 0. {
            return; // Handle key events and skip computing physics if paused
        }
        if self.thrust_increase {
            self.thrust = (self.thrust + delta_time as f32).min(1.);
        }
        if self.thrust_decrease {
            self.thrust = (self.thrust - delta_time as f32).max(0.);
        }
        if self.aileron_increase {
            self.aileron = (self.aileron + delta_time as f32).min(1.);
        }
        if self.aileron_decrease {
            self.aileron = (self.aileron - delta_time as f32).max(-1.);
        }
        if self.elevator_increase {
            self.elevator = (self.elevator + delta_time as f32).min(1.);
        }
        if self.elevator_decrease {
            self.elevator = (self.elevator - delta_time as f32).max(-1.);
        }
        if self.rudder_increase {
            self.rudder = (self.rudder + delta_time as f32).min(1.);
        }
        if self.rudder_decrease {
            self.rudder = (self.rudder - delta_time as f32).max(-1.);
        }
        let invrot = body.rotation().inverse();
        for wing in &mut self.wings {
            let control = match wing.control {
                Control::Aileron => self.aileron,
                Control::Elevator => self.elevator,
                Control::Rudder => self.rudder,
                _ => 0.,
            };
            let wing_rot;
            let wing_invrot;
            if control != 0. {
                wing_rot = body.rotation() * Rotation3::new(wing.axis * wing.sensitivity * control);
                wing_invrot = wing_rot.inverse();
            } else {
                wing_rot = *body.rotation();
                wing_invrot = invrot;
            }
            let linvel = wing_invrot.transform_vector(&body.linvel());
            let drag = wing.aero * linvel;
            let global_drag = wing_rot.transform_vector(&drag);
            body.apply_impulse(global_drag, true);
            let relpos = body.rotation().transform_vector(&wing.pos);
            let torque = global_drag.cross(&relpos);
            body.apply_torque_impulse(torque, true);
            wing.force = global_drag;
        }
        if self.touching_ground {
            let torque = Vector3::new(0., 300. * self.thrust * self.rudder, 0.);
            let global_torque = body.rotation().transform_vector(&torque);
            body.apply_torque_impulse(global_torque, true);
        }
        let impulse = Vector3::new(0., 0., -500. * self.thrust);
        let forward_impulse = body.rotation().transform_vector(&impulse);
        body.apply_impulse(forward_impulse, true);
    }

    pub fn transform(&self, rigid_body_set: &RigidBodySet) -> Mat4 {
        let ball_body = &rigid_body_set[self.body_handle];
        let trans_vec = ball_body.translation();
        let trans = Mat4::from_translation(Vec3::new(trans_vec.x, trans_vec.y, trans_vec.z));
        let rot = ball_body.rotation();
        let rv = rot.vector();
        let rot = Mat4::from(Quat::new(rot.w, rv.x, rv.y, rv.z));
        trans * rot
    }

    pub fn pos(&self, rigid_body_set: &RigidBodySet) -> Vec3 {
        let ball_body = &rigid_body_set[self.body_handle];
        let trans_vec = ball_body.translation();
        Vec3::new(trans_vec.x, trans_vec.y, trans_vec.z)
    }

    pub fn reset(&mut self, rigid_body_set: &mut RigidBodySet) {
        let body = &mut rigid_body_set[self.body_handle];
        body.set_position(
            Isometry::new(Vector3::new(0., 20., 0.), Vector3::zero()),
            true,
        );
        body.set_rotation(UnitQuaternion::identity(), true);
    }

    pub fn _contact(&mut self, contact: ContactForceEvent) {
        if 0. < contact.total_force_magnitude {
            self.touching_ground = true;
        } else {
            self.touching_ground = false;
        }
    }

    pub fn collide(&mut self, collision: CollisionEvent) {
        match collision {
            CollisionEvent::Started(h1, h2, _) | CollisionEvent::Stopped(h1, h2, _) => {
                if h1 != self.collider_handle && h2 != self.collider_handle {
                    return;
                }
            }
        }
        if collision.started() {
            self.touching_ground = true;
        } else if collision.stopped() {
            self.touching_ground = false;
        }
    }

    pub fn load_model(
        mut model_src: &[u8],
        context: &Context,
    ) -> Result<Vec<Gm<Mesh, PhysicalMaterial>>, Box<dyn Error>> {
        let models = load_mqo_scale(&mut model_src, None, 1. / 30.0, &|| ())?;
        // let models = vec![uv_sphere(10)];
        let meshes: Vec<_> = models
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
        Ok(meshes)
    }
}

pub(crate) struct ControlMesh {
    pub surface: Gm<Mesh, ColorMaterial>,
    pub arrow: Gm<Mesh, ColorMaterial>,
    pub transform: Mat4,
    pub pos: Vec3,
}

impl Vehicle {
    pub fn control_meshes(&self, context: &Context) -> Vec<ControlMesh> {
        self.wings
            .iter()
            .map(|wing| {
                let surface_mesh = TriMesh::square();
                let mut surface = Gm::new(
                    Mesh::new(&context, &surface_mesh),
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
                let arrow_mesh = TriMesh::arrow(0.8, 0.5, 8);
                let mut arrow = Gm::new(
                    Mesh::new(&context, &arrow_mesh),
                    ColorMaterial::new(
                        &context,
                        &CpuMaterial {
                            roughness: 0.6,
                            metallic: 0.6,
                            lighting_model: LightingModel::Cook(
                                NormalDistributionFunction::TrowbridgeReitzGGX,
                                GeometryFunction::SmithSchlickGGX,
                            ),
                            albedo: Srgba {
                                r: 0,
                                g: 255,
                                b: 255,
                                a: 255,
                            },
                            ..Default::default()
                        },
                    ),
                );
                let pos = wing.pos;
                let pos2 = Vec3::new(pos.x, pos.y, pos.z);
                let rot = match wing.control {
                    Control::Aileron | Control::Elevator => Mat4::from_angle_x(Deg(90.)),
                    Control::Rudder => Mat4::from_angle_y(Deg(90.)),
                    _ => Mat4::from_scale(1.),
                };
                let transform = Mat4::from_translation(pos2) * rot;
                surface.set_transformation(transform);
                ControlMesh {
                    surface,
                    arrow,
                    transform,
                    pos: pos2,
                }
            })
            .collect()
    }

    pub fn wing_forces<'a>(&'a self) -> impl Iterator<Item = Vec3> + 'a {
        self.wings.iter().map(|wing| {
            let v = wing.force;
            Vec3::new(v.x, v.y, v.z)
            // Vec3::new(10., 10., 10.)
        })
    }
}

/// Control surface definition.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Control {
    None,
    Aileron,
    Elevator,
    Rudder,
}

/// An internal structure that representing a wing and its parameters.
struct Wing {
    /// Position of the wing's center, relative to center of mass
    pos: Vector3<f32>,
    /// The aerodynamic tensor, defines how force is applied to the wing.
    aero: Matrix<f32>,
    #[allow(dead_code)]
    /// Name of the wing, just for debugging
    name: String,
    control: Control,
    /// The aerodynamic tensor is rotated around this axis if this control surface is manipulated.
    axis: Vector<f32>,
    /// Sensitivity of this control surface when this surface is manipulated.
    sensitivity: f32,
    /// Cached force from previous frame for visualization
    force: Vector<f32>,
}

const MAIN_WING_TENSOR: Matrix<f32> = Matrix::new(-0.1, 0., 0., 0., -6.5, 0., 0., -0.6, -0.025);
const TAIL_WING_TENSOR: Matrix<f32> = Matrix::new(-0.1, 0., 0., 0., -1.9, 0., 0., -0.0, -0.015);
const RUDDER_TENSOR: Matrix<f32> = Matrix::new(-1.5, 0., 0., 0., -0.05, 0., 0., 0., -0.015);

fn _quatrotquat(this: &Quat, v: &Vec3) -> Quat {
    let q = Quat::from_sv(0., *v);
    let mut qr = q * *this;
    qr += *this;
    qr.normalize()
}
