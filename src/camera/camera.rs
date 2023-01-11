use cgmath::{perspective, Deg, Matrix4, Point3, Vector3};

use super::uniform::CameraUniform;

pub struct Camera {
    pub(crate) eye: Point3<f32>,
    pub(crate) target: Point3<f32>,
    pub(crate) up: Vector3<f32>,
    pub(crate) aspect: f32,
    pub(crate) fovy: f32,
    pub(crate) znear: f32,
    pub(crate) zfar: f32,
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

impl Camera {
    pub fn new(width: &u32, height: &u32) -> Self {
        Self {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vector3::unit_y(),
            aspect: *width as f32 / *height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn build_projection(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let projection = perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * projection * view
    }
}

pub struct CameraStaging {
    pub camera: Camera,
    pub model_rotation: Deg<f32>,
}

impl CameraStaging {
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            model_rotation: Deg(0.0),
        }
    }

    pub fn update_camera(&self, camera_uniform: &mut CameraUniform) {
        camera_uniform.model_view_proj = (OPENGL_TO_WGPU_MATRIX
            * self.camera.build_projection()
            * cgmath::Matrix4::from_angle_z(self.model_rotation))
        .into();
    }
}
