use wgpu::{Buffer, BufferAddress, CommandEncoder, Device, Queue, util::DeviceExt};

const GROWTH_RATE: usize = 2;

/// Similar to [BufferVec], just doesn't use fixed element sizes.
pub struct FlexBuffer {
    buffer: Buffer,
    len: usize,
    maxlen: usize,
}

#[allow(dead_code)]
impl FlexBuffer {
    const START_MAXLEN: usize = 16;
    pub fn get_buffer(&self) -> &Buffer {
        &self.buffer
    }
    pub fn new(device: &Device) -> Self {
        Self {
            len: 0,
            maxlen: Self::START_MAXLEN,
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("buffer_vec"),
                contents: bytemuck::cast_slice(vec![0_u8; Self::START_MAXLEN].as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            }),
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn push(&mut self, data: &[u8], device: &Device, encoder: &mut CommandEncoder, queue: &Queue) {
        if (self.len() + data.len()) > self.maxlen {
            self.reserve(self.maxlen*GROWTH_RATE, device, encoder);
        }
        queue.write_buffer(&self.buffer, self.len() as u64, data);
        self.len = self.len() + data.len();
    }
    pub fn reserve(&mut self, new_size: usize, device: &Device, encoder: &mut CommandEncoder) {
        debug_assert!(new_size > self.len);
        self.maxlen = new_size;
        let new_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("buffer_vec"),
            contents: bytemuck::cast_slice(vec![0_u8; new_size].as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
        });
        encoder.copy_buffer_to_buffer(
            &self.buffer,
            0,
            &new_buffer,
            0,
            Some(BufferAddress::from_le((self.len) as u64))
        );
        self.buffer = new_buffer;
    }
}



pub struct BufferVec {
    pub element_size: usize,
    pub buffer: Buffer,
    len: usize,
    maxlen: usize,
    
}

#[allow(dead_code)]
impl BufferVec {
const START_MAXLEN: usize = 1;
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn delete(&mut self, index: usize, device: &Device, encoder: &mut CommandEncoder) {
        self.len -= 1;
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("buffer_vec"),
            contents: bytemuck::cast_slice(vec![0_u8; self.element_size].as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
        });
        // encoder.copy_buffer_to_buffer(
        //     &self.buffer,
        //     self.len as u64,
        //     &self.buffer,
        //     index as u64,
        //     Some(self.element_size as u64)
        // );

        encoder.copy_buffer_to_buffer(
            &self.buffer,
            self.len as u64,
            &buffer,
            0,
            Some(self.element_size as u64)
        );
        encoder.copy_buffer_to_buffer(
            &buffer,
            0,
            &self.buffer,
            index as u64,
            Some(self.element_size as u64)
        );
    }
    pub fn new(element_size: usize,device: &Device) -> Self {
        debug_assert_eq!(element_size % 4, 0, "element_size must be multiple of 4");
        Self {
            len: 0,
            element_size,
            maxlen: Self::START_MAXLEN,
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("buffer_vec"),
                contents: bytemuck::cast_slice(vec![0_u8; element_size * Self::START_MAXLEN].as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            }),
        }
    }
    pub fn write_elem(&self, index: usize, data: &[u8], queue: &Queue) {
        debug_assert_eq!(data.len(), self.element_size, "tried to write invalid length of data to buffervec");
        queue.write_buffer(&self.buffer, (index*self.element_size) as u64, data);
    }
    pub fn push(&mut self, elem: &[u8], device: &Device, queue: &Queue, encoder: &mut CommandEncoder) {
        debug_assert_eq!(elem.len(),self.element_size, "invalid element length");
        if self.len >= self.maxlen {
            self.reserve(self.maxlen*GROWTH_RATE, device, encoder);
        }
        queue.write_buffer(&self.buffer, (self.len*self.element_size) as u64, elem);
        self.len += 1;
    }
    pub fn pop(&mut self) {
        self.len -= 1;
    }
    pub fn reserve(&mut self, new_size: usize, device: &Device, encoder: &mut CommandEncoder) {
        debug_assert!(new_size > self.len);
        self.maxlen = new_size;
        let new_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("buffer_vec"),
            contents: bytemuck::cast_slice(vec![0_u8; self.element_size*new_size].as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
        });
        encoder.copy_buffer_to_buffer(
            &self.buffer,
            0,
            &new_buffer,
            0,
            Some(BufferAddress::from_le((self.len*self.element_size) as u64))
        );
        self.buffer = new_buffer;
    }
}