use std::any::Any;

use hashbrown::HashMap;
use legion::{IntoQuery, World};
use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, BufferBindingType, BufferUsages, CommandEncoder, Device, PrimitiveTopology,
    RenderPassDescriptor, RenderPipeline, ShaderStages, SurfaceConfiguration,
};

use crate::{
    material::Material,
    projection::{camera::Camera, Projection},
};

use super::{
    buffer::BufferData,
    graphics,
    model::{Mesh, Transform},
    pipeline::PipelineBuilder,
    Descriptable,
};

pub struct RenderContainer {
    // This is a HashMap, where the key is an identifier of a material, and the value is the
    // pipeline to use for every mesh utilizing the same material.
    pipelines: HashMap<i32, RenderPipeline>,

    // bind groups
    projection_bind_group: BindGroup,
    projection_bind_layout: BindGroupLayout,
}

impl RenderContainer {
    pub fn new(projection: Projection, device: &Device) -> Self {
        let projection_layout = RenderContainer::bind_group_layout(device);

        let camera_buffer = BufferData::new(
            vec![Camera::default()],
            BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            device,
        );

        let projection_buffer = BufferData::new(
            vec![projection],
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            device,
        );

        let projection_bind_group = graphics::graphics_data()
            .container()
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &projection_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: projection_buffer.buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: camera_buffer.buffer.as_entire_binding(),
                    },
                ],
            });

        Self {
            pipelines: HashMap::new(),
            projection_bind_group,
            projection_bind_layout: projection_layout,
        }
    }

    pub fn bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("bind group layout"),
            entries: &[
                BindGroupLayoutEntry {
                    count: None,
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                },
                BindGroupLayoutEntry {
                    count: None,
                    binding: 1,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                },
            ],
        })
    }

    pub fn render_meshes(
        &mut self,
        device: &Device,
        encoder: &mut CommandEncoder,
        world: &mut World,
    ) {
        let descriptor = RenderPassDescriptor {
            label: None,
            color_attachments: &[],
            depth_stencil_attachment: None,
        };

        for (mesh, _, material) in <(&Mesh, &Transform, &Material)>::query().iter(world) {
            let mut pass = encoder.begin_render_pass(&descriptor);

            let vertex_buffer = &mesh.vertex_buffer;
            // let material_group = material.create_bind_group(device);
            // todo: don't leak this resource
            let material_group = Box::leak(Box::new(material.create_bind_group(device)));

            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            pass.set_bind_group(1, &self.projection_bind_group, &[]);
            pass.set_bind_group(3, material_group, &[]);

            pass.draw_indexed(0..1, 1, 0..1);
        }
    }

    /// Inserts a pipeline for a render component with a specified vertex format.
    ///
    /// # Parameters
    ///
    /// - `device`: A reference to the device that will be used to create the pipeline.
    /// - `surface_config`: A reference to the configuration of the surface that the pipeline will render to.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The type of the render component that the pipeline will be used to render.
    /// - `V`: The type of the vertex format that the pipeline will use.
    ///
    /// # Example
    ///
    /// ```
    /// let device = ...;
    /// let surface_config = ...;
    /// let mut render_container = RenderContainer::new();
    /// render_container.insert_typed_pipeline::<MyRenderComponent, MyVertex>(&device, &surface_config);
    /// ```
    pub fn insert_typed_pipeline<T: RenderComponent + 'static, V: Descriptable>(
        &mut self,
        device: &Device,
        material: Material,
        surface_config: &SurfaceConfiguration,
    ) {
        let id = material.id();
        let topology = T::topology();

        if !self.pipelines.contains_key(&id) {
            self.pipelines.insert(
                id,
                PipelineBuilder::<V>::new()
                    .with_topology(topology)
                    .with_surface_config(surface_config)
                    .layouts(&[
                        &self.projection_bind_layout,
                        &Material::layout_binding(device),
                    ])
                    .build(device),
            );
        }
    }
}

pub trait RenderComponent: Any {
    // Returns the primitive topology for this render component.
    fn topology() -> PrimitiveTopology;
}

struct MeshType {}

impl RenderComponent for MeshType {
    fn topology() -> PrimitiveTopology {
        PrimitiveTopology::LineStrip
    }
}
