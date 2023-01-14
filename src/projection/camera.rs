use glam::{Mat4, Vec3};

use crate::renderer::buffer::Buffered;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Vec3,
    pub ty: CameraType,
}

impl Buffered for Camera {
    type Type = [[f32; 4]; 4];

    fn pod(self) -> Self::Type {
        let matrix: Mat4 = self.into();
        matrix.pod()
    }
}

impl Into<Mat4> for Camera {
    fn into(self) -> Mat4 {
        let view = match self.ty {
            CameraType::LookAt(point) => Mat4::look_at_rh(self.position, point, Vec3::Y),
            CameraType::LookTo(point) => Mat4::look_to_rh(self.position, point, Vec3::Y),
        };

        view
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            ty: CameraType::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CameraType {
    LookAt(Vec3),
    LookTo(Vec3),
}

impl Default for CameraType {
    fn default() -> Self {
        Self::LookTo(Vec3::Z)
    }
}
