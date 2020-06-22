mod window;

use pixel_renderer::{self as pix, xcb};
use rasterizer::{math, math::vec3, math::vec4, rasterize, Vertex};
use window as win;
use xcb_util::keysyms;

const WINDOW_WIDTH: usize = 1920;
const WINDOW_HEIGHT: usize = 1080;

// cube made up of 12 triangles. courtesy of opengl-tutorial.org
static CUBE_VERTICES: [Vertex; 36] = [
    Vertex::new(vec3(-1.0, -1.0, -1.0), vec4(0.583, 0.771, 0.014, 1.0)),
    Vertex::new(vec3(-1.0, -1.0, 1.0), vec4(0.609, 0.115, 0.436, 1.0)),
    Vertex::new(vec3(-1.0, 1.0, 1.0), vec4(0.327, 0.483, 0.844, 1.0)),
    Vertex::new(vec3(1.0, 1.0, -1.0), vec4(0.822, 0.569, 0.201, 1.0)),
    Vertex::new(vec3(-1.0, -1.0, -1.0), vec4(0.435, 0.602, 0.223, 1.0)),
    Vertex::new(vec3(-1.0, 1.0, -1.0), vec4(0.310, 0.747, 0.185, 1.0)),
    Vertex::new(vec3(1.0, -1.0, 1.0), vec4(0.597, 0.770, 0.761, 1.0)),
    Vertex::new(vec3(-1.0, -1.0, -1.0), vec4(0.559, 0.436, 0.730, 1.0)),
    Vertex::new(vec3(1.0, -1.0, -1.0), vec4(0.359, 0.583, 0.152, 1.0)),
    Vertex::new(vec3(1.0, 1.0, -1.0), vec4(0.483, 0.596, 0.789, 1.0)),
    Vertex::new(vec3(1.0, -1.0, -1.0), vec4(0.559, 0.861, 0.639, 1.0)),
    Vertex::new(vec3(-1.0, -1.0, -1.0), vec4(0.195, 0.548, 0.859, 1.0)),
    Vertex::new(vec3(-1.0, -1.0, -1.0), vec4(0.014, 0.184, 0.576, 1.0)),
    Vertex::new(vec3(-1.0, 1.0, 1.0), vec4(0.771, 0.328, 0.970, 1.0)),
    Vertex::new(vec3(-1.0, 1.0, -1.0), vec4(0.406, 0.615, 0.116, 1.0)),
    Vertex::new(vec3(1.0, -1.0, 1.0), vec4(0.676, 0.977, 0.133, 1.0)),
    Vertex::new(vec3(-1.0, -1.0, 1.0), vec4(0.971, 0.572, 0.833, 1.0)),
    Vertex::new(vec3(-1.0, -1.0, -1.0), vec4(0.140, 0.616, 0.489, 1.0)),
    Vertex::new(vec3(-1.0, 1.0, 1.0), vec4(0.997, 0.513, 0.064, 1.0)),
    Vertex::new(vec3(-1.0, -1.0, 1.0), vec4(0.945, 0.719, 0.592, 1.0)),
    Vertex::new(vec3(1.0, -1.0, 1.0), vec4(0.543, 0.021, 0.978, 1.0)),
    Vertex::new(vec3(1.0, 1.0, 1.0), vec4(0.279, 0.317, 0.505, 1.0)),
    Vertex::new(vec3(1.0, -1.0, -1.0), vec4(0.167, 0.620, 0.077, 1.0)),
    Vertex::new(vec3(1.0, 1.0, -1.0), vec4(0.347, 0.857, 0.137, 1.0)),
    Vertex::new(vec3(1.0, -1.0, -1.0), vec4(0.055, 0.953, 0.042, 1.0)),
    Vertex::new(vec3(1.0, 1.0, 1.0), vec4(0.714, 0.505, 0.345, 1.0)),
    Vertex::new(vec3(1.0, -1.0, 1.0), vec4(0.783, 0.290, 0.734, 1.0)),
    Vertex::new(vec3(1.0, 1.0, 1.0), vec4(0.722, 0.645, 0.174, 1.0)),
    Vertex::new(vec3(1.0, 1.0, -1.0), vec4(0.302, 0.455, 0.848, 1.0)),
    Vertex::new(vec3(-1.0, 1.0, -1.0), vec4(0.225, 0.587, 0.040, 1.0)),
    Vertex::new(vec3(1.0, 1.0, 1.0), vec4(0.517, 0.713, 0.338, 1.0)),
    Vertex::new(vec3(-1.0, 1.0, -1.0), vec4(0.053, 0.959, 0.120, 1.0)),
    Vertex::new(vec3(-1.0, 1.0, 1.0), vec4(0.393, 0.621, 0.362, 1.0)),
    Vertex::new(vec3(1.0, 1.0, 1.0), vec4(0.673, 0.211, 0.457, 1.0)),
    Vertex::new(vec3(-1.0, 1.0, 1.0), vec4(0.820, 0.883, 0.371, 1.0)),
    Vertex::new(vec3(1.0, -1.0, 1.0), vec4(0.982, 0.099, 0.879, 1.0)),
];

