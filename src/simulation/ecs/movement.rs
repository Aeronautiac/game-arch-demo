use bevy_ecs::{component::Component, world::World};
use enumflags2::{BitFlags, bitflags};

use crate::{common::Fixed, simulation::ecs::velocity::Velocity};

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
}

const BOOST_MULT: Fixed = Fixed::lit("1.5");

pub fn movement_system(world: &mut World) {
    let mut query = world.query::<(&Movement, &mut Velocity)>();
    for (mvmt, mut vel) in query.iter_mut(world) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        if mvmt.intent.contains(MovementDir::Left) {
            x -= 1;
        }
        if mvmt.intent.contains(MovementDir::Right) {
            x += 1;
        }
        if mvmt.intent.contains(MovementDir::Up) {
            y -= 1;
        }
        if mvmt.intent.contains(MovementDir::Down) {
            y += 1;
        }

        vel.x = Fixed::from_num(x) * BOOST_MULT * 5000;
        vel.y = Fixed::from_num(y) * BOOST_MULT * 5000;
    }
}
