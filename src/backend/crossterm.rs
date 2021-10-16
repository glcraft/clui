
use std::io::{Stdout, Write};

use crossterm::{cursor::MoveTo, queue, style::{Print, SetBackgroundColor, SetForegroundColor}, terminal::{Clear, ClearType}};

use crate::util::{self, Position, Rect, clamp};
use crate::{style, terminal_size};
use super::{Result, Frame};

pub struct CrosstermFrame {
    screen: Rect,
    term: Stdout
}

impl CrosstermFrame {
    pub fn new() -> Self {
        let (w,h) = terminal_size();
        Self {
            screen: Rect { x: 0, y: 0, w, h },
            term: std::io::stdout(),
        }
    }
    pub fn is_in(&self, pos: Position) -> bool {
        let Position{x,y} = pos;
        x>=self.screen.x as i16 && x<(self.screen.x+self.screen.w) as i16 && 
        y>=self.screen.y as i16 && y<(self.screen.y+self.screen.h) as i16
    }
}

impl Frame for CrosstermFrame {
    
    fn get_screen(&self) -> Rect {
        self.screen.clone()
    }
    
    fn clear(&mut self)  -> Result<()> {
        queue!(self.term, Clear(ClearType::All))
    }
    fn flush(&mut self)  -> Result<()> {
        self.term.flush()
    }
    fn write_str(&mut self, pos: Position, text: &str) -> Result<()>{
        let Position{x,y} = pos;
        if y >= self.screen.y as i16 && y < (self.screen.y+self.screen.h) as i16 {
            let text = util::cut_text(text, pos.x, (self.screen.x as i16, (self.screen.x + self.screen.w) as i16));
            if text.len() > 0 {
                let x1 = clamp(self.screen.x  as i16, (self.screen.x+self.screen.w) as i16, x);
                return queue!(self.term, MoveTo(x1 as u16, y as u16), Print(text))
            }
        }
        Ok(())
    }
    fn set_text_color(&mut self, color: style::Color) -> Result<()> {
        queue!(self.term, SetForegroundColor(color.into()))
    }
    fn set_background_color(&mut self, color: style::Color) -> Result<()> {
        queue!(self.term, SetBackgroundColor(color.into()))
    }
    fn write_char(&mut self, pos: Position, ch: char) -> Result<()> {
        if self.is_in(pos) {
            queue!(self.term, MoveTo(pos.x as u16, pos.y as u16), Print(ch))
        } else {
            Ok(())
        }
    }
}

impl From<crate::style::Color> for crossterm::style::Color {
    fn from(v: crate::style::Color) -> Self {
        use style::Color::*;
        match v {
            None => crossterm::style::Color::Reset,
            Black => crossterm::style::Color::Black,
            DarkGrey => crossterm::style::Color::DarkGrey,
            Red => crossterm::style::Color::Red,
            DarkRed => crossterm::style::Color::DarkRed,
            Green => crossterm::style::Color::Green,
            DarkGreen => crossterm::style::Color::DarkGreen,
            Yellow => crossterm::style::Color::Yellow,
            DarkYellow => crossterm::style::Color::DarkYellow,
            Blue => crossterm::style::Color::Blue,
            DarkBlue => crossterm::style::Color::DarkBlue,
            Magenta => crossterm::style::Color::Magenta,
            DarkMagenta => crossterm::style::Color::DarkMagenta,
            Cyan => crossterm::style::Color::Cyan,
            DarkCyan => crossterm::style::Color::DarkCyan,
            White => crossterm::style::Color::White,
            Grey => crossterm::style::Color::Grey,
            Rgb { r, g, b } => crossterm::style::Color::Rgb{r,g,b},
            AnsiValue(v) => crossterm::style::Color::AnsiValue(v),
        }
    }
}