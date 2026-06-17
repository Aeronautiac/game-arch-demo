use bevy_ecs::component::Component;

use crate::common::Vec2F;

#[derive(Component, Clone)]
pub struct Transform {
    pub position: Vec2F,
}
