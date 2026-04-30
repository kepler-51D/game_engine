use wgpu::{BindGroupLayout, Buffer, Device, Queue, util::DeviceExt};
use glam::{Mat4};

/// stores a list of primitives
pub struct Mesh {
    pub primitives: Vec<Primitive>,
}
/// references one vertex buffer and index buffer
pub struct Primitive {
    pub vertex_buffer_ref: usize,
    pub index_buffer_ref: usize,
}
pub struct Node {
    pub transform: Mat4,
    pub mesh: Option<usize>,
    pub children: Vec<usize>,
}
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    // pub bind_group: wgpu::BindGroup,
}
impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub nodes: Vec<Node>,
    pub root_nodes: Vec<usize>,
    pub vertex_buffers: Vec<Buffer>,
    pub index_buffers: Vec<Buffer>,
    pub textures: Vec<Texture>,
}
#[repr(C)]
#[derive(bytemuck::Zeroable,bytemuck::NoUninit, Clone, Copy)]
pub struct Vertex {
    pub position: [f32;3],
    pub uv: [f32; 2],
}
impl Model {
    pub fn load_model(
        name: &str,
        device: &Device,
        queue: &Queue,
        texture_bind_group_layout: &BindGroupLayout,
    ) -> Self {
        let (doc, buffers, images) = gltf::import(name).unwrap();
        // println!("{}",doc.as_json().to_string_pretty().unwrap());
        // todo!() process images
        let mut new_self = Model {
            meshes: Vec::new(),
            nodes: Vec::new(),
            vertex_buffers: Vec::new(),
            index_buffers: Vec::new(),
            root_nodes: Vec::new(),
            textures: Vec::new(),
        };
        // for image in images {

        // }
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
                    // format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    format: wgpu::TextureFormat::Rgba32Float,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                }
            );

            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    aspect: wgpu::TextureAspect::All,
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                &image.pixels,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * dimensions.0),
                    rows_per_image: Some(dimensions.1),
                },
                size,
            );

            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let sampler = device.create_sampler(
                &wgpu::SamplerDescriptor {
                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                    address_mode_w: wgpu::AddressMode::ClampToEdge,
                    mag_filter: wgpu::FilterMode::Linear,
                    min_filter: wgpu::FilterMode::Nearest,
                    mipmap_filter: wgpu::MipmapFilterMode::Nearest,
                    ..Default::default()
                }
            );
            // let texture_buffer = device.create_buffer_init(
            //     &wgpu::util::BufferInitDescriptor {
            //         label: Some("Light VB"),
            //         contents: bytemuck::cast_slice(image.pixels.as_slice()),
            //         usage: wgpu::BufferUsages::VERTEX,
            //     }
            // );
            // let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            //     layout: &texture_bind_group_layout,
            //     entries: &[wgpu::BindGroupEntry {
            //         binding: 0,
            //         resource: texture_buffer,
            //     }],
            //     label: None,
            // });
            new_self.textures.push(Texture {
                sampler,
                view,
                texture,
            });
        }
        for scene in doc.scenes() {
            for node in scene.nodes() {
                Model::parse_node(&mut new_self, &node, buffers.as_slice(), device);
            }
        }
        todo!();
    }
    fn parse_node(model: &mut Model, node: &gltf::Node, buffers: &[gltf::buffer::Data], device: &Device) -> usize {
        let mut new_node = Node {
            transform: Mat4::from_cols_array_2d(&node.transform().matrix()),
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
                    let uvs: Vec<[f32;2]> = reader.read_tex_coords(0).unwrap().into_f32().collect();
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
                    });
                    model.index_buffers.push(device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("Light VB"),
                            contents: bytemuck::cast_slice(indices.as_slice()),
                            usage: wgpu::BufferUsages::VERTEX,
                        }
                    ));
                    model.vertex_buffers.push(device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("Light VB"),
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
            None => {panic!()}
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