use crate::{colors::Colors, framebuffer::Framebuffer};

#[allow(dead_code)]
pub struct Triangle {
    v1: [f64; 2],
    v2: [f64; 2],
    v3: [f64; 2],
    fill_color: Colors,
}

#[allow(dead_code)]
impl Triangle {
    pub fn new(v1: [f64; 2], v2: [f64; 2], v3: [f64; 2], fill_color: Colors) -> Self {
        Self {
            v1,
            v2,
            v3,
            fill_color,
        }
    }

    pub fn draw(&self, fb: &mut Framebuffer) {
        fb.draw_filled_triangle(self.v1, self.v2, self.v3, self.fill_color);
    }
}

#[allow(dead_code)]
pub struct Square {
    v1: [f64; 2],
    v2: [f64; 2],
    v3: [f64; 2],
    v4: [f64; 2],
    fill_color: Colors,
}

#[allow(dead_code)]
impl Square {
    pub fn new(v1: [f64; 2], v2: [f64; 2], v3: [f64; 2], v4: [f64; 2], fill_color: Colors) -> Self {
        Self {
            v1,
            v2,
            v3,
            v4,
            fill_color,
        }
    }

    pub fn draw(&self, fb: &mut Framebuffer) {
        fb.draw_filled_square(self.v1, self.v2, self.v3, self.v4, self.fill_color);
    }
}
