use std::ops::Range;
use crate::util::{Position, cut_text_range};
use super::{Widget, Frame, util::{Rect, TuiResult, Utf8Manip}};

struct Line {
    pos_x: i16,
    text: String,
    range_txt: Range<usize>//(usize, usize)
}
impl Line {
    fn get_text<'s>(&'s self) -> &'s str {
        &self.text[self.range_txt.clone()]
    }
}

pub enum AlignmentX {
    Left,
    Center,
    Right
}
pub enum AlignmentY {
    Top,
    Middle,
    Bottom
}

pub struct Alignments(pub AlignmentX, pub AlignmentY);
impl Default for Alignments {
    fn default() -> Self {
        Self(AlignmentX::Left, AlignmentY::Top)
    }
}
pub struct Paragraph {
    pos_y: i16,
    lines: Vec<Line>,
    alignment: Alignments
    // TODO: scroll
    // TODO: spans (style)
    // TODO: 
    // TODO: 
}

impl Paragraph {
    pub fn new() -> Paragraph {
        Paragraph {
            pos_y: 0,
            lines: vec![],
            alignment: Default::default(),
        }
    }
    /// Note: new lines work only with ln, cr are removed
    pub fn with(text: &str, alignment: Alignments) -> Paragraph {
        let mut p = Paragraph::new();
        p.set_text(text);
        p.set_alignment(alignment);
        p
    }

    pub fn set_text(&mut self, text: &str) {
        self.lines = text
            .split('\n')
            .map(|s| s.chars()
                .filter(|c| *c!='\r')
                .collect::<String>()
            )
            .map(|s| {
                let slen = s.len();
                Line{ pos_x: 0, text: s, range_txt: 0..slen }
            })
            .collect();
    }
    pub fn set_alignment(&mut self, alignment: Alignments) {
        self.alignment = alignment;
    }
}

impl Widget for Paragraph {
    fn render(&self, frame: &mut Frame) -> TuiResult<()> {
        let mut y=self.pos_y;
        for line in self.lines.iter() {
            frame.write_str(Position { x: line.pos_x, y: y }, line.get_text())?;
            y+=1;
        };
        Ok(())
    }

    fn on_update_layout(&mut self, zone: Rect) {
        let Alignments(al_x, al_y) = &self.alignment;
        let (zx,zy,zw,zh) = (zone.x as i16, zone.y as i16, zone.w as i16, zone.h as i16);
        self.lines.iter_mut().for_each(|line| {
            let lilen = line.text.u8_len() as i16;
            line.pos_x = match al_x {
                AlignmentX::Left => zx,
                AlignmentX::Center => zx+(zw-lilen)/2,
                AlignmentX::Right => zx+zw-lilen-1,
            };
            line.range_txt = cut_text_range(&line.text, line.pos_x, (zx as i16, (zx+zw) as i16))
        });
        let nlines = self.lines.len() as i16;
        self.pos_y = match al_y {
            AlignmentY::Top => zy,
            AlignmentY::Middle => zy+(zh - nlines)/2,
            AlignmentY::Bottom => zy+zh-nlines,
        };
    }
}