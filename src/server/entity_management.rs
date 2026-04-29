use game::player_manager::player_base::Player;
use glam::{Vec3,IVec3};
use wgpu::naga::FastHashMap;

pub const CELL_SIZE: Vec3 = Vec3::new(25.0,25.0,25.0);

/// called once per frame per manager
pub trait Manager {
    /// dt is time since last frame
    fn update(&mut self, dt: f32);
}

#[derive(Clone, Copy, Default)]
pub struct Projectile {
    pub pos: Vec3,
    pub velocity: Vec3,
}
#[derive(Default)]
pub struct ProjectileManager {
    pub world_grid: FastHashMap<IVec3,Vec<Projectile>>,
}
impl Manager for ProjectileManager {
    fn update(&mut self, dt: f32) {
        let mut moves: Vec<(IVec3, Projectile)> = Vec::new();

        for (cell_pos, cell_list) in &mut self.world_grid {
            cell_list.retain_mut(|projectile| {
                projectile.pos += projectile.velocity * dt;

                let new_key = {
                    let key = projectile.pos.div_euclid(CELL_SIZE);
                    IVec3::new(key.x as i32, key.y as i32, key.z as i32)
                };
                projectile.pos = projectile.pos.rem_euclid(CELL_SIZE);

                if new_key != *cell_pos {
                    moves.push((new_key, *projectile));
                    false
                } else {
                    true
                }
            });
        }

        for (new_key, projectile) in moves {
            self.world_grid
                .entry(new_key)
                .or_insert_with(Vec::new)
                .push(projectile);
        }
    }
}

pub struct ServerPlayer {
    pub core: Player,
}
#[derive(Default)]
pub struct PlayerManager {
    pub players: Vec<Player>
}
impl Manager for PlayerManager {
    fn update(&mut self, dt: f32) {
        
    }
}
pub struct Enemy {
    pub pos: Vec3,
    pub velocity: Vec3,
    pub health: Vec3,
}