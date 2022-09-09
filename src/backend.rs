use std::io;
use thiserror::Error;

mod crossterm;
pub use self::crossterm::{CrosstermBackend, CursorShape};

use crate::{screen::{Size, Point}, IEvent};

#[derive(Error, Debug)]
pub enum BError {
    #[error("Failure to run command: {0:?}")]
    Backend(io::Error),
}
impl From<io::Error> for BError {
    fn from(err: io::Error) -> BError {
        BError::Backend(err)
    }
}

pub type BResult<T> = std::result::Result<T, BError>;

pub trait Backend {
    fn read_event(&mut self) -> BResult<IEvent>;
    fn refresh(&mut self) -> BResult<()>;

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
