pub mod buffer;
pub mod container;
pub mod graphics;
pub mod model;
pub mod pipeline;

pub trait Descriptable {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}
