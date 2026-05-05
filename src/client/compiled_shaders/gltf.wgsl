struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct Light {
    position: vec3<f32>,
    color: vec3<f32>
}

@group(2) @binding(0)
var<uniform> light: Light;

struct VertexInput {
    @location(0)
    position: vec3<f32>,
    @location(1)
    texture_coords: vec2<f32>,
    @location(2)
    normal: vec3<f32>,
    @location(3)
    tangent: vec3<f32>,
    @location(4)
    bitangent: vec3<f32>
}

struct InstanceInput {
    @location(5)
    model_matrix_0: vec4<f32>,
    @location(6)
    model_matrix_1: vec4<f32>,
    @location(7)
    model_matrix_2: vec4<f32>,
    @location(8)
    model_matrix_3: vec4<f32>,
    @location(9)
    normal_matrix_0: vec3<f32>,
    @location(10)
    normal_matrix_1: vec3<f32>,
    @location(11)
    normal_matrix_2: vec3<f32>
}

struct VertexOutput {
    @builtin(position)
    clip_position: vec4<f32>,
    @location(0)
    texture_coords: vec2<f32>,
    @location(1)
    tangent_position: vec3<f32>,
    @location(2)
    tangent_light_position: vec3<f32>,
    @location(3)
    tangent_view_position: vec3<f32>
}

@vertex
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    let model_matrix = mat4x4<f32>(instance.model_matrix_0, instance.model_matrix_1, instance.model_matrix_2, instance.model_matrix_3);
    let normal_matrix = mat3x3<f32>(instance.normal_matrix_0, instance.normal_matrix_1, instance.normal_matrix_2);
    let world_normal = normalize(normal_matrix * model.normal);
    let world_tangent = normalize(normal_matrix * model.tangent);
    let world_bitangent = normalize(normal_matrix * model.bitangent);
    let tangent_matrix = transpose(mat3x3<f32>(world_tangent, world_bitangent, world_normal));
    let world_position = vec4<f32>(model.position, 1.0);
    var out: VertexOutput;
    out.clip_position = camera.view_proj * world_position;
    out.texture_coords = model.texture_coords;
    out.tangent_position = tangent_matrix * world_position.xyz;
    out.tangent_view_position = tangent_matrix * camera.view_pos.xyz;
    out.tangent_light_position = tangent_matrix * light.position;
    return out;
}

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;

@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let object_color: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.texture_coords);
    let result = object_color.xyz;
    let base: f32 = 8.0;
    let b = log2(round(exp2(log2(base) * length(result)))) / log2(base);
    let result_rounded = normalize(result) * b;
    return vec4<f32>(result_rounded, object_color.a);
}
