//! Projector shadow budget and atlas packing. Sun cascades keep their three
//! original tiles; projector maps keep half a cascade's resolution as the
//! atlas grows, so increasing the budget never reduces per-map quality.

use glam::{Mat4, Vec3, Vec4};

pub const MAX_PROJECTOR_SHADOWS: usize = 16;

/// Point lights that can cast shadows at once. Aurora addition: the Legacy
/// Viewer's point lights cross walls (multiPointLightF has no shadow term).
/// A point light takes one map of the atlas, after the projectors' maps, cut
/// in `POINT_FACES` faces (the sides of a cube around the light).
pub const MAX_POINT_SHADOWS: usize = 8;
pub const POINT_FACES: usize = 6;
/// Faces of a cube map see a little more than 90 degrees, so that the
/// filter's taps near an edge stay on their face. `POINT_GUARD` in common.wgsl.
pub const POINT_GUARD: f32 = 1.04;
/// Nearest caster of a point light, `point_shadow_near`. A light usually sits
/// inside its own lamp: what is this close to it does not block it.
/// `POINT_NEAR` in common.wgsl.
pub const POINT_NEAR: f32 = 0.25;

/// Budget of a frame: the projectors keep theirs, then the point lights take
/// the maps left within `MAX_PROJECTOR_SHADOWS` and the device's limits.
pub fn budget_split(resolution: u32, projectors: u8, points: u8, max_dimension: u32) -> (u8, u8) {
    let points = points.min(MAX_POINT_SHADOWS as u8);
    let total = projectors.saturating_add(points);
    let count = effective_limit(resolution, total, max_dimension);
    let projectors = projectors.min(count);
    (projectors, (count - projectors).min(points))
}

pub fn point_near(radius: f32) -> f32 {
    POINT_NEAR.min(radius * 0.25)
}

/// Axes of face `face` of a cube around a light: forward, right and up, the
/// same in `point_shadow` (common.wgsl). Right-handed, so that the casters
/// keep their winding.
fn point_axes(face: usize) -> (Vec3, Vec3, Vec3) {
    let axis = |i: usize| Vec3::AXES[i % 3];
    let a = face / 2;
    let (forward, up) = if face.is_multiple_of(2) {
        (axis(a), -axis(a + 2))
    } else {
        (-axis(a), axis(a + 2))
    };
    (forward, axis(a + 1), up)
}

/// World to the clip space of a face of a point light's map: depth 0 at
/// `point_near`, 1 at the light's reach.
pub fn point_face_matrix(center: Vec3, radius: f32, face: usize) -> Mat4 {
    let (forward, right, up) = point_axes(face);
    let near = point_near(radius);
    let a = radius / (radius - near);
    let row = |axis: Vec3, scale: f32, shift: f32| (axis * scale).extend(-(center.dot(axis) * scale + shift));
    Mat4::from_cols(
        row(right, 1.0 / POINT_GUARD, 0.0),
        row(up, 1.0 / POINT_GUARD, 0.0),
        row(forward, a, a * near),
        row(forward, 1.0, 0.0),
    )
    .transpose()
}

/// The box around a light for the caster culling of its map, as a matrix for
/// `gpu_cull::view_planes` (which keeps four sides and the top: what is under
/// the box stays a caster).
pub fn point_cull_matrix(center: Vec3, radius: f32) -> Mat4 {
    Mat4::from_scale(Vec3::splat(1.0 / radius.max(1e-3))) * Mat4::from_translation(-center)
}

/// Where a face lies in the atlas: x, y and its side in texels. Three faces
/// across a map and two down; `point_shadow` in common.wgsl reads the same.
pub fn point_face_origin(cascade: u32, slot: usize, face: usize) -> (u32, u32, u32) {
    let (x, y) = map_origin(cascade, slot);
    let side = cascade / 2 / 3;
    (x + (face as u32 % 3) * side, y + (face as u32 / 3) * side, side)
}

/// Whether a face can light anything the camera sees: false when the pyramid
/// of the face lies wholly outside a plane of the view (with a metre of
/// margin, for the other eye of a stereo view).
pub fn point_face_seen(planes: &[Vec4; 6], center: Vec3, radius: f32, face: usize) -> bool {
    let (forward, right, up) = point_axes(face);
    let far = radius * POINT_GUARD;
    let corners = [(1.0, 1.0), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)].map(|(x, y)| center + (forward + right * x + up * y) * far);
    !planes.iter().any(|p| {
        let outside = |c: Vec3| p.truncate().dot(c) + p.w < -1.0;
        outside(center) && corners.iter().all(|c| outside(*c))
    })
}

pub fn atlas_size(cascade: u32, count: u8) -> (u32, u32) {
    let extra = u32::from(count.min(MAX_PROJECTOR_SHADOWS as u8)).saturating_sub(4).div_ceil(4);
    (cascade * 2, cascade * 2 + extra * (cascade / 2))
}

pub fn map_origin(cascade: u32, slot: usize) -> (u32, u32) {
    let half = cascade / 2;
    if slot < 4 {
        (cascade + slot as u32 % 2 * half, cascade + slot as u32 / 2 * half)
    } else {
        let k = slot as u32 - 4;
        (k % 4 * half, cascade * 2 + k / 4 * half)
    }
}

