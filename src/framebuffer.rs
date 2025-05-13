use crate::colors::Colors;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

#[allow(dead_code)]
impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn clear(&mut self, color: Colors) {
        self.buffer.fill(color.as_u32());
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, color: Colors) {
        if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
            self.buffer[y as usize * self.width + x as usize] = color.as_u32();
        }
    }

    pub fn draw_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, color: Colors) {
        let mut x = x0;
        let mut y = y0;
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1.0 } else { -1.0 };
        let sy = if y0 < y1 { 1.0 } else { -1.0 };
        let mut err = dx + dy;

        loop {
            self.set_pixel(x.round() as isize, y.round() as isize, color);
            if (x - x1).abs() < f64::EPSILON && (y - y1).abs() < f64::EPSILON {
                break;
            }
            let e2 = 2.0 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_filled_triangle(
        &mut self,
        v1: [f64; 2],
        v2: [f64; 2],
        v3: [f64; 2],
        color: Colors,
    ) {
        // Compute bounding box of the triangle
        let min_x = v1[0].min(v2[0]).min(v3[0]).floor().max(0.0) as usize;
        let max_x = v1[0]
            .max(v2[0])
            .max(v3[0])
            .ceil()
            .min(self.width as f64 - 1.0) as usize;
        let min_y = v1[1].min(v2[1]).min(v3[1]).floor().max(0.0) as usize;
        let max_y = v1[1]
            .max(v2[1])
            .max(v3[1])
            .ceil()
            .min(self.height as f64 - 1.0) as usize;

        // Helper to compute edge function
        fn edge(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> f64 {
            (c[0] - a[0]) * (b[1] - a[1]) - (c[1] - a[1]) * (b[0] - a[0])
        }

        // Precompute area of the triangle (used for barycentric coordinates)
        let area = edge(v1, v2, v3);

        // Loop through bounding box
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = [x as f64 + 0.5, y as f64 + 0.5]; // Pixel center

                // Compute barycentric weights
                let w0 = edge(v2, v3, p);
                let w1 = edge(v3, v1, p);
                let w2 = edge(v1, v2, p);

                // If inside the triangle (all weights same sign as area)
                if (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 && area > 0.0)
                    || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0 && area < 0.0)
                {
                    self.set_pixel(x as isize, y as isize, color);
                }
            }
        }
    }
}
