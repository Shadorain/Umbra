use thiserror::Error;

use crate::{backend, Backend, BackendSetter, BError, IEvent, BResult};

#[derive(Error, Debug)]
pub enum UError {
    #[error("Internal backend error: {0}")]
    Backend(BError),
}

impl From<BError> for UError {
    fn from(err: BError) -> UError {
        UError::Backend(err)
    }
}

pub type UResult<T> = std::result::Result<T, UError>;

pub struct Umbra {
    backend: Box<dyn Backend>,
}

impl Umbra {
    pub fn new() -> Self {
        Self {
            backend: Box::new(backend::CrosstermBackend::new()),
        }
    }

    pub fn read_event(&mut self) -> UResult<IEvent> {
        Ok(self.backend.read_event()?)
    }
}

impl Default for Umbra {
    fn default() -> Self {
        Self::new()
    }
}

impl BackendSetter for Umbra {
    fn set_crossterm(&mut self) -> BResult<()> {
        self.backend = Box::new(backend::CrosstermBackend::new());
        backend::CrosstermBackend::init()?;
        Ok(())
    }
    fn set_custom(&mut self, backend: impl Backend + 'static) -> BResult<()> {
        self.backend = Box::new(backend);
        Ok(())
    }
}
