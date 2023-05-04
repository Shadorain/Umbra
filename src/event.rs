use bitflags::bitflags;

use super::{Point, Size};

/// # Umbra Events
///
/// All events that can happen while running Umbra.
pub enum UEvent {
    Input(IEvent),
    Screen(SEvent),
}

/// Screen events
pub enum SEvent {}

/// Input events
pub enum IEvent {
    Key(KeyModifiers, Key),
    Mouse(Point),
    Resize(Size),
    Paste(String),

    FocusGained,
    FocusLost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    F(u8),
    Backspace,
    Delete,
    Left,
    Down,
    Up,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Enter,
    Insert,
    Esc,
    Null,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    Menu,
    KeypadBegin,
    Media(MediaKey),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKey {
    Next,
    Previous,
    FastForward,
    Rewind,
    Stop,
    PlayPause,
    VolumeMute,
    VolumeUp,
    VolumeDown,
    BrightnessDown,
    BrightnessUp,
    KbdIllumToggle,
    KbdIllumDown,
    KbdIllumUp,
}

bitflags! {
    /// KeyModifiers
    ///
    /// Taken from crossterm
    /// [crossterm::KeyModifiers](https://docs.rs/crossterm/latest/crossterm/event/struct.KeyModifiers.html)
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct KeyModifiers: u8 {
        const SHIFT = 0b0000_0001;
        const CONTROL = 0b0000_0010;
        const ALT = 0b0000_0100;
        const SUPER = 0b0000_1000;
        const HYPER = 0b0001_0000;
        const META = 0b0010_0000;
        const NONE = 0b0000_0000;
    }
}

impl Default for KeyModifiers {
    fn default() -> Self {
        KeyModifiers::NONE
    }
}
