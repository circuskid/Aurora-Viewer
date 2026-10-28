// AURORA_DLSS_DEBUG=reproject (dlss.rs): how well the motion vectors given
// to DLSS carry last frame's image onto this one. Each pixel's colour is
// compared with last frame's at the place its motion points to, the
// jitters of both frames taken out. Dim grey: the scene; red: the
// difference. A still scene and correct motion stay grey but for the edges
// and the shading that really changed; wrong motion lights up red.
@group(0) @binding(0) var motion: texture_2d<f32>;
@group(0) @binding(1) var color: texture_2d<f32>;
@group(0) @binding(2) var prev_color: texture_2d<f32>;
@group(0) @binding(3) var linear: sampler;
// xy: last frame's jitter minus this frame's, in render pixels
@group(0) @binding(4) var<uniform> jitter: vec4<f32>;
struct Vertex { @builtin(position) position: vec4<f32>, @location(0) uv: vec2<f32> };
@vertex fn vs_main(@builtin(vertex_index) index: u32) -> Vertex {
    let p = vec2<f32>(f32((index << 1u) & 2u), f32(index & 2u));
    var v: Vertex;
    v.position = vec4<f32>(p * 2.0 - 1.0, 0.0, 1.0);
    v.uv = vec2<f32>(p.x, 1.0 - p.y);
    return v;
}
fn tone(c: vec3<f32>) -> vec3<f32> {
    return c / (vec3<f32>(1.0) + c);
}
@fragment fn fs_main(v: Vertex) -> @location(0) vec4<f32> {
    let size = vec2<i32>(textureDimensions(motion));
    let p = clamp(vec2<i32>(v.uv * vec2<f32>(size)), vec2<i32>(0), size - 1);
    let m = textureLoad(motion, p, 0).xy;
    let cur = tone(textureLoad(color, p, 0).rgb);
    let q = vec2<f32>(p) + 0.5 + m + jitter.xy;
    let prev = tone(textureSampleLevel(prev_color, linear, q / vec2<f32>(size), 0.0).rgb);
    let err = length(cur - prev);
    let grey = dot(cur, vec3<f32>(0.3, 0.59, 0.11)) * 0.35;
    return vec4<f32>(vec3<f32>(grey) + vec3<f32>(clamp(err * 6.0, 0.0, 1.0), 0.0, 0.0), 1.0);
}
