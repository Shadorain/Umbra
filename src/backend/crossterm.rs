pub use crossterm::cursor::CursorShape;
use crossterm::{cursor as c, event as e, queue, style as s, terminal as t};
use std::io::{BufWriter, Stdout, Write};

use crate::event::{KeyModifiers, MediaKey};
use crate::screen::{Point, Size};
use crate::{IEvent, Key};

use super::{BResult, Backend};

pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl From<Rgb> for s::Color {
    fn from(rgb: Rgb) -> s::Color {
        let Rgb { r, g, b } = rgb;
        s::Color::Rgb { r, g, b }
    }
}

impl From<e::KeyModifiers> for KeyModifiers {
    fn from(modifiers: e::KeyModifiers) -> KeyModifiers {
        KeyModifiers::from_bits(modifiers.bits()).unwrap_or_default()
    }
}
impl From<e::KeyCode> for Key {
    fn from(key: e::KeyCode) -> Key {
        match key {
            e::KeyCode::Char(k) => Key::Char(k),
            e::KeyCode::F(n) => Key::F(n),
            e::KeyCode::Backspace => Key::Backspace,
            e::KeyCode::Delete => Key::Delete,
            e::KeyCode::Left => Key::Left,
            e::KeyCode::Down => Key::Down,
            e::KeyCode::Up => Key::Up,
            e::KeyCode::Right => Key::Right,
            e::KeyCode::Home => Key::Home,
            e::KeyCode::End => Key::End,
            e::KeyCode::PageUp => Key::PageUp,
            e::KeyCode::PageDown => Key::PageDown,
            e::KeyCode::Tab => Key::Tab,
            e::KeyCode::BackTab => Key::BackTab,
            e::KeyCode::Enter => Key::Enter,
            e::KeyCode::Insert => Key::Insert,
            e::KeyCode::Esc => Key::Esc,
            e::KeyCode::Null => Key::Null,
            e::KeyCode::CapsLock => Key::CapsLock,
            e::KeyCode::ScrollLock => Key::ScrollLock,
            e::KeyCode::NumLock => Key::NumLock,
            e::KeyCode::PrintScreen => Key::PrintScreen,
            e::KeyCode::Pause => Key::Pause,
            e::KeyCode::Menu => Key::Menu,
            e::KeyCode::KeypadBegin => Key::KeypadBegin,
            e::KeyCode::Media(m) => Key::Media(m.into()),
            e::KeyCode::Modifier(_) => Key::Null,
        }
    }
}
impl From<e::MediaKeyCode> for MediaKey {
    fn from(media: e::MediaKeyCode) -> MediaKey {
        match media {
            e::MediaKeyCode::TrackNext => MediaKey::Next,
            e::MediaKeyCode::TrackPrevious => MediaKey::Previous,
            e::MediaKeyCode::FastForward => MediaKey::FastForward,
            e::MediaKeyCode::Rewind => MediaKey::Rewind,
            e::MediaKeyCode::Stop => MediaKey::Stop,
            e::MediaKeyCode::PlayPause => MediaKey::PlayPause,
            e::MediaKeyCode::MuteVolume => MediaKey::VolumeMute,
            e::MediaKeyCode::RaiseVolume => MediaKey::VolumeUp,
            e::MediaKeyCode::LowerVolume => MediaKey::VolumeDown,
            _ => unimplemented!(),
        }
    }
}

pub struct CrosstermBackend {
    buffer: BufWriter<Stdout>,
}

#[allow(unused)]
impl CrosstermBackend {
    pub fn new() -> Self {
        CrosstermBackend {
            buffer: BufWriter::new(std::io::stdout()),
        }
    }

    pub fn init() -> BResult<()> {
        queue!(std::io::stdout(), t::EnterAlternateScreen, s::ResetColor, t::Clear(t::ClearType::All))?;
        t::enable_raw_mode()?;
        Ok(())
    }

    /// Queues the current line to be cleared
    pub fn clear_line(&mut self) -> BResult<()> {
        Ok(queue!(self.buffer, t::Clear(t::ClearType::CurrentLine))?)
    }

