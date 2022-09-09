// mod draw;
// mod pane;
// mod workspace;

// use std::collections::{HashMap, VecDeque};
// use thiserror::Error;

// use self::workspace::Workspace;
//
// #[derive(Error, Debug)]
// pub enum ScreenError {
//     #[error("Placeholder Err")]
//     Component(ComponentError),
// }
//
// #[derive(Error, Debug)]
// pub enum ComponentError {}

// pub type CResult<T> = std::result::Result<T, ComponentError>;
// type SResult<T> = std::result::Result<T, ScreenError>;

#[derive(Clone, Copy)]
pub struct Coords<T> {
    pub x: T,
    pub y: T,
}

impl<T> From<(T, T)> for Coords<T> {
    fn from(item: (T, T)) -> Self {
        Coords {
            x: item.0,
            y: item.1,
        }
    }
}

impl<T> Coords<T> {
    pub fn new(x: T, y: T) -> Self {
        Coords { x, y }
    }
}

pub type Point = Coords<u16>;
pub type Size = Coords<u16>;

// pub enum Event { }
//
// pub trait Component {
//     fn draw(&self) -> CResult<()> {
//         Ok(())
//     }
//     fn update(&mut self) -> CResult<()> {
//         Ok(())
//     }
// }
//
// pub struct Screen {
//     event_queue: VecDeque<Event>,
//     workspaces: HashMap<u16, Workspace>,
// }
//
// impl Screen {
//     pub fn new() -> Self {
//         Self {
//             event_queue: VecDeque::new(),
//             workspaces: HashMap::new(),
//         }
//     }
//     pub fn send_update(&mut self, event: Event) {
//         self.event_queue.push_back(event);
//     }
//     pub fn update(&mut self) -> SResult<()> {
//         Ok(())
//     }
// }
