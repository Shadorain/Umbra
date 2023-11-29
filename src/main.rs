#![allow(dead_code)]
use bevy_ecs::prelude::*;

mod backend;
use backend::{
    Backend, Buffer, CrosstermBackend, Direction, Distance, Point, Rect, Size, Terminal,
};
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

    pub terminal: Terminal,
}

impl Umbra {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn read_event(&self) -> Option<IEvent> {
        self.terminal.read_event().unwrap()
    }
    pub fn update(&mut self) {
        self.terminal.update().expect("Backend flush failed.");
        self.schedule.run(&mut self.world);
    }
}

fn main() -> std::io::Result<()> {
    let mut umbra = Umbra::new();
    // umbra.backend.cursor_show().unwrap();

    let mut draw: Vec<(Point, backend::Cell)> = Vec::new();
    for y in 0..40 {
        for x in 0..150 {
            if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
                draw.push(((x, y).into(), backend::Cell::new('█')));
            }
        }
    }
    umbra
        .terminal
        .draw(Rect::default(), draw.into_iter())
        .unwrap();

    loop {
        umbra.update();

        if let true = handle_events(&mut umbra) {
            break;
        }
    }

    Ok(())
}

fn handle_events(umbra: &mut Umbra) -> bool {
    if let Some(event) = umbra.read_event() {
        match event {
            // Quit on `C-q` keypress
            IEvent::Key(modif, key) => match key {
                Key::Char('q') => {
                    if modif == KeyModifiers::CONTROL {
                        return true;
                    }
                }
                // Key::Char('h') => umbra
                //     .backend
                //     .cursor_move(Distance(Direction::Left, 1))
                //     .unwrap(),
                // Key::Char('j') => umbra
                //     .backend
                //     .cursor_move(Distance(Direction::Down, 1))
                //     .unwrap(),
                // Key::Char('k') => umbra
                //     .backend
                //     .cursor_move(Distance(Direction::Up, 1))
                //     .unwrap(),
                // Key::Char('l') => umbra
                //     .backend
                //     .cursor_move(Distance(Direction::Right, 1))
                //     .unwrap(),
                _ => (),
            },
            IEvent::Mouse(_) => (),
            IEvent::Paste(s) => println!("Paste {0}\r", s),
            IEvent::Resize(r) => println!("Screen has been resized to: ({0}, {1})\r", r.x, r.y),
            IEvent::FocusGained => println!("Window gained focus\r"),
            IEvent::FocusLost => println!("Window lost focus\r"),
        }
    }
    false
}
