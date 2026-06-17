use smallvec::SmallVec;

use crate::common::{Fixed, Vec2F, Vec3F};

pub mod forces;
pub mod velocity;

pub type SourceID = u32;

// the last bit is the free bit
// if the free bit is set, the slot is empty
#[derive(Clone, Debug)]
pub struct SourceMeta(SourceID);

impl SourceMeta {
    pub fn is_free(&self) -> bool {
        (self.0 & (1 << 31)) > 0
    }

    pub fn set_free(&mut self, val: bool) {
        if val {
            self.0 |= 1 << 31
        } else {
            self.0 &= !(1 << 31)
        }
    }

    pub fn id(&self) -> SourceID {
        self.0 & !(1 << 31)
    }
}

#[derive(Clone, Debug)]
pub struct PhysicsSource {
    pub meta: SourceMeta,
    pub val: Vec3F,
}

impl PhysicsSource {
    pub fn new(id: SourceID) -> Self {
        PhysicsSource {
            meta: SourceMeta(id),
            val: Vec3F::default(),
        }
    }

    pub fn is_free(&self) -> bool {
        self.meta.is_free()
    }

    pub fn id(&self) -> SourceID {
        self.meta.id()
    }
}

#[derive(Clone, Debug)]
pub struct PhysicsSources {
    pub cached: Vec3F,
    pub next_src_id: SourceID,
    pub sources: SmallVec<[PhysicsSource; 8]>, // this will be usually be so small that its not
                                               // even worth doing anything like a slotmap
                                               // but it will probably be necessary for more complex games
}

impl PhysicsSources {
    pub fn new_source(&mut self) -> SourceID {
        let id = self.next_src_id;
        let src = PhysicsSource::new(id);
        let old_idx = self.sources.iter().position(|src| src.is_free());
        if let Some(idx) = old_idx {
            self.sources[idx] = src;
        } else {
            self.sources.push(src);
        }
        self.next_src_id += 1;
        id
    }

    pub fn get_source_mut(&mut self, id: SourceID) -> Option<&mut PhysicsSource> {
        let idx = self
            .sources
            .iter()
            .position(|src| src.id() == id && !src.is_free())?;
        self.sources.get_mut(idx)
    }

    pub fn set_source(&mut self, src: SourceID, new_val: Vec3F) -> bool {
        let old_val = self.get_source_mut(src);
        if let Some(old) = old_val {
            let old_val = old.val;
            old.val = new_val;
            if new_val != old_val {
                let mut val = Vec3F::new(Fixed::lit("0"), Fixed::lit("0"), Fixed::lit("0"));
                for src in self.sources.iter().filter(|src| !src.is_free()) {
                    val += src.val;
                }
                self.cached = val;
            }

            true
        } else {
            false
        }
    }

    pub fn get(&self) -> Vec3F {
        self.cached
    }

    pub fn new() -> Self {
        PhysicsSources {
            cached: Vec3F::new(Fixed::lit("0"), Fixed::lit("0"), Fixed::lit("0")),
            next_src_id: 0,
            sources: SmallVec::new(),
        }
    }
}

pub fn from_linear(vec: Vec2F) -> Vec3F {
    Vec3F::new(vec[0], vec[1], Fixed::lit("0"))
}

pub fn to_linear(vec: Vec3F) -> Vec2F {
    Vec2F::new(vec[0], vec[1])
}

pub fn to_angular(vec: Vec3F) -> Fixed {
    vec[2]
}
