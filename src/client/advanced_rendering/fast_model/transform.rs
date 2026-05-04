use glam::{Quat, Vec3};
use wgpu::{Queue, util::DeviceExt};

/// uses dual quaternion logic
/// 
/// NOT FOR INSTANCING, ONLY FOR INTERNAL BONE USE
pub struct Transform {
    cpu_side: (Quat,Quat),
    buffer_dirty: bool,
    gpu_side: wgpu::Buffer,
}
impl Transform {
    pub fn write(&mut self,data: (Quat,Quat)) {
        self.cpu_side = data;
        self.buffer_dirty = true;
    }
    pub fn get_buffer(&mut self, queue: &Queue) -> &wgpu::Buffer {
        if self.buffer_dirty {
            queue.write_buffer(&self.gpu_side, 0, bytemuck::cast_slice(&[self.cpu_side.0,self.cpu_side.1]));
            self.buffer_dirty = false;
        }
        &self.gpu_side
    }
    pub fn default(device: &wgpu::Device) -> Self {
        Self {
            cpu_side: (Quat::IDENTITY,Quat::IDENTITY),
            buffer_dirty: false,
            gpu_side: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bone_transform"),
                contents: bytemuck::cast_slice(&[Quat::IDENTITY,Quat::IDENTITY]),
                usage: wgpu::BufferUsages::VERTEX,
            })
        }
    }
    pub fn new(data: (Quat, Quat), device: &wgpu::Device) -> Self {
        Self {
            cpu_side: data,
            buffer_dirty: false,
            gpu_side: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bone_transform"),
                contents: bytemuck::cast_slice(&[data.0,data.1]),
                usage: wgpu::BufferUsages::VERTEX,
            })
        }
    }
}

#[repr(C)]
#[derive(bytemuck::Zeroable,bytemuck::NoUninit,Clone,Copy)]
struct ModelInstanceData {
    rot: Quat,
    pos: Vec3,
    _padding1: u32,
    scale: Vec3,
    _padding2: u32,
}

pub struct ModelInstance {
    pos: Vec3,
    rot: Quat,
    scale: Vec3,
    buffer_dirty: bool,
    buffer: wgpu::Buffer,
}
impl ModelInstance {
    pub fn write_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.buffer_dirty = true;
    }
    pub fn write_rot(&mut self, rot: Quat) {
        self.rot = rot;
        self.buffer_dirty = true;
    }
    pub fn write_scale(&mut self, scale: Vec3) {
        self.scale = scale;
        self.buffer_dirty = true;
    }
    pub fn get_buffer(&mut self, queue: &Queue) -> &wgpu::Buffer {
        if self.buffer_dirty {
            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[ModelInstanceData {
                pos: self.pos,
                rot: self.rot,
                scale: self.scale,
                _padding1: 0,
                _padding2: 0,
            }]));
            self.buffer_dirty = false;
        }
        &self.buffer
    }
    pub fn default(device: &wgpu::Device) -> Self {
        Self {
            pos: Vec3::ZERO,
            rot: Quat::IDENTITY,
            scale: Vec3::ONE,
            buffer_dirty: false,
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bone_transform"),
                contents: bytemuck::cast_slice(&[ModelInstanceData {pos: Vec3::ZERO, rot: Quat::IDENTITY, scale: Vec3::ONE, _padding1: 0, _padding2: 0}]),
                usage: wgpu::BufferUsages::VERTEX,
            })
        }
    }
    pub fn new(pos: Vec3, rot: Quat, scale: Vec3, device: &wgpu::Device) -> Self {
        Self {
            pos,
            rot,
            scale,
            buffer_dirty: false,
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bone_transform"),
                contents: bytemuck::cast_slice(&[ModelInstanceData {pos, rot, scale, _padding1: 0, _padding2: 0}]),
                usage: wgpu::BufferUsages::VERTEX,
            })
        }
    }
}