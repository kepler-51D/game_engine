use std::ops::Range;

use wgpu::{Buffer, Device, Queue, util::DeviceExt};
use glam::{Mat4, Quat};

use crate::advanced_rendering::{fast_model::transform::Transform, texture::Texture};

/// stores a list of primitives
pub struct Mesh {
    pub primitives: Vec<Primitive>,
}
/// references one vertex buffer and index buffer
pub struct Primitive {
    pub vertex_buffer_ref: usize,
    pub index_buffer_ref: usize,
    pub material_ref: usize,
}
pub struct Node {
    pub transform: Transform,
    pub mesh: Option<usize>,
    pub children: Vec<usize>,
}
// pub struct Texture {
//     pub texture: wgpu::Texture,
//     pub view: wgpu::TextureView,
//     pub sampler: wgpu::Sampler,
//     pub bind_group: wgpu::BindGroup,
// }
// impl Texture {
//     pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
// }
pub struct Material {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group: wgpu::BindGroup,    
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub nodes: Vec<Node>,
    pub root_nodes: Vec<usize>,
    pub vertex_buffers: Vec<Buffer>,
    pub index_buffers: Vec<(Buffer,usize)>,
    pub materials: Vec<Material>,
}
#[repr(C)]
#[derive(bytemuck::Zeroable, bytemuck::NoUninit, Clone, Copy)]
pub struct Vertex {
    pub position: [f32;3],
    pub uv: [f32; 2],
}
impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute { // position
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute { //  uvs
                    offset: size_of::<[f32;3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // wgpu::VertexAttribute { // normal
                //     offset: size_of::<[f32;5]>() as wgpu::BufferAddress,
                //     shader_location: 2,
                //     format: wgpu::VertexFormat::Float32x3,
                // },
                // wgpu::VertexAttribute { // tangent
                //     offset: size_of::<[f32; 8]>() as wgpu::BufferAddress,
                //     shader_location: 3,
                //     format: wgpu::VertexFormat::Float32x3,
                // },
                // wgpu::VertexAttribute { // bit tangent
                //     offset: size_of::<[f32; 11]>() as wgpu::BufferAddress,
                //     shader_location: 4,
                //     format: wgpu::VertexFormat::Float32x3,
                // },
            ]
        }
    }
}

