use std::sync::RwLock;

use wgpu::Device;

use crate::model::Mesh;

pub struct Meshes {
    meshes: Vec<Handle<Mesh>>,
    device: &'static Device,
}

impl Meshes {
    pub fn queue(&mut self, mesh: Mesh) {
        
        todo!()
    }   
}

pub struct Handle<T> {
    inner: RwLock<T>,
}

impl<T> Handle<T> {
    pub fn new(initial: T) -> Self {
        Self {
            inner: RwLock::new(initial),
        }
    }
}