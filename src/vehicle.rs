use three_d_asset::{InnerSpace, Mat4, Quat, Vec3, Zero};

pub(crate) struct Vehicle {
    pub pos: Vec3,
    pub velo: Vec3,
    pub rot: Quat,
    pub avelo: Vec3,
}

impl Vehicle {
    pub fn new() -> Self {
        Self {
            pos: Vec3::zero(),
            velo: Vec3::zero(),
            rot: Quat::from_sv(1., Vec3::zero()),
            avelo: Vec3::zero(),
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.pos += self.velo * delta_time as f32;
        self.rot = quatrotquat(&self.rot, &(self.avelo * delta_time as f32));
    }

    pub fn transform(&self) -> Mat4 {
        Mat4::from_translation(self.pos) * Mat4::from(self.rot)
    }
}

fn quatrotquat(this: &Quat, v: &Vec3) -> Quat {
    let q = Quat::from_sv(0., *v);
    let mut qr = q * *this;
    qr += *this;
    qr.normalize()
}
