use super::math::*; // TODO: don't

// alternative to 'draw_line_clipped()' that uses 64 bit ints to ensure
// no overflow. should be slower, but doesn't impose any limits on
// the size of the input coordinates (given that they're f32s)
pub fn draw_line_clipped_i64(
    v1: Ivec2,
    v2: Ivec2,
    canvas: &mut [u32],
    canvas_width: usize,
    canvas_height: usize,
) {
    // dimensions must be even (dunno why they wouldn't be)
    debug_assert!(canvas_height % 2 == 0 && canvas_width % 2 == 0);

    // use 64 bit ints to avoid overflow
    let mut v1 = lvec2(v1.x as i64, v1.y as i64);
    let mut v2 = lvec2(v2.x as i64, v2.y as i64);

    // whether line is longer vertically than horizontally
    let is_steep = (v2.y - v1.y).abs() > (v2.x - v1.x).abs();

    // if line is steep, swap x for y and iterate on rows instead of columns
    if is_steep {
        std::mem::swap(&mut v1.x, &mut v1.y);
        std::mem::swap(&mut v2.x, &mut v2.y);
    }

    // set 'start' to be the point with the lowest x-coordinate
    let (mut start, mut end) = if v1.x > v2.x { (v2, v1) } else { (v1, v2) };

    let delta_x = end.x - start.x;
    let delta_y = (end.y - start.y).abs();

    let sign_delta_y = if end.y - start.y > 0 { 1 } else { -1 };

    let (x_max, y_max) = if is_steep {
        (canvas_height as i64 - 1, canvas_width as i64 - 1)
    } else {
        (canvas_width as i64 - 1, canvas_height as i64 - 1)
    };

    // perform clipping
    {
        if start.x >= x_max {
            // both points are outside of canvas
            return;
        }

        if start.x < 0 {
            if end.x < 0 {
                // outside of canvas
                return;
            }

            let new_x = 0;
            // use similar triangles to compute intersection of
            // line and canvas edge (without floating point arithmetic)
            let new_y = (end.y - start.y) * (0 - start.x) / (end.x - start.x) + start.y;

            start = lvec2(new_x, new_y);
        }

        // NOTE: this assumes the canvas dimensions are even,
        // hence the assert in the beginning of the function
        let y_half = (y_max + 1) / 2;
        let start_y_sign_bit = start.y >> 31;
        let end_y_sign_bit = end.y >> 31;

        // if 'start.y' is greater than y_max or less than
        // zero, adjust 'start' to fit inside canvas
        if (y_half - start.y).abs() - start_y_sign_bit >= y_half {
            if (y_half - end.y).abs() - (end_y_sign_bit) >= y_half {
                // both are outside of canvas
                return;
            }

            let new_y = (!start_y_sign_bit * y_max).abs();
            let new_x = (end.x - start.x) * (new_y - start.y) / (end.y - start.y) + start.x;

            start = lvec2(new_x, new_y);
        }

        // if 'end.y' is greater than y_max or less than
        // zero, adjust 'end' to fit inside canvas
        if (y_half - end.y).abs() - end_y_sign_bit >= y_half {
            let new_y = (!end_y_sign_bit * y_max).abs();
            let new_x = (start.x - end.x) * (new_y - end.y) / (start.y - end.y) + end.x;

            end = lvec2(new_x, new_y);
        }
    }

    // 'err_bound' represents the distance from the current pixel center
    // to the line directly below/above, multiplied by (-2) * delta_x. when
    // this is smaller than 0, y is incremented/decremented
    let mut err_bound = delta_x;
    let mut y = start.y;

    for x in start.x..end.x.min(x_max) {
        // account for the fact that we may have swapped our x and y-values
        if is_steep {
            canvas[x as usize * canvas_width + y as usize] =
                unsafe { std::mem::transmute::<[u8; 4], _>([0, 0, 0, 255]) };
        } else {
            canvas[y as usize * canvas_width + x as usize] =
                unsafe { std::mem::transmute::<[u8; 4], _>([0, 0, 0, 255]) };
        }

        err_bound -= 2 * delta_y;
        // err_bound being <= 0 is equivalent to the distance from
        // the line to the current pixel center being >= 0.5
        if err_bound <= 0 {
            // increment or decrement y by 1 depending on
            // whether we're moving downwards or upwards
            y += sign_delta_y;
            // readjust err_bound so it represents distance from
            // line to next pixel (multiplied by (-2) * delta_x)
            err_bound += 2 * delta_x;
        }
    }
}

