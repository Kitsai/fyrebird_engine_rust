use std::result;

use anyhow::Result;
use specs::{
    Builder, Component, Entity, EntityBuilder, World, WorldExt, storage::GenericWriteStorage,
};

use crate::{game::GameEngine, time::Time};

#[derive(Debug)]
pub struct TimeRef<'a>(&'a Time);

pub struct Scene {
    world: World,
}

impl Scene {
    pub fn new(game: &'static GameEngine) -> Self {
        let time = TimeRef(game.get_time());

        let mut world = World::new();

        world.insert(time);

        Self { world }
    }

    pub fn add_entity(&mut self) -> Entity {
        self.world.create_entity().build()
    }

    pub fn add_entity_with(&mut self) -> EntityBuilder {
        self.world.create_entity()
    }

    pub fn add_component<C: Component>(&mut self, entity: Entity, comp: C) -> Result<()> {
        self.world.write_component::<C>().insert(entity, comp)?;
        Ok(())
    }
}
