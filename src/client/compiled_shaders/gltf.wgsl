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

struct BoneTransformInput {
    @location(5)
    real_part: vec4<f32>,
    @location(6)
    dual_part: vec4<f32>
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

fn rotate_vec3_by_quat(v: vec3<f32>, q: vec4<f32>) -> vec3<f32> {
    let u = q.xyz;
    let s = q.w;
    return 2.0 * dot(u, v) * u + (s * s - dot(u, u)) * v + 2.0 * s * cross(u, v);
}

@vertex
fn vs_main(model: VertexInput, bone_transform: BoneTransformInput) -> VertexOutput {
    let bone_rot = bone_transform.real_part;
    let bone_pos = (bone_transform.dual_part * vec4<f32>(-bone_rot.xyz, bone_rot.w) * 2.0).xyz;
    let world_position = vec4<f32>((model.position + rotate_vec3_by_quat(bone_pos, bone_rot)) * 10, 1.0);
    var out: VertexOutput;
    out.clip_position = camera.view_proj * world_position;
    out.texture_coords = model.texture_coords;
    return out;
}

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;

@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let object_color: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.texture_coords);
    let object_normal: vec4<f32> = vec4<f32>(0.0, 0.0, 1.0, 0.0);
    let ambient_strength = 0.0;
    let ambient_color = light.color * ambient_strength;
    let tangent_normal = vec3<f32>(0.0, 0.0, 1.0);
    let light_dir = normalize(in.tangent_light_position - in.tangent_position);
    let view_dir = normalize(in.tangent_view_position - in.tangent_position);
    let half_dir = normalize(view_dir + light_dir);
    let diffuse_strength = max(dot(tangent_normal, light_dir), 0.0);
    let diffuse_color = light.color * diffuse_strength;
    let specular_strength = pow(max(dot(tangent_normal, half_dir), 0.0), 32.0);
    let specular_color = specular_strength * light.color;
    let result = (ambient_color + diffuse_color + specular_color) * object_color.xyz;
    let base: f32 = 8.0;
    return vec4<f32>(result, object_color.a);
}
