use glam::{Vec3,Quat};

use crate::advanced_rendering::instance::Instance;
#[derive(Default)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
}
// impl Transform {
//     pub fn to_bytes(&self) -> &[u8] {
        
//     }
// }