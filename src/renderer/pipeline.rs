use std::marker::PhantomData;

use wgpu::{
    include_wgsl, BindGroupLayout, DepthStencilState, Device, Face, FragmentState, FrontFace,
    MultisampleState, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
    RenderPipeline, SurfaceConfiguration,
};

use super::Descriptable;

pub struct PipelineBuilder<'a, T: Descriptable> {
    surface_config: Option<&'a SurfaceConfiguration>,

    layout_groups: &'a [&'a BindGroupLayout],
    depth_stencil_state: Option<DepthStencilState>,
    topology: Option<PrimitiveTopology>,
    fragment_state: Option<FragmentState<'a>>,
    phantom: PhantomData<T>,
}

impl<'a, T: Descriptable> PipelineBuilder<'a, T> {
    pub fn new() -> Self {
        Self {
            surface_config: None,
            depth_stencil_state: None,
            topology: None,
            fragment_state: None,
            layout_groups: &[],
            phantom: PhantomData,
        }
    }

    pub fn with_surface_config(mut self, surface_config: &'a SurfaceConfiguration) -> Self {
        self.surface_config = Some(surface_config);
        self
    }

    pub fn layouts(mut self, layouts: &'a [&'a BindGroupLayout]) -> Self {
        self.layout_groups = layouts;
        self
    }

    pub fn with_depth_stencil_state(mut self, depth_stencil_state: DepthStencilState) -> Self {
        self.depth_stencil_state = Some(depth_stencil_state);
        self
    }

    pub fn with_topology(mut self, topology: PrimitiveTopology) -> Self {
        self.topology = Some(topology);
        self
    }

    pub fn with_fragment_state(mut self, fragment_state: FragmentState<'a>) -> Self {
        self.fragment_state = Some(fragment_state);
        self
    }

    pub fn build(self, device: &Device) -> RenderPipeline {
        let vs_module = device.create_shader_module(include_wgsl!("./mod.rs"));

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("PipelineBuilder-generated pipeline layout"),
            bind_group_layouts: self.layout_groups,
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "vs_main",
                buffers: &[T::desc()],
            },
            fragment: self.fragment_state,
            primitive: PrimitiveState {
                topology: self.topology.expect("Topology is not set!"),
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("PipelineBuilder-generated render pipeline"),
            layout: Some(&pipeline_layout),
            depth_stencil: self.depth_stencil_state,
            multiview: None,
        })
    }
}
