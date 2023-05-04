use std::io;
use thiserror::Error;

mod crossterm;
pub use self::crossterm::{CrosstermBackend, SetCursorStyle};
use super::IEvent;

#[derive(Default, Clone, Copy)]
pub struct Coords<T> {
    pub x: T,
    pub y: T,
}

impl<T> From<(T, T)> for Coords<T> {
    fn from((x, y): (T, T)) -> Self {
        Coords { x, y }
    }
}

impl<T> Coords<T> {
    pub fn new(x: T, y: T) -> Self {
        Coords { x, y }
    }
}

pub type Point = Coords<u16>;
pub type Size = Coords<u16>;

#[allow(dead_code)]
pub struct DrawBuffer<'a> {
    buf: &'a str,
}

impl<'a> DrawBuffer<'a> {
    pub fn new(buf: &'a str) -> Self {
        Self { buf }
    }
}

impl<'a> From<&'a str> for DrawBuffer<'a> {
    fn from(buf: &'a str) -> Self {
        Self { buf }
    }
}

impl<'a> std::fmt::Display for DrawBuffer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.buf)
    }
}

#[derive(Error, Debug)]
pub enum BError {
    #[error("Failure to run command: {0:?}")]
    Internal(io::Error),
    #[error("No backend exists")]
    NoBackend,
}
/// Translates internal IO errors into BError::Internal
impl From<io::Error> for BError {
    fn from(err: io::Error) -> BError {
        BError::Internal(err)
    }
}

/// Backend Result type that takes BError's as eerror type
pub type BResult<T> = std::result::Result<T, BError>;

/// Generic Interface exposing key functions for an internal
/// backend implementation.
pub trait Backend {
    fn read_event(&self) -> BResult<Option<IEvent>>;
    fn refresh(&mut self) -> BResult<()>;

    fn draw_at<'a>(&mut self, pos: impl Into<Point>, buf: impl Into<DrawBuffer<'a>>) -> BResult<()>;

    fn set_title(&mut self, title: &str) -> BResult<()>;
    fn screen_size(&mut self) -> Size;
    fn screen_clear(&mut self) -> BResult<()>;

    fn cursor_show(&mut self) -> BResult<()>;
    fn cursor_hide(&mut self) -> BResult<()>;
    fn cursor_get(&mut self) -> BResult<Point>;
    fn cursor_set(&mut self, point: Point) -> BResult<()>;
    fn cursor_style(&mut self, style: SetCursorStyle, blink: bool) -> BResult<()>;
}

/// Trait for setting internal backend
/// Doesn't exactly need to be a trait, but nice for logical grouping.
pub trait BackendSetter {
    fn set_crossterm(&mut self) -> BResult<()>;
    fn set_custom(&mut self, backend: impl Backend + 'static) -> BResult<()>;
}
