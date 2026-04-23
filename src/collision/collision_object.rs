use glam::{Quat, Vec3};
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}
pub struct Sphere {
    pub pos: Vec3,
    pub radius: f32,
}
pub struct Obb {
    pub pos: Vec3,
    pub half_size: Vec3,
    pub rotation: Quat,
}

pub fn sphere_sphere_colliding(sphere1: &Sphere, sphere2: &Sphere) -> bool {
    (sphere2.pos-sphere1.pos).length().abs() <= (sphere1.radius+sphere2.radius)
}
pub fn point_sphere_colliding(point: &Vec3, sphere: &Sphere) -> bool {
    (sphere.pos-point).length() <= sphere.radius
}
pub fn sphere_aabb_colliding(sphere: &Sphere, aabb: &Aabb) -> bool {
        let mut sq_dist: f32 = 0.0;
        if sphere.pos.x < aabb.min.x {sq_dist += (aabb.min.x - sphere.pos.x) * (aabb.min.x - sphere.pos.x);}
        if sphere.pos.x > aabb.max.x {sq_dist += (sphere.pos.x - aabb.max.x) * (sphere.pos.x - aabb.max.x);}

        if sphere.pos.y < aabb.min.y {sq_dist += (aabb.min.y - sphere.pos.y) * (aabb.min.y - sphere.pos.y);}
        if sphere.pos.y > aabb.max.y {sq_dist += (sphere.pos.y - aabb.max.y) * (sphere.pos.y - aabb.max.y);}
        
        if sphere.pos.z < aabb.min.z {sq_dist += (aabb.min.z - sphere.pos.z) * (aabb.min.z - sphere.pos.z);}
        if sphere.pos.z > aabb.max.z {sq_dist += (sphere.pos.z - aabb.max.z) * (sphere.pos.z - aabb.max.z);}

        sq_dist < (sphere.radius*sphere.radius)
}
pub fn point_aabb_colliding(point: &Vec3, aabb: &Aabb) -> bool {
    point.x >= aabb.min.x && point.x <= aabb.max.x &&
    point.y >= aabb.min.y && point.y <= aabb.max.y &&
    point.z >= aabb.min.z && point.z <= aabb.max.z
}
pub fn aabb_aabb_colliding(aabb1: &Aabb, aabb2: &Aabb) -> bool {
    let corners: [Vec3; 8] = [
        aabb2.min,
        Vec3::new(aabb2.min.x,aabb2.min.y,aabb2.max.z),
        Vec3::new(aabb2.min.x,aabb2.max.y,aabb2.min.z),
        Vec3::new(aabb2.min.x,aabb2.max.y,aabb2.max.z),

        Vec3::new(aabb2.max.x,aabb2.min.y,aabb2.min.z),
        Vec3::new(aabb2.max.x,aabb2.min.y,aabb2.max.z),
        Vec3::new(aabb2.max.x,aabb2.max.y,aabb2.min.z),
        aabb2.max,
    ];
    for corner in corners {
        if point_aabb_colliding(&corner, aabb1) {
            return true;
        }
    }
    false

}