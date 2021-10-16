#[derive(Debug,Clone)]
pub enum Color {
    None,
    Black,
    DarkGrey,
    Red,
    DarkRed,
    Green,
    DarkGreen,
    Yellow,
    DarkYellow,
    Blue,
    DarkBlue,
    Magenta,
    DarkMagenta,
    Cyan,
    DarkCyan,
    White,
    Grey,
    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
    AnsiValue(u8),
}

pub struct Text {
    text: String,
    front_color: Color,
    back_color: Color,
}

impl Default for Text {
    fn default() -> Self {
        Self { 
            text: Default::default(), 
            front_color: Color::None, 
            back_color: Color::None
        }
    }
}