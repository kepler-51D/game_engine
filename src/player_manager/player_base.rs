use std::f32::consts::PI;
use glam::{Vec2, Vec3};
use wgpu::naga::FastHashSet;
use winit::keyboard::KeyCode;
use crate::collision::collision_object::Aabb;

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub pos: Vec3,
    pub velocity: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub collider: Aabb,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            collider: Aabb {
                min: Vec3::new(-1.0,-1.0,-1.0),
                max: Vec3::new( 1.0, 1.0, 1.0),
            },
            pos: Vec3::ZERO,
            velocity: Vec3::ZERO,
            yaw: -PI/2.0,
            pitch: 0.0,
        }
    }
}
impl Player {
    const MAX_SPEED: f32 = 40.0;
    const MOUSE_SENSITIVITY: f32 = 0.01;
    const PLAYER_SPEED: f32 = 4.0;
    pub fn get_forward_dir(&self) -> Vec3 {
        Vec3::X.rotate_axis(Vec3::Y, self.yaw)
    }
    pub fn get_right_dir(&self) -> Vec3 {
        Vec3::Z.rotate_axis(Vec3::Y, self.yaw)
    }

    pub fn update(&mut self, dt: f32) {
        let acceleration = Vec3::ZERO;
        self.velocity += acceleration*Self::PLAYER_SPEED;
        self.pos += self.velocity*dt;

        if self.velocity.length() > Self::MAX_SPEED {
            self.velocity = self.velocity.normalize() * Self::MAX_SPEED;
        } else if self.velocity.length() < 0.0001 {
            self.velocity = Vec3::ZERO;
        }
        self.velocity -= self.velocity * 10.0 * dt;
    }
    pub fn process_wasd_input(&mut self, keys: &FastHashSet<KeyCode>) -> Vec3 {
        let mut ret_vec = Vec3::ZERO;
        if keys.contains(&KeyCode::KeyW) {
            ret_vec += self.get_forward_dir();
        }
        if keys.contains(&KeyCode::KeyS) {
            ret_vec += -self.get_forward_dir();
        }
        if keys.contains(&KeyCode::KeyA) {
            ret_vec += -self.get_right_dir();
        }
        if keys.contains(&KeyCode::KeyD) {
            ret_vec += self.get_right_dir();
        }
        if ret_vec == Vec3::ZERO {return Vec3::ZERO}
        ret_vec.normalize()
    }
    pub fn process_mouse_input(&mut self, delta: Vec2) {
        self.yaw -= delta.x * Self::MOUSE_SENSITIVITY;
        self.pitch += -delta.y * Self::MOUSE_SENSITIVITY;
        self.pitch = self.pitch.clamp(-PI/6.0, 0.0);
    }
}
