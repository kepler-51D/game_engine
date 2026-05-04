struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0)
    position: vec3<f32>,
    @location(1)
    texture_coords: vec2<f32>
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
fn vs_main(model: VertexInput) -> VertexOutput {
    let world_position = vec4<f32>(model.position, 1.0);
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
    let result = object_color.xyz;
    let base: f32 = 8.0;
    let b = log2(round(exp2(log2(base) * length(result)))) / log2(base);
    let result_rounded = normalize(result) * b;
    return vec4<f32>(result_rounded, object_color.a);
}
