use anyhow::Result;
use specs::{Dispatcher, DispatcherBuilder, WorldExt};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalPosition, LogicalSize, Position},
    event_loop::ActiveEventLoop,
    window::{Fullscreen, Window},
};

use crate::{
    behavior::BehaviorSystem,
    collision::CollisionSystem,
    physics::Physics2DSystem,
    scene::Scene,
    time::Time,
    vulkan::{self, VulkanModule},
    window::{BaseWindowAttr, GameWindow},
};

pub struct GameEngine<'a> {
    time: Time,
    scenes: Vec<Scene>,
    active_scene: Option<&'a mut Scene>,
    dispatcher: Dispatcher<'static, 'static>,
    window: GameWindow,
    vulkan: Option<VulkanModule>,
    started: bool,
}

impl GameEngine<'_> {
    fn update(&mut self) {
        // Update systems
        if let Some(scene) = &mut self.active_scene {
            self.dispatcher.dispatch(&scene.world);
            scene.world.maintain();
        }
    }

    pub fn get_time(&self) -> &Time {
        &self.time
    }

    pub fn builder() -> GameEngineBuilder {
        GameEngineBuilder {
            window_title: "Fyrebird_data".to_owned(),
            window_height: 720.0,
            window_width: 1280.0,
            window_aspect_ratio: 16.0 / 9.0,
        }
    }

    fn start(&mut self, event_loop: &ActiveEventLoop) -> Result<()> {
        self.window.init(event_loop)?;

        if let Some(window) = self.window.window() {
            self.vulkan = Some(unsafe { VulkanModule::create(window)? });
        }

        Ok(())
    }
}

impl ApplicationHandler for GameEngine<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.started {
            self.started = true;
            self.start(event_loop);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
    }
}

pub struct GameEngineBuilder {
    window_title: String,
    window_height: f32,
    window_width: f32,
    window_aspect_ratio: f32,
}

impl GameEngineBuilder {
    /// Build the game engine using the current values.
    pub fn build(self) -> Result<GameEngine<'static>> {
        let time = Time::new();
        let dispatcher = DispatcherBuilder::new()
            .with(BehaviorSystem, "behaviors", &[])
            .with(CollisionSystem, "collisions", &["behaviors"])
            .with(Physics2DSystem, "physics", &["collisions"])
            .build();

        let base_attr = BaseWindowAttr {
            title: self.window_title,
            height: self.window_height,
            width: self.window_width,
            fullscreen: Some(Fullscreen::Borderless(None)),
        };

        Ok(GameEngine {
            time,
            scenes: Vec::new(),
            active_scene: None,
            dispatcher,
            window: GameWindow::new(base_attr),
            vulkan: None,
            started: false,
        })
    }

    /// Sets game window title
    pub fn title(&mut self, name: String) -> &mut GameEngineBuilder {
        self.window_title = name;
        self
    }

    /// Sets game window height
    pub fn window_height(&mut self, h: f32) -> &mut GameEngineBuilder {
        self.window_height = h;
        self.window_aspect_ratio = self.window_width / self.window_height;
        self
    }

    /// Sets game window width
    pub fn window_width(&mut self, w: f32) -> &mut GameEngineBuilder {
        self.window_width = w;
        self.window_aspect_ratio = self.window_width / self.window_height;
        self
    }

    /// Sets window aspect ratio and adaps the window width based on current height
    pub fn aspect_ratio(&mut self, ratio: f32) -> &mut GameEngineBuilder {
        self.window_aspect_ratio = ratio;
        self.window_width = ratio * self.window_height;
        self
    }
}
