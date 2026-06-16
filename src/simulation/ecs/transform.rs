use bevy_ecs::component::Component;

use crate::common::Fixed;

#[derive(Component, Clone)]
pub struct Transform {
    pub x: Fixed,
    pub y: Fixed,
}
