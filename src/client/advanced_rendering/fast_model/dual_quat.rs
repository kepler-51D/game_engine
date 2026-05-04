use glam::{Quat, Vec3, Vec4};

#[repr(C)]
#[derive(Default, Clone, Copy, bytemuck::NoUninit, bytemuck::Zeroable)]
pub struct DualQuat {
    real: Quat,
    dual: Quat,
}
impl DualQuat {
    fn new(real: Quat, dual: Quat) -> Self {
        Self {
            real,
            dual,
        }
    }
    pub fn from(pos: Vec3, rot: Quat) -> Self {
        Self::new(
            rot,
            (Quat::from_vec4(Vec4::new(pos.x,pos.y,pos.z,0.0)) * rot) * 0.5
        )
    }
    pub fn transform(&self, pos: Vec3) -> Vec3 {
        let rotated_point = self.real * pos;
        rotated_point + (self.dual * 2.0 * self.real.inverse()).xyz()
    }
}