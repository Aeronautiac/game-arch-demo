use bevy_ecs::{component::Component, world::World};

use crate::common::time::Time;
use crate::common::{Fixed, Vec2F, Vec3F};
use crate::simulation::ecs::physics::PhysicsSources;
use crate::simulation::ecs::physics::velocity::Velocity;

#[derive(Component, Clone, Debug)]
pub struct Forces {
    pub sources: PhysicsSources,
}

impl Forces {
    pub fn get(&self) -> Vec3F {
        self.sources.get()
    }

    pub fn new() -> Self {
        Forces {
            sources: PhysicsSources::new(),
        }
    }

    pub fn from_linear(vec: Vec2F) -> Vec3F {
        Vec3F::new(vec[0], vec[1], Fixed::lit("0"))
    }
}

pub fn apply_forces(world: &mut World, dt: Time) {
    let mut query = world.query::<(&Forces, &mut Velocity)>();
    for (force, mut vel) in query.iter_mut(world) {
        vel.base += force.get() * dt.sec();
    }
}
