use bevy_ecs::{component::Component, world::World};
use enumflags2::{BitFlags, bitflags};

use crate::{
    common::{Fixed, Vec2F},
    simulation::ecs::velocity::{SourceID, Velocity},
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
    pub boost: Fixed,
    pub vel_src: Option<SourceID>,
}

const BOOST_MULT: Fixed = Fixed::lit("1.5");
const MOVE_SPEED: Fixed = Fixed::lit("1500");

pub fn movement_system(world: &mut World) {
    let mut query = world.query::<(&mut Movement, &mut Velocity)>();
    for (mut mvmt, mut vel) in query.iter_mut(world) {
        if mvmt.vel_src.is_none() {
            mvmt.vel_src = Some(vel.new_source());
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
        vec *= MOVE_SPEED;

        vel.set_source(mvmt.vel_src.unwrap(), vec, Fixed::lit("0"));
    }
}
