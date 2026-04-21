use glam::Vec3;

pub struct AabbCollider {
    pub min: Vec3,
    pub max: Vec3,
}
pub struct SphereCollider {
    pub pos: Vec3,
    pub radius: f32,
}