use specs::{Dispatcher, DispatcherBuilder};

use crate::{
    behavior::BehaviorSystem,
    collision::CollisionSystem,
    physics::Physics2DSystem,
    scene::Scene,
    time::Time,
    window::{Window, WindowBuilder},
};

pub struct GameEngine<'a> {
    time: Time,
    scenes: Vec<Scene>,
    active_scene: Option<&'a mut Scene>,
    dispatcher: Dispatcher<'static, 'static>,
    window: Window,
}

impl GameEngine<'_> {
    pub fn run(&mut self) {
        self.main_loop();
    }

    fn main_loop(&mut self) {
        self.time.update();

        todo!(
            "Main loop of the game that should and when close requested or all scenes are closed."
        )
    }

    pub fn get_time(&self) -> &Time {
        &self.time
    }
}

impl Default for GameEngine<'_> {
    fn default() -> Self {
        let dispatcher = DispatcherBuilder::new()
            .with(CollisionSystem, "collision", &[])
            .with(Physics2DSystem, "physics", &["collision"])
            .with(BehaviorSystem, "behaviors", &["physics"]);

        Self {
            time: Time::default(),
            scenes: Vec::new(),
            active_scene: None,
            dispatcher: dispatcher.build(),
            window: WindowBuilder::new().build(),
        }
    }
}
