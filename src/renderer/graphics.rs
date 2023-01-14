use std::fmt::Debug;

use once_cell::sync::OnceCell;
use parking_lot::{RwLock, RwLockReadGuard};
use wgpu::{Device, Features, Queue, Surface, SurfaceConfiguration};
use winit::{
    event_loop::EventLoopBuilder,
    platform::windows::EventLoopBuilderExtWindows,
    window::{Window, WindowBuilder},
};

static GRAPHICS_DATA: OnceCell<GraphicsData> = OnceCell::new();

pub fn graphics_data<'a>() -> &'a GraphicsData {
    GRAPHICS_DATA.get().unwrap()
}

pub struct GraphicsData {
    container: RwLock<GraphicContainer>,
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
}

pub struct GraphicContainer {
    pub device: Device,
    pub surface: Surface,
    pub config: SurfaceConfiguration,
    pub queue: Queue,
}

impl GraphicContainer {
    pub async fn new(window: &Window, features: Features) -> Self {
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
            device,
            surface,
            queue,
            config,
        }
    }
}

pub async fn run(window_title: &str) {
    let event_loop = EventLoopBuilder::new().with_any_thread(true).build();
    let window = WindowBuilder::new()
        .with_title(window_title)
        .build(&event_loop)
        .unwrap();

    let features = Features::empty();

    let container = GraphicContainer::new(&window, features).await;
    let data = GraphicsData {
        container: RwLock::new(container),
    };

    GRAPHICS_DATA.set(data).expect("owo what's this");
}
