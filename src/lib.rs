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
//! umbra.set_title("Umbra TUI!")?;
//! 
//! loop {
//!     if let Some(event) = umbra.read_input()? {
//!         match event {
//!             // Quit on `C-q` keypress
//!             IEvent::Key(modif, key) => if key == Key::Char('q') && modif == KeyModifiers::CONTROL { break; },
//!             IEvent::Mouse(m) => println!("Mouse ptr: ({0}, {1})\r", m.x, m.y),
//!             IEvent::Resize(r) => println!("Screen has been resized to: ({0}, {1})\r", r.x, r.y),
//!             IEvent::Paste(s) => println!("Paste {0}\r", s),
//!             IEvent::FocusGained => println!("Window gained focus\r"),
//!             IEvent::FocusLost => println!("Window lost focus\r"),
//!         }
//!     }
//!     // umbra.refresh(); // Not yet implemented
//! }
//!
//! ```
mod components;
mod screen;

mod umbra;
pub use umbra::{UError, UResult, Umbra};

mod event;
pub use event::{IEvent, Key, KeyModifiers, MediaKey};

mod backend;
pub use backend::{Backend, BResult, BError};
