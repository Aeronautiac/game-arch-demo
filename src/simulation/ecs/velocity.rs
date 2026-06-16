use bevy_ecs::{component::Component, world::World};

use crate::common::Fixed;
use crate::common::time::Time;
use crate::simulation::ecs::transform::Transform;

#[derive(Component, Clone)]
pub struct Velocity {
    pub x: Fixed,
    pub y: Fixed,
}

pub fn apply_velocity(world: &mut World, dt: Time) {
    let mut query = world.query::<(&Velocity, &mut Transform)>();
    for (vel, mut pos) in query.iter_mut(world) {
        pos.x += vel.x * dt.sec();
        pos.y += vel.y * dt.sec();
    }
}
