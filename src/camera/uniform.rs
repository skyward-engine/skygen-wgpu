use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, SquareMatrix};

use super::camera::Camera;

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
    pub model_view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            model_view_proj: Matrix4::identity().into(),
        }
    }

    pub fn update_view_projection(&mut self, camera: &Camera) {
        self.model_view_proj = camera.build_projection().into();
    }
}
