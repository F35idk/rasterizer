use std::ops::{Mul, Sub};

use super::math::*; // TODO: don't

#[inline]
// returns a positive integer if the point 'v0' is
// to the right of the vector 'v1v2' in raster space
fn edge<T>(v0: Vec2<T>, v1: Vec2<T>, v1v2: Vec2<T>) -> T
where
    T: Mul<Output = T> + Sub<Output = T>,
{
    let v1v0 = v0 - v1;
    // swap x and y when computing the determinant of the matrix [v1v0, v1v2]
    // to compensate for the fact that up is negative in raster space
    v1v0.y * v1v2.x - v1v0.x * v1v2.y
}

pub fn rasterize_tri_fixed_point(
    v1: Vec3<f32>,
    v1_color: Vec4<f32>,
    v2: Vec3<f32>,
    v2_color: Vec4<f32>,
    v3: Vec3<f32>,
    v3_color: Vec4<f32>,
    canvas: &mut [u32],
    depth_buffer: &mut [f32],
    canvas_width: usize,
    canvas_height: usize,
) {
    // FIXME: casting from f32 to i32 when the f32 has a greater value than i32::MAX
    // is u.b in rust atm. even ignoring that, we should still clip values that are
    // too large (and may overflow later) before passing them to this function

    // get edge vectors in fixed point (28.4 format)
    let v3v2_i = Vec2::<i32>::from(Vec2::from(v2) * 16.0) - Vec2::from(Vec2::from(v3) * 16.0);
    let v2v1_i = Vec2::<i32>::from(Vec2::from(v1) * 16.0) - Vec2::from(Vec2::from(v2) * 16.0);
    let v1v3_i = Vec2::<i32>::from(Vec2::from(v3) * 16.0) - Vec2::from(Vec2::from(v1) * 16.0);

    // convert vertices to fixed point
    let v1_i = Vec2::<i32>::from(Vec2::from(v1) * 16.0);
    let v2_i = Vec2::<i32>::from(Vec2::from(v2) * 16.0);
    let v3_i = Vec2::<i32>::from(Vec2::from(v3) * 16.0);

    // compute twice the (signed) area of the triangle v1v2v3
    let area_tri = (v3v2_i.x * v2v1_i.y - v3v2_i.y * v2v1_i.x) >> 4;

    // get bounding box coordinates
    let y_max = (clamp_i(
        0,
        (canvas_height as i32) << 4,
        v1_i.y.max(v2_i.y.max(v3_i.y)),
        // round up to nearest pixel center by adding 0.4 (7 in 28.4 format),
        // truncating and then adding 0.5 when computing 'start_pixel' later
    ) + 7)
        >> 4;
    let y_min = (clamp_i(
        0,
        (canvas_height as i32) << 4,
        v1_i.y.min(v2_i.y.min(v3_i.y)),
    ) + 7)
        >> 4;
    let x_max = (clamp_i(
        0,
        (canvas_width as i32) << 4,
        v1_i.x.max(v2_i.x.max(v3_i.x)),
    ) + 7)
        >> 4;
    let x_min = (clamp_i(
        0,
        (canvas_width as i32 - 1) << 4,
        v1_i.x.min(v2_i.x.min(v3_i.x)),
    ) + 7)
        >> 4;

    let start_pixel = vec2::<i32>((x_min << 4) + 8, (y_min << 4) + 8); // add 0.5 (for pixel center)

    // calculate initial barycentric coordinates/subtriangle areas for each corner vertex
    let mut area1_initial = (edge(start_pixel, v2_i, v2v1_i) >> 4) - 1;
    let mut area2_initial = (edge(start_pixel, v3_i, v3v2_i) >> 4) - 1;
    let mut area3_initial = (edge(start_pixel, v1_i, v1v3_i) >> 4) - 1;

    // precompute the increase in barycentric coordinate values per
    // pixel moved down (y_step) and per pixel moved right (x_step)
    let area1_x_step = v2_i.y - v1_i.y;
    let area1_y_step = v1_i.x - v2_i.x;
    let area2_x_step = v3_i.y - v2_i.y;
    let area2_y_step = v2_i.x - v3_i.x;
    let area3_x_step = v1_i.y - v3_i.y;
    let area3_y_step = v3_i.x - v1_i.x;

    // set to true if the triangle edges are 'top' or 'left' edges, set to false otherwise
    let top_or_left_v3v2 = (v3v2_i.y == 0 && v3v2_i.x > 0) || v3v2_i.y < 0;
    let top_or_left_v2v1 = (v2v1_i.y == 0 && v2v1_i.x > 0) || v2v1_i.y < 0;
    let top_or_left_v1v3 = (v1v3_i.y == 0 && v1v3_i.x > 0) || v1v3_i.y < 0;

    // set to all 1s if triangle vertices are in ccw order and all 0s otherwise
    let is_ccw_mask = (area_tri > 0) as u32 * std::u32::MAX;

    // and with 'is_ccw_mask' to ensure early return if triangle vertices aren't ccw
    // TODO: iterate zig-zag or tile-wise instead of over bounding box
    for i in (y_min as u32)..(y_max as u32 & is_ccw_mask) {
        let i = i as usize;

        let mut area1 = area1_initial;
        let mut area2 = area2_initial;
        let mut area3 = area3_initial;

        for j in x_min..x_max {
            let j = j as usize;

            // get inverse of z-coordinates
            let v1_z_recip = v1.z.recip();
            let v2_z_recip = v2.z.recip();
            let v3_z_recip = v3.z.recip();

            // compute colors divided by z-coordinate ('perspective projected' colors)
            let v1_color_persp = v1_color / v1.z;
            let v2_color_persp = v2_color / v2.z;
            let v3_color_persp = v3_color / v3.z;

            // add biases to areas to include top/left edge cases
            area1 += top_or_left_v2v1 as i32;
            area2 += top_or_left_v3v2 as i32;
            area3 += top_or_left_v1v3 as i32;

            if area1 >= 0 && area2 >= 0 && area3 >= 0 {
                // compute weights/barycentric coords
                let weight_v1 = area2 as f32 / area_tri as f32;
                let weight_v2 = area3 as f32 / area_tri as f32;
                let weight_v3 = area1 as f32 / area_tri as f32;

                // get (reciprocal of) pixel z-position by interpolating
                let z_recip =
                    v1_z_recip * weight_v1 + v2_z_recip * weight_v2 + v3_z_recip * weight_v3;

                if depth_buffer[i * canvas_width + j] < z_recip {
                    depth_buffer[i * canvas_width + j] = z_recip;

                    // get pixel color by interpolating (with perspective correction)
                    let r = (v1_color_persp.x * weight_v1
                        + v2_color_persp.x * weight_v2
                        + v3_color_persp.x * weight_v3)
                        * z_recip.recip();
                    let g = (v1_color_persp.y * weight_v1
                        + v2_color_persp.y * weight_v2
                        + v3_color_persp.y * weight_v3)
                        * z_recip.recip();
                    let b = (v1_color_persp.z * weight_v1
                        + v2_color_persp.z * weight_v2
                        + v3_color_persp.z * weight_v3)
                        * z_recip.recip();
                    let a = (v1_color_persp.w * weight_v1
                        + v2_color_persp.w * weight_v2
                        + v3_color_persp.w * weight_v3)
                        * z_recip.recip();

                    canvas[i * canvas_width + j] = unsafe {
                        std::mem::transmute([
                            (r * 255.0) as u8,
                            (g * 255.0) as u8,
                            (b * 255.0) as u8,
                            (a * 255.0) as u8,
                        ])
                    };
                }
            }

            // move right by one pixel
            area1 += area1_x_step;
            area2 += area2_x_step;
            area3 += area3_x_step;
        }

        // move down by one pixel
        area1_initial += area1_y_step;
        area2_initial += area2_y_step;
        area3_initial += area3_y_step;
    }
}

