use glam::{vec3, Mat4, Quat, Vec3};
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
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
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
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.translation += translation;
    }

    pub fn rotate(&mut self, rotation: Quat) {
        self.rotation = rotation * self.rotation;
    }

    pub fn scale(&mut self, scale: Vec3) {
        self.scale *= scale;
    }

    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
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

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

impl Buffered for Transform {
    type Type = [[f32; 4]; 4];

    fn pod(self) -> Self::Type {
        self.matrix().to_cols_array_2d()
    }
}
