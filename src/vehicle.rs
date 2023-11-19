use rapier3d::{
    na::{UnitQuaternion, Vector3},
    prelude::*,
};
use three_d::{Event, FrameInput, Key};
use three_d_asset::{InnerSpace, Mat4, Quat, Vec3, Zero};

pub(crate) struct Vehicle {
    pub body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
    pub thrust: f32,
    thrust_increase: bool,
    thrust_decrease: bool,
    pub rudder: f32,
    rudder_increase: bool,
    rudder_decrease: bool,
    pub touching_ground: bool,
}

impl Vehicle {
    pub fn new((body_handle, collider_handle): (RigidBodyHandle, ColliderHandle)) -> Self {
        Self {
            body_handle,
            collider_handle,
            thrust: 0.,
            thrust_increase: false,
            thrust_decrease: false,
            rudder: 0.,
            rudder_increase: false,
            rudder_decrease: false,
            touching_ground: false,
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
                    self.rudder_increase = true;
                }
                Event::KeyRelease { kind: Key::A, .. } => {
                    self.rudder_increase = false;
                }
                Event::KeyPress { kind: Key::D, .. } => {
                    self.rudder_decrease = true;
                }
                Event::KeyRelease { kind: Key::D, .. } => {
                    self.rudder_decrease = false;
                }
                _ => {}
            }
        }
        if self.thrust_increase {
            self.thrust = (self.thrust + delta_time as f32).min(1.);
        }
        if self.thrust_decrease {
            self.thrust = (self.thrust - delta_time as f32).max(0.);
        }
        if self.rudder_increase {
            self.rudder = (self.rudder + delta_time as f32).min(1.);
        }
        if self.rudder_decrease {
            self.rudder = (self.rudder - delta_time as f32).max(-1.);
        }
        body.apply_impulse(Vector3::new(0., 0., 100. * self.thrust), true);
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
}

fn _quatrotquat(this: &Quat, v: &Vec3) -> Quat {
    let q = Quat::from_sv(0., *v);
    let mut qr = q * *this;
    qr += *this;
    qr.normalize()
}
