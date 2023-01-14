use wgpu::{Buffer, BufferUsages};

#[derive(Debug)]
pub struct BufferData<T: Buffered> {
    pub values: Vec<T>,
    pub buffer: Buffer,
    pub modified: bool,
    pub len: usize,
}

impl<T: Buffered> BufferData<T> {
    pub fn new(values: Vec<T>, usage: BufferUsages) -> Self {
        // let buffer
        todo!()
    }

    pub fn add(&mut self, value: T) {
        self.values.push(value);
        self.modified = true;
        self.len += 1;
    }
}

pub trait Buffered {
    type Type;
    fn pod(self) -> Self::Type;
}

impl Buffered for u16 {
    type Type = u16;
    fn pod(self) -> Self::Type {
        self
    }
}
