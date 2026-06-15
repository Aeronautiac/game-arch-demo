use hecs::World;
use macroquad::miniquad::native::linux_x11::libx11::Time;

use crate::simulation::ecs::transform::Transform;

pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub fn apply_velocity(world: &mut World, dt: Time) {
    for (vel, pos) in world.query_mut::<(&Velocity, &mut Transform)>() {
        pos.x += vel.x * dt as f32;
        pos.y += vel.y * dt as f32;
    }
}
