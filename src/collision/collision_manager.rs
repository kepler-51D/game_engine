use crate::collision::collision_object::CollisionShapeVariant;

// const MASK_COUNT: usize = 16;
type Mask = u16;
const MASK_COUNT: usize = std::mem::size_of::<Mask>() * 8;

pub struct CollisionManager {
    pub colliders: Vec<CollisionShape>,
}
pub struct CollisionShape {
    pub variant: CollisionShapeVariant,
    pub collision_mask: Mask,
}
impl CollisionShape {
    pub fn is_colliding(&self, other: &Self) -> bool {
        if (self.collision_mask & other.collision_mask) != 0 {
            self.variant.is_colliding(&other.variant)
        }
        else {
            false
        }
    }
}