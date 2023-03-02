pub mod color;
pub mod style;
pub mod styled;

#[cfg(test)]
mod tests {
    use super::color::*;

    #[test]
    fn create_color() {
        let color_1 = Color::new(255, 120, 5);
        assert_eq!(
            color_1,
            Color {
                r: 255,
                g: 120,
                b: 5
            }
        );
    }
}
