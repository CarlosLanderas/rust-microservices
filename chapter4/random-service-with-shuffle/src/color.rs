#[derive(Clone, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub const WHITE: Color = Color { red: 0xFF, green: 0xFF, blue: 0xFF};
pub const BLACK: Color = Color { red: 0x00, green: 0x00, blue: 0x00};


