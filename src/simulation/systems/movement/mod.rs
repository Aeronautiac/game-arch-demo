use enumflags2::bitflags;
use hecs::World;

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MovementIntent {
    Left = 1 << 0,
    Right = 1 << 1,
    Up = 1 << 2,
    Down = 1 << 3,
}

pub struct Movement {
    pub dir: MovementIntent,
    pub boost: f32,
}

// everything with movement and velocity has its velocity updated based on movement
pub fn movement_system(world: &mut World) {
    for (id, (pos, vel)) in world.query_mut::<(&Movement, &mut Velocity)>() {}
}
