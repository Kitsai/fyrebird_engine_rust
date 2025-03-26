use specs::{Entities, System};

pub mod rigid_body;

pub struct Physics2DSystem;

impl<'a> System<'a> for Physics2DSystem {
    type SystemData = (Entities<'a>,);
    fn run(&mut self, data: Self::SystemData) {}
}