/// Effective budget at the requested sun resolution on this device. Never
/// silently lower sun or projector resolution to accommodate more maps.
pub fn effective_limit(resolution: u32, requested: u8, max_dimension: u32) -> u8 {
    let size = resolution.clamp(512, 8192).min(max_dimension / 2);
    let count = requested.min(MAX_PROJECTOR_SHADOWS as u8);
    (0..=count).rev().find(|&n| atlas_size(size, n).1 <= max_dimension).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projector_atlas_preserves_resolution_and_does_not_overlap_the_sun() {
        for count in 0..=MAX_PROJECTOR_SHADOWS as u8 {
            let size = 4096;
            let half = size / 2;
            let (width, height) = atlas_size(size, count);
            let mut rectangles = vec![(0, 0, size), (size, 0, size), (0, size, size)];
            for slot in 0..usize::from(count) {
                let (x, y) = map_origin(size, slot);
                assert!(x + half <= width && y + half <= height);
                assert!(
                    rectangles
                        .iter()
                        .all(|&(a, b, extent)| x + half <= a || a + extent <= x || y + half <= b || b + extent <= y)
                );
                rectangles.push((x, y, half));
            }
        }
        assert_eq!(atlas_size(4096, 2), (8192, 8192));
        assert_eq!(atlas_size(4096, 10), (8192, 12288));
    }

    #[test]
    fn projector_budget_respects_texture_limits_without_changing_resolution() {
        assert_eq!(effective_limit(4096, 10, 16384), 10);
        assert_eq!(effective_limit(4096, 16, 8192), 4);
        assert_eq!(effective_limit(2048, 16, 8192), 16);
    }

    #[test]
    fn point_light_budget_comes_after_the_projectors_and_two_lights_grow_nothing() {
        assert_eq!(budget_split(4096, 2, 0, 16384), (2, 0));
        assert_eq!(budget_split(4096, 2, 2, 16384), (2, 2));
        assert_eq!(atlas_size(4096, 4), atlas_size(4096, 2));
        assert_eq!(budget_split(4096, 14, 8, 16384), (14, 2));
        assert_eq!(budget_split(4096, 2, 200, 16384).1, MAX_POINT_SHADOWS as u8);
        // a device that cannot grow the atlas keeps the projectors first
        assert_eq!(budget_split(4096, 4, 4, 8192), (4, 0));
    }

    // the same point, seen from the shader's side (`point_shadow`)
    fn shader_face(v: Vec3) -> (usize, f32, f32, f32) {
        let a = v.abs();
        let (axis, s, u, w) = if a.x >= a.y && a.x >= a.z {
            (0, v.x, v.y, v.z)
        } else if a.y >= a.z {
            (1, v.y, v.z, v.x)
        } else {
            (2, v.z, v.x, v.y)
        };
        let up = if s > 0.0 { -w } else { w };
        (axis * 2 + usize::from(s < 0.0), u / s.abs(), up / s.abs(), s.abs())
    }

    #[test]
    fn point_face_matrix_agrees_with_the_shader_on_every_side() {
        let center = Vec3::new(10.0, -3.0, 22.0);
        let radius = 12.0;
        for v in [
            Vec3::new(4.0, 1.0, -2.0),
            Vec3::new(-4.0, 1.0, 2.0),
            Vec3::new(1.0, 5.0, 2.0),
            Vec3::new(-1.0, -5.0, -2.0),
            Vec3::new(2.0, 1.0, 6.0),
            Vec3::new(-2.0, 1.0, -6.0),
        ] {
            let (face, x, y, d) = shader_face(v);
            let clip = point_face_matrix(center, radius, face) * (center + v).extend(1.0);
            assert!((clip.w - d).abs() < 1e-3);
            assert!((clip.x / clip.w - x / POINT_GUARD).abs() < 1e-4, "{v} x");
            assert!((clip.y / clip.w - y / POINT_GUARD).abs() < 1e-4, "{v} y");
            let near = point_near(radius);
            let depth = radius / (radius - near) * (d - near) / d;
            assert!((clip.z / clip.w - depth).abs() < 1e-4, "{v} z");
        }
    }

    #[test]
    fn point_face_axes_are_right_handed_and_every_face_has_its_place() {
        let mut seen = Vec::new();
        for face in 0..POINT_FACES {
            let (forward, right, up) = point_axes(face);
            assert!((right.cross(up) + forward).length() < 1e-6, "face {face}");
            let origin = point_face_origin(4096, 3, face);
            assert!(!seen.contains(&origin));
            let (x, y) = map_origin(4096, 3);
            assert!(origin.0 + origin.2 <= x + 2048 && origin.1 + origin.2 <= y + 2048);
            seen.push(origin);
        }
    }

    #[test]
    fn a_face_turned_away_from_the_view_is_left_out() {
        // the view: what lies east of x = 0, seen from there
        let mut planes = [Vec4::new(0.0, 0.0, 0.0, 1.0); 6];
        planes[0] = Vec4::new(1.0, 0.0, 0.0, 0.0);
        let center = Vec3::new(-5.0, 0.0, 0.0);
        assert!(point_face_seen(&planes, center, 10.0, 0));
        assert!(!point_face_seen(&planes, center, 10.0, 1));
        assert!(point_face_seen(&planes, center, 10.0, 2));
    }

    #[test]
    fn point_cull_box_holds_what_the_light_reaches_and_not_the_rest() {
        let planes = crate::gpu_cull::view_planes(point_cull_matrix(Vec3::new(5.0, 5.0, 5.0), 3.0)).map(|p| p.to_array());
        assert!(crate::gpu_cull::sphere_in(&planes, Vec3::new(7.0, 5.0, 5.0), 0.1));
        assert!(crate::gpu_cull::sphere_in(&planes, Vec3::new(9.0, 5.0, 5.0), 2.0));
        assert!(!crate::gpu_cull::sphere_in(&planes, Vec3::new(9.0, 5.0, 5.0), 0.1));
        assert!(!crate::gpu_cull::sphere_in(&planes, Vec3::new(5.0, 5.0, 9.0), 0.1));
    }
}
