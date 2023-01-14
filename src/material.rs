use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BufferBindingType, BufferDescriptor,
    BufferUsages, Device, ShaderStages,
};

use crate::texture::Texture;

pub struct Material {
    pub diffuse_color: [f32; 4],
    pub diffuse_texture: Texture,
    pub normal_texture: Texture,
    pub reflectance: f32,
    pub metalness: f32,
}

impl Material {
    const LAYOUT_DESCRIPTOR: BindGroupLayoutDescriptor<'static> = {
        let diffuse_texture = Texture::create_bind_layout_entries(1);
        let normal_texture = Texture::create_bind_layout_entries(3);

        BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                // diffuse color entry
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
                // diffuse texture entries
                diffuse_texture[0],
                diffuse_texture[1],
                // normal texture entries
                normal_texture[0],
                normal_texture[1],
                // reflectance buffer
                BindGroupLayoutEntry {
                    binding: 5,
                    count: None,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                },
                // metalness buffer
                BindGroupLayoutEntry {
                    binding: 6,
                    count: None,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                },
            ],
        }
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