struct ModelViewTransforms {
    view_mat: math::Mat4,
    cam_pos: math::Vec3,
    cam_rotation_x: f32,
    cam_rotation_y: f32,
    cam_rotation_z: f32,
}

impl ModelViewTransforms {
    fn new(pos: math::Vec3, rotation_x: f32, rotation_y: f32, rotation_z: f32) -> Self {
        let translate_mat = math::Mat4 {
            x4: [1.0, 0.0, 0.0, -pos.x],
            y4: [0.0, 1.0, 0.0, -pos.y],
            z4: [0.0, 0.0, 1.0, -pos.z],
            w4: [0.0, 0.0, 0.0, 1.0000],
        };

        let rotate_x_mat = math::Mat4 {
            x4: [1.0, 0.00000000000000, 0.00000000000000, 0.0],
            y4: [0.0, rotation_x.cos(), -rotation_x.sin(), 0.0],
            z4: [0.0, rotation_x.sin(), rotation_x.cos(), 0.00],
            w4: [0.0, 0.00000000000000, 0.000000000000000, 1.0],
        };

        let rotate_y_mat = math::Mat4 {
            x4: [rotation_y.cos(), 0.00, rotation_y.sin(), 0.0],
            y4: [0.000000000000000, 1.0, 0.00000000000000, 0.0],
            z4: [-rotation_y.sin(), 0.0, rotation_y.cos(), 0.0],
            w4: [0.000000000000000, 0.0, 0.00000000000000, 1.0],
        };

        let rotate_z_mat = math::Mat4 {
            x4: [rotation_z.cos(), -rotation_z.sin(), 0.0, 0.0],
            y4: [rotation_z.sin(), rotation_z.cos(), 0.00, 0.0],
            z4: [0.00000000000000, 0.000000000000000, 1.0, 0.0],
            w4: [0.00000000000000, 0.000000000000000, 0.0, 1.0],
        };

        // needed to ensure transformed vertices have their w-component set to -z.
        // this way 'rasterize()' will perform proper perspective division on them
        let z_to_w_mat = math::Mat4 {
            x4: [1.0, 0.0, 0.00, 0.0],
            y4: [0.0, 1.0, 0.00, 0.0],
            z4: [0.0, 0.0, 1.00, 0.0],
            w4: [0.0, 0.0, -1.0, 0.0],
        };

        let view_mat = z_to_w_mat * rotate_z_mat * rotate_y_mat * rotate_x_mat * translate_mat;

        Self {
            view_mat,
            cam_pos: pos,
            cam_rotation_x: rotation_x,
            cam_rotation_y: rotation_y,
            cam_rotation_z: rotation_z,
        }
    }

    fn update(&mut self, pos: math::Vec3, rotation_x: f32, rotation_y: f32, rotation_z: f32) {
        let pos_delta = pos - self.cam_pos;
        let rotation_x_delta = rotation_x - self.cam_rotation_x;
        let rotation_y_delta = rotation_y - self.cam_rotation_y;
        let rotation_z_delta = rotation_z - self.cam_rotation_z;

        let new_rotate_x = math::Mat4 {
            x4: [1.0, 0.00000000000000000000, 0.000000000000000000000, 0.0],
            y4: [0.0, rotation_x_delta.cos(), -rotation_x_delta.sin(), 0.0],
            z4: [0.0, rotation_x_delta.sin(), rotation_x_delta.cos(), 0.00],
            w4: [0.0, 0.00000000000000000000, 0.000000000000000000000, 1.0],
        };

        let new_rotate_y = math::Mat4 {
            x4: [rotation_y_delta.cos(), 0.00, rotation_y_delta.sin(), 0.0],
            y4: [0.000000000000000000000, 1.0, 0.00000000000000000000, 0.0],
            z4: [-rotation_y_delta.sin(), 0.0, rotation_y_delta.cos(), 0.0],
            w4: [0.000000000000000000000, 0.0, 0.00000000000000000000, 1.0],
        };

        let new_rotate_z = math::Mat4 {
            x4: [rotation_z_delta.cos(), -rotation_z_delta.sin(), 0.0, 0.0],
            y4: [rotation_z_delta.sin(), rotation_z_delta.cos(), 0.00, 0.0],
            z4: [0.00000000000000000000, 0.000000000000000000000, 1.0, 0.0],
            w4: [0.00000000000000000000, 0.000000000000000000000, 0.0, 1.0],
        };

        let z_to_w_mat = math::Mat4 {
            x4: [1.0, 0.0, 0.00, 0.0],
            y4: [0.0, 1.0, 0.00, 0.0],
            z4: [0.0, 0.0, 1.00, 0.0],
            w4: [0.0, 0.0, -1.0, 0.0],
        };

        let final_mat = z_to_w_mat * new_rotate_z * new_rotate_y * new_rotate_x;
        // subtract deltas instead of multiplying with a separate translation
        // matrix (translation matrices don't go well with 'z_to_w_mat')
        self.view_mat.x4[3] -= pos_delta.x;
        self.view_mat.y4[3] -= pos_delta.y;
        self.view_mat.z4[3] -= pos_delta.z;

        *self = Self {
            view_mat: final_mat * self.view_mat,
            cam_pos: pos,
            cam_rotation_x: rotation_x,
            cam_rotation_y: rotation_y,
            cam_rotation_z: rotation_z,
        };
    }
}

