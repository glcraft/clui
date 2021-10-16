use crate::event_listener::Event;
use crate::{backend::Frame, util::{self, Rect, TuiResult}};

mod text_scroll;
mod paragraph;
pub use text_scroll::*;
pub use paragraph::*;

pub trait Widget {
    fn render(&self, frame: &mut Frame) -> TuiResult<()>;
    /// `zone` position and dimension are **inclusive**
    fn on_update_layout(&mut self, zone: Rect);
    /// Optionnel
    fn on_event(&mut self, _event: Event) {}
}
