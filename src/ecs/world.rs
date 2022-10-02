use hecs::*;

pub type World = hecs::World;
pub type Entity = hecs::Entity;

// pub trait Component: Send + Sync + 'static {}
//
// impl<T: Send + Sync + 'static> Component for T {}
//
// pub struct World {
//     world: hecs::World,
// }
//
// impl World {
//     pub fn new() -> Self {
//         Self { world: hecs::World::new() }
//     }
//     
//     pub fn spawn_batch()
// }
