use glam::{Vec2, Vec3};
use std::mem::size_of;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: Vec3,
    pub texture_coords: Vec2,
    pub normal: Vec3,
    pub tangent: Vec3,
    pub bitangent: Vec3,
}
impl Vertex {
    // pub const fn new() -> Self {
    //     Self {
    //         pos: Vec3::ZERO,
    //         normal: Vec3::ZERO,
    //         texture_coords: Vec2::ZERO,
    //         tangent: Vec3::ZERO,
    //         bitangent: Vec3::ZERO,
    //     }
    // }
    
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute { // position
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute { // texture coords
                    offset: size_of::<[f32;3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute { // normal
                    offset: size_of::<[f32;5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 11]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ]
        }
    }
}