use crate::{colors::Colors, framebuffer::Framebuffer};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct ColoredVertex {
    pub pos: [f64; 2],
    pub color: Colors,
}

#[allow(dead_code)]
impl ColoredVertex {
    pub fn draw_gourad_triangle(
        fb: &mut Framebuffer,
        v0: ColoredVertex,
        v1: ColoredVertex,
        v2: ColoredVertex,
    ) {
        fn edge(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> f64 {
            (c[0] - a[0]) * (b[1] - a[1]) - (c[1] - a[1]) * (b[0] - a[0])
        }

        let min_x = v0.pos[0].min(v1.pos[0]).min(v2.pos[0]).floor().max(0.0) as usize;
        let max_x = v0.pos[0]
            .max(v1.pos[0])
            .max(v2.pos[0])
            .ceil()
            .min(fb.width as f64 - 1.0) as usize;
        let min_y = v0.pos[1].min(v1.pos[1]).min(v2.pos[1]).floor().max(0.0) as usize;
        let max_y = v0.pos[1]
            .max(v1.pos[1])
            .max(v2.pos[1])
            .ceil()
            .min(fb.height as f64 - 1.0) as usize;

        let area = edge(v0.pos, v1.pos, v2.pos);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = [x as f64 + 0.5, y as f64 + 0.5];

                let w0 = edge(v1.pos, v2.pos, p);
                let w1 = edge(v2.pos, v0.pos, p);
                let w2 = edge(v0.pos, v1.pos, p);

                let inside = if area > 0.0 {
                    w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
                } else {
                    w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0
                };

                if inside {
                    let w0_n = w0 / area;
                    let w1_n = w1 / area;
                    let w2_n = w2 / area;

                    let c0 = v0.color.to_rgb_f32();
                    let c1 = v1.color.to_rgb_f32();
                    let c2 = v2.color.to_rgb_f32();

                    let r = w0_n * c0[0] as f64 + w1_n * c1[0] as f64 + w2_n * c2[0] as f64;
                    let g = w0_n * c0[1] as f64 + w1_n * c1[1] as f64 + w2_n * c2[1] as f64;
                    let b = w0_n * c0[2] as f64 + w1_n * c1[2] as f64 + w2_n * c2[2] as f64;

                    let color = Colors::from_rgb_f32(r as f32, g as f32, b as f32);
                    fb.set_pixel(x as isize, y as isize, color);
                }
            }
        }
    }
}
