pub const CLEARV: u8 = 0b0000_0000;
pub const BOLD: u8 = 0b0000_0001;
pub const UNDERLINE: u8 = 0b0000_0010;
pub const REVERSED: u8 = 0b0000_0100;
pub const ITALIC: u8 = 0b0000_1000;
pub const BLINK: u8 = 0b0001_0000;
pub const HIDDEN: u8 = 0b0010_0000;
pub const DIMMED: u8 = 0b0100_0000;
pub const STRIKETHROUGH: u8 = 0b1000_0000;

pub const CLEAR: Style = Style(CLEARV);

static STYLES: [(u8, Styles); 8] = [
    (BOLD, Styles::Bold),
    (DIMMED, Styles::Dimmed),
    (UNDERLINE, Styles::Underline),
    (REVERSED, Styles::Reversed),
    (ITALIC, Styles::Italic),
    (BLINK, Styles::Blink),
    (HIDDEN, Styles::Hidden),
    (STRIKETHROUGH, Styles::Strikethrough),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Style(u8);

impl Style {
    pub fn contains(&self, style: Styles) -> bool {
        let style_u8 = style.to_u8();
        self.0 & style_u8 == style_u8
    }

    pub fn to_str(self) -> String {
        let styles = Styles::from_u8(self.0).unwrap_or_default();
        styles
            .iter()
            .map(|s| s.to_str())
            .collect::<Vec<&str>>()
            .join(";")
    }

    pub fn add(&mut self, style: Styles) {
        self.0 |= style.to_u8();
    }
}

impl Default for Style {
    fn default() -> Self {
        Self(Styles::Clear.to_u8())
    }
}

// impl Default for Style {
//     fn default() -> Self {
//         Style {}
//     }
// }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Styles {
    Clear,
    Bold,
    Dimmed,
    Underline,
    Reversed,
    Italic,
    Blink,
    Hidden,
    Strikethrough,
}

impl Styles {
    pub fn to_u8(self) -> u8 {
        use Styles::*;
        match self {
            Clear => CLEARV,
            Bold => BOLD,
            Underline => UNDERLINE,
            Reversed => REVERSED,
            Italic => ITALIC,
            Blink => BLINK,
            Hidden => HIDDEN,
            Dimmed => DIMMED,
            Strikethrough => STRIKETHROUGH,
        }
    }

    pub fn to_str<'a>(self) -> &'a str {
        use Styles::*;
        match self {
            Clear => "",
            Bold => "1",
            Dimmed => "2",
            Italic => "3",
            Underline => "4",
            Blink => "5",
            Reversed => "7",
            Hidden => "8",
            Strikethrough => "9",
        }
    }

    pub fn from_u8(byte: u8) -> Option<Vec<Styles>> {
        // just copied from Colored crate sources sry.
        if byte == Styles::Clear.to_u8() {
            return None;
        }

        let res: Vec<Styles> = STYLES
            .iter()
            .filter(|(mask, _)| (0 != (byte & mask)))
            .map(|&(_, value)| value)
            .collect();

        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn print_styled() {
        println!("\x1b[1m{}\u{001b}[0m", "TEST");
    }
}
