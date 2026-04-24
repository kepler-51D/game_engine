use glam::{Vec3,Quat};

#[derive(Default)]
#[allow(dead_code)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
}