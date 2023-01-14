use image::{DynamicImage, GenericImageView, ImageError};
use wgpu::{
    BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, Extent3d, ImageCopyTexture,
    ImageDataLayout, Origin3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn from_image_3d(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &image::DynamicImage,
        label: Option<&str>,
        is_normal: bool,
    ) -> anyhow::Result<Self> {
        Self::from_image(device, queue, image, label, is_normal, TextureDimension::D3)
    }

    pub fn from_image_2d(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &image::DynamicImage,
        label: Option<&str>,
        is_normal: bool,
    ) -> anyhow::Result<Self> {
        Self::from_image(device, queue, image, label, is_normal, TextureDimension::D2)
    }

    pub fn from_image_1d(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &image::DynamicImage,
        label: Option<&str>,
        is_normal: bool,
    ) -> anyhow::Result<Self> {
        Self::from_image(device, queue, image, label, is_normal, TextureDimension::D1)
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &image::DynamicImage,
        label: Option<&str>,
        is_normal: bool,
        dimension: TextureDimension,
    ) -> anyhow::Result<Self> {
        let rgba = image.to_rgba8();
        let (width, height) = image.dimensions();

        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            format: if is_normal {
                TextureFormat::Rgba8Unorm
            } else {
                TextureFormat::Rgba8UnormSrgb
            },
        });

        queue.write_texture(
            ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
            },
            &rgba,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * width),
                rows_per_image: std::num::NonZeroU32::new(height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        anyhow::Ok(Self {
            texture,
            view,
            sampler,
        })
    }
}

struct DynamicImageInner(DynamicImage);

impl TryInto<DynamicImageInner> for &[u8] {
    type Error = ImageError;

    fn try_into(self) -> Result<DynamicImageInner, Self::Error> {
        Ok(DynamicImageInner(image::load_from_memory(self)?))
    }
}

impl Into<DynamicImageInner> for DynamicImage {
    fn into(self) -> DynamicImageInner {
        DynamicImageInner(self)
    }
}

impl Texture {
    pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;

    pub fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        label: &str,
    ) -> Self {
        let size = Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };

        let desc = TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
        };

        let texture = device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
        }
    }
}

impl Texture {
    pub fn bind_group_layout(device: &wgpu::Device, offset: u32) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &Self::create_bind_layout_entries(offset),
            label: Some("texture_bind_group_layout"),
        })
    }

    pub const fn create_bind_layout_entries(offset: u32) -> [wgpu::BindGroupLayoutEntry; 2] {
        [
            wgpu::BindGroupLayoutEntry {
                binding: offset,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: offset + 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ]
    }

    pub fn create_bind_entries(&self, binding_start: u32) -> [BindGroupEntry; 2] {
        [
            BindGroupEntry {
                binding: binding_start + 0,
                resource: wgpu::BindingResource::TextureView(&self.view),
            },
            BindGroupEntry {
                binding: binding_start + 1,
                resource: wgpu::BindingResource::Sampler(&self.sampler),
            },
        ]
    }
}
