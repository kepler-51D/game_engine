mod entity_management;

use std::{time::Instant};

use crate::entity_management::{Enemy, Manager, PlayerManager, ProjectileManager};

pub struct Server {
    pub name: String,
    pub players: PlayerManager,
    pub projectile_entities: ProjectileManager,
    pub enemy_entities: Vec<Enemy>,

}
impl Manager for Server {
    fn update(&mut self, dt: f32) {
        self.projectile_entities.update(dt);
    }
}
impl Default for Server {
    fn default() -> Self {
        Self {
            name: String::from("test server"),
            enemy_entities: Vec::new(),
            projectile_entities: ProjectileManager::default(),
            players: PlayerManager::default(),
        }
    }
}
fn main() {
    let mut server = Server::default();
    let mut last_frame_time = Instant::now();
    loop {
        let time = Instant::now();
        let dt = (time - last_frame_time).as_secs_f32();
        last_frame_time = time;
        server.update(dt);
    }
}