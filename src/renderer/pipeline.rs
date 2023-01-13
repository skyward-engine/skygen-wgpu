use std::marker::PhantomData;

use wgpu::{
    include_wgsl, BindGroupLayout, BlendComponent, BlendFactor, BlendOperation, ColorTargetState,
    ColorWrites, DepthStencilState, Device, Face, FrontFace, MultisampleState,
    PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline,
    SurfaceConfiguration,
};

use super::Descriptable;

pub struct PipelineBuilder<'a, T: Descriptable> {
    surface_config: Option<&'a SurfaceConfiguration>,
    texture_group_layout: Option<&'a BindGroupLayout>,
    transform_group_layout: Option<&'a BindGroupLayout>,
    depth_stencil_state: Option<DepthStencilState>,
    topology: Option<PrimitiveTopology>,
    phantom: PhantomData<T>,
}

impl<'a, T: Descriptable> PipelineBuilder<'a, T> {
    pub fn new() -> Self {
        Self {
            surface_config: None,
            texture_group_layout: None,
            transform_group_layout: None,
            depth_stencil_state: None,
            topology: None,
            phantom: PhantomData,
        }
    }

    pub fn with_surface_config(mut self, surface_config: &'a SurfaceConfiguration) -> Self {
        self.surface_config = Some(surface_config);
        self
    }

    pub fn with_texture_group_layout(
        mut self,
        texture_group_layout: Option<&'a BindGroupLayout>,
    ) -> Self {
        self.texture_group_layout = texture_group_layout;
        self
    }

    pub fn with_transform_group_layout(
        mut self,
        transform_group_layout: Option<&'a BindGroupLayout>,
    ) -> Self {
        self.transform_group_layout = transform_group_layout;
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

    pub fn build(self, device: &Device) -> RenderPipeline {
        let vs_module = device.create_shader_module(include_wgsl!("./mod.rs"));
        let fs_module = device.create_shader_module(include_wgsl!("./mod.rs"));

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("PipelineBuilder-generated pipeline layout"),
            bind_group_layouts: &[
                self.texture_group_layout.unwrap(),
                self.transform_group_layout.unwrap(),
            ],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "vs_main",
                buffers: &[T::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: self.surface_config.unwrap().format,
                    write_mask: ColorWrites::ALL,
                    blend: Some(wgpu::BlendState {
                        color: BlendComponent {
                            src_factor: BlendFactor::One,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: BlendComponent {
                            src_factor: BlendFactor::One,
                            dst_factor: BlendFactor::One,
                            operation: BlendOperation::Add,
                        },
                    }),
                })],
            }),
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
