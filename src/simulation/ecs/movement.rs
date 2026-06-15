use bevy_ecs::{component::Component, world::World};
use enumflags2::{BitFlags, bitflags};
use fixed::types::I32F32;

use crate::simulation::ecs::velocity::Velocity;

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MovementDir {
    Left  = 1 << 0,
    Right = 1 << 1,
    Up    = 1 << 2,
    Down  = 1 << 3,
}

pub type MovementIntent = BitFlags<MovementDir>;

#[derive(Component)]
pub struct Movement {
    pub dir: MovementIntent,
    pub boost: I32F32,
}

const BOOST_MULT: I32F32 = I32F32::lit("1.5");

pub fn movement_system(world: &mut World) {
    let mut query = world.query::<(&Movement, &mut Velocity)>();
    for (mvmt, mut vel) in query.iter_mut(world) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        if mvmt.dir.contains(MovementDir::Left)  { x -= 1; }
        if mvmt.dir.contains(MovementDir::Right) { x += 1; }
        if mvmt.dir.contains(MovementDir::Up)    { y -= 1; }
        if mvmt.dir.contains(MovementDir::Down)  { y += 1; }

        if x > 0 { x /= x; }
        if y > 0 { y /= y; }

        vel.x = I32F32::from_num(x) * BOOST_MULT;
        vel.y = I32F32::from_num(y) * BOOST_MULT;
    }
}
