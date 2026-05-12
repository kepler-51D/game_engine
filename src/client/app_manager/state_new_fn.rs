use std::{collections::HashSet, f32::consts::PI, sync::Arc};

use glam::{Mat4, Quat, Vec3, Vec4};
use wgpu::{BackendOptions, InstanceFlags, MemoryBudgetThresholds, include_wgsl, util::DeviceExt};
use winit::window::Window;

use crate::{
    advanced_rendering::{
        camera, extendable_buffer::BufferVec, fast_model::{dual_quat::DualQuat, gltf}, instance::{Instance, InstanceRaw}, lighting::LightUniform, model::Model, render_vertex::Vertex, texture::Texture
    }, app_manager::{
        camera::CameraUniform, render_pipeline::create_render_pipeline,
        state::State
    }, player_controller::player::Player
};
use game::collision::bullet_manager::{Bullet, BulletManager};

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<State> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: InstanceFlags::default(),
            memory_budget_thresholds: MemoryBudgetThresholds::default(),
            backend_options: BackendOptions::default(),
            // display: Some(Box::new(dyn WgpuHasDisplayHandle::default())),
            display: None,
            
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });
        let light_uniform = LightUniform {
            pos: Vec3::from_array([2.0, 2.0, 2.0]),
            _padding0: 0,
            col: Vec3::from_array([1.0, 1.0, 1.0]),
            _padding1: 0,
        };

        let light_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Light VB"),
                contents: bytemuck::cast_slice(&[light_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let light_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: None,
            });

        let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &light_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }],
            label: None,
        });
        let camera = camera::Camera::new((0.0, 5.0, 10.0), -90.0*PI/180.0, -20.0*PI/180.0);
        let projection = camera::Projection::new(config.width, config.height, 45.0*PI/180.0, 0.1, 100.0);
        let mut camera_uniform: CameraUniform = CameraUniform {
            pos: Vec4::default(),
            matrix: Mat4::default(),
        };
        camera_uniform.update_view_proj(&camera, &projection);
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let time_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Time Buffer"),
                contents: bytemuck::cast_slice(&[0.0f32]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: time_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        // let light_render_pipeline = {
        //     let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        //         label: Some("Light Pipeline Layout"),
        //         bind_group_layouts: &[Some(&camera_bind_group_layout), Some(&light_bind_group_layout)],
        //         immediate_size: 0,
        //         // push_constant_ranges: &[],

        //     });
        //     let shader = wgpu::ShaderModuleDescriptor {
        //         label: Some("Light Shader"),
        //         source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/light.wgsl").into()),
        //     };
        //     create_render_pipeline(
        //         &device,
        //         &layout,
        //         config.format,
        //         Some(Texture::DEPTH_FORMAT),
        //         &[Vertex::desc()],
        //         shader,
        //     )
        // };

        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("main shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../compiled_shaders/main.wgsl").into()),
        };
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    Some(&camera_bind_group_layout),
                    Some(&texture_bind_group_layout),
                    Some(&light_bind_group_layout),
                ],
                immediate_size: 0,
            });
        let render_pipeline = create_render_pipeline(
            &device,
            Some("render pipeline"),
            &render_pipeline_layout,
            config.format,
            Some(Texture::DEPTH_FORMAT),
            &[Vertex::desc(), InstanceRaw::desc()],
            shader,
        );
        let mut models: Vec<(Model,BufferVec)> = Vec::new();
        models.push((
            Model::load_model_obj("cube.obj", &device, &queue, &texture_bind_group_layout).await.unwrap(),
            BufferVec::new(size_of::<InstanceRaw>(), &device)
        ));
        models.push((
            Model::load_model_obj("flat.obj", &device, &queue, &texture_bind_group_layout).await.unwrap(),
            BufferVec::new(size_of::<InstanceRaw>(), &device)
        ));
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        models[0].1.push(bytemuck::cast_slice(&[InstanceRaw::default()]), &device, &queue, &mut encoder);
        models[0].1.push(bytemuck::cast_slice(&[Instance::default().instance_to_raw()]), &device, &queue, &mut encoder);
        models[1].1.push(bytemuck::cast_slice(&[Instance {rotation: Quat::default(),_padding: 0, position: Vec3::new(0.0,-1.0,0.0)}.instance_to_raw()]), &device, &queue, &mut encoder);
        
        let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");
        surface.configure(&device, &config);
        let _ = window.set_cursor_grab(winit::window::CursorGrabMode::Locked);
        window.set_cursor_visible(false);
        let player = Player::new(&mut models, &device, &queue, &mut encoder, &texture_bind_group_layout).await;
        
        let mut bullet_manager = BulletManager::new();
        bullet_manager.create_bullet(Bullet {pos: Vec3::ZERO, velocity: Vec3::ZERO});


        let model = crate::advanced_rendering::fast_model::gltf::Model::load_model("res/test.glb",&device, &queue,&mut encoder, &texture_bind_group_layout).await;
        
        queue.submit(std::iter::once(encoder.finish()));
        let gltf_render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Gltf Render Pipeline Layout"),
                bind_group_layouts: &[
                    Some(&camera_bind_group_layout),
                    Some(&texture_bind_group_layout),
                    Some(&light_bind_group_layout),
                ],
                immediate_size: 0,
            });

        let gltf_render_pipeline = create_render_pipeline(
            &device,
            Some("gltf render pipeline"),
            &gltf_render_pipeline_layout,
            config.format,
            Some(Texture::DEPTH_FORMAT),
            &[gltf::Vertex::desc(), DualQuat::desc()],
            include_wgsl!("../compiled_shaders/gltf.wgsl"),
        );
        Ok(Self {
            gltf_render_pipeline,
            gltf_test_model: model,
            time_buffer,
            // slow_motion: false,
            // time_scale: 1.0,
            bullet_manager,
            models,
            keys: HashSet::new(),
            player,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            projection,
            mouse_pressed: false,
            // light_render_pipeline,
            light_uniform,
            light_bind_group,
            light_buffer,
            depth_texture,
            render_pipeline,
            surface,
            device,
            queue,
            config,
            is_surface_configured: true,
            window,
        })
    }
}