pub fn _rasterize_tri_f(
    v1: Vec3<f32>,
    v1_color: Vec4<f32>,
    v2: Vec3<f32>,
    v2_color: Vec4<f32>,
    v3: Vec3<f32>,
    v3_color: Vec4<f32>,
    canvas: &mut [u32],
    depth_buffer: &mut [f32],
    canvas_width: usize,
    canvas_height: usize,
) {
    let v3v2 = v2 - v3;
    let v2v1 = v1 - v2;
    let v1v3 = v3 - v1;

    // compute twice the (signed) area of the triangle v1v2v3
    let area_tri = v3v2.x * v2v1.y - v3v2.y * v2v1.x;

    // get bounding box coordinates
    let y_max = clamp_f(0.0, canvas_height as f32, v1.y.max(v2.y.max(v3.y)));
    let y_min = clamp_f(0.0, canvas_height as f32, v1.y.min(v2.y.min(v3.y)));
    let x_max = clamp_f(0.0, canvas_width as f32, v1.x.max(v2.x.max(v3.x)));
    let x_min = clamp_f(0.0, canvas_width as f32, v1.x.min(v2.x.min(v3.x)));

    let start_pixel = vec2(x_min.floor() + 0.5, y_min.floor() + 0.5);

    // calculate initial barycentric coordinates/subtriangle areas for each corner vertex
    let mut area1_initial = edge(start_pixel, Vec2::from(v2), Vec2::from(v2v1));
    let mut area2_initial = edge(start_pixel, Vec2::from(v3), Vec2::from(v3v2));
    let mut area3_initial = edge(start_pixel, Vec2::from(v1), Vec2::from(v1v3));

    // precompute the increase in barycentric coordinate values per
    // pixel moved down (y_step) and per pixel moved right (x_step)
    let area1_x_step = v2.y - v1.y;
    let area1_y_step = v1.x - v2.x;
    let area2_x_step = v3.y - v2.y;
    let area2_y_step = v2.x - v3.x;
    let area3_x_step = v1.y - v3.y;
    let area3_y_step = v3.x - v1.x;

    // set to true if the triangle edges are 'top' or 'left' edges, set to false otherwise
    let top_or_left_v3v2 = (v3v2.y == 0.0 && v3v2.x > 0.0) || v3v2.y < 0.0;
    let top_or_left_v2v1 = (v2v1.y == 0.0 && v2v1.x > 0.0) || v2v1.y < 0.0;
    let top_or_left_v1v3 = (v1v3.y == 0.0 && v1v3.x > 0.0) || v1v3.y < 0.0;

    // set 'is_ccw_mask' to all 1s if triangle vertices are in ccw order and all 0s otherwise
    let is_ccw_mask = (area_tri > 0.0) as u16 * std::u16::MAX;

    // and with 'is_ccw_mask' to ensure early return if triangle vertices aren't ccw
    // TODO: iterate zig-zag or tile-wise instead of over bounding box
    for i in (y_min as u16)..(y_max as u16 & is_ccw_mask) {
        let i = i as usize;

        let mut area1 = area1_initial;
        let mut area2 = area2_initial;
        let mut area3 = area3_initial;

        for j in (x_min as u16)..(x_max as u16) {
            let j = j as usize;

            // precompute inverse of z-coordinates
            let v1_z_recip = v1.z.recip();
            let v2_z_recip = v2.z.recip();
            let v3_z_recip = v3.z.recip();

            // precompute colors divided by z-coordinate ('perspective projected' colors)
            let v1_color_persp = v1_color / v1.z;
            let v2_color_persp = v2_color / v2.z;
            let v3_color_persp = v3_color / v3.z;

            let mut is_inside = area1 > 0.0 && area2 > 0.0 && area3 > 0.0;

            // set 'is_inside' to true if pixel is on a top/left edge
            is_inside |= (area1 == 0.0) && area2 > 0.0 && area3 > 0.0 && top_or_left_v2v1;
            is_inside |= (area2 == 0.0) && area1 > 0.0 && area3 > 0.0 && top_or_left_v3v2;
            is_inside |= (area3 == 0.0) && area1 > 0.0 && area2 > 0.0 && top_or_left_v1v3;

            if is_inside {
                // compute weights/barycentric coords
                let weight_v1 = area2 / area_tri;
                let weight_v2 = area3 / area_tri;
                let weight_v3 = area1 / area_tri;

                // get (reciprocal of) pixel z-position by interpolating
                let z = v1_z_recip * weight_v1 + v2_z_recip * weight_v2 + v3_z_recip * weight_v3;

                if depth_buffer[i * canvas_width + j] < z {
                    depth_buffer[i * canvas_width + j] = z;

                    // get pixel color by interpolating (with perspective correction)
                    let r = (v1_color_persp.x * weight_v1
                        + v2_color_persp.x * weight_v2
                        + v3_color_persp.x * weight_v3)
                        * z.recip();
                    let g = (v1_color_persp.y * weight_v1
                        + v2_color_persp.y * weight_v2
                        + v3_color_persp.y * weight_v3)
                        * z.recip();
                    let b = (v1_color_persp.z * weight_v1
                        + v2_color_persp.z * weight_v2
                        + v3_color_persp.z * weight_v3)
                        * z.recip();
                    let a = (v1_color_persp.w * weight_v1
                        + v2_color_persp.w * weight_v2
                        + v3_color_persp.w * weight_v3)
                        * z.recip();

                    canvas[i * canvas_width + j] = unsafe {
                        std::mem::transmute([
                            (r * 255.0) as u8,
                            (g * 255.0) as u8,
                            (b * 255.0) as u8,
                            (a * 255.0) as u8,
                        ])
                    };
                }
            }

            // move right by one pixel
            area1 += area1_x_step;
            area2 += area2_x_step;
            area3 += area3_x_step;
        }

        // move down by one pixel
        area1_initial += area1_y_step;
        area2_initial += area2_y_step;
        area3_initial += area3_y_step;
    }
}
