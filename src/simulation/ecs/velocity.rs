use bevy_ecs::{component::Component, world::World};
use fixed::types::I32F32;

use crate::common::Time;
use crate::simulation::ecs::transform::Transform;

#[derive(Component)]
pub struct Velocity {
    pub x: I32F32,
    pub y: I32F32,
}

pub fn apply_velocity(world: &mut World, dt: Time) {
    let dt_fixed = I32F32::from_num(dt);
    let mut query = world.query::<(&Velocity, &mut Transform)>();
    for (vel, mut pos) in query.iter_mut(world) {
        pos.x += vel.x * dt_fixed;
        pos.y += vel.y * dt_fixed;
    }
}
