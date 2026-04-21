use std::{collections::HashSet, f32::consts::PI, sync::Arc};

use glam::{Mat4, Quat, Vec3, Vec4};
use wgpu::{BackendOptions, BindGroupLayout, Buffer, Device, InstanceFlags, MemoryBudgetThresholds, Queue, util::DeviceExt};
use winit::window::Window;

use crate::{advanced_rendering::{
    extendable_buffer::BufferVec,
    camera, instance::{Instance, InstanceRaw},
    lighting::LightUniform, model::Model, render_vertex::Vertex, texture::Texture}, app_manager::{camera::CameraUniform, camera_controller::CameraController, render_pipeline::create_render_pipeline,
        state::{CAMERA_ROTATION_SPEED, State}}, mesh_instance::MeshInstance, player_controller::player::Player, transform::Transform};

impl State {
    pub async fn load_models(models: &mut Vec<Model>, device: &Device, queue: &Queue, texture_bind_group_layout: &BindGroupLayout) -> Result<(),()> {
        models.push(Model::load_model("cube.obj", device, queue, texture_bind_group_layout).await.unwrap());
        models.push(Model::load_model("flat.obj", device, queue, texture_bind_group_layout).await.unwrap());
        Ok(())
    }
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
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        let light_render_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Light Pipeline Layout"),
                bind_group_layouts: &[Some(&camera_bind_group_layout), Some(&light_bind_group_layout)],
                immediate_size: 0,
                // push_constant_ranges: &[],

            });
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Light Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/light.wgsl").into()),
            };
            create_render_pipeline(
                &device,
                &layout,
                config.format,
                Some(Texture::DEPTH_FORMAT),
                &[Vertex::desc()],
                shader,
            )
        };

        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("Normal Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/main.wgsl").into()),
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
                // push_constant_ranges: &[],
            });
        let render_pipeline = create_render_pipeline(
            &device,
            &render_pipeline_layout,
            config.format,
            Some(Texture::DEPTH_FORMAT),
            &[Vertex::desc(), InstanceRaw::desc()],
            shader,
        );
        let instances = (0..Instance::NUM_INSTANCES_PER_ROW).flat_map(|z| {
            (0..Instance::NUM_INSTANCES_PER_ROW).map(move |x| {
                let position = Vec3 { x: (x*4) as f32, y: 0.0, z: (z*4) as f32 } - Instance::INSTANCE_DISPLACEMENT;

                let rotation = if position == Vec3::ZERO {
                    Quat::from_axis_angle(Vec3::Z, 0.0)
                } else {
                    Quat::from_axis_angle(position.normalize(), PI/4.0)
                };

                Instance {
                    position,_padding:0, rotation,
                }
            })
        }).collect::<Vec<_>>();
        let instance_data = instances.iter().map(Instance::instance_to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let mut models: Vec<(Model,BufferVec)> = Vec::new();
        models.push((
            Model::load_model("cube.obj", &device, &queue, &texture_bind_group_layout).await.unwrap(),
            BufferVec::new(size_of::<InstanceRaw>(), &device)
        ));
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        models[0].1.push(bytemuck::cast_slice(&[InstanceRaw::default()]), &device, &queue, &mut encoder);
        models[0].1.push(bytemuck::cast_slice(&[Instance::default().instance_to_raw()]), &device, &queue, &mut encoder);

        queue.submit(std::iter::once(encoder.finish()));

        let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");
        surface.configure(&device, &config);
        let _ = window.set_cursor_grab(winit::window::CursorGrabMode::Locked);
        window.set_cursor_visible(false);

        let mut player: Player = Player {
            buffer_in_date: false,
            speed: Vec3::ZERO,
            yaw: -PI/2.0,
            pitch: 0.0,
            transform: Transform::default(&device),
            mesh_instance: MeshInstance { model_index: 0, instance_index: 0 },
        };
        Ok(Self {
            models,
            keys: HashSet::new(),
            player,
            texture_bind_group_layout,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            projection,
            mouse_pressed: false,
            light_render_pipeline,
            light_uniform,
            light_bind_group,
            light_buffer,
            instance_buffer,
            instances,
            depth_texture,
            cam:camera,
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