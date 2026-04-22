use std::sync::Arc;

use crate::app_manager::state::State;
use glam::{Vec2};
use instant::Instant;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::Window,
};

pub struct App {
    pub state: Option<State>,
    pub last_render_time: Instant,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: None,
            last_render_time: instant::Instant::now(),
        }
    }
}
impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let state = pollster::block_on(State::new(window)).unwrap();
        self.state = Some(state);

    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        self.state = Some(event);
    }
    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        let current_state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };
        
        match event {
            DeviceEvent::MouseMotion { delta } => {
                current_state.player.mouse_input(
                    Vec2::new(delta.0 as f32,delta.1 as f32)
                );
            },
            DeviceEvent::Added => {},
            DeviceEvent::Removed => {},
            // DeviceEvent::MouseWheel { delta } => {},
            // DeviceEvent::Motion { axis, value } => {},
            // DeviceEvent::Button { button, state } => {},
            // DeviceEvent::Key(raw_key_event) => {},
            _ => {}
        }
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let current_state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                current_state.handle_key(event_loop, code, key_state.is_pressed());
                current_state.input(&event);
            }
            WindowEvent::RedrawRequested => {
                let now = instant::Instant::now();
                let dt = now - self.last_render_time;
                self.last_render_time = now;
                current_state.player.input(&current_state.keys,dt.as_secs_f32());
                current_state.update(dt);
                current_state.render_world().unwrap();

                println!("{}", 1.0/dt.as_secs_f32());
            }
            WindowEvent::Resized(val) => {
                current_state.resize(val.width,val.height);
            }
            _ => {}
        }
    }
    // ...
}
