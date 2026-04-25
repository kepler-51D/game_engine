use std::net::Ipv4Addr;

use glam::Vec3;
use steamworks::{Client, Server, ServerMode};

pub struct ServerState {
    pub steam_server: (Server,Client),
    pub players: Vec<Player>,
}
impl Default for ServerState {
    fn default() -> Self {
        let ret = Self {
            players: Vec::new(),
            steam_server: match Server::init(
                Ipv4Addr::new(0, 0, 0, 0),
                27015, 27016, ServerMode::Authentication, "1.0.0") {
                    Ok(val) => {
                        val
                    },
                    Err(err) => {
                        println!("{err}");
                        panic!()
                    }
                }
        };
        ret.steam_server.0.set_product("game game");
        ret.steam_server.0.set_game_description("game game game");
        ret.steam_server.0.set_server_name("rawrrrr");
        ret.steam_server.0.set_max_players(8);
        ret.steam_server.0.set_dedicated_server(true);
        ret
    }
}
impl ServerState {
    pub fn connect_player(&mut self) {

    }
}

pub struct Player {
    pub pos: Vec3,
    pub velocity: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}
fn main() {
    let server_state = ServerState::default();
    loop {

    }
}