use rapier3d::{
    na::{UnitQuaternion, Vector3},
    prelude::*,
};
use three_d::{Event, FrameInput, Key};
use three_d_asset::{InnerSpace, Mat4, Quat, Vec3, Zero};

pub(crate) struct Vehicle {
    pub body_handle: RigidBodyHandle,
    pub thrust: f32,
    thrust_increase: bool,
    thrust_decrease: bool,
}

impl Vehicle {
    pub fn new(body_handle: RigidBodyHandle) -> Self {
        Self {
            body_handle,
            thrust: 0.,
            thrust_increase: false,
            thrust_decrease: false,
        }
    }

    pub fn update(
        &mut self,
        delta_time: f64,
        rigid_body_set: &mut RigidBodySet,
        frame_input: &FrameInput,
    ) {
        let ball_body = &mut rigid_body_set[self.body_handle];
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
                _ => {}
            }
        }
        if self.thrust_increase {
            self.thrust = dbg!((self.thrust + delta_time as f32).min(1.));
        }
        if self.thrust_decrease {
            self.thrust = (self.thrust - delta_time as f32).max(0.);
        }
        ball_body.apply_impulse(Vector3::new(0., 0., 100. * self.thrust), true);
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
}

fn _quatrotquat(this: &Quat, v: &Vec3) -> Quat {
    let q = Quat::from_sv(0., *v);
    let mut qr = q * *this;
    qr += *this;
    qr.normalize()
}
