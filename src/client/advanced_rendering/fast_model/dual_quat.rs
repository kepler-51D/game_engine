use std::ops::{Add, Div, Mul, Sub};

use glam::{Quat, Vec3, Vec4};

#[repr(C)]
#[derive(Default, Clone, Copy, bytemuck::NoUninit, bytemuck::Zeroable)]
pub struct DualQuat {
    real: Quat,
    dual: Quat,
}
impl DualQuat {
    // pub const IDENTITY: Self = Self::from(Vec3::ZERO, Quat::IDENTITY);
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute { // real part
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute { // dual part
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
    fn new(real: Quat, dual: Quat) -> Self {
        Self {
            real,
            dual,
        }
    }
    pub fn from(pos: Vec3, rot: Quat) -> Self {
        Self::new(
            rot,
            // (Quat::from_vec4(Vec4::new(pos.x,pos.y,pos.z,0.0)) * rot) * 0.5
            (rot * Quat::from_vec4(Vec4::new(pos.x, pos.y, pos.z, 0.0))) * 0.5
        )
    }
    pub fn get_rot(self) -> Quat {
        self.real
    }
    pub fn get_pos(self) -> Vec3 {
        (self.dual * self.real.conjugate() * 2.0).xyz()
    }
    pub fn transform(&self, pos: Vec3) -> Vec3 {
        let rotated_point = self.real * pos;
        rotated_point + (self.dual * 2.0 * self.real.conjugate()).xyz()
    }
    pub fn dot(self, rhs: Self) -> f32 {
        self.real.dot(rhs.real)
    }
    pub fn normalise(self) -> Self {
        // let real_len = self.real.length_recip();
        let real_norm = self.real.length();
        Self {
            real: self.real.normalize(),
            // dual: self.dual * real_len - self.real * self.real.dot(self.dual),
            dual: (self.dual - self.real * self.real.dot(self.dual) / real_norm) / real_norm,
        }
    }
    pub fn inverse(self) -> Self {
        Self {
            real: self.real.inverse(),
            dual: self.dual * self.real.inverse() * self.real.inverse()
        }
    }
    pub fn conjugate(self) -> Self {
        Self {
            real: self.real.conjugate(),
            dual: self.dual.conjugate(),
        }
    }
    pub fn rotate(self, rotation: Quat) -> Self {
        Self {
            real: self.real * rotation,
            dual: self.dual
        }
    }
}
impl Mul for DualQuat {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real,
            dual: (self.real * rhs.dual + self.dual * rhs.real)
        }
    }
}
impl Mul<f32> for DualQuat {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            real: self.real * rhs,
            dual: self.dual * rhs,
        }
    }
}
impl Div<f32> for DualQuat {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            real: self.real / rhs,
            dual: self.dual / rhs,
        }
    }
}
impl Add for DualQuat {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            dual: self.dual + rhs.dual,
        }
    }
}
impl Sub for DualQuat {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            dual: self.dual - rhs.dual,
        }
    }
}
