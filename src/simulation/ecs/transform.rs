use bevy_ecs::component::Component;
use glam::Vec2;

#[derive(Component, Clone, Default)]
pub struct Transform {
    pub position: Vec2,
    rotation: f32, // wrap around at >= 360
}

impl Transform {
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    // need to figure out wrap around semantics
    pub fn set_rotation(&mut self, val: f32) {
        self.rotation = val;
    }
}
