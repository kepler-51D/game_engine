mod advanced_rendering;
mod app_manager;
mod player_controller;
mod transform;
mod collision;
mod networking;

use winit::event_loop::EventLoop;
use crate::app_manager::app::App;

pub fn run() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
fn main() {
    run().unwrap();
}
