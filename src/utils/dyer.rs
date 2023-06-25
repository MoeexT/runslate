const COLOR_END: &str = "\x1b[0m";

pub trait Dye {
    fn dye(&self, color: Colors) -> String;
}

impl Dye for String {
    fn dye(&self, color: Colors) -> String {
        let c: String = color.into();
        c + &self + COLOR_END
    }
}

impl Dye for &str {
    fn dye(&self, color: Colors) -> String {
        let c: String = color.into();
        c + self + COLOR_END
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

// color map reference: https://en.wikipedia.org/wiki/ANSI_escape
impl From<Colors> for String {
    fn from(value: Colors) -> Self {
        match value {
            Colors::Black => String::from("\x1b[30m"),
            Colors::Red => String::from("\x1b[31m"),
            Colors::Green => String::from("\x1b[32m"),
            Colors::Yellow => String::from("\x1b[33m"),
            Colors::Blue => String::from("\x1b[34m"),
            Colors::Magenta => String::from("\x1b[35m"),
            Colors::Cyan => String::from("\x1b[36m"),
            Colors::White => String::from("\x1b[37m"),
            Colors::BrightBlack => String::from("\x1b[90m"),
            Colors::BrightRed => String::from("\x1b[91m"),
            Colors::BrightGreen => String::from("\x1b[92m"),
            Colors::BrightYellow => String::from("\x1b[93m"),
            Colors::BrightBlue => String::from("\x1b[94m"),
            Colors::BrightMagenta => String::from("\x1b[95m"),
            Colors::BrightCyan => String::from("\x1b[96m"),
            Colors::BrightWhite => String::from("\x1b[97m"),
        }
    }
}
