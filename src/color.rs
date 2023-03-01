#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    /// Color structure that contains red green and blue colors from 0 to 255 value.
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create Color struct instance
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
