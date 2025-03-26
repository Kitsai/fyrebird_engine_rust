pub mod shapes;

use specs::{Component, VecStorage};
use specs::{Entities, ReadStorage, System};
use specs_derive::Component;

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, Collider>);
    fn run(&mut self, data: Self::SystemData) {}
}

#[derive(Debug, Component, Default)]
#[storage(VecStorage)]
pub struct Collider;
