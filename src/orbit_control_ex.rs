//! Customized `OrbitControl` implementation, copied and edited from
//! https://github.com/asny/three-d/blob/master/src/renderer/control/orbit_control.rs

use three_d::*;

///
/// A control that makes the camera orbit around a target.
///
pub struct OrbitControlEx {
    zoom_speed: f32,
    target: Vec3,
    pan_speed: f32,
    min_distance: f32,
    max_distance: f32,
}

pub struct OrbitControlExBuilder {
    zoom_speed: f32,
    target: Vec3,
    min_distance: f32,
    max_distance: f32,
    pan_speed: f32,
}

impl OrbitControlExBuilder {
    pub fn target(&mut self, val: Vec3) -> &mut Self {
        self.target = val;
        self
    }

    pub fn min_distance(&mut self, val: f32) -> &mut Self {
        self.min_distance = val;
        self
    }

    pub fn max_distance(&mut self, val: f32) -> &mut Self {
        self.max_distance = val;
        self
    }

    pub fn pan_speed(&mut self, val: f32) -> &mut Self {
        self.pan_speed = val;
        self
    }

    pub fn zoom_speed(&mut self, val: f32) -> &mut Self {
        self.zoom_speed = val;
        self
    }

    pub fn build(&mut self) -> OrbitControlEx {
        OrbitControlEx {
            zoom_speed: self.zoom_speed,
            target: self.target,
            pan_speed: self.pan_speed,
            min_distance: self.min_distance,
            max_distance: self.max_distance,
        }
    }
}

impl OrbitControlEx {
    pub fn builder() -> OrbitControlExBuilder {
        OrbitControlExBuilder {
            target: Vec3::zero(),
            min_distance: 0.01,
            max_distance: 10.,
            pan_speed: 0.01,
            zoom_speed: 0.01,
        }
    }

    /// Creates a new orbit control with the given target and minimum and maximum distance to the target.
    pub fn _build(target: Vec3, min_distance: f32, max_distance: f32) -> Self {
        Self::builder()
            .target(target)
            .min_distance(min_distance)
            .max_distance(max_distance)
            .build()
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> bool {
        let mut change = false;
        for event in events.iter_mut() {
            match event {
                Event::MouseMotion {
                    delta,
                    button: Some(MouseButton::Left),
                    handled,
                    ..
                } => {
                    let speed = self.pan_speed * self.target.distance(*camera.position()) + 0.001;
                    camera.rotate_around_with_fixed_up(
                        &self.target,
                        speed * delta.0,
                        speed * delta.1,
                    );
                    *handled = true;
                    change = true;
                }
                Event::MouseWheel { delta, handled, .. } => {
                    let speed = self.zoom_speed * self.target.distance(*camera.position()) + 0.001;
                    camera.zoom_towards(
                        &self.target,
                        speed * delta.1,
                        self.min_distance,
                        self.max_distance,
                    );
                    *handled = true;
                    change = true;
                }
                _ => {}
            }
        }
        change
    }

    pub fn target(&self) -> Vec3 {
        self.target
    }

    pub fn set_target(&mut self, new_target: Vec3) {
        self.target = new_target;
    }
}

fn _smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).max(0.0).min(1.0);
    t * t * (3.0 - 2.0 * t)
}
