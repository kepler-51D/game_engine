use std::{collections::HashSet, f32::consts::PI};

use glam::{Mat4, Quat, Vec2, Vec3};
use wgpu::{BindGroupLayout, CommandEncoder, Device, Queue};
use winit::{keyboard::KeyCode};

use crate::{
    advanced_rendering::{
        extendable_buffer::BufferVec, instance::{Instance, InstanceRaw}, mesh_instance::MeshInstance, model::Model
    }, collision::collision_object::Aabb, transform::Transform
};

pub const CAM_OFFSET: Vec3 = Vec3::new(3.0,3.0,-8.0);

const PLAYER_SPEED: f32 = 10.0;
const MOUSE_SENSITIVITY: f32 = 0.01;
/// add to State struct
pub struct Player {
    pub transform: Transform,
    pub yaw: f32,
    pub pitch: f32,
    pub speed: Vec3,
    pub mesh_instance: MeshInstance,
    // pub collider_index: usize,
    pub collider: Aabb,
}
impl Player {
    pub async fn new(
        meshes: &mut Vec<(Model,BufferVec,)>,
        // collision_manager: &mut CollisionManager,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        texture_bind_group_layout: &BindGroupLayout
    ) -> Self {
        let mut player = Self {
            collider: Aabb {
                min: Vec3::new(-1.0,-1.0,-1.0),
                max: Vec3::new(-1.0,-1.0,-1.0),
            },
            speed: Vec3::ZERO,
            yaw: -PI/2.0,
            pitch: 0.0,
            transform: Transform::default(),
            mesh_instance: MeshInstance { model_index:0, instance_index: 0},
            // collider_index: collision_manager.colliders.len()
        };
        let mut found = false;
        for (index, mesh) in meshes.iter().enumerate() {
            if mesh.0.name == "cube.obj" {
                player.mesh_instance.model_index = index;
                found = true;
            }
        }
        if !found {
            player.mesh_instance.model_index = meshes.len();
            meshes.push((
                Model::load_model("cube.obj", device, queue, texture_bind_group_layout).await.unwrap(),
                BufferVec::new(size_of::<InstanceRaw>(), device)
            ));
        }
        meshes[player.mesh_instance.model_index].1.push(vec![0u8; size_of::<InstanceRaw>()].as_slice(), device, queue, encoder);
        player
    }
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
        // match &mut collision_manager.colliders[self.collider_index].variant {
        //     CollisionShapeVariant::Sphere { pos, radius } => {
        //         *pos = self.transform.position;
        //     }
        //     _ => {todo!()}
        // }
        
        // if collision_manager.colliders[self.collider_index].is_colliding(&CollisionShape {
        //     variant: CollisionShapeVariant::Sphere { pos: Vec3::ZERO, radius: 5.0 },
        //     collision_mask: 1
        // }) {
        //     println!("hello");
        // }


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
