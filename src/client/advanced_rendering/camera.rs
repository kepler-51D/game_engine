use glam::{Mat4,Vec3};
#[derive(Debug)]
pub struct Camera {
    pub position: Vec3,
    pub yaw: f32, // radians
    pub pitch: f32, // radians
}

impl Camera {
    pub fn new<
        V: Into<Vec3>,
        Y: Into<f32>,
        P: Into<f32>,
    >(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    pub fn calc_matrix(&self) -> Mat4 {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        Mat4::look_to_rh(
            self.position,
            Vec3::new(
                cos_pitch * cos_yaw,
                sin_pitch,
                cos_pitch * sin_yaw
            ).normalize(),
            Vec3::Y,
        )
    }
    
}

pub struct Projection {
    pub aspect: f32,
    pub fovy: f32, // radians
    pub znear: f32,
    pub zfar: f32,
}
impl Projection {
    pub fn new<F: Into<f32>>(
        width: u32,
        height: u32,
        fovy: F,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> Mat4 {
        // OPENGL_TO_WGPU_MATRIX * perspective(self.fovy, self.aspect, self.znear, self.zfar)
        Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar)
    }
}