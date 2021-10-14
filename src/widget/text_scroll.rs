use std::{convert::{Infallible, TryInto}, str::FromStr};
use super::{Widget, Frame, util::{self, Position, Rect, TuiResult, cut_text}};

#[derive(Debug, Clone, Default)]
pub struct TextScroll {
    text: String,
    text_length: usize,
    pos: Position,
    borders:(u16,u16),
    reverse: bool
}

impl TextScroll {
    pub fn new() -> Self { 
        Default::default()
    }
    fn limit_min_x(&self) -> i16{
        -(self.text_length as i16)
    }
    pub fn reset_pos(&mut self) {
        self.pos.x = -(self.text.len() as i16);
    }
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.text_length = self.text.chars().count();
    }
}
impl From<String> for TextScroll {
    fn from(s: String) -> Self {
        let mut t = TextScroll::new();
        t.set_text(s);
        t
    }
}
impl FromStr for TextScroll {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut t = TextScroll::new();
        let s = s.try_into()?;
        t.set_text(s);
        Ok(t)
    }
}

impl Widget for TextScroll {
    fn render(&self, frame: &mut Frame) -> TuiResult<()> {
        let pos = Position {
            x: util::clamp(self.borders.0 as i16, self.borders.1 as i16, self.pos.x),
            y: self.pos.y,
        };
        frame.write_str(pos, cut_text(&self.text, self.pos.x, (self.borders.0 as i16, self.borders.1 as i16)))
    }

    fn on_update_layout(&mut self, zone: Rect) {
        self.pos.x-=1;
        if self.pos.x<zone.x as i16+self.limit_min_x() {
            self.pos.x = (zone.x+zone.w) as i16;
        }
        self.pos.y = (zone.y+zone.h/2) as i16;
        self.borders = (zone.x,zone.x+zone.w);
    }
}