use enumflags2::{BitFlags, bitflags};
use hecs::World;

use crate::simulation::ecs::velocity::Velocity;

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

pub struct Movement {
    pub dir: MovementIntent,
    pub boost: f32,
}

static BOOST_MULT: f32 = 1.5;

// everything with movement and velocity has its velocity updated based on movement
pub fn movement_system(world: &mut World) {
    for (mvmt, vel) in world.query_mut::<(&Movement, &mut Velocity)>() {
        let mut x = 0;
        let mut y = 0;

        if mvmt.dir.contains(MovementDir::Left) {
            x -= 1;
        }
        if mvmt.dir.contains(MovementDir::Right) {
            x += 1;
        }
        if mvmt.dir.contains(MovementDir::Up) {
            y -= 1;
        }
        if mvmt.dir.contains(MovementDir::Down) {
            y += 1;
        }

        if x > 0 {
            x /= x;
        }
        if y > 0 {
            y /= y;
        }

        vel.x = (x as f32) * BOOST_MULT;
        vel.y = (y as f32) * BOOST_MULT;
    }
}
