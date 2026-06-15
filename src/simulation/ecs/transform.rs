use bevy_ecs::component::Component;
use fixed::types::I32F32;

#[derive(Component)]
pub struct Transform {
    pub x: I32F32,
    pub y: I32F32,
}
