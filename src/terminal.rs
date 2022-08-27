use crossterm::{cursor as c, event as e, execute, queue, style as s, terminal as t, ErrorKind};
use std::io::{self, stdout, Stdout, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TermError {
    #[error("Failure to initialize terminal")]
    Init,
    #[error("Failure to deinitialize terminal")]
    DeInit,
    #[error("Failure to run command: {0:?}")]
    Command(io::Error),
}
impl From<ErrorKind> for TermError {
    fn from(err: ErrorKind) -> TermError {
        TermError::Command(err)
    }
}

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

/// Event is a wrapper around crossterm's Event enum
/// NOTE: Resize taken out because it should be handled immediately
pub enum IEvent {
    Key(e::KeyEvent),
    Paste(String),
    FocusGained,
    FocusLost,
    // Mouse(e::MouseEvent),
    // Resize(crossterm::event::Resize),
    // None,
}

#[derive(Clone, Copy)]
pub struct Size {
    cols: u16,
    rows: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: Stdout,
}

type Result<T> = std::result::Result<T, TermError>;

impl Terminal {
    pub fn new() -> Result<Self> {
        let size = t::size()?;
        Ok(Self {
            size: Size {
                cols: size.0,
                rows: size.1,
            },
            _stdout: stdout(),
        })
    }

    pub fn flush(&mut self) -> Result<()> {
        Ok(self._stdout.flush()?)
    }

    /// Reads and returns an Event from the terminal
    /// Handles resize events automatically
    /// NOTE: Handles Mouse and Resize events from here
    pub fn read_event(&mut self) -> Result<IEvent> {
        loop {
            match e::read()? {
                e::Event::Key(key) => return Ok(IEvent::Key(key)),
                e::Event::Mouse(mouse) => self.cursor_position(mouse.column, mouse.row)?,
                e::Event::Resize(x, y) => self.size = Size { cols: x, rows: y },
                e::Event::Paste(s) => return Ok(IEvent::Paste(s)),
                e::Event::FocusGained => return Ok(IEvent::FocusGained),
                e::Event::FocusLost => return Ok(IEvent::FocusLost),
            }
        }
    }

    /// Queues a cursor hide command
    pub fn cursor_hide(&mut self) -> Result<()> {
        Ok(queue!(self._stdout, c::Hide)?)
    }
    /// Queues a cursor show command
    pub fn cursor_show(&mut self) -> Result<()> {
        Ok(queue!(self._stdout, c::Show)?)
    }

    /// Queue cursor position to be updated
    ///
    /// * `col`: column position
    /// * `row`: row position
    pub fn cursor_position(&mut self, col: u16, row: u16) -> Result<()> {
        Ok(queue!(self._stdout, c::MoveTo(col, row))?)
    }

    /// Queues the screen to be cleared
    pub fn clear_screen(&mut self) -> Result<()> {
        Ok(queue!(self._stdout, t::Clear(t::ClearType::All))?)
    }
    /// Queues the current line to be cleared
    pub fn clear_line(&mut self) -> Result<()> {
        Ok(queue!(self._stdout, t::Clear(t::ClearType::CurrentLine))?)
    }

    pub fn fg_set(&mut self, rgb: Rgb) -> Result<()> {
        Ok(queue!(self._stdout, s::SetForegroundColor(rgb.into()))?)
    }
    pub fn bg_set(&mut self, rgb: Rgb) -> Result<()> {
        Ok(queue!(self._stdout, s::SetBackgroundColor(rgb.into()))?)
    }
    pub fn fg_reset(&mut self) -> Result<()> {
        Ok(queue!(
            self._stdout,
            s::SetForegroundColor(s::Color::Reset)
        )?)
    }
    pub fn bg_reset(&mut self) -> Result<()> {
        Ok(queue!(
            self._stdout,
            s::SetBackgroundColor(s::Color::Reset)
        )?)
    }

    pub fn size_get(&self) -> &Size {
        &self.size
    }
    pub fn size_set(&mut self, size: Size) -> Result<()> {
        self.size = size;
        Ok(queue!(
            self._stdout,
            t::SetSize(self.size.cols, self.size.rows)
        )?)
    }

    pub fn init(&mut self) -> Result<()> {
        execute!(
            self._stdout,
            t::EnterAlternateScreen,
            s::ResetColor,
            t::Clear(t::ClearType::All),
            c::MoveTo(0, 0),
        )
        .map_err(|_| TermError::Init)?;

        t::enable_raw_mode().map_err(|_| TermError::Init)
    }
    pub fn deinit(&mut self) -> Result<()> {
        execute!(
            self._stdout,
            s::ResetColor,
            c::Show,
            t::LeaveAlternateScreen
        )
        .map_err(|_| TermError::DeInit)
    }
}
