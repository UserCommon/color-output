use crate::color::Color;
use std::{
    borrow::{Cow, ToOwned},
    fmt::Display,
    io::Write,
};

#[derive(PartialEq, Clone)]
pub struct StyledContent<'a> {
    /// StyledContent is a struct that provides colored terminal output
    /// It has content field that containing string or &str,
    /// foreground that is text color and background that is background color.
    pub content: Cow<'a, str>,
    pub foreground: Color,
    pub background: Color,
}

impl<'a> StyledContent<'a> {
    /// Create new StyledContent struct.
    pub fn new(foreground: Color, background: Color, content: Cow<'a, str>) -> Self {
        Self {
            content,
            foreground,
            background,
        }
    }

    /// Set background for StyledContent
    pub fn bg(&mut self, bg: Color) {
        self.background = bg;
    }

    /// Set foreground for StyledContent
    pub fn fg(&mut self, fg: Color) {
        self.foreground = fg;
    }

    /// Set content for StyledContent
    pub fn content(&mut self, content: Cow<'a, str>) {
        self.content = content;
    }
}

impl<'a> Display for StyledContent<'a> {
    //"\x1b[48;2;255;0;0mtext\u{001b}[0ming")
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\x1b[38;2;{r_f};{g_f};{b_f}m\x1b[48;2;{r_b};{g_b};{b_b}m{content}\u{001b}[0m\u{001b}[0m",
            r_b = self.background.r,
            g_b = self.background.g,
            b_b = self.background.b,
            r_f = self.foreground.r,
            g_f = self.foreground.g,
            b_f = self.foreground.b,
            content = self.content.to_string()
        )
    }
}

impl<'a> Default for StyledContent<'a> {
    fn default() -> Self {
        Self {
            content: Cow::Owned("".to_string()),
            foreground: Color {
                r: 255,
                g: 255,
                b: 255,
            },
            background: Color { r: 0, g: 0, b: 0 },
        }
    }
}
