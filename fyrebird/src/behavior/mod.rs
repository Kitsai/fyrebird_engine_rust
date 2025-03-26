#![allow(unused_variables)]
use crate::time::Time;
use specs::storage::VecStorage;
use specs::{Component, Entities, Entity, Join, ReadExpect, System, World, Write, WriteStorage};
use specs_derive::Component;

pub trait Behavior: Send + Sync {
    fn start(&mut self, entity: Entity, world: &mut World) {}
    fn update(&mut self, entity: Entity, world: &mut World, _dt: f32) {}
    fn fixed_update(&mut self, entity: Entity, world: &mut World) {}
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BehaviorContainer {
    pub behaviors: Vec<Box<dyn Behavior>>,
}

pub struct BehaviorSystem;

impl<'a> System<'a> for BehaviorSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, World>,
        ReadExpect<'a, Time>,
        WriteStorage<'a, BehaviorContainer>,
    );

    fn run(&mut self, (entities, mut world, time, mut behaviors): Self::SystemData) {
        for (entity, behavior_container) in (&entities, &mut behaviors).join() {
            for behavior in &mut behavior_container.behaviors {
                behavior.update(entity, &mut world, time.delta_time);
            }
        }
    }
}
