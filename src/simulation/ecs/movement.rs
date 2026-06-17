use bevy_ecs::{component::Component, world::World};
use enumflags2::{BitFlags, bitflags};

use crate::{
    common::{Fixed, Vec2F},
    simulation::ecs::physics::{SourceID, forces::Forces, from_linear},
};

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MovementDir {
    Left = 1 << 0,
    Right = 1 << 1,
    Up = 1 << 2,
    Down = 1 << 3,
}

pub type MovementIntent = BitFlags<MovementDir>;

#[derive(Component)]
pub struct Movement {
    pub intent: MovementIntent,
    pub phys_src: Option<SourceID>,
}

const MOVE_ACCEL: Fixed = Fixed::lit("100");

pub fn movement_system(world: &mut World) {
    let mut query = world.query::<(&mut Movement, &mut Forces)>();
    for (mut mvmt, mut forces) in query.iter_mut(world) {
        if mvmt.phys_src.is_none() {
            mvmt.phys_src = Some(forces.sources.new_source());
        }

        const ONE: Fixed = Fixed::lit("1");
        let mut vec = Vec2F::default();
        if mvmt.intent.contains(MovementDir::Left) {
            vec[0] -= ONE;
        }
        if mvmt.intent.contains(MovementDir::Right) {
            vec[0] += ONE;
        }
        if mvmt.intent.contains(MovementDir::Up) {
            vec[1] -= ONE;
        }
        if mvmt.intent.contains(MovementDir::Down) {
            vec[1] += ONE;
        }

        let magnitude = (vec[0] * vec[0] + vec[1] * vec[1]).sqrt();
        if magnitude != Fixed::lit("0") {
            vec /= magnitude;
        }
        vec *= MOVE_ACCEL;

        forces
            .sources
            .set_source(mvmt.phys_src.unwrap(), from_linear(vec));
    }
}
