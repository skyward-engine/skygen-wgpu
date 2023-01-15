use std::fmt::Debug;

use legion::World;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use wgpu::{Device, Features, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;

use crate::projection::Projection;

use super::container::RenderContainer;

pub struct GraphicsData {
    pub container: RwLock<GraphicContainer>,
}

impl Debug for GraphicsData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("GraphicsData")
    }
}

impl GraphicsData {
    pub fn container(&self) -> RwLockReadGuard<GraphicContainer> {
        self.container.read()
    }

    pub fn container_mut(&self) -> RwLockWriteGuard<GraphicContainer> {
        self.container.write()
    }
}

pub struct GraphicContainer {
    pub device: Device,
    pub surface: Surface,
    pub config: SurfaceConfiguration,
    pub queue: Queue,
    pub render_container: RenderContainer,
}

impl GraphicContainer {
    pub async fn new(window: &Window, features: Features, projection: Projection) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    limits: wgpu::Limits::default(),
                    features,
                },
                None,
            )
            .await
            .unwrap();

        let size = window.inner_size();
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        Self {
            render_container: RenderContainer::new(projection, &device),
            device,
            surface,
            queue,
            config,
        }
    }

    pub(crate) fn render(&mut self, world: &mut World) -> Result<(), wgpu::SurfaceError> {
        let renderer = &self.render_container;
        let output = self.surface.get_current_texture()?;
        let device = &self.device;
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        renderer.render_meshes(device, &mut encoder, world);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
