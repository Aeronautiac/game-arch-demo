use bevy_ecs::{component::Component, world::World};

use crate::common::time::Time;
use crate::common::{Fixed, Vec3F};
use crate::simulation::ecs::physics::{PhysicsSources, to_linear};
use crate::simulation::ecs::transform::Transform;

#[derive(Component, Clone, Debug)]
pub struct Velocity {
    pub base: Vec3F,
    pub sources: PhysicsSources,
}

impl Velocity {
    pub fn get(&self) -> Vec3F {
        self.base + self.sources.get()
    }

    pub fn new() -> Self {
        Velocity {
            sources: PhysicsSources::new(),
            base: Vec3F::new(Fixed::lit("0"), Fixed::lit("0"), Fixed::lit("0")),
        }
    }
}

pub fn apply_velocity(world: &mut World, dt: Time) {
    let mut query = world.query::<(&Velocity, &mut Transform)>();
    for (vel, mut tform) in query.iter_mut(world) {
        tform.position += to_linear(vel.get()) * dt.sec();
    }
}
