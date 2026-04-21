use glam::{Vec3,Quat};
use wgpu::{Buffer, Device, Queue, util::DeviceExt};

use crate::advanced_rendering::instance::Instance;

#[derive(Default)]
pub enum TransformBuffer {
    Dirty,
    Valid,
    #[default]
    None,
}
// #[derive(Default)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub buffer: TransformBuffer,
    pub buffer_data: Buffer,
}
impl Transform {
    pub fn default(device: &Device) -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::default(),
            buffer: TransformBuffer::Dirty,
            buffer_data: device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("transform buffer"),
                contents: bytemuck::cast_slice(&[Instance {
                    position: Vec3::ZERO,
                    rotation: Quat::default(),
                    _padding: 0,
                }.instance_to_raw()]),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }),
        }
    }
    pub fn update_buffer(&mut self, queue: &Queue, device: &Device) {
        match self.buffer {
            TransformBuffer::Dirty => {
                queue.write_buffer(&self.buffer_data, 0, bytemuck::cast_slice(&[
                    Instance {
                        _padding: 0,
                        position: self.position,
                        rotation: self.rotation,
                    }.instance_to_raw()
                ]));
            },
            TransformBuffer::None => {
                self.buffer_data = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("transform buffer"),
                    contents: bytemuck::cast_slice(&[Instance {
                        position: Vec3::ZERO,
                        rotation: Quat::default(),
                        _padding: 0,
                    }.instance_to_raw()]),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
            },
            TransformBuffer::Valid => {}
        }
    }
}