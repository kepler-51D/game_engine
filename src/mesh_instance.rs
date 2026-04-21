use glam::{Quat, Vec3};
use wgpu::Buffer;

use crate::transform::Transform;

/// transform contains position and rotation of instance.
/// most implementations add scale, but I dont think thats necessary for now.
/// 
/// model_index is the index of the model. to get the model, index the models vec in the State struct
pub struct MeshInstance {
    pub model_index: usize,
    pub instance_index: usize,
}