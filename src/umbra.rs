use thiserror::Error;

use crate::{backend, backend::BackendSetter, BError, BResult, Backend, IEvent};

#[derive(Error, Debug)]
pub enum UError {
    #[error("Internal backend error: {0}")]
    Backend(BError),
}

/// Translates a Backend error into UError::Backend
impl From<BError> for UError {
    fn from(err: BError) -> UError {
        UError::Backend(err)
    }
}

/// Umbra Result type that takes UError's as error type
pub type UResult<T> = std::result::Result<T, UError>;

/// Main structure of Umbra TUI
pub struct Umbra {
    /// A field that allows access to a private backend
    /// implementation.
    backend: Box<dyn Backend>,
}

impl Umbra {
    /// Creates a new Umbra instance
    pub fn new() -> UResult<Self> {
        Ok(Self {
            backend: Self::try_backend()?,
        })
    }

    /// Reads an event, Some(e) if found, None otherwise
    /// Calls internal Backend
    pub fn read_input(&mut self) -> UResult<Option<IEvent>> {
        Ok(self.backend.read_event()?)
    }

    /// Sets the window title
    /// Calls internal Backend
    pub fn set_title(&mut self, title: &'static str) -> UResult<()> {
        Ok(self.backend.set_title(title)?)
    }

    /// Attempts to find a configured backend and returns it if
    /// found. If it fails it will log and return an error.
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
    /// Sets the internal backend to crossterm
    fn set_crossterm(&mut self) -> BResult<()> {
        self.backend = Box::new(backend::CrosstermBackend::init()?);
        Ok(())
    }
    /// Sets the internal backend to a custom user backend.
    /// NOTE: it needs to hold a static lifetime
    fn set_custom(&mut self, backend: impl Backend + 'static) -> BResult<()> {
        self.backend = Box::new(backend);
        Ok(())
    }
}
