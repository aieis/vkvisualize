struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}
    
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = vec4<f32>(model.position, 1);
    out.tex_coords = model.tex_coords;
    
    return out;    
}

@group(0) @binding(0)
var t_tex: texture_depth_2d;

@group(0) @binding(1)
var s_tex: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let d = textureSample(t_tex, s_tex, in.tex_coords);
    return vec4<f32>(f32(d), 0, 0, 1);
}
