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

    // shits done using smth called Barycentric Coordinates
    pub fn draw_filled_triangle(
        &mut self,
        v1: [f64; 2],
        v2: [f64; 2],
        v3: [f64; 2],
        color: Colors,
    ) {
        // finding bounding box
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

        // snippet to find the edges
        fn edge(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> f64 {
            (c[0] - a[0]) * (b[1] - a[1]) - (c[1] - a[1]) * (b[0] - a[0])
        }

        // precomputing the area of the triangle: this is where the real magic happens for filling
        // in the triangle
        let area = edge(v1, v2, v3);

        // looping through the bounding box and painting one line at a time
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = [x as f64 + 0.5, y as f64 + 0.5]; // the pixel center 

                // compute barycentric weights
                let w0 = edge(v2, v3, p);
                let w1 = edge(v3, v1, p);
                let w2 = edge(v1, v2, p);

                // if the coordinates are inside the triangle (if all the weights are the same
                // as the area) then u do this
                if (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 && area > 0.0)
                    || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0 && area < 0.0)
                {
                    self.set_pixel(x as isize, y as isize, color);
                }
            }
        }
    }

    // i wont make a specific square rasterizer because GPU rendering generally splits polygons
    // into triangles anyway so ill make a general polygon rasterizer that splits itself into
    // multiple triangles
    // nvm i just did that lmfao
    pub fn draw_filled_square(
        &mut self,
        v1: [f64; 2],
        v2: [f64; 2],
        v3: [f64; 2],
        v4: [f64; 2],
        color: Colors,
    ) {
        let triangle1 = [v1, v2, v4];
        let triangle2 = [v2, v3, v4];

        // bro im so dumb i coulda just done this from the start instead of typing all that code
        self.draw_filled_triangle(triangle1[0], triangle1[1], triangle1[2], color);
        self.draw_filled_triangle(triangle2[0], triangle2[1], triangle2[2], color);

        /*
        // edge function
        fn edge(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> f64 {
            (c[0] - a[0]) * (b[1] - a[1]) - (c[1] - a[1]) * (b[0] - a[0])
        }

        // rendering triangle1
        let t1_min_x = v1[0].min(v2[0]).min(v4[0]).floor().max(0.0) as usize;
        let t1_max_x = v1[0]
            .max(v2[0])
            .max(v4[0])
            .ceil()
            .min(self.width as f64 - 1.0) as usize;
        let t1_min_y = v1[1].min(v2[1]).min(v4[1]).floor().max(0.0) as usize;
        let t1_max_y = v1[1]
            .max(v2[1])
            .max(v4[1])
            .ceil()
            .min(self.height as f64 - 1.0) as usize;

        let area1 = edge(v1, v2, v4);

        for y in t1_min_y..=t1_max_y {
            for x in t1_min_x..=t1_max_x {
                let p = [x as f64 + 0.5, y as f64 + 0.5];

                let w0 = edge(v2, v4, p);
                let w1 = edge(v4, v1, p);
                let w2 = edge(v1, v2, p);

                if (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 && area1 > 0.0)
                    || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0 && area1 < 0.0)
                {
                    self.set_pixel(x as isize, y as isize, color);
                }
            }
        }

        // rendering triangle2
        let t2_min_x = v2[0].min(v3[0]).min(v4[0]).floor().max(0.0) as usize;
        let t2_max_x = v2[0]
            .max(v3[0])
            .max(v4[0])
            .ceil()
            .min(self.width as f64 - 1.0) as usize;
        let t2_min_y = v2[1].min(v3[1]).min(v4[1]).floor().max(0.0) as usize;
        let t2_max_y = v2[1]
            .max(v3[1])
            .max(v4[1])
            .ceil()
            .min(self.height as f64 - 1.0) as usize;

        let area2 = edge(v2, v3, v4);

        for y in t2_min_y..=t2_max_y {
            for x in t2_min_x..=t2_max_x {
                let p = [x as f64 + 0.5, y as f64 + 0.5];

                let w0 = edge(v2, v3, p);
                let w1 = edge(v3, v4, p);
                let w2 = edge(v4, v2, p);

                if (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 && area2 > 0.0)
                    || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0 && area2 < 0.0)
                {
                    self.set_pixel(x as isize, y as isize, color);
                }
            }
        }
        */
    }
}
