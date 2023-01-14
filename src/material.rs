use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BufferBindingType, BufferDescriptor,
    BufferUsages, Device, ShaderStages,
};

pub struct Material {
    pub color: [f32; 4],
    pub reflectance: f32,
    pub metalness: f32,
}

impl Material {
    const LAYOUT_DESCRIPTOR: BindGroupLayoutDescriptor<'static> = BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                count: None,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            },
            BindGroupLayoutEntry {
                binding: 1,
                count: None,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            },
            BindGroupLayoutEntry {
                binding: 2,
                count: None,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            },
        ],
    };

    pub fn id(&self) -> i32 {
        0
    }

    pub fn create_bind_group<'a>(&self, device: &Device) -> BindGroup {
        fn create_descriptor<'a>(size: usize) -> BufferDescriptor<'a> {
            BufferDescriptor {
                label: None,
                size: size.try_into().unwrap(),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }
        }

        let float_descriptor = create_descriptor(4);
        let color_descriptor = create_descriptor(4 * std::mem::size_of::<[f32; 4]>());

        let color_buf = device.create_buffer(&color_descriptor);
        let reflectance_buf = device.create_buffer(&float_descriptor);
        let metalness_buf = device.create_buffer(&float_descriptor);

        let layout = &Self::layout_binding(device);

        device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: [&color_buf, &reflectance_buf, &metalness_buf]
                .iter()
                .enumerate()
                .map(|(index, buf)| BindGroupEntry {
                    binding: index.try_into().unwrap(),
                    resource: BindingResource::Buffer(buf.as_entire_buffer_binding()),
                })
                .collect::<Vec<BindGroupEntry>>()
                .as_slice(),
        })
    }

    pub fn layout_binding<'a>(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&Self::LAYOUT_DESCRIPTOR)
    }
}
