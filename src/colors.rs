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

    pub fn as_u32(&self) -> u32 {
        self.value
    }
}
