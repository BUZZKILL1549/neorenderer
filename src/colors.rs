#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Colors {
    value: u32,
}

#[allow(dead_code)]
impl Colors {
    pub const RED: Colors = Colors { value: 0xFF0000 };
    pub const GREEN: Colors = Colors { value: 0x00FF00 };
    pub const BLUE: Colors = Colors { value: 0x0000FF };
    pub const BLACK: Colors = Colors { value: 0x000000 };
    pub const WHITE: Colors = Colors { value: 0xFFFFFF };

    pub fn from_rgb_f32(r: f32, g: f32, b: f32) -> Colors {
        let r_u8 = (r.clamp(0.0, 1.0) * 255.0).round() as u8;
        let g_u8 = (g.clamp(0.0, 1.0) * 255.0).round() as u8;
        let b_u8 = (b.clamp(0.0, 1.0) * 255.0).round() as u8;

        let value = ((r_u8 as u32) << 16) | ((g_u8 as u32) << 8) | (b_u8 as u32);
        Colors { value }
    }

    pub fn to_rgb_f32(&self) -> [f32; 3] {
        let r = ((self.value >> 16) & 0xFF) as f32 / 255.0;
        let g = ((self.value >> 8) & 0xFF) as f32 / 255.0;
        let b = (self.value & 0xFF) as f32 / 255.0;
        [r, g, b]
    }

    pub fn as_u32(&self) -> u32 {
        self.value
    }
}
