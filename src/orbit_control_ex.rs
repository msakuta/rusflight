//! Customized `OrbitControl` implementation, copied and edited from
//! https://github.com/asny/three-d/blob/master/src/renderer/control/orbit_control.rs

use three_d::{control::CameraControl, *};

///
/// A control that makes the camera orbit around a target.
///
pub struct OrbitControlEx {
    control: CameraControl,
    zoom_speed: f32,
}

pub struct OrbitControlExBuilder {
    target: Vec3,
    min_distance: f32,
    max_distance: f32,
    pan_speed: f32,
    zoom_speed: f32,
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
            control: CameraControl {
                left_drag_horizontal: CameraAction::OrbitLeft {
                    target: self.target,
                    speed: self.pan_speed,
                },
                left_drag_vertical: CameraAction::OrbitUp {
                    target: self.target,
                    speed: self.pan_speed,
                },
                scroll_vertical: CameraAction::Zoom {
                    min: self.min_distance,
                    max: self.max_distance,
                    speed: self.zoom_speed,
                    target: self.target,
                },
                ..Default::default()
            },
            zoom_speed: self.zoom_speed,
        }
    }
}

impl OrbitControlEx {
    pub fn builder() -> OrbitControlExBuilder {
        OrbitControlExBuilder {
            target: Vec3::zero(),
            min_distance: 0.01,
            max_distance: 10.,
            pan_speed: 0.5,
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
        if let CameraAction::Zoom { speed, target, .. } = &mut self.control.scroll_vertical {
            let x = target.distance(*camera.position());

            *speed = self.zoom_speed * x + 0.001;
        }
        self.control.handle_events(camera, events)
    }
}

fn _smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).max(0.0).min(1.0);
    t * t * (3.0 - 2.0 * t)
}
