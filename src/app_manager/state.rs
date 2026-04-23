use std::{collections::HashSet, sync::Arc};
use glam::{Quat, Vec3};
use wgpu::{CurrentSurfaceTexture};
use crate::{advanced_rendering::{extendable_buffer::BufferVec, lighting::LightUniform, model::{DrawModel, Model}}, collision::bullet_manager::BulletManager};
use crate::app_manager::{camera::CameraUniform};
use crate::player_controller::player::{CAM_OFFSET, Player};
use winit::{
    event::{ElementState, KeyEvent, MouseButton, WindowEvent}, event_loop::ActiveEventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window
};
use crate::advanced_rendering::{camera, texture::Texture};

#[derive(PartialEq)]
pub enum Pipeline {
    Default,
    None,
}

/// render and game state
pub struct State {
    pub current_pipeline: Pipeline,
    pub bullet_manager: BulletManager,
    pub models: Vec<(Model, BufferVec)>,
    pub keys: HashSet<KeyCode>,
    pub player: Player,

    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub render_pipeline: wgpu::RenderPipeline,
    pub is_surface_configured: bool,
    pub window: Arc<Window>,

    pub mouse_pressed: bool,
    pub projection: camera::Projection,
    pub camera_bind_group: wgpu::BindGroup,
    pub camera_buffer: wgpu::Buffer,
    pub camera_uniform: CameraUniform,
    pub depth_texture: Texture,

    // pub texture_bind_group_layout: BindGroupLayout,
    pub light_uniform: LightUniform,
    pub light_buffer: wgpu::Buffer,
    pub light_bind_group: wgpu::BindGroup,
    // pub light_render_pipeline: RenderPipeline,
}
impl State {
    pub fn update(&mut self, _dt: instant::Duration) {
        if self.bullet_manager.aabb_colliding_with_bullet(&self.player.collider).is_some() {
            println!("hit by bullet");
        }
        let old_position: Vec3 = self.light_uniform.pos;
        self.light_uniform.pos =
            Quat::from_axis_angle((0.0, 1.0, 0.0).into(), 0.001)
                * old_position
                ;
        self.player.update(&self.queue,&self.models[self.player.mesh_instance.model_index].1);
        self.queue.write_buffer(&self.light_buffer, 0, bytemuck::cast_slice(&[self.light_uniform]));
        self.camera_uniform = CameraUniform {
            pos: (self.player.transform.position + CAM_OFFSET).to_homogeneous(),
            matrix: self.projection.calc_matrix() * self.player.calc_matrix(),
        };
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));

    }
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state,
                        repeat,
                        ..
                    },
                    ..
            } => {
                if !*repeat {
                    match state {
                        ElementState::Pressed => {
                            self.keys.insert(*key);
                        },
                        ElementState::Released => {
                            self.keys.remove(key);
                        }
                    }
                }
                true
            },
            WindowEvent::MouseWheel { delta: _, .. } => {
                true
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
            self.depth_texture = Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
            self.projection.resize(width, height);
        }
    }
    pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            (KeyCode::Backspace, true) => {}
            _ => {}
        }
    }
    pub fn render_world(&mut self) -> Result<(), wgpu::Error> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }
            
        let output = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(val) => {
                val
            },
            CurrentSurfaceTexture::Lost => {
                todo!()
            },
            CurrentSurfaceTexture::Occluded => {
                todo!()
            },
            CurrentSurfaceTexture::Outdated => {
                todo!()
            },
            CurrentSurfaceTexture::Suboptimal(val) => {
                val
            },
            CurrentSurfaceTexture::Timeout => {
                todo!()
            },
            CurrentSurfaceTexture::Validation => {
                todo!()
            }
        };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut render_pass: wgpu::RenderPass<'_> = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.05,
                                    g: 0.05,
                                    b: 0.025,
                                    a: 0.0,
                                }
                            ),
                            // load: wgpu::LoadOp::DontCare(unsafe {LoadOpDontCare::enabled()}),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            
            
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            // render_pass.set_pipeline(&self.light_render_pipeline);    
            // for (model, trans_buffers) in &self.models {
            //     render_pass.set_vertex_buffer(1, trans_buffers.buffer.slice(..));
            //     render_pass.draw_light_model_instanced(
            //         model,
            //         0..(trans_buffers.len as u32),
            //         &self.camera_bind_group,
            //         &self.light_bind_group
            //     );
            // }
            if self.current_pipeline != Pipeline::Default {
                render_pass.set_pipeline(&self.render_pipeline);
            }
            for (model, trans_buffers) in &self.models {
                render_pass.set_vertex_buffer(1, trans_buffers.buffer.slice(..));
                render_pass.draw_model_instanced(
                    model,
                    0..(trans_buffers.len as u32),
                    &self.camera_bind_group,
                    &self.light_bind_group
                );
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}