use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

use bytemuck::{Pod, Zeroable};
use wgpu::{
    util::DeviceExt, BindGroup, Buffer, BufferDescriptor, BufferUsages, CommandEncoder, Device,
    RenderPass, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor,
    RenderPipeline,
};

use crate::vertex::Vertex;

pub struct Meshes<'a> {
    render_pass: HashMap<TypeId, RenderPass<'static>>,
    encoder: CommandEncoder,
    marker: PhantomData<&'a str>,
}

impl<'a> Meshes<'a> {
    pub fn add<T: Render>(&'static mut self, device: &Device, mesh: &'a mut T) {
        let type_id = TypeId::of::<T>();

        if !self.render_pass.contains_key(&type_id) {
            let pass = {
                self.encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Render Pass"),
                    depth_stencil_attachment: None,
                    color_attachments: &[],
                })
            };

            self.render_pass.insert(type_id, pass);
        }

        let render_pass = self
            .render_pass
            .get_mut(&type_id)
            .expect("Expected render pass to be registered! What went wrong?");

        mesh.draw(device, render_pass);
    }

    pub fn dynamic<T: Render>(&'static mut self, device: &Device, mesh: T) {
        self.add::<T>(device, Box::leak(Box::new(mesh)))
    }
}

pub trait Render: 'static {
    fn draw<'a>(&'a mut self, device: &Device, render_pass: &mut RenderPass<'a>);
}

struct MeshRender {
    vertex_buffer: Buffer,
}

impl Render for MeshRender {
    fn draw<'a>(&'a mut self, device: &Device, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    }
}
