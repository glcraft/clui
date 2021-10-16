mod crossterm;
pub use self::crossterm::*;

use crate::util::{Rect, Position};
use crate::style;
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Frame {
    fn get_screen(&self) -> Rect;
    fn clear(&mut self)  -> Result<()> ;
    fn flush(&mut self)  -> Result<()> ;
    fn set_text_color(&mut self, color: style::Color) -> Result<()>;
    fn set_background_color(&mut self, color: style::Color) -> Result<()>;
    fn write_str(&mut self, pos: Position, text: &str) -> Result<()>;
    fn write_char(&mut self, pos: Position, ch: char) -> Result<()> ;
}