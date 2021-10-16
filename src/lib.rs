pub mod widget;
pub mod event_listener;
pub mod util;

use util::*;
pub mod backend;

pub fn terminal_size() -> (u16,u16) {
    use terminal_size::{Height, Width};
    terminal_size::terminal_size().and_then( |(Width(w),Height(h))| Some((w,h))).or_else(|| Some((50,25))).unwrap()
}
