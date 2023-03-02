use crate::color::Color;
use crate::style::*;
use std::fmt::Display;

pub trait Colorize {
    fn fg(self, color: Color) -> StyledContent;
    fn bg(self, color: Color) -> StyledContent;

    fn clear(self) -> StyledContent;
    fn bold(self) -> StyledContent;
    fn normal(self) -> StyledContent;
    fn dimmed(self) -> StyledContent;
    fn italic(self) -> StyledContent;
    fn underline(self) -> StyledContent;
    fn blink(self) -> StyledContent;
    fn reverse(self) -> StyledContent;
    fn reversed(self) -> StyledContent;
    fn hidden(self) -> StyledContent;
    fn strikethrough(self) -> StyledContent;
}

#[derive(Debug, PartialEq, Clone)]
pub struct StyledContent {
    /// StyledContent is a struct that provides colored terminal output
    /// It has content field that containing string or &str,
    /// foreground that is text color and background that is background color.
    pub content: String,
    pub foreground: Color,
    pub background: Color,
    pub style: Style,
}

impl StyledContent {
    /// Create new StyledContent struct.
    pub fn new(foreground: Color, background: Color, content: String, style: Style) -> Self {
        Self {
            content,
            foreground,
            background,
            style,
        }
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    pub fn get_fg(&self) -> Color {
        self.foreground.clone()
    }

    pub fn get_bg(&self) -> Color {
        self.background.clone()
    }

    pub fn get_style(&self) -> Style {
        self.style.clone()
    }

    pub fn styled_content(&self) -> String {
        format!("\x1b[{}m{}", self.style.to_str(), self.content)
    }
}

impl Display for StyledContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\x1b[38;2;{r_f};{g_f};{b_f}m\x1b[48;2;{r_b};{g_b};{b_b}m{content}\u{001b}[0m",
            r_b = self.background.r,
            g_b = self.background.g,
            b_b = self.background.b,
            r_f = self.foreground.r,
            g_f = self.foreground.g,
            b_f = self.foreground.b,
            content = self.styled_content()
        )
    }
}

impl Default for StyledContent {
    fn default() -> Self {
        Self {
            content: String::default(),
            foreground: Color {
                r: 255,
                g: 255,
                b: 255,
            },
            background: Color { r: 0, g: 0, b: 0 },
            style: Style::default(),
        }
    }
}

impl<'a> From<&'a str> for StyledContent {
    fn from(value: &'a str) -> Self {
        Self {
            content: String::from(value),
            ..StyledContent::default()
        }
    }
}

impl Colorize for StyledContent {
    fn bg(mut self, color: Color) -> StyledContent {
        self.background = color;
        self
    }

    fn fg(mut self, color: Color) -> StyledContent {
        self.foreground = color;
        self
    }

    fn clear(mut self) -> StyledContent {
        Self {
            content: self.content,
            ..Default::default()
        }
    }

    fn bold(mut self) -> StyledContent {
        self.style.add(Styles::Bold);
        self
    }

    fn normal(mut self) -> StyledContent {
        self.clear()
    }

    fn dimmed(mut self) -> StyledContent {
        self.style.add(Styles::Dimmed);
        self
    }

    fn italic(mut self) -> StyledContent {
        self.style.add(Styles::Italic);
        self
    }

    fn underline(mut self) -> StyledContent {
        self.style.add(Styles::Underline);
        self
    }

    fn blink(mut self) -> StyledContent {
        self.style.add(Styles::Blink);
        self
    }

    fn reverse(mut self) -> StyledContent {
        self.reversed()
    }

    fn reversed(mut self) -> StyledContent {
        self.style.add(Styles::Reversed);
        self
    }

    fn hidden(mut self) -> StyledContent {
        self.style.add(Styles::Hidden);
        self
    }

    fn strikethrough(mut self) -> StyledContent {
        self.style.add(Styles::Strikethrough);
        self
    }
}

impl<'a> Colorize for &'a str {
    fn fg(self, color: Color) -> StyledContent {
        StyledContent {
            content: String::from(self),
            foreground: color,
            ..Default::default()
        }
    }

    fn bg(self, color: Color) -> StyledContent {
        StyledContent {
            content: String::from(self),
            background: color,
            ..Default::default()
        }
    }

    fn clear(self) -> StyledContent {
        StyledContent {
            content: String::from(self),
            style: CLEAR,
            ..Default::default()
        }
    }

    fn bold(self) -> StyledContent {
        StyledContent::from(self).bold()
    }

    fn normal(self) -> StyledContent {
        self.clear()
    }

    fn dimmed(self) -> StyledContent {
        StyledContent::from(self).dimmed()
    }

    fn italic(self) -> StyledContent {
        StyledContent::from(self).italic()
    }

    fn underline(self) -> StyledContent {
        StyledContent::from(self).underline()
    }

    fn blink(self) -> StyledContent {
        StyledContent::from(self).blink()
    }

    fn reverse(self) -> StyledContent {
        StyledContent::from(self).reverse()
    }

    fn reversed(self) -> StyledContent {
        StyledContent::from(self).reversed()
    }

    fn hidden(self) -> StyledContent {
        StyledContent::from(self).hidden()
    }

    fn strikethrough(self) -> StyledContent {
        StyledContent::from(self).strikethrough()
    }
}

#[cfg(test)]
mod test {
    use super::{Color, Colorize, Style, StyledContent};
    #[test]
    fn print_styled() {
        let styled = StyledContent::new(
            Color { r: 255, g: 0, b: 0 },
            Color { r: 0, g: 0, b: 200 },
            "test".to_string(),
            Style::default(),
        );
        println!("{}", styled);
    }

    #[test]
    fn styled_bg_fg_functionality() {
        let text_str = "wow";
        let text_string = "wow".to_string();
        let mut st_ct = StyledContent::default();
        st_ct.style.add(crate::style::Styles::Underline);
        st_ct.style.add(crate::style::Styles::Bold);
        st_ct.content = String::from("TESTTT");
        println!(
            "{}, {}, {}",
            text_str.bg(Color {
                r: 255,
                ..Default::default()
            }),
            text_string.fg(Color {
                g: 255,
                ..Default::default()
            }),
            st_ct.fg(Color { r: 0, g: 0, b: 255 })
        );

        assert_eq!(
            text_str.fg(Color::default()),
            text_string.fg(Color::default())
        );
    }

    #[test]
    fn testy_style() {
        let mut test = "testy"
            .fg(Color {
                r: 255,
                b: 120,
                g: 0,
            })
            .bold()
            .strikethrough()
            .dimmed()
            .underline();

        println!("{}", test);
        println!("{}", test.clear());
    }
}
