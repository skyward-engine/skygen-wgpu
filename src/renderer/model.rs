use glam::{vec3, EulerRot, Mat4, Quat, Vec3};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, VertexBufferLayout, VertexStepMode,
};

use crate::vertex::{ColoredVertex, Vertices};

use super::{
    buffer::{BufferData, Buffered},
    graphics::graphics_data,
    Descriptable,
};

#[derive(Debug)]
pub struct Mesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: BufferData<u16>,
}

impl Mesh {
    pub fn new(vertices: Vertices, indices: Vec<u16>) -> Self {
        let vertex_buffer =
            graphics_data()
                .container()
                .device
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    usage: BufferUsages::VERTEX,
                    contents: vertices.contents(),
                });

        let index_buffer = BufferData::new(indices, BufferUsages::INDEX);

        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn cube(size: f32, color: [f32; 4]) -> Self {
        Self::rect((size, size, size), color)
    }

    pub fn rect(sizes: (f32, f32, f32), color: [f32; 4]) -> Self {
        let vertices = [
            vec3(-sizes.0, -sizes.1, sizes.2),
            vec3(sizes.0, -sizes.1, sizes.2),
            vec3(-sizes.0, sizes.1, sizes.2),
            vec3(sizes.0, sizes.1, sizes.2),
            vec3(-sizes.0, -sizes.1, -sizes.2),
            vec3(sizes.0, -sizes.1, -sizes.2),
            vec3(-sizes.0, sizes.1, -sizes.2),
            vec3(sizes.0, sizes.1, -sizes.2),
        ];

        #[rustfmt::skip]
        let indices = [
            0, 2, 1, 1, 2, 3,
            1, 3, 5, 5, 3, 7,
            5, 7, 4, 4, 7, 6,
            4, 6, 0, 0, 6, 2,
            4, 0, 5, 5, 0, 1,
            6, 7, 2, 2, 7, 3,
        ];

        let mapped = vertices.map(|position| ColoredVertex {
            position: [position.x, position.y, position.z],
            color,
        });

        Self::new(Vertices::colored(&mapped), Vec::from(indices))
    }
}

pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl Transform {
    const ATTRIBS: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
        0 => Float32x4,
        1 => Float32x4,
        2 => Float32x4,
        3 => Float32x4,
    ];

    pub fn new() -> Transform {
        Transform {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
        }
    }

    pub fn translate(mut self, x: f32, y: f32, z: f32) -> Self {
        self.x += x;
        self.y += y;
        self.z += z;
        self
    }

    pub fn rotate(mut self, yaw: f32, pitch: f32, roll: f32) -> Self {
        self.yaw += yaw;
        self.pitch += pitch;
        self.roll += roll;
        self
    }

    pub fn matrix(&self) -> Mat4 {
        let rot_mat = Mat4::from_euler(EulerRot::XYZ, self.pitch, self.yaw, self.roll);
        let trans_mat = Mat4::from_translation(Vec3::new(self.x, self.y, self.z));

        trans_mat * rot_mat
    }
}

impl Descriptable for Transform {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<[[f32; 4]; 4]>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

impl Buffered for Transform {
    type Type = [[f32; 4]; 4];

    fn pod(self) -> Self::Type {
        self.matrix().to_cols_array_2d()
    }
}
