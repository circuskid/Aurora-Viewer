// Motion vectors of the avatars for DLSS (dlss.rs), concatenated after
// common.wgsl, object.wgsl and env.wgsl. The opaque draws of the frame again,
// over the camera's motion (dlss_motion.wgsl): a rigged mesh or a worn prim
// placed this frame as the depth prepass placed it, and as last frame's
// palettes (a copy kept at the end of each frame) placed it. Only where it is
// what the scene depth shows (an alpha-masked cut-out or a nearer surface
// keeps the camera's motion). Everything else is left out by its vertices.
// Previous minus current, in render pixels, without the jitter.
struct AvatarMotion {
    // last frame's camera and this frame's, without the jitter, both
    // relative to this frame's eye
    prev_eye_view_proj: mat4x4<f32>,
    eye_view_proj: mat4x4<f32>,
    size: vec4<f32>, // render size, z: coverage debug
};
@group(3) @binding(10) var<uniform> avatar_motion: AvatarMotion;
@group(3) @binding(11) var<storage, read> prev_palettes: array<mat4x4<f32>>;
@group(3) @binding(12) var avatar_motion_depth: texture_depth_2d;

// joint_matrix, skin_matrix and placed_model (object.wgsl) on last frame's
// palettes. The live morphs (lip sync) are this frame's in both.
fn prev_joint_matrix(rec: DrawRecord, j: u32) -> mat4x4<f32> {
    let b = skin_binds[rec.flags.z + j];
    return prev_palettes[rec.flags.y + b.joint.x] * b.inverse_bind;
}

fn prev_skin_matrix(rec: DrawRecord, j: vec4<u32>, w: vec4<f32>) -> mat4x4<f32> {
    let ws = max(w.x + w.y + w.z + w.w, 1e-4);
    var m = prev_joint_matrix(rec, j.x) * w.x;
    if (w.y > 0.0) { m += prev_joint_matrix(rec, j.y) * w.y; }
    if (w.z > 0.0) { m += prev_joint_matrix(rec, j.z) * w.z; }
    if (w.w > 0.0) { m += prev_joint_matrix(rec, j.w) * w.w; }
    return m * (1.0 / ws);
}

fn prev_placed_model(rec: DrawRecord, in: VsIn, eye: vec3<f32>) -> mat4x4<f32> {
    if ((rec.flags.x & FLAG_SKINNED) != 0u) {
        let skin = prev_skin_matrix(rec, in.joints, in.weights);
        if ((rec.flags.x & FLAG_ANCHORED) != 0u) {
            return from_eye(prev_palettes[rec.flags.y + SKELETON_ANCHOR], eye) * skin;
        }
        return from_eye(rec.model, eye) * skin;
    }
    return from_eye(prev_palettes[rec.flags.y], eye) * rec.model;
}

struct AvatarMotionOut {
    @invariant @builtin(position) clip: vec4<f32>,
    @location(0) cur: vec4<f32>,
    @location(1) prev: vec4<f32>,
};

@vertex
fn vs_avatar_motion(in: VsIn) -> AvatarMotionOut {
    let rec = records[in.instance];
    var out: AvatarMotionOut;
    if ((rec.flags.x & (FLAG_SKINNED | FLAG_ANCHORED)) == 0u) {
        // outside the view: the whole draw is clipped
        out.clip = vec4<f32>(2.0, 2.0, 2.0, 1.0);
        return out;
    }
    let p = vec4<f32>(vertex_position(rec, in), 1.0);
    // vs_depth's position, to meet the depth it wrote
    let ep = eye_model(rec, in) * p;
    out.clip = frame.eye_view_proj * ep;
    out.cur = avatar_motion.eye_view_proj * ep;
    out.prev = avatar_motion.prev_eye_view_proj * (prev_placed_model(rec, in, frame.eye.xyz) * p);
    return out;
}

fn avatar_motion_pixel(clip: vec4<f32>) -> vec2<f32> {
    let ndc = clip.xy / clip.w;
    return vec2<f32>((ndc.x * 0.5 + 0.5) * avatar_motion.size.x, (0.5 - ndc.y * 0.5) * avatar_motion.size.y);
}

@fragment
fn fs_avatar_motion(in: AvatarMotionOut) -> @location(0) vec4<f32> {
    let d = textureLoad(avatar_motion_depth, vec2<i32>(in.clip.xy), 0);
    if (abs(in.clip.z - d) > max(d, in.clip.z) * 1e-4 || abs(in.prev.w) < 1e-12 || abs(in.cur.w) < 1e-12) {
        discard;
    }
    // size.z: AURORA_DLSS_DEBUG=coverage, the pixels written at full motion
    if (avatar_motion.size.z > 0.5) {
        return vec4<f32>(1.0, 1.0, 0.0, 0.0);
    }
    return vec4<f32>(avatar_motion_pixel(in.prev) - avatar_motion_pixel(in.cur), 0.0, 0.0);
}
