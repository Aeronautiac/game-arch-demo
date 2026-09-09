use bevy_ecs::component::Component;
use glam::Vec2;

#[derive(Component, Clone, Default)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32, // wrap around at >= 360
}
