mod color;
mod styled;

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn print_styled() {
        let styled = StyledContent::new(
            Color { r: 255, g: 0, b: 0 },
            Color { r: 0, g: 0, b: 200 },
            Cow::Borrowed("test"),
        );
        println!("{}", styled);
    }

    #[test]
    fn styled_bg_fg_functionality() {
        let mut styled = StyledContent::default();
        styled.bg(Color { r: 255, g: 0, b: 0 });
        styled.content(Cow::Borrowed("TEST"));
        println!("{}", styled);
    }
}
