use crate::{colors::Colors, framebuffer::Framebuffer};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct ColoredVertex {
    pub pos: [f32; 4],
    pub color: Colors,
}

// this thing now has depth to it. pretty cool, huh?
impl ColoredVertex {
    pub fn draw_gouraud_triangle(
        fb: &mut Framebuffer,
        v0: ColoredVertex,
        v1: ColoredVertex,
        v2: ColoredVertex,
    ) {
        fn edge(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> f32 {
            (c[0] - a[0]) * (b[1] - a[1]) - (c[1] - a[1]) * (b[0] - a[0])
        }

        let (inv_w0, p0, col0) = prepare(&v0);
        let (inv_w1, p1, col1) = prepare(&v1);
        let (inv_w2, p2, col2) = prepare(&v2);

        let min_x = p0[0].min(p1[0]).min(p2[0]).floor().max(0.0) as usize;
        let max_x = p0[0]
            .max(p1[0])
            .max(p2[0])
            .ceil()
            .min(fb.width as f32 - 1.0) as usize;
        let min_y = p0[1].min(p1[1]).min(p2[1]).floor().max(0.0) as usize;
        let max_y = p0[1]
            .max(p1[1])
            .max(p2[1])
            .ceil()
            .min(fb.height as f32 - 1.0) as usize;

        let area = edge(p0, p1, p2);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = [x as f32 + 0.5, y as f32 + 0.5];

                let w0 = edge(p1, p2, p);
                let w1 = edge(p2, p0, p);
                let w2 = edge(p0, p1, p);

                let inside = if area > 0.0 {
                    w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
                } else {
                    w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0
                };

                if inside {
                    let w0_n = w0 / area;
                    let w1_n = w1 / area;
                    let w2_n = w2 / area;

                    let inv_w_interp = w0_n * inv_w0 + w1_n * inv_w1 + w2_n * inv_w2;

                    let r = (w0_n * col0[0] + w1_n * col1[0] + w2_n * col2[0]) / inv_w_interp;
                    let g = (w0_n * col0[1] + w1_n * col1[1] + w2_n * col2[1]) / inv_w_interp;
                    let b = (w0_n * col0[2] + w1_n * col1[2] + w2_n * col2[2]) / inv_w_interp;

                    let final_color = Colors::from_rgb_f32(r, g, b);
                    fb.set_pixel(x as isize, y as isize, final_color);
                }
            }
        }

        fn prepare(v: &ColoredVertex) -> (f32, [f32; 2], [f32; 3]) {
            let inv_w = 1.0 / v.pos[3];
            let screen_pos = [v.pos[0] * inv_w, v.pos[1] * inv_w];
            let rgb = v.color.to_rgb_f32();
            let col_over_w = [rgb[0] * inv_w, rgb[1] * inv_w, rgb[2] * inv_w];
            (inv_w, screen_pos, col_over_w)
        }
    }
}