fn pixels_to_u32<'a>(pixel_renderer: &'a mut pix::PixelRenderer) -> &'a mut [u32] {
    unsafe {
        std::slice::from_raw_parts_mut(
            pixel_renderer.get_pixels().as_mut_ptr() as *mut u32,
            pixel_renderer.get_pixels().len() / 4,
        )
    }
}

fn main() {
    let mut window = win::XcbWindowWrapper::new(
        "rasterizer 3d example",
        WINDOW_WIDTH as u16,
        WINDOW_HEIGHT as u16,
    )
    .unwrap();

    let mut pixel_renderer = pix::PixelRenderer::new(
        &mut window.connection,
        window.win,
        WINDOW_WIDTH as u64,
        WINDOW_HEIGHT as u64,
    )
    .unwrap();

    let key_syms = keysyms::KeySymbols::new(&window.connection);
    let mut depth_buf = vec![0f32; WINDOW_WIDTH * WINDOW_HEIGHT];

    // initial camera settings
    let mut fov = (90f32).to_radians();
    let mut cam_tilt = 0f32;
    let mut cam_pan = -50f32.to_radians();
    let mut cam_pos = vec3(4.0, 0.0, 2.0);
    let mut mv = ModelViewTransforms::new(cam_pos, cam_tilt, cam_pan, 0.0);

    // clear screen to white
    {
        for p in pixel_renderer.get_pixels() {
            *p = std::u8::MAX;
        }
    }

    // draw initial frame
    {
        let pixels = pixels_to_u32(&mut pixel_renderer);

        rasterize(
            &CUBE_VERTICES,
            None,
            fov,
            pixels,
            &mut depth_buf,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            // transform vertex coords to camera space in 'vertex shader' closure
            |vert| math::vec4_from_3(vert.pos, 1.0) * mv.view_mat,
        );
    }

    window.map_and_flush();

    // enter game loop
    loop {
        let img_index = pixel_renderer.render_frame();
        pixel_renderer.present(img_index);

        if let Some(e) = window.connection.poll_for_event() {
            match e.response_type() as u8 & !0x80 {
                xcb::KEY_PRESS => {
                    let key_press: &xcb::KeyPressEvent = unsafe { xcb::cast_event(&e) };
                    let mods = key_press.state();
                    let key_sym = key_syms.press_lookup_keysym(key_press, 0);

                    match (key_sym, mods) {
                        // control + q to exit
                        (0x71, mods) if mods & (1 << 2) != 0 => return,
                        // wasd keys to move
                        (0x77, _) => cam_pos.z -= 0.3,
                        (0x73, _) => cam_pos.z += 0.3,
                        (0x64, _) => cam_pos.x += 0.3,
                        (0x61, _) => cam_pos.x -= 0.3,
                        // up and down arrow keys to tilt camera
                        (0xff52, _) => cam_tilt -= (2f32).to_radians(),
                        (0xff54, _) => cam_tilt += (2f32).to_radians(),
                        // left and right arrow keys to pan camera
                        (0xff51, _) => cam_pan -= (2f32).to_radians(),
                        (0xff53, _) => cam_pan += (2f32).to_radians(),
                        // space to move upwards
                        (0x20, _) => cam_pos.y += 0.3,
                        // left control to move downwards
                        (0xffe3, _) => cam_pos.y -= 0.3,
                        // h and l keys to increase and decrease fov
                        (0x68, _) => fov += (5f32).to_radians(),
                        (0x6c, _) => fov -= (5f32).to_radians(),
                        _ => (),
                    }

                    // clear screen
                    for p in pixel_renderer.get_pixels() {
                        *p = std::u8::MAX;
                    }

                    // update modelview matrix
                    mv.update(cam_pos, cam_tilt, cam_pan, 0.0);

                    // draw new frame
                    {
                        let pixels = pixels_to_u32(&mut pixel_renderer);
                        for d in depth_buf.iter_mut() {
                            *d = 0.0
                        }

                        rasterize(
                            &CUBE_VERTICES,
                            None,
                            fov,
                            pixels,
                            &mut depth_buf,
                            WINDOW_WIDTH,
                            WINDOW_HEIGHT,
                            |vert| math::vec4_from_3(vert.pos, 1.0) * mv.view_mat,
                        );
                    }
                }
                _ => (),
            }
        }
    }
}
