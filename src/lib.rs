//! # Umbra TUI Framework
//! > *A simple yet highly configurable framework to generate flexible and fast TUIs*
//!
//! ## Example
//!
//! ```rust,no_run
//! use umbra::{IEvent, Umbra};
//!
//! // NOTE: Umbra assumes that it will be responsible for
//! // setting up the screen and raw mode
//! let mut umbra: Umbra = Umbra::new().expect("Umbra error: {0}");
//! umbra.init().expect("Panic... umbra initialization failed");
//!
//! loop {
//!     match umbra.read_event().unwrap() {
//!         IEvent::Key(key) => print!("Key {:?}", key),
//!         IEvent::Paste(s) => print!("Paste {0}", s),
//!         IEvent::FocusGained => print!("Window gained focus"),
//!         IEvent::FocusLost => print!("Window lost focus"),
//!     }
//!     umbra.refresh();
//! }
//!
//! ```
mod screen;

mod umbra;
pub use umbra::{UError, UResult, Umbra};

mod event;
pub use event::{IEvent, Key, KeyModifiers, MediaKey};

mod backend;
pub use backend::{Backend, BackendSetter, BResult, BError};
