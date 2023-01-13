use std::any::TypeId;

use hashbrown::HashMap;
use wgpu::{BindGroupLayout, Device, PrimitiveTopology, RenderPipeline, SurfaceConfiguration};

use super::{pipeline::PipelineBuilder, ColoredVertex, TexturedVertex};

pub struct RenderContainer {
    pipelines: HashMap<TypeId, RenderPipeline>,
}

impl RenderContainer {
    pub fn insert_pipeline_colored<T: RenderComponent + 'static>(
        &mut self,
        device: &Device,
        surface_config: &SurfaceConfiguration,
    ) {
        let type_id = TypeId::of::<T>();
        let topology = T::topology();

        let (texture, transform) = (
            T::texture_bind_group_layout(),
            T::transform_bind_group_layout(),
        );

        if !self.pipelines.contains_key(&type_id) {
            self.pipelines.insert(
                type_id,
                PipelineBuilder::<ColoredVertex>::new()
                    .with_topology(topology)
                    .with_surface_config(surface_config)
                    .with_texture_group_layout(texture)
                    .with_transform_group_layout(transform)
                    .build(device),
            );
        }
    }

    pub fn insert_pipeline_textured<T: RenderComponent + 'static>(
        &mut self,
        device: &Device,
        surface_config: &SurfaceConfiguration,
    ) {
        let type_id = TypeId::of::<T>();
        let topology = T::topology();

        let (texture, transform) = (
            T::texture_bind_group_layout(),
            T::transform_bind_group_layout(),
        );

        if !self.pipelines.contains_key(&type_id) {
            self.pipelines.insert(
                type_id,
                PipelineBuilder::<TexturedVertex>::new()
                    .with_topology(topology)
                    .with_surface_config(surface_config)
                    .with_texture_group_layout(texture)
                    .with_transform_group_layout(transform)
                    .build(device),
            );
        }
    }
}

pub trait RenderComponent {
    // Returns the primitive topology for this render component.
    fn topology() -> PrimitiveTopology;

    // Returns the bind group layout for the texture, if one exists.
    fn texture_bind_group_layout<'a>() -> Option<&'a BindGroupLayout>;

    // Returns the bind group layout for the transform, if one exists.
    fn transform_bind_group_layout<'a>() -> Option<&'a BindGroupLayout>;
}