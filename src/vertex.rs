use std::borrow::Cow;

use bytemuck::{Pod, Zeroable};

use crate::renderer::Descriptable;

pub enum Vertices<'a> {
    ColorVertices(Cow<'a, [ColoredVertex]>),
    TextureVertices(Cow<'a, [TexturedVertex]>),
}

impl<'a> Vertices<'a> {
    pub fn contents(&self) -> &'a [u8] {
        todo!()
    }

    pub fn colored(vertices: &'a [ColoredVertex]) -> Self {
        Self::ColorVertices(Cow::Borrowed(vertices))
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ColoredVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct TexturedVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Descriptable for ColoredVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

impl ColoredVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x4
    ];
}

impl Descriptable for TexturedVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

impl TexturedVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2
    ];
}
