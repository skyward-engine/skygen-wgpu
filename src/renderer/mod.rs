pub mod container;
pub mod pipeline;

pub trait Descriptable {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

pub struct TexturedVertex {}
pub struct ColoredVertex {}

impl Descriptable for TexturedVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        todo!()
    }
}

impl Descriptable for ColoredVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        todo!()
    }
}
