use glam::{Quat, Vec3};

use crate::collision::colliders::{AabbCollider,SphereCollider};
pub enum CollisionShape {
    Aabb {
        min: Vec3,
        max: Vec3,
    },
    Sphere {
        pos: Vec3,
        radius: f32,
    },
    Obb {
        pos: Vec3,
        half_size: Vec3,
        rotation: Quat,
    },
}