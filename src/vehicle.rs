use rapier3d::prelude::*;
use three_d_asset::{InnerSpace, Mat4, Quat, Vec3, Zero};

pub(crate) struct Vehicle {
    pub pos: Vec3,
    pub velo: Vec3,
    pub rot: Quat,
    pub avelo: Vec3,
    pub body_handle: RigidBodyHandle,
}

impl Vehicle {
    pub fn new(body_handle: RigidBodyHandle) -> Self {
        Self {
            pos: Vec3::zero(),
            velo: Vec3::zero(),
            rot: Quat::from_sv(1., Vec3::zero()),
            avelo: Vec3::zero(),
            body_handle,
        }
    }

    pub fn update(&mut self, delta_time: f64, rigid_body_set: &RigidBodySet) {
        self.pos += self.velo * delta_time as f32;
        self.rot = quatrotquat(&self.rot, &(self.avelo * delta_time as f32));
    }

    pub fn transform(&self, rigid_body_set: &RigidBodySet) -> Mat4 {
        let ball_body = &rigid_body_set[self.body_handle];
        let trans_vec = ball_body.translation();
        let trans = Mat4::from_translation(Vec3::new(trans_vec.x, trans_vec.y, trans_vec.z));
        let rot = ball_body.rotation();
        let rv = rot.vector();
        let rot = Mat4::from(Quat::new(rot.w, rv.x, rv.y, rv.z));
        trans * rot
        // Mat4::from_translation(self.pos) * Mat4::from(self.rot)
    }
}

fn quatrotquat(this: &Quat, v: &Vec3) -> Quat {
    let q = Quat::from_sv(0., *v);
    let mut qr = q * *this;
    qr += *this;
    qr.normalize()
}
