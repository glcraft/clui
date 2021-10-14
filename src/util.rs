use std::ops::Range;

#[derive(Debug,Clone, Copy, Default)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}
impl Position {
    pub fn to_tuple(&self) -> (i16, i16) {
        (self.x, self.y)
    }
}

#[derive(Debug,Clone, Default)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

#[inline]
pub fn min<T: PartialOrd>(v1: T, v2: T) -> T {
    if v1<v2 {v1} else {v2}
}
#[inline]
pub fn max<T: PartialOrd>(v1: T, v2: T) -> T {
    if v1>v2 {v1} else {v2}
}
#[inline]
pub fn clamp<T: PartialOrd>(v_min: T, v_max: T, x: T) -> T {
    max(v_min, min(v_max, x))
}

pub fn cut_text_range(text: &str, pos: i16, borders: (i16, i16)) -> Range<usize> {
    let offset = (
        borders.0 - pos,
        borders.1 - 1 - pos,
    );
    let text_offset = (clamp(0, text.len() as i16, offset.0) as usize, clamp(0, text.len() as i16, offset.1) as usize);
    char_to_utf8_index(text.as_bytes(), text_offset.0)..char_to_utf8_index(text.as_bytes(), text_offset.1)
}
pub fn cut_text<'a>(text: &'a str, pos: i16, borders: (i16, i16)) -> &'a str {
    
    &text[cut_text_range(text, pos, borders)]
}
pub fn utf8_len(c: u8) -> usize {
    if c&0x80 == 0 {1}
    else if c&0xE0 == 0xC0 {2}
    else if c&0xF0 == 0xE0 {3}
    else if c&0xF8 == 0xF0 {4}
    else {panic!("not utf8 front char")}
}
pub fn char_to_utf8_index(s:&[u8], mut ind: usize) -> usize{
    let len = s.len();
    let mut i=0;
    while i < len && ind > 0  {
        i+=utf8_len(s[i]);
        ind-=1;
    }
    i
}
use std::ops::Deref;
pub type TuiResult<T> = Result<T, std::io::Error>;

pub trait Utf8Manip 
    where Self: Deref<Target=str>
{
    #[inline]
    fn u8_len(&self) -> usize {
        self.chars().count()
    }
    fn u8_index(&self, mut ind: usize) -> usize {
        let s = self.as_bytes();
        let len = s.len();
        let mut i=0;
        while i < len && ind > 0  {
            i+=utf8_len(s[i]);
            ind-=1;
        }
        i
    }
}

impl<S: Deref<Target=str>> Utf8Manip for S 
{}