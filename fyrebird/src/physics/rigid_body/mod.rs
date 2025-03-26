use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct RigidBody2D;
