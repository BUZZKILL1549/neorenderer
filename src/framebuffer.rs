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
}
