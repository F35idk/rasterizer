mod draw_line;
pub mod math;
mod rasterize;

use draw_line as line;
use rasterize as rast;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub pos: math::Vec3,
    pub color: math::Vec4,
}

impl Vertex {
    pub const fn new(pos: math::Vec3, color: math::Vec4) -> Self {
        Self { pos, color }
    }

    pub const fn new_black(pos: math::Vec3) -> Self {
        Self {
            pos,
            color: math::vec4(0.0, 0.0, 0.0, 1.0),
        }
    }
}

// projects a vertex from camera space to to raster space
// while preserving (the negative of) the z-coordinate
fn project_to_raster_space(
    vert: math::Vec4,
    h_fov: f32,
    width_screen_space: f32,
    width_raster_space: usize,
    aspect_ratio: f32,
) -> math::Vec3 {
    let height_screen_space = width_screen_space * aspect_ratio;
    // horizontal fov decides the simulated distance between the canvas and the eye.
    // note that this distance is not used for clipping (it does not change the
    // actual position of the canvas, i.e the near clipping plane), it only serves
    // to scale the projected points based on the fov parameter.
    let eye_canvas_dist = (width_screen_space / 2.0) / (h_fov / 2.0).tan();

    // divide x and y by w to go from homogenous to cartesian coordinates.
    // this performs the perspective divide if w was set to z (or -z)
    let x = (vert.x / vert.w) * eye_canvas_dist;
    let y = (vert.y / vert.w) * eye_canvas_dist;

    // normalize
    let ndc_x = (x + width_screen_space / 2.0) / width_screen_space;
    let ndc_y = (y + height_screen_space / 2.0) / height_screen_space;

    // get raster space coordinates
    let raster_x = ndc_x * width_raster_space as f32;
    let raster_y = (1.0 - ndc_y) * width_raster_space as f32 * aspect_ratio;

    math::vec3(raster_x, raster_y, -vert.z)
}

pub fn rasterize<F>(
    vertices: &[Vertex],
    indices: Option<&[u32]>,
    h_fov: f32,
    canvas: &mut [u32],
    depth_buffer: &mut [f32],
    canvas_width: usize,
    canvas_height: usize,
    mut vert_shader: F,
) where
    F: FnMut(Vertex) -> math::Vec4,
{
    assert!(canvas_width <= 2048 && canvas_height <= 2048);
    let aspect_ratio = canvas_height as f32 / canvas_width as f32;

    if let Some(ind) = indices {
        // TODO: it would be nice to be able to use chunks_exact() here
        for tri_indices in ind.chunks(3) {
            let tri_verts = [
                vertices[tri_indices[0] as usize],
                vertices[tri_indices[1] as usize],
                vertices[tri_indices[2] as usize],
            ];
            // final raster space vertices to draw the triangle lines between
            let mut raster_verts = [None, None, None];

            for (i, vert) in tri_verts.iter().enumerate() {
                // run 'vertex shader' on vertex
                let processed_vert = vert_shader(*vert);

                // discard if behind or inside camera (in camera space)
                if processed_vert.z >= 0.0 {
                    continue;
                }

                raster_verts[i] = Some(project_to_raster_space(
                    processed_vert,
                    h_fov,
                    1.0,
                    canvas_width,
                    aspect_ratio,
                ));
            }

            match raster_verts {
                [Some(v1), Some(v2), Some(v3)] => {
                    rast::rasterize_tri_fixed_point(
                        v1,
                        tri_verts[0].color,
                        v2,
                        tri_verts[1].color,
                        v3,
                        tri_verts[2].color,
                        canvas,
                        depth_buffer,
                        canvas_width,
                        canvas_height,
                    );
                }
                _ => (),
            }
        }
    } else {
        for tri in vertices.chunks(3) {
            let mut raster_verts = [None, None, None];

            for (i, vert) in tri.iter().enumerate() {
                let processed_vert = vert_shader(*vert);

                if processed_vert.z >= 0.0 {
                    continue;
                }

                raster_verts[i] = Some(project_to_raster_space(
                    processed_vert,
                    h_fov,
                    1.0,
                    canvas_width,
                    aspect_ratio,
                ));
            }

            match raster_verts {
                [Some(v1), Some(v2), Some(v3)] => {
                    rast::rasterize_tri_fixed_point(
                        v1,
                        tri[0].color,
                        v2,
                        tri[1].color,
                        v3,
                        tri[2].color,
                        canvas,
                        depth_buffer,
                        canvas_width,
                        canvas_height,
                    );
                }
                _ => (),
            }
        }
    }
}

pub fn draw_lines<F>(
    vertices: &[Vertex],
    indices: Option<&[u32]>,
    h_fov: f32,
    canvas: &mut [u32],
    canvas_width: usize,
    canvas_height: usize,
    mut vert_shader: F,
) where
    F: FnMut(Vertex) -> math::Vec4,
{
    let aspect_ratio = canvas_height as f32 / canvas_width as f32;

    if let Some(ind) = indices {
        for tri_indices in ind.chunks(3) {
            let tri_verts = [
                vertices[tri_indices[0] as usize],
                vertices[tri_indices[1] as usize],
                vertices[tri_indices[2] as usize],
            ];

            let mut raster_verts = [None, None, None, None];

            for (i, vert) in tri_verts.iter().enumerate() {
                let processed_vert = vert_shader(*vert);

                if processed_vert.z >= 0.0 {
                    continue;
                }

                let raster_vert =
                    project_to_raster_space(processed_vert, h_fov, 1.0, canvas_width, aspect_ratio);

                raster_verts[i] = Some(math::ivec2(raster_vert.x as i32, raster_vert.y as i32));
            }

            // ensure that a line will be drawn between the last and the first vertex
            raster_verts[3] = raster_verts[0];

            for verts in raster_verts.windows(2) {
                match verts {
                    [Some(v1), Some(v2)] => {
                        line::draw_line_clipped_i64(*v1, *v2, canvas, canvas_width, canvas_height)
                    }
                    _ => (),
                }
            }
        }
    } else {
        for tri in vertices.chunks(3) {
            let mut raster_verts = [None, None, None, None];

            for (i, vert) in tri.iter().enumerate() {
                let processed_vert = vert_shader(*vert);

                if processed_vert.z >= 0.0 {
                    continue;
                }

                let raster_vert =
                    project_to_raster_space(processed_vert, h_fov, 1.0, canvas_width, aspect_ratio);

                raster_verts[i] = Some(math::ivec2(raster_vert.x as i32, raster_vert.y as i32));
            }

            // ensure that a line will be drawn between the last and the first vertex
            raster_verts[3] = raster_verts[0];

            for verts in raster_verts.windows(2) {
                match verts {
                    [Some(v1), Some(v2)] => {
                        line::draw_line_clipped_i64(*v1, *v2, canvas, canvas_width, canvas_height)
                    }
                    _ => (),
                }
            }
        }
    }
}
