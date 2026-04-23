use glam::Vec3;

use crate::collision::collision_object::{Aabb, point_aabb_colliding};

/// before the free index, bullets are used, after and including, bullets are unused
pub struct BulletManager {
    free: usize,
    bullet_pool: Vec<Bullet>,
}
impl BulletManager {
    pub fn aabb_colliding_with_bullet(&self, aabb: &Aabb) -> Option<usize> {
        (0..self.free).find(|&i| point_aabb_colliding(&self.bullet_pool[i].pos, aabb))
    }
    pub fn update_bullets(&mut self) {
        for bullet in &mut self.bullet_pool {
            bullet.pos += bullet.velocity;
        }
    }
    pub fn new() -> Self {
        Self {
            free: 0,
            bullet_pool: Vec::new(),
        }
    }
    /// returns index of new bullet
    pub fn create_bullet(&mut self, new_bullet: Bullet) -> usize {
        if self.free >= self.bullet_pool.len() {
            self.bullet_pool.push(new_bullet);
            self.free
        }
        else {
            self.bullet_pool[self.free] = new_bullet;
            self.free += 1;
            self.free - 1
        }
    }
    /// gets the amount of bullets that are currently free.
    pub fn get_unused(&self) -> usize {
        self.bullet_pool.len()-self.free
    }
    /// gets a reference to a bullet specified by index
    pub fn get_bullet(&self, index: usize) -> &Bullet {
        &self.bullet_pool[index]
    }
    /// keep in mind this moves the bullet at the end of the vec
    pub fn destroy_bullet(&mut self, index: usize) {
        self.free -= 1;
        self.bullet_pool[index] = self.bullet_pool[self.free];
    }
}
#[derive(Clone, Copy)]
pub struct Bullet {
    pub pos: Vec3,
    pub velocity: Vec3,
}