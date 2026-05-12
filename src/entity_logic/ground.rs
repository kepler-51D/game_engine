use glam::Vec3;
use crate::collision::collision_object::{Aabb, Sphere, point_aabb_colliding, sphere_aabb_colliding};

pub struct CollisionTreeNode {
    pub collider: Aabb,
    pub children: Vec<usize>,
}
pub struct Ground {
    pub collision_tree: Vec<CollisionTreeNode>,
}
impl Ground {
    pub fn point_colliding(&self, point: &Vec3) -> bool {
        self.point_colliding_recursive(point, 0)
    }
    fn point_colliding_recursive(&self, point: &Vec3, root: usize) -> bool {
        let node = &self.collision_tree[root];
        if !point_aabb_colliding(point, &node.collider) {
            return false;
        }
        if node.children.is_empty() {
            return true;
        }
        for child in &node.children {
            if self.point_colliding_recursive(point, *child) {
                return true;
            }
        }
        false
    }
    pub fn sphere_colliding(&self, sphere: &Sphere) -> bool {
        self.sphere_colliding_recursive(sphere, 0)
    }
    fn sphere_colliding_recursive(&self, sphere: &Sphere, root: usize) -> bool {
        let node = &self.collision_tree[root];
        if !sphere_aabb_colliding(sphere, &node.collider) {
            return false;
        }
        if node.children.is_empty() {
            return true;
        }
        for child in &node.children {
            if self.sphere_colliding_recursive(sphere, *child) {
                return true;
            }
        }
        false
    }
}