impl Model {
    pub async fn load_model(
        name: &str,
        device: &Device,
        queue: &Queue,
        layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let (doc, buffers, images) = gltf::import(name).unwrap();

        let mut new_self = Model {
            meshes: Vec::new(),
            nodes: Vec::new(),
            vertex_buffers: Vec::new(),
            index_buffers: Vec::new(),
            root_nodes: Vec::new(),
            materials: Vec::new(),
        };
        let normal_texture = Texture::load_texture("cube-normal.png", device, queue, true).await.unwrap();
        for image in doc.images() {
            let image = &images[image.index()];
            // let rgba = img.to_rgba8();
            let dimensions = (image.width,image.height);

            let size = wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            };
            let texture = device.create_texture(
                &wgpu::TextureDescriptor {
                    label: Some("hello"),
                    size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                }
            );

            // match image.format {
            //     Format::R16G16B16A16 => {
            //         println!("0");
            //     },
            //     Format::R32G32B32A32FLOAT => {
            //         println!("1");
            //     },
            //     Format::R8 => {
            //         println!("2");
            //     },
            //     Format::R8G8 => {
            //         println!("3");
            //     },
            //     Format::R8G8B8 => {
            //         println!("4");
            //     },
            //     Format::R8G8B8A8 => {
            //         println!("5");
            //     },
            //     Format::R16 => {
            //         println!("6");
            //     },
            //     Format::R16G16 => {
            //         println!("7");
            //     },
            //     Format::R16G16B16 => {
            //         println!("8");
            //     },
            //     Format::R32G32B32FLOAT => {
            //         println!("9");
            //     },
            // }
            let mut pixels: Vec<f32> = Vec::new();
            for i in 0..image.pixels.len()/3 {
                pixels.push(image.pixels[i] as f32 / 256.0);
                pixels.push(image.pixels[i+1] as f32 / 256.0);
                pixels.push(image.pixels[i+2] as f32 / 256.0);
                pixels.push(1.0);
            }
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    aspect: wgpu::TextureAspect::All,
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                bytemuck::cast_slice(&pixels),
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(16 * dimensions.0),
                    rows_per_image: Some(dimensions.1),
                },
                size,
            );
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let sampler = device.create_sampler(
                &wgpu::SamplerDescriptor {
                    address_mode_u: wgpu::AddressMode::Repeat,
                    address_mode_v: wgpu::AddressMode::Repeat,
                    address_mode_w: wgpu::AddressMode::Repeat,
                    mag_filter: wgpu::FilterMode::Nearest,
                    min_filter: wgpu::FilterMode::Nearest,
                    mipmap_filter: wgpu::MipmapFilterMode::Nearest,
                    ..Default::default()
                }
            );
            new_self.materials.push(Material {
                sampler: sampler.clone(),
                view: view.clone(),
                texture,
                bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&sampler),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::TextureView(&normal_texture.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 3,
                            resource: wgpu::BindingResource::Sampler(&normal_texture.sampler),
                        },
                    ],
                    label: Some(name),
                })
            });
        }
        
        for scene in doc.scenes() {
            for node in scene.nodes() {
                new_self.root_nodes.push(new_self.nodes.len());
                Model::parse_node(&mut new_self, &node, buffers.as_slice(), device);
            }
        }
        new_self
    }
    fn parse_node(model: &mut Model, node: &gltf::Node, buffers: &[gltf::buffer::Data], device: &Device) -> usize {
        let mut new_node = Node {
            // transform: Mat4::from_cols_array_2d(&node.transform().matrix()),
            transform: Transform::new((Quat::IDENTITY,Quat::IDENTITY), device),
            children: Vec::new(),
            mesh: None,
        };
        new_node.mesh = match node.mesh() {
            Some(mesh) => {
                let mut primitives = Vec::new();
                for primitive in mesh.primitives() {
                    let reader = primitive.reader(|buf| {
                        Some(&buffers[buf.index()])
                    });
                    let positions: Vec<[f32;3]> = reader.read_positions().unwrap().collect();
                    let normals: Vec<[f32;3]> = reader.read_normals().unwrap().collect();
                    let uvs: Vec<[f32;2]> = match reader.read_tex_coords(0) {
                        Some(val) => {val.into_f32().collect()}
                        None => {
                            vec![[0.0;2]; positions.len()]
                        }
                    };
                    let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();
                    let material = primitive.material();
                    let base_colour = material.pbr_metallic_roughness().base_color_factor();
                    let texture_index = material.pbr_metallic_roughness().base_color_texture().map(|t| {
                        t.texture().source().index()
                    });
                    let mut vertex_buffer = Vec::new();
                    for i in 0..positions.len() {
                        vertex_buffer.push(Vertex {
                            position: positions[i],
                            uv: uvs[i],
                        });
                    }
                    
                    primitives.push(Primitive {
                        vertex_buffer_ref: model.vertex_buffers.len(),
                        index_buffer_ref: model.vertex_buffers.len(),
                        material_ref: texture_index.unwrap_or(0),
                    });
                    model.index_buffers.push((device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("index buffer"),
                            contents: bytemuck::cast_slice(indices.as_slice()),
                            usage: wgpu::BufferUsages::INDEX,
                        }
                    ),indices.len()));
                    model.vertex_buffers.push(device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("vertex buffer"),
                            contents: bytemuck::cast_slice(vertex_buffer.as_slice()),
                            usage: wgpu::BufferUsages::VERTEX,
                        }
                    ));
                }
                let new_mesh = Mesh {
                    primitives,
                };
                model.meshes.push(new_mesh);
                Some(model.meshes.len()-1)
            },
            None => {None}
        };
        let self_index = model.nodes.len();
        model.nodes.push(new_node);
        for child in node.children() {
            let address = Model::parse_node(model, &child, buffers, device);
            model.nodes[self_index].children.push(address);
        }
        self_index
    }
}

pub trait DrawModelGltf<'a> {
    fn draw_node_instanced(
        &mut self,
        transform: &Transform,
        model: &'a Model,
        instances: Range<u32>,
        node_index: usize,
    );
    fn draw_model_gltf(
        &mut self,
        device: &Device,
        model: &'a Model,
    );
    #[allow(dead_code)]
    fn draw_model_gltf_instanced(
        &mut self,
        device: &Device,
        model: &'a Model,
        instances: Range<u32>,
    );
}
impl<'a, 'b> DrawModelGltf<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_node_instanced(
        &mut self,
        transform: &Transform,
        model: &'b Model,
        instances: Range<u32>,
        node_index: usize,
    ) {
        let node = &model.nodes[node_index];

        if let Some(mesh_index) = node.mesh {
            let mesh = &model.meshes[mesh_index];
            for primitive in &mesh.primitives {
                self.set_bind_group(1, &model.materials[primitive.material_ref].bind_group, &[]);
                self.set_vertex_buffer(0, model.vertex_buffers[primitive.vertex_buffer_ref].slice(..));
                self.set_index_buffer(model.index_buffers[primitive.index_buffer_ref].0.slice(..), wgpu::IndexFormat::Uint32);
                self.draw_indexed(0..(model.index_buffers[primitive.index_buffer_ref].1 as u32), 0, instances.clone());
            }
        }
        for child in &node.children {
            self.draw_node_instanced(&node.transform, model, instances.clone(), *child);
        }
    }
    fn draw_model_gltf(
        &mut self,
        device: &Device,
        model: &'b Model,
    ) {
        self.draw_model_gltf_instanced(device, model, 0..1);
    }
    fn draw_model_gltf_instanced(
        &mut self,
        device: &Device,
        model: &'b Model,
        instances: Range<u32>,
    ) {
        // self.set_bind_group(0, camera_bind_group, &[]);
        // self.set_bind_group(2, light_bind_group, &[]);
        for root_node in &model.root_nodes {
            self.draw_node_instanced(&Transform::default(device), model, instances.clone(), *root_node);
        }
    }
}