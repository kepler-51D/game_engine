use wgpu::Buffer;
use glam::{Quat, Vec2, Vec3};
use crate::advanced_rendering::texture::Texture;

pub struct Mesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
}
pub struct Material {
    pub diffuse_texture: Texture,
    pub normal_texture: Texture,
    pub bind_group: wgpu::BindGroup,
}
pub struct Vertex {
    pub pos: Vec3,
    pub texture_coords: Vec2,
    pub normal: Vec3,
    pub tangent: Vec3,
    pub bitangent: Vec3,
}
pub struct DynamicModel {
    pub mesh: Mesh,
    pub material: Material,
    
}
pub struct Bone {
    pub offset: Vec3,
    pub rotation: Quat,
}