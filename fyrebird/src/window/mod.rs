use anyhow::Result;
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event_loop::ActiveEventLoop,
    window::{Fullscreen, Window, WindowAttributes},
};

pub struct BaseWindowAttr {
    pub title: String,
    pub height: f32,
    pub width: f32,
    pub fullscreen: Option<Fullscreen>,
}

pub struct GameWindow {
    window: Option<Window>,
    base_attr: BaseWindowAttr,
}

impl GameWindow {
    pub fn new(attr: BaseWindowAttr) -> Self {
        Self {
            window: None,
            base_attr: attr,
        }
    }

    pub fn init(&mut self, event_loop: &ActiveEventLoop) -> Result<()> {
        let window_attr = Window::default_attributes()
            .with_active(true)
            .with_visible(true)
            .with_title(&self.base_attr.title)
            .with_inner_size(LogicalSize::new(
                self.base_attr.width,
                self.base_attr.height,
            ))
            .with_fullscreen(Some(Fullscreen::Borderless(None)))
            .with_resizable(true)
            .with_position(LogicalPosition::new(0.0, 0.0));

        self.window = Some(event_loop.create_window(window_attr).unwrap());
        Ok(())
    }

    pub fn window(&self) -> Option<&Window> {
        self.window.as_ref()
    }
}
