use once_cell::sync::OnceCell;
use parking_lot::{RwLock, RwLockReadGuard};
use wgpu::{Device, Features, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;

static GRAPHICS_DATA: OnceCell<GraphicsData> = OnceCell::new();

pub fn graphics_data<'a>() -> &'a GraphicsData {
    GRAPHICS_DATA.get().unwrap()
}

pub struct GraphicsData {
    container: RwLock<GraphicContainer>,
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
        todo!()
    }
}
