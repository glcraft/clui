use std::io::{Stdout, Write};

use crossterm::{cursor::MoveTo, queue, style::Print, terminal::{Clear, ClearType}};

pub mod widget;
pub mod event_listener;
pub mod util;

use util::*;

pub fn terminal_size() -> (u16,u16) {
    use terminal_size::{Height, Width};
    terminal_size::terminal_size().and_then( |(Width(w),Height(h))| Some((w,h))).or_else(|| Some((50,25))).unwrap()
}

pub struct Frame {
    screen: Rect,
    term: Stdout
}

impl Frame {
    pub fn new() -> Self {
        let (w,h) = terminal_size();
        Self {
            screen: Rect { x: 0, y: 0, w, h },
            term: std::io::stdout(),
        }
    }
    pub fn get_screen(&self) -> Rect {
        self.screen.clone()
    }
    pub fn is_in(&self, pos: Position) -> bool {
        let Position{x,y} = pos;
        x>=self.screen.x as i16 && x<(self.screen.x+self.screen.w) as i16 && 
        y>=self.screen.y as i16 && y<(self.screen.y+self.screen.h) as i16
    }
    pub fn clear(&mut self)  -> Result<(), std::io::Error> {
        queue!(self.term, Clear(ClearType::All))
    }
    pub fn flush(&mut self)  -> Result<(), std::io::Error> {
        self.term.flush()
    }
    pub fn write_str(&mut self, pos: Position, text: &str) -> Result<(), std::io::Error>{
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
    pub fn write_char(&mut self, pos: Position, ch: char) -> Result<(), std::io::Error> {
        if self.is_in(pos) {
            queue!(self.term, MoveTo(pos.x as u16, pos.y as u16), Print(ch))
        } else {
            Ok(())
        }
    }
}