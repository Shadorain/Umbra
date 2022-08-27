use thiserror::Error;

use crate::terminal::{IEvent, TermError, Terminal};

#[derive(Error, Debug)]
pub enum UError {
    #[error("Terminal: {0}")]
    Terminal(TermError),
}

impl From<TermError> for UError {
    fn from(err: TermError) -> UError {
        UError::Terminal(err)
    }
}

pub type UResult<T> = std::result::Result<T, UError>;

pub struct Umbra {
    term: Terminal,
}

impl Umbra {
    pub fn new() -> UResult<Self> {
        Ok(Self {
            term: Terminal::new()?,
        })
    }

    pub fn init(&mut self) -> UResult<()> {
        Ok(self.term.init()?)
    }

    pub fn read_event(&mut self) -> UResult<IEvent> {
        Ok(self.term.read_event()?)
    }
}