// draws lines between vertices 'v1' and 'v2' on 'canvas' using
// bresenham's while clipping to the boundaries of 'canvas'
pub fn _draw_line_clipped(
    mut v1: Ivec2,
    mut v2: Ivec2,
    canvas: &mut [u32],
    canvas_width: usize,
    canvas_height: usize,
) {
    // dimensions must be even
    debug_assert!(canvas_height % 2 == 0 && canvas_width % 2 == 0);

    // whether line is longer vertically than horizontally
    let is_steep = (v2.y - v1.y).abs() > (v2.x - v1.x).abs();

    // if line is steep, swap x for y and iterate on rows instead of columns
    if is_steep {
        std::mem::swap(&mut v1.x, &mut v1.y);
        std::mem::swap(&mut v2.x, &mut v2.y);
    }

    // set 'start' to be the point with the lowest x-coordinate
    let (mut start, mut end) = if v1.x > v2.x { (v2, v1) } else { (v1, v2) };

    let delta_x = end.x - start.x;
    let delta_y = (end.y - start.y).abs();

    let sign_delta_y = if end.y - start.y > 0 { 1 } else { -1 };

    let (x_max, y_max) = if is_steep {
        (canvas_height as i32 - 1, canvas_width as i32 - 1)
    } else {
        (canvas_width as i32 - 1, canvas_height as i32 - 1)
    };

    // perform clipping
    {
        if start.x >= x_max {
            // both points are outside of canvas
            return;
        }

        if start.x < 0 {
            if end.x < 0 {
                // outside of canvas
                return;
            }

            let new_x = 0;
            // use similar triangles to compute intersection of
            // line and canvas edge (without floating point arithmetic)
            let new_y = (end.y - start.y) * (0 - start.x) / (end.x - start.x) + start.y;

            start = ivec2(new_x, new_y);
        }

        // NOTE: this assumes the canvas dimensions are even,
        // hence the assert in the beginning of the function
        let y_half = (y_max + 1) / 2;
        let start_y_sign_bit = start.y >> 31;
        let end_y_sign_bit = end.y >> 31;

        // if 'start.y' is greater than y_max or less than
        // zero, adjust 'start' to fit inside canvas
        if (y_half - start.y).abs() - start_y_sign_bit >= y_half {
            if (y_half - end.y).abs() - (end_y_sign_bit) >= y_half {
                // both are outside of canvas
                return;
            }

            let new_y = (!start_y_sign_bit * y_max).abs();
            let new_x = (end.x - start.x) * (new_y - start.y) / (end.y - start.y) + start.x;

            start = ivec2(new_x, new_y);
        }

        // if 'end.y' is greater than y_max or less than
        // zero, adjust 'end' to fit inside canvas
        if (y_half - end.y).abs() - end_y_sign_bit >= y_half {
            let new_y = (!end_y_sign_bit * y_max).abs();
            let new_x = (start.x - end.x) * (new_y - end.y) / (start.y - end.y) + end.x;

            end = ivec2(new_x, new_y);
        }
    }

    // 'err_bound' represents the distance from the current pixel center
    // to the line directly below/above, multiplied by (-2) * delta_x. when
    // this is smaller than 0, y is incremented/decremented
    let mut err_bound = delta_x;
    let mut y = start.y;

    for x in start.x..end.x.min(x_max) {
        // account for the fact that we may have swapped our x and y-values
        if is_steep {
            canvas[x as usize * canvas_width + y as usize] =
                unsafe { std::mem::transmute::<[u8; 4], _>([0, 0, 0, 255]) };
        } else {
            canvas[y as usize * canvas_width + x as usize] =
                unsafe { std::mem::transmute::<[u8; 4], _>([0, 0, 0, 255]) };
        }

        err_bound -= 2 * delta_y;
        // err_bound <= 0 is equivalent to the distance from
        // the line to the current pixel center being >= 0.5
        if err_bound <= 0 {
            // increment or decrement y by 1 depending on
            // whether we're moving downwards or upwards
            y += sign_delta_y;
            // readjust err_bound so it represents distance
            // from next pixel (multiplied by (-2) * delta_x)
            err_bound += 2 * delta_x;
        }
    }
}
