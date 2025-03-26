use crate::arith::{Point2, Vec2, point2, vec2};

use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Transform2D {
    pub position: Point2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            position: point2(0.0, 0.0),
            rotation: 0.0,
            scale: vec2(0.0, 0.0),
        }
    }
}
