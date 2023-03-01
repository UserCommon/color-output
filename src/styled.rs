use crate::color::Color;
use std::fmt::Display;

pub trait Colorize {
    fn fg(self, color: Color) -> StyledContent;
    fn bg(self, color: Color) -> StyledContent;
}

#[derive(Debug, PartialEq, Clone)]
pub struct StyledContent {
    /// StyledContent is a struct that provides colored terminal output
    /// It has content field that containing string or &str,
    /// foreground that is text color and background that is background color.
    content: String,
    foreground: Color,
    background: Color,
    // TODO style
}

impl StyledContent {
    /// Create new StyledContent struct.
    pub fn new(foreground: Color, background: Color, content: String) -> Self {
        Self {
            content,
            foreground,
            background,
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
}

impl Display for StyledContent {
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
}
