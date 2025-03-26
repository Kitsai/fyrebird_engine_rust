pub struct Window;

#[derive(Default)]
pub struct WindowBuilder;

impl WindowBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> Window {
        Window {}
    }
}