    pub fn fg_set(&mut self, rgb: Rgb) -> BResult<()> {
        Ok(queue!(self.buffer, s::SetForegroundColor(rgb.into()))?)
    }
    pub fn bg_set(&mut self, rgb: Rgb) -> BResult<()> {
        Ok(queue!(self.buffer, s::SetBackgroundColor(rgb.into()))?)
    }
    #[allow(unused)]
    pub fn fg_reset(&mut self) -> BResult<()> {
        Ok(queue!(self.buffer, s::SetForegroundColor(s::Color::Reset))?)
    }
    #[allow(unused)]
    pub fn bg_reset(&mut self) -> BResult<()> {
        Ok(queue!(self.buffer, s::SetBackgroundColor(s::Color::Reset))?)
    }
    #[allow(unused)]
    pub fn color_reset(&mut self) -> BResult<()> {
        Ok(queue!(self.buffer, s::ResetColor)?)
    }
}

impl Drop for CrosstermBackend {
    /// If the backend is dropped, we want to make sure that we leave the alternate screen
    /// Will panic if fails
    fn drop(&mut self) {
        queue!(self.buffer, s::ResetColor, c::Show, t::LeaveAlternateScreen).unwrap();
        self.buffer.flush().unwrap();
        t::disable_raw_mode().unwrap();
    }
}

impl Backend for CrosstermBackend {
    /// Reads and returns an Event from the terminal
    /// Handles resize events automatically
    /// NOTE: Handles Mouse and Resize events from here
    fn read_event(&mut self) -> BResult<IEvent> {
        Ok(match e::read()? {
            e::Event::Key(key) => IEvent::Key(key.modifiers.into(), key.code.into()),
            e::Event::Mouse(mouse) => IEvent::Mouse(Point::from((mouse.column, mouse.row))),
            e::Event::Resize(x, y) => IEvent::Resize((x, y).into()),
            e::Event::Paste(s) => IEvent::Paste(s),
            e::Event::FocusGained => IEvent::FocusGained,
            e::Event::FocusLost => IEvent::FocusLost,
        })
    }

    /// Refreshes the screen
    fn refresh(&mut self) -> BResult<()> {
        Ok(self.buffer.flush()?)
    }

    /// Gets the current size of the screen
    fn screen_size(&mut self) -> Size {
        t::size().unwrap_or_default().into()
    }

    /// Queues the screen to be cleared
    fn screen_clear(&mut self) -> BResult<()> {
        Ok(queue!(self.buffer, t::Clear(t::ClearType::All))?)
    }

    /// Queues a cursor hide command
    fn cursor_hide(&mut self) -> BResult<()> {
        Ok(queue!(self.buffer, c::Hide)?)
    }
    /// Queues a cursor show command
    fn cursor_show(&mut self) -> BResult<()> {
        Ok(queue!(self.buffer, c::Show)?)
    }

    /// Queue cursor position to be updated
    ///
    /// * `Point`: Position of the cursor
    fn cursor_set(&mut self, point: Point) -> BResult<()> {
        Ok(queue!(self.buffer, c::MoveTo(point.x, point.y))?)
    }
    /// Queues a cursor retrieval command
    fn cursor_get(&mut self) -> BResult<Point> {
        /* Crossterm's Position Retrieval function is heavily blocking
         * (worst case: 2 seconds) I don't exactly like this so will
         * write to the buffer myself. */
        // Ok(self.buffer.write_all(b"\x1B[6n")?)
        Ok(c::position()?.into())
    }

    /// Queue cursor shape to be updated
    ///
    /// * `shape`: Shape of the cursor
    /// * `blink`: Whether or not cursor should blink
    fn cursor_shape(&mut self, shape: CursorShape, blink: bool) -> BResult<()> {
        queue!(self.buffer, c::SetCursorShape(shape))?;
        if blink {
            Ok(queue!(self.buffer, c::EnableBlinking)?)
        } else {
            Ok(queue!(self.buffer, c::DisableBlinking)?)
        }
    }
}
