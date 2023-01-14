use std::borrow::Cow;

use bytemuck::{Pod, Zeroable};

use crate::renderer::Descriptable;

pub struct Vertices<'a> {
    contents: Cow<'a, [Vertex]>,
}

impl<'a> Vertices<'a> {
    pub fn contents(&'a self) -> &'a [u8] {
        bytemuck::cast_slice(&self.contents)
    }

    pub fn vertices(vertices: &'a [Vertex]) -> Self {
        Self {
            contents: Cow::Borrowed(vertices),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Descriptable for Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2
    ];
}
