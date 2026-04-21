use wgpu::{Buffer, BufferAddress, CommandEncoder, Device, Queue, util::DeviceExt};

const START_MAXLEN: usize = 1;
const GROWTH_RATE: usize = 2;
pub struct BufferVec {
    pub element_size: usize,
    pub buffer: Buffer,
    pub len: usize,
    pub maxlen: usize,
}

impl BufferVec {
    pub fn new(element_size: usize,device: &Device) -> Self {
        println!("{element_size}");
        assert_eq!(element_size % 4, 0, "element_size must be multiple of 4");
        Self {
            len: 0,
            element_size,
            maxlen: START_MAXLEN,
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("buffer_vec"),
                contents: bytemuck::cast_slice(vec![0_u8; element_size * START_MAXLEN].as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            }),
        }
    }
    pub fn write_elem(&self, index: usize, data: &[u8], queue: &Queue) {
        assert_eq!(data.len(), self.element_size, "tried to write invalid length of data to buffervec");
        queue.write_buffer(&self.buffer, (index*self.element_size) as u64, data);
    }
    pub fn push(&mut self, elem: &[u8], device: &Device, queue: &Queue, encoder: &mut CommandEncoder) {
        assert_eq!(elem.len(),self.element_size);
        if (self.len >= self.maxlen) {
            self.reserve(self.maxlen*GROWTH_RATE, device, encoder);
        }
        queue.write_buffer(&self.buffer, (self.len*self.element_size) as u64, elem);
        self.len += 1;
    }
    pub fn reserve(&mut self, new_size: usize, device: &Device, encoder: &mut CommandEncoder) {
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