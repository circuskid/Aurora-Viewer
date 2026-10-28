// Camera motion vectors for DLSS (dlss.rs): each pixel's world point,
// rebuilt from the scene depth, projected by last frame's camera. In render
// pixels, previous minus current, without the jitter. Only the camera's
// motion: moving objects and skinned avatars have none of their own yet.
struct U {
    // the jittered camera of this frame, inverted: the point the pixel shows
    inv_view_proj: mat4x4<f32>,
    // this frame's and last frame's cameras, without the jitter
    view_proj: mat4x4<f32>,
    prev_view_proj: mat4x4<f32>,
    size: vec4<f32>,
};
@group(0) @binding(0) var depth: texture_depth_2d;
@group(0) @binding(1) var<uniform> u: U;

@vertex fn vs_main(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    let p = vec2<f32>(f32((index << 1u) & 2u), f32(index & 2u));
    return vec4<f32>(p * 2.0 - 1.0, 0.0, 1.0);
}

fn to_pixel(clip: vec4<f32>) -> vec2<f32> {
    let ndc = clip.xy / clip.w;
    return vec2<f32>((ndc.x * 0.5 + 0.5) * u.size.x, (0.5 - ndc.y * 0.5) * u.size.y);
}

@fragment fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let d = textureLoad(depth, vec2<i32>(pos.xy), 0);
    let uv = pos.xy / u.size.xy;
    // reverse Z: the sky (0) is a direction, w = 0, which the projections
    // below still turn into a pixel (the camera's turn, not its move)
    let world = u.inv_view_proj * vec4<f32>(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, d, 1.0);
    let prev = u.prev_view_proj * world;
    let cur = u.view_proj * world;
    if abs(prev.w) < 1e-12 || abs(cur.w) < 1e-12 {
        return vec4<f32>(0.0);
    }
    return vec4<f32>(to_pixel(prev) - to_pixel(cur), 0.0, 0.0);
}
