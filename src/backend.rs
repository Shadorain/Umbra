use std::{fmt::Display, io, ops::Deref};
use thiserror::Error;

mod crossterm;
pub use self::crossterm::{CrosstermBackend, SetCursorStyle, Stylize};
// use self::crossterm::StyledContent;
use super::IEvent;

#[derive(Default, Clone, Copy, PartialEq)]
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
impl Point {
    pub fn from_index(index: usize, size: Size) -> Self {
        let y = (index as f32 / size.x as f32).floor() as usize;
        let x = index - (y * size.x as usize);
        Self {
            x: x as u16,
            y: y as u16,
        }
    }
    pub fn to_index(self, size: Size) -> usize {
        (self.y * size.x + self.x) as usize
    }
}

pub type Size = Coords<u16>;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Rect {
    pub point: Point,
    pub width: u16,
    pub height: u16,
}
impl Deref for Rect {
    type Target = Point;

    fn deref(&self) -> &Self::Target {
        &self.point
    }
}
impl Rect {
    pub fn area(&self) -> u16 {
        self.width * self.height
    }
    pub fn size(&self) -> Size {
        Size {
            x: self.width,
            y: self.height,
        }
    }
    pub fn intersects(&self, rect: &Self) -> bool {
        self.x < rect.x + rect.width
            && rect.x < self.x + self.width
            && self.y < rect.y + rect.height
            && rect.y < self.y + self.height
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub glyph: char,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.glyph)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self { glyph: ' ' }
    }
}

impl Cell {
    pub fn new(glyph: char) -> Self {
        Self { glyph }
    }
}

pub struct Terminal {
    backend: Box<dyn Backend>,
    buffer: Buffer,
}
impl Default for Terminal {
    fn default() -> Self {
        Self {
            backend: Box::new(CrosstermBackend::default()),
            buffer: Default::default(),
        }
    }
}
impl Terminal {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn draw(&self, rect: Rect, content: impl IntoIterator<Item = Cell>) {
        if let Some(c) = self.buffer.diff(rect, content) {
            self.backend.draw_at(&c.into_iter());
        }
    }
    pub fn read_event(&self) -> BResult<Option<IEvent>> {
        let event = self.backend.read_event()?;
        if let Some(e) = event {
            match e {
                IEvent::Resize(r) => self.buffer.resize(r),
                _ => (),
            }
        }
        Ok(event)
    }

    pub fn update(&mut self) -> BResult<()> {
        self.backend.refresh()
    }
}

/// Collection of `Cell`s directly rendered to the backend.
#[derive(Default)]
pub struct Buffer {
    frame: Rect,
    cells: Vec<Cell>,
}

impl Buffer {
    pub fn new(frame: Rect) -> Self {
        Self {
            frame,
            cells: vec![Cell::default(); frame.area() as usize],
        }
    }

    pub fn diff(
        &self,
        rect: Rect,
        content: impl IntoIterator<Item = Cell>,
    ) -> Option<Vec<(Point, Cell)>> {
        let mut changes: Vec<(Point, Cell)> = Vec::new();
        let index = rect.to_index(self.frame.size());
        for (i, cell) in content.into_iter().enumerate() {
            if cell != self.get_at(index + i) {
                changes.push((Point::from_index(index + i, self.frame.size()), cell));
            }
        }
        if !changes.is_empty() {
            Some(changes)
        } else {
            None
        }
    }

    pub fn resize(&mut self, size: Size) {}

    fn get_at(&self, index: usize) -> Cell {
        self.cells[index]
    }
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Distance(pub Direction, pub u16);

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

    fn draw_at(&mut self, buf: &dyn Iterator<Item = (Point, Cell)>) -> BResult<()>;

    fn set_title(&mut self, title: &str) -> BResult<()>;
    fn screen_size(&mut self) -> Size;
    fn screen_clear(&mut self) -> BResult<()>;

    fn cursor_show(&mut self) -> BResult<()>;
    fn cursor_hide(&mut self) -> BResult<()>;
    fn cursor_get(&mut self) -> BResult<Point>;
    fn cursor_set(&mut self, point: Point) -> BResult<()>;
    fn cursor_move(&mut self, distance: Distance) -> BResult<()>;
    fn cursor_style(&mut self, style: SetCursorStyle, blink: bool) -> BResult<()>;
}

/// Trait for setting internal backend
/// Doesn't exactly need to be a trait, but nice for logical grouping.
pub trait BackendSetter {
    fn set_crossterm(&mut self) -> BResult<()>;
    fn set_custom(&mut self, backend: impl Backend + 'static) -> BResult<()>;
}
