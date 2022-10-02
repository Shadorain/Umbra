mod draw;

use thiserror::Error;

pub use draw::DrawBuffer;

use crate::ecs::{World, Entity};

#[derive(Error, Debug)]
pub enum ScreenError {
    #[error("Renderer Error: {0:?}")]
    Renderer(RenderError),
}

#[derive(Error, Debug)]
pub enum RenderError {}

type SResult<T> = std::result::Result<T, ScreenError>;

#[derive(Default, Clone, Copy)]
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

pub trait Element {
    fn draw(&self) -> SResult<()>;
    fn update(&mut self) -> SResult<()>;
}

pub struct Screen {
    world: World,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }
    
    pub fn add_element<E: Element>(&mut self, element: E) {
        
    }
}
