//! # Umbra TUI Framework
//! > *A simple yet highly configurable framework to generate flexible and fast TUIs*
//!
//! ## Example
//!
//! ```rust
//! use umbra::{IEvent, Umbra};
//!
//! /// NOTE: Umbra assumes that it will be responsible for
//! /// setting up the screen and raw mode
//! /// Will use first included backend as well by default.
//! let mut umbra: Umbra = Umbra::new();
//! 
//! loop {
//!     Some(event) => match event {
//!         IEvent::Key(modif, key) => if key == Key::Char('q') && modif == KeyModifiers::CONTROL { break; },
//!         IEvent::Mouse(m) => println!("Mouse ptr: ({0}, {1})", m.x, m.y),
//!         IEvent::Resize(r) => println!("Screen has been resized to: ({0}, {1})", r.x, r.y),
//!         IEvent::Paste(s) => print!("Paste {0}", s),
//!         IEvent::FocusGained => print!("Window gained focus"),
//!         IEvent::FocusLost => print!("Window lost focus"),
//!     },
//!     None => (),
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
pub use backend::{Backend, BResult, BError};
