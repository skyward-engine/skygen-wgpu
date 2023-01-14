use bytemuck::Pod;
use glam::Mat4;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device, Queue,
};

#[derive(Debug)]
pub struct BufferData<T: Buffered> {
    pub values: Vec<T>,
    pub buffer: Buffer,
    pub modified: bool,
    pub len: usize,
}

impl<T: Buffered + Copy> BufferData<T> {
    pub fn new(values: Vec<T>, usage: BufferUsages, device: &Device) -> Self {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&values.iter().copied().map(T::pod).collect::<Vec<_>>()),
            usage,
        });

        Self {
            len: values.len(),
            values,
            buffer,
            modified: false,
        }
    }

    pub fn update(&mut self, device: &Device, queue: &Queue) {
        let vec = self.values.iter().copied().map(T::pod).collect::<Vec<_>>();

        if self.len != self.values.len() {
            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&vec))
        } else {
            self.buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vec),
                usage: self.buffer.usage(),
            });
        }

        self.len = self.values.len();
        self.modified = false;
    }

    pub fn get(&mut self, device: &Device, queue: &Queue) -> &Buffer {
        if self.modified {
            self.update(device, queue);
            self.modified = false;
        }
        &self.buffer
    }

    pub fn replace(&mut self, values: Vec<T>) {
        self.modified = true;
        self.values = values;
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.modified = true;
        self.values.remove(index)
    }

    pub fn push(&mut self, value: T) {
        self.values.push(value);
        self.modified = true;
    }
}

pub trait Buffered {
    type Type: Pod;
    fn pod(self) -> Self::Type;
}

impl Buffered for u16 {
    type Type = u16;
    fn pod(self) -> Self::Type {
        self
    }
}

impl Buffered for Mat4 {
    type Type = [[f32; 4]; 4];

    fn pod(self) -> Self::Type {
        self.to_cols_array_2d()
    }
}
