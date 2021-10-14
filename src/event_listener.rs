use bitflags::bitflags;
/// Based on crossterm table https://docs.rs/crossterm/0.21.0/crossterm/event/enum.KeyCode.html
pub enum KeyCodes {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Null,
    Esc,
}

bitflags! {
    /// Based on https://docs.rs/crossterm/0.21.0/src/crossterm/event.rs.html#376-385
    pub struct KeyModifiers: u8 {
        const SHIFT = 0b0000_0001;
        const CONTROL = 0b0000_0010;
        const ALT = 0b0000_0100;
        const NONE = 0b0000_0000;
    }
}


pub enum Event {
    OnKey{
        key: KeyCodes,
        modifiers: KeyModifiers
    },
    OnChar(char),
    OnMouseClick {
        position: (u16,u16)
    }
}

pub trait EventListener {
    fn on_event(event: &Event);
}