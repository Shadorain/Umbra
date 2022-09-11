use thiserror::Error;

use crate::{backend, Backend, backend::BackendSetter, BError, IEvent, BResult};

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
    pub fn new() -> UResult<Self> {
        Ok(Self {
            backend: Self::try_backend()?,
        })
    }

    pub fn read_event(&mut self) -> UResult<IEvent> {
        Ok(self.backend.read_event()?)
    }

    fn try_backend() -> BResult<Box<dyn Backend>> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "crossterm-backend")] {
                Ok(Box::new(backend::CrosstermBackend::init()?))
            } else {
                log::warn!("No backends have been found, set one or create your own.")
                Err(BError::NoBackend)
            }
        }
    }
}

impl Default for Umbra {
    /// Default implementation for Umbra
    /// NOTE: Panics, use Umbra::new() for more control
    fn default() -> Self {
        Self::new().expect("Failed to initialize Umbra structure")
    }
}

impl BackendSetter for Umbra {
    fn set_crossterm(&mut self) -> BResult<()> {
        self.backend = Box::new(backend::CrosstermBackend::init()?);
        Ok(())
    }
    fn set_custom(&mut self, backend: impl Backend + 'static) -> BResult<()> {
        self.backend = Box::new(backend);
        Ok(())
    }
}
