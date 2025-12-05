use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};

#[cfg(feature = "reflect")]
use bevy_ecs::reflect::ReflectComponent;

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;

/// A marker component for a cube
#[derive(Component, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
pub struct Cube(
    // This boolean serves no purpose. Temporary workaround for https://github.com/wasvy-org/wasvy/issues/30
    bool,
);

/// A singleton component
#[derive(Component, Serialize, Deserialize)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
pub struct Settings {
    pub delta: f32,
    pub count: u32,
    pub spin_speed: f32,
}
