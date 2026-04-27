

/// Signed area of the triangle `(p0, p1, p2)`.
/// The sign is used for point-in-triangle orientation tests.
fn area_of_a_triangle(
    p0: [f32;2],
    p1: [f32;2],
    p2: [f32;2]) -> f32 {                
    // problem 1: implement below

    // comment out the next line
    return 0.0
}

/// Rasterize a filled triangle using edge-orientation tests at pixel centers.
fn draw_triangle(
    p0: [f32; 2],
    p1: [f32; 2],
    p2: [f32; 2],
    pix2rgba: &mut [[u8; 4]],
    width: usize,
    color: [u8; 4],
) {
    let height = pix2rgba.len() / width;
    for ih in 0..height {
        for iw in 0..width {
            // Sample at pixel center to reduce aliasing bias.
            let p = [iw as f32 + 0.5, ih as f32 + 0.5];
            let a01 = area_of_a_triangle(p, p0, p1);
            let a12 = area_of_a_triangle(p, p1, p2);
            let a20 = area_of_a_triangle(p, p2, p0);
            if a01 > 0.0 && a12 > 0.0 && a20 > 0.0 {
                pix2rgba[ih * width + iw] = color;
            }
        }
    }
}

/// Rasterize a filled polygon via winding number.
fn draw_polygon(vtx2xy: &[[f32; 2]], img_data: &mut [[u8; 4]], width: usize, color: [u8; 4]) {
    let num_vtx = vtx2xy.len();
    let height = img_data.len() / width;
    for ih in 0..height {
        for iw in 0..width {
            let x = iw as f32 + 0.5;
            let y = ih as f32 + 0.5;
            let mut winding_number: f32 = 0.0;
            for i_edge in 0..num_vtx {
                let i0 = i_edge;
                let i1 = (i_edge + 1) % num_vtx;
                let p0x = vtx2xy[i0][0] - x;
                let p0y = vtx2xy[i0][1] - y;
                let p1x = vtx2xy[i1][0] - x;
                let p1y = vtx2xy[i1][1] - y;
                // problem 2: implement below

                // end of edit
            }
            // Convert accumulated angle from radians to winding count.
            winding_number /= 2.0 * std::f32::consts::PI;
            let int_winding = winding_number.round() as i32;
            if int_winding == 1 {
                img_data[ih * width + iw] = color;
            }
        }
    }
}

/// Draw a line using the DDA (Digital Differential Analyzer) algorithm.
fn dda_line(
    [x0, y0]: [f32; 2],
    [x1, y1]: [f32; 2],
    img_data: &mut [[u8; 4]],
    width: usize,
    color: [u8; 4],
) {
    let height = img_data.len() / width;
    let dx = x1 - x0;
    let dy = y1 - y0;
    // Step one pixel at a time along the dominant axis.
    let steps = dx.abs().max(dy.abs()).ceil() as u32;
    if steps == 0 {
        return;
    }
    let x_inc = dx / steps as f32;
    let y_inc = dy / steps as f32;
    // problem 3: implement below

    // end of edit
}

/// Draw a circle centred at (`cx`, `cy`) with integer radius `r`
fn draw_circle(
    [cx, cy]: [f32; 2],
    r: f32,
    img_data: &mut [[u8; 4]],
    width: usize,
    color: [u8; 4],
) {
    let height = img_data.len() / width;
    // Helper: plot one pixel if it is inside the canvas
    let mut plot = |x: f32, y: f32| {
        if x >= 0. && x < width as f32 && y >= 0. && y < height as f32 {
            img_data[y as usize * width + x as usize] = color;
        }
    };
    // Split into two sweeps around 45 degrees to avoid steep-slope gaps.
    let t = r * std::f32::consts::FRAC_1_SQRT_2;
    // problem 4: implement below

    // end of edit
}

/// Render one animation frame at normalized time `time` in `[0, 1)`.
fn render_frame(time: f32, width: usize, height: usize) -> Vec<[u8; 4]> {
    let mut pix2rgba: Vec<[u8; 4]> = vec![[255u8; 4]; width * height];

    // Polygon
    {
        let vtx2xy = vec![
            [55.0, 5.],
            [5. + time * 150., 5. + time * 100.],
            [15., 55.],
            [15., 85.],
            [85., 85.],
            [85., 5.],
        ];
        draw_polygon(&vtx2xy, &mut pix2rgba, width, [100, 255, 200, 255]);
    }

    // Triangle
    {
        // Rotate around pivot `c`.
        let transform = |[x, y]: [f32; 2], [cx, cy]: [f32; 2], rad: f32| -> [f32; 2] {
            let dx = x - cx;
            let dy = y - cy;
            [
                dx * rad.cos() + dy * rad.sin() + cx,
                -dx * rad.sin() + dy * rad.cos() + cy,
            ]
        };
        let c = [5., 5.];
        let p0 = transform([5., 5.], c, time);
        let p1 = transform([15., 45.], c, time);
        let p2 = transform([45., 15.], c, time);
        draw_triangle(p0, p1, p2, &mut pix2rgba, width, [255, 100, 250, 255]);
    }

    // circumference
    {
        let r = 10.0 + 25.0 * (time * 2.0 * std::f32::consts::PI).sin().abs();
        draw_circle([50.0, 50.0], r, &mut pix2rgba, width, [220, 80, 30, 255]);
    }

    // Radiating lines
    {
        let rex = 30.0 + 60.0 * (time * 2.0 * std::f32::consts::PI).cos();
        let rey = 50.0 + 60.0 * (time * 2.0 * std::f32::consts::PI).sin();
        dda_line(
            [30.0, 50.0],
            [rex, rey],
            &mut pix2rgba,
            width,
            [0, 0, 0, 255],
        );
    }

    pix2rgba
}

fn main() {
    const WIDTH: usize = 120;
    const HEIGHT: usize = 100;
    const NUM_FRAMES: usize = 36; // one full rotation
    // Build an infinitely looping GIF.
    let file = std::fs::File::create("output.gif").expect("Failed to create output.gif");
    let mut encoder = image::codecs::gif::GifEncoder::new_with_speed(file, 10);
    encoder
        .set_repeat(image::codecs::gif::Repeat::Infinite)
        .expect("Failed to set GIF repeat");
    for i_frame in 0..NUM_FRAMES {
        let time = i_frame as f32 / NUM_FRAMES as f32;
        let rgba = render_frame(time, WIDTH, HEIGHT);
        // save frame as GIF
        let img = image::RgbaImage::from_raw(WIDTH as u32, HEIGHT as u32, rgba.into_flattened())
            .expect("Failed to build RGBA image");
        let frame = image::Frame::from_parts(img, 0, 0, image::Delay::from_numer_denom_ms(80, 1));
        encoder
            .encode_frame(frame)
            .expect("Failed to encode GIF frame");
    }
    println!("Saved output.gif ({NUM_FRAMES} frames)");
}
