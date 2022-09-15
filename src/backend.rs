use std::io;
use thiserror::Error;

mod crossterm;
pub use self::crossterm::{CrosstermBackend, CursorShape};

use crate::{screen::{Size, Point, DrawBuffer}, IEvent};

#[derive(Error, Debug)]
pub enum BError {
    #[error("Failure to run command: {0:?}")]
    Internal(io::Error),
    #[error("No backend exists")]
    NoBackend,
}
impl From<io::Error> for BError {
    fn from(err: io::Error) -> BError {
        BError::Internal(err)
    }
}

pub type BResult<T> = std::result::Result<T, BError>;

pub trait Backend {
    fn read_event(&mut self) -> BResult<Option<IEvent>>;
    fn refresh(&mut self) -> BResult<()>;

    fn draw_at(&mut self, pos: Point, buf: DrawBuffer) -> BResult<()>;

    fn set_title(&mut self, title: &str) -> BResult<()>;
    fn screen_size(&mut self) -> Size;
    fn screen_clear(&mut self) -> BResult<()>;

    fn cursor_show(&mut self) -> BResult<()>;
    fn cursor_hide(&mut self) -> BResult<()>;
    fn cursor_get(&mut self) -> BResult<Point>;
    fn cursor_set(&mut self, point: Point) -> BResult<()>;
    fn cursor_shape(&mut self, shape: CursorShape, blink: bool) -> BResult<()>;
}

pub trait BackendSetter {
    fn set_crossterm(&mut self) -> BResult<()>;
    fn set_custom(&mut self, backend: impl Backend + 'static) -> BResult<()>;
}
