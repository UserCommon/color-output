pub mod color;
pub mod styled;

#[cfg(test)]
mod tests {
    use super::color::*;
    use super::styled::*;

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
            "test".to_string(),
        );
        println!("{}", styled);
    }

    #[test]
    fn styled_bg_fg_functionality() {
        let text_str = "wow";
        let text_string = "wow".to_string();
        println!(
            "{}, {}",
            text_str.bg(Color {
                r: 255,
                ..Default::default()
            }),
            text_string.fg(Color {
                g: 255,
                ..Default::default()
            })
        );

        assert_eq!(
            text_str.fg(Color::default()),
            text_string.fg(Color::default())
        );
    }
}
