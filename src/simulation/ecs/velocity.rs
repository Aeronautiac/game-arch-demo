use bevy_ecs::{component::Component, world::World};
use smallvec::SmallVec;

use crate::common::time::Time;
use crate::common::{Fixed, Vec2F, Vec3F};
use crate::simulation::ecs::transform::Transform;

pub type SourceID = u32;

// This can be optimized more later, just getting a minimal working example.

#[derive(Clone, Debug)]
pub struct Source<T> {
    pub meta: SourceID, // gives you like 2 billion sources
    // the last bit is a "destroyed" bit meaning this slot can be used again
    pub val: T,
}

impl<T> Source<T> {
    pub fn is_free(&self) -> bool {
        (self.meta & (1 << 31)) == 0
    }

    pub fn set_free(&mut self) {
        self.meta |= 1 << 31
    }

    pub fn id(&self) -> SourceID {
        self.meta & !(1 << 31)
    }

    pub fn new(id: SourceID, val: T) -> Self {
        Source { val, meta: id }
    }
}

#[derive(Component, Clone)]
pub struct Velocity {
    pub cached: Vec3F,
    pub sources: SmallVec<[Source<Vec3F>; 8]>, // this will be so small all the time that its not
    // even worth doing anything like using a slotmap right now
    pub next_src_id: SourceID,
    pub dirty: bool,
}

impl Velocity {
    pub fn linear(&self) -> Vec2F {
        Vec2F::new(self.cached[0], self.cached[1])
    }

    pub fn angular(&self) -> Fixed {
        self.cached[2]
    }

    pub fn new_source(&mut self) -> SourceID {
        let id = self.next_src_id;
        let src = Source::new(
            id,
            Vec3F::new(Fixed::lit("0"), Fixed::lit("0"), Fixed::lit("0")),
        );
        let old_idx = self.sources.iter().position(|src| !src.is_free());
        if let Some(idx) = old_idx {
            self.sources[idx] = src;
        } else {
            self.sources.push(src);
        }
        self.next_src_id += 1;
        id
    }

    pub fn get_source_mut(&mut self, id: SourceID) -> Option<&mut Source<Vec3F>> {
        let idx = self
            .sources
            .iter()
            .position(|src| src.id() == id && src.is_free())?;
        self.sources.get_mut(idx)
    }

    pub fn set_source(&mut self, src: SourceID, lin: Vec2F, ang: Fixed) -> bool {
        let mut dirty = self.dirty;
        let new_val = Vec3F::new(lin[0], lin[1], ang);
        let old_val = self.get_source_mut(src);
        let res = if let Some(old) = old_val {
            if old.val != new_val {
                dirty = true;
                old.val = new_val;
            }
            true
        } else {
            false
        };
        self.dirty = dirty;
        res
    }

    pub fn new() -> Self {
        Velocity {
            cached: Vec3F::default(),
            sources: SmallVec::new(),
            dirty: false,
            next_src_id: 0,
        }
    }
}

pub fn apply_velocity(world: &mut World, dt: Time) {
    let mut query = world.query::<(&mut Velocity, &mut Transform)>();
    for (mut vel, mut tform) in query.iter_mut(world) {
        // only re-eval if velocity was changed
        if vel.dirty {
            let mut val = Vec3F::new(Fixed::lit("0"), Fixed::lit("0"), Fixed::lit("0"));
            for src in vel.sources.iter() {
                val += src.val;
            }
            vel.cached = val;
            vel.dirty = false;
        }

        tform.position += vel.linear() * dt.sec();
    }
}
