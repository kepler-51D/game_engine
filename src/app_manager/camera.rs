use glam::{Mat4, Vec4};
use crate::advanced_rendering::camera;

#[repr(C)]
#[derive(Debug,bytemuck::Pod,Copy,Clone,bytemuck::Zeroable)]
pub struct CameraUniform {
    pub pos: Vec4,
    pub matrix: Mat4,
}
impl CameraUniform {
    pub fn update_view_proj(&mut self, camera: &camera::Camera, projection: &camera::Projection) {
        self.pos = camera.position.to_homogeneous();
        self.matrix = projection.calc_matrix() * camera.calc_matrix();
    }
}