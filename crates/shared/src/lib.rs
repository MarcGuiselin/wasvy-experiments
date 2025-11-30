use bevy_ecs::component::Component;
use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};

/// A marker component for a cube
#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct Cube;

/// A singleton component
#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct Settings {
    pub delta: f32,
    pub spin_speed: f32,
}
