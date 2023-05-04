#![allow(dead_code)]
use bevy_ecs::prelude::*;

mod backend;
use backend::{Backend, CrosstermBackend, Point, Size};
mod event;
use event::{IEvent, Key, KeyModifiers};

#[derive(Component)]
pub struct Visible;

#[derive(Component)]
pub struct Tile;

#[derive(Bundle)]
pub struct Renderable {
    visible: Visible,
}

#[derive(Default)]
struct Umbra {
    pub world: World,
    pub schedule: Schedule,

    pub backend: CrosstermBackend,
}

impl Umbra {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn read_event(&self) -> Option<IEvent> {
        self.backend.read_event().unwrap()
    }
}

fn main() -> std::io::Result<()> {
    let mut umbra = Umbra::new();

    loop {
        umbra.backend.draw_at((0, 0), "test").unwrap();

        if let Some(event) = umbra.read_event() {
            match event {
                // Quit on `C-q` keypress
                IEvent::Key(modif, key) => {
                    if key == Key::Char('q') && modif == KeyModifiers::CONTROL {
                        break;
                    }
                }
                IEvent::Mouse(_) => (),
                IEvent::Paste(s) => println!("Paste {0}\r", s),
                IEvent::Resize(r) => println!("Screen has been resized to: ({0}, {1})\r", r.x, r.y),
                IEvent::FocusGained => println!("Window gained focus\r"),
                IEvent::FocusLost => println!("Window lost focus\r"),
            }
        }
    }

    Ok(())
}
