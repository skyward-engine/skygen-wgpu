use std::any::TypeId;

use hashbrown::HashMap;
use wgpu::{BindGroupLayout, Device, PrimitiveTopology, RenderPipeline, SurfaceConfiguration};

use super::{pipeline::PipelineBuilder, ColoredVertex, Descriptable, TexturedVertex};

pub struct RenderContainer {
    pipelines: HashMap<TypeId, RenderPipeline>,
}

impl RenderContainer {
    /// Inserts a pipeline for a render component that uses a "colored" vertex format.
    ///
    /// # Parameters
    ///
    /// - `device`: A reference to the device that will be used to create the pipeline.
    /// - `surface_config`: A reference to the configuration of the surface that the pipeline will render to.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The type of the render component that the pipeline will be used to render.
    ///
    /// # Example
    ///
    /// ```
    /// let device = ...;
    /// let surface_config = ...;
    /// let mut render_container = RenderContainer::new();
    /// render_container.insert_pipeline_colored::<MyRenderComponent>(&device, &surface_config);
    /// ```
    pub fn insert_pipeline_colored<T: RenderComponent + 'static>(
        &mut self,
        device: &Device,
        surface_config: &SurfaceConfiguration,
    ) {
        self.insert_typed_pipeline::<T, ColoredVertex>(device, surface_config)
    }

    /// Inserts a pipeline for a render component that uses a "textured" vertex format.
    ///
    /// # Parameters
    ///
    /// - `device`: A reference to the device that will be used to create the pipeline.
    /// - `surface_config`: A reference to the configuration of the surface that the pipeline will render to.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The type of the render component that the pipeline will be used to render.
    ///
    /// # Example
    ///
    /// ```
    /// let device = ...;
    /// let surface_config = ...;
    /// let mut render_container = RenderContainer::new();
    /// render_container.insert_pipeline_textured::<MyRenderComponent>(&device, &surface_config);
    /// ```
    pub fn insert_pipeline_textured<T: RenderComponent + 'static>(
        &mut self,
        device: &Device,
        surface_config: &SurfaceConfiguration,
    ) {
        self.insert_typed_pipeline::<T, TexturedVertex>(device, surface_config)
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
                PipelineBuilder::<V>::new()
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
