use std::f32::INFINITY;

use glam::{Mat4, Vec2};

use crate::{degrees, renderer::buffer::Buffered};

pub mod camera;

#[derive(Debug, Clone, Copy)]
pub enum Projection {
    Ortographic { origin: Vec2, size: Vec2 },
    Perspective { aspect_ratio: f32, fov: f32 },
    Unknown(Mat4),
}

impl Projection {
    #[must_use]
    pub fn new_perspective(fov: f32) -> Self {
        let (width, height): (f32, f32) = (1.0, 1.0); //todo:
        let ratio = width / height;

        Self::Perspective {
            aspect_ratio: ratio,
            fov,
        }
    }
}

impl Buffered for Projection {
    type Type = Mat4;
    fn pod(self) -> Self::Type {
        self.into()
    }
}

impl Into<Mat4> for Projection {
    fn into(self) -> Mat4 {
        match self {
            Self::Perspective { aspect_ratio, fov } => {
                let fov_y = degrees(fov);
                let near = 0.1;
                let far = INFINITY;

                Mat4::perspective_rh(fov_y, aspect_ratio, near, far)
            }
            _ => unimplemented!("todo!"),
        }
    }
}
