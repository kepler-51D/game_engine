use glam::{Quat, Vec3};
pub enum CollisionShapeVariant {
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
    Point {
        pos: Vec3,
    }
}
pub fn sphere_sphere_colliding(pos1: Vec3, pos2: Vec3, radius1: f32, radius2: f32) -> bool {
    (pos1-pos2).length().abs() <= (radius1+radius2)
}
pub fn point_sphere_colliding(pos1: Vec3, pos2: Vec3, radius: f32) -> bool {
    (pos2-pos1).length() <= radius
}
pub fn sphere_aabb_colliding(pos: Vec3, radius: f32, min: Vec3, max: Vec3) -> bool {
        let mut sq_dist: f32 = 0.0;
        if pos.x < min.x {sq_dist += (min.x - pos.x) * (min.x - pos.x);}
        if pos.x > max.x {sq_dist += (pos.x - max.x) * (pos.x - max.x);}
        if pos.y < min.y {sq_dist += (min.y - pos.y) * (min.y - pos.y);}
        if pos.y > max.y {sq_dist += (pos.y - max.y) * (pos.y - max.y);}
        if pos.z < min.z {sq_dist += (min.z - pos.z) * (min.z - pos.z);}
        if pos.z > max.z {sq_dist += (pos.z - max.z) * (pos.z - max.z);}

        sq_dist < (radius*radius)
}
pub fn point_aabb_colliding(pos: Vec3, min: Vec3, max: Vec3) -> bool {
    pos.x >= min.x && pos.x <= max.x &&
    pos.y >= min.y && pos.y <= max.y &&
    pos.z >= min.z && pos.z <= max.z
}
pub fn aabb_aabb_colliding(min1: Vec3, max1: Vec3, min2: Vec3, max2: Vec3) -> bool {
    let corners: [Vec3; 8] = [
        min2,
        Vec3::new(min2.x,min2.y,max2.z),
        Vec3::new(min2.x,max2.y,min2.z),
        Vec3::new(min2.x,max2.y,max2.z),

        Vec3::new(max2.x,min2.y,min2.z),
        Vec3::new(max2.x,min2.y,max2.z),
        Vec3::new(max2.x,max2.y,min2.z),
        max2,
    ];
    for corner in corners {
        if point_aabb_colliding(corner, min1, max1) {
            return true;
        }
    }
    false

}
impl CollisionShapeVariant {
    pub fn is_colliding(&self, other: &Self) -> bool {
        match self {
            Self::Sphere {pos,radius} => {
                let self_pos = pos;
                let self_radius = radius;
                match other {
                    Self::Sphere { pos, radius } => {
                        sphere_sphere_colliding(*self_pos, *pos, *radius, *self_radius)
                    },
                    Self::Aabb { min, max } => {
                        sphere_aabb_colliding(*self_pos, *self_radius, *min, *max)
                    },
                    Self::Point { pos } => {
                        point_sphere_colliding(*pos, *self_pos, *self_radius)
                    }
                    _ => {false},
                }
            },
            Self::Aabb { min, max } => {
                let self_min = min;
                let self_max = max;
                match other {
                    Self::Sphere { pos, radius } => {
                        sphere_aabb_colliding(*pos, *radius, *self_min, *self_max)
                    },
                    Self::Aabb { min, max } => {
                        aabb_aabb_colliding(*self_min, *self_max, *min, *max)
                    },
                    Self::Point { pos } => {
                        point_aabb_colliding(*pos, *self_min, *self_max)
                    }
                    _ => {false},
                }
            }
            Self::Point { pos } => {
                let self_pos = pos;
                match other {
                    Self::Aabb { min, max } => {
                        point_aabb_colliding(*self_pos, *min, *max)
                    }
                    Self::Sphere { pos, radius } => {
                        point_sphere_colliding(*self_pos, *pos, *radius)
                    }
                    Self::Point { pos: _ } => {
                        false // points cant collide with each other
                    }
                    _ => {false}
                }
            }
            _ => {false}
        }
    }
}