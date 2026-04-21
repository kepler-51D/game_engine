use std::{collections::HashSet, f32::consts::PI};

use glam::{Mat4, Quat, Vec2, Vec3};
use wgpu::{Buffer, Device, Queue, util::DeviceExt};
use winit::{event::ElementState, keyboard::KeyCode};

use crate::{advanced_rendering::{instance::{Instance, InstanceRaw}, model::{DrawModel, Model},extendable_buffer::BufferVec}, mesh_instance::MeshInstance, transform::Transform};

pub const CAM_OFFSET: Vec3 = Vec3::new(3.0,3.0,-8.0);

const PLAYER_SPEED: f32 = 10.0;
const MOUSE_SENSITIVITY: f32 = 0.01;
/// add to State struct
pub struct Player {
    pub transform: Transform,
    pub yaw: f32,
    pub pitch: f32,
    pub speed: Vec3,
    pub buffer_in_date: bool,
    pub mesh_instance: MeshInstance,
}
impl Player {
    pub fn calc_matrix(&self) -> Mat4 {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = (-self.yaw).sin_cos();
        
        Mat4::look_to_rh(
            self.transform.position+self.get_right_dir()*CAM_OFFSET.x+self.get_forward_dir()*CAM_OFFSET.z + Vec3::new(0.0,CAM_OFFSET.y,0.0),
            Vec3::new(
                cos_pitch * cos_yaw,
                sin_pitch,
                cos_pitch * sin_yaw,
            ).normalize(),
            Vec3::Y,
        )
    }
    pub fn get_forward_dir(&self) -> Vec3 {
        Vec3::X.rotate_axis(Vec3::Y, self.yaw)
    }
    pub fn get_right_dir(&self) -> Vec3 {
        Vec3::Z.rotate_axis(Vec3::Y, self.yaw)
    }
    pub fn update(&mut self, queue: &Queue, buffer: &BufferVec) {
        self.transform.position += self.speed;

        buffer.write_elem(self.mesh_instance.instance_index, bytemuck::cast_slice(&[Instance {
            _padding: 0,
            position: self.transform.position,
            rotation: Quat::from_axis_angle(Vec3::Y,self.yaw),
        }.instance_to_raw()]), queue);
    }
    pub fn input(&mut self, keys: &HashSet<KeyCode>, delta: f32) -> bool {
        if keys.contains(&KeyCode::KeyW) {
            self.transform.position += self.get_forward_dir()*PLAYER_SPEED*delta;
        }
        if keys.contains(&KeyCode::KeyS) {
            self.transform.position += -self.get_forward_dir()*PLAYER_SPEED*delta;
        }
        if keys.contains(&KeyCode::KeyA) {
            self.transform.position += -self.get_right_dir()*PLAYER_SPEED*delta;
        }
        if keys.contains(&KeyCode::KeyD) {
            self.transform.position += self.get_right_dir()*PLAYER_SPEED*delta;
        }
        true
    }
    pub fn mouse_input(&mut self, delta: Vec2) {
        self.yaw -= delta.x * MOUSE_SENSITIVITY;
        self.pitch += -delta.y * MOUSE_SENSITIVITY;
        self.pitch = self.pitch.clamp(-PI/6.0, 0.0);
    }
}
