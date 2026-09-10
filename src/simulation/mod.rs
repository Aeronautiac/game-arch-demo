use std::collections::VecDeque;

use bevy_ecs::{entity::Entity, schedule::Schedule, world::World};
use glam::Vec2;

use crate::simulation::ecs::transform::Transform;

pub mod ecs;

pub type TickInt = u64;
pub type TimeInt = u64;
pub type TimeFloat = f32;

#[derive(Debug)]
pub enum SimErr {
    EntityNotFound,
}

// responses will use generalized structures
// you must extract the correct response based on context
pub enum SimResponse {
    Null,
    Entity(Entity),
}

pub type SimResult = Result<SimResponse, SimErr>;

// before any action is executed, all the ticks within dt will be simulated.
// to implement time dilation, simply multiply the dt passed into the sim interaction by some factor.
#[derive(Clone)]
pub enum InputPayload {
    Null, // null actions are ephemeral on the timeline and are intended only to force a state update for rendering
    CreatePlayer,
}

impl InputPayload {
    pub fn exec(&mut self, sim: &mut Simulation) -> SimResult {
        match self {
            Self::Null => Ok(SimResponse::Null),
            Self::CreatePlayer => Ok(SimResponse::Null),
        }
    }
}

#[derive(Clone)]
pub struct SimInput {
    pub payload: InputPayload,
    pub dt: TimeInt,
}

#[derive(Clone)]
pub enum ViewDataPayload {
    Physical {
        entity_id: Entity,
        transform: Transform,
        velocity: Vec2,
        angular_velocity: f32,
    },
}

#[derive(Clone)]
pub struct ViewData {
    pub payload: ViewDataPayload,
    pub tick: TickInt,
}

#[derive(Clone)]
pub struct LossyViewBuffer {
    pub data: VecDeque<ViewData>,
}

impl LossyViewBuffer {
    pub fn new(initial_capacity: usize) -> Self {
        let mut v = VecDeque::new();
        v.reserve(initial_capacity);
        LossyViewBuffer { data: v }
    }

    // if there are too many entries in the batch, we reserve more room so nothing is immediately lost.
    // also, if even after a compaction, there's still not enough room, we expand.
    //
    // degradation only occurs across multiple ticks, i.e., when we overflow on current storage,
    // but not capacity. we then compact, losing an even distribution of half of all data.
    pub fn append(&mut self, batch: &mut Vec<ViewData>) {
        let batch_len = batch.len();
        let capacity = self.data.capacity();
        let size = self.data.len();

        // not enough total capacity, grow
        if batch_len > capacity {
            let diff = batch_len - capacity;
            self.data.reserve(diff);
        }
        let capacity = self.data.capacity();

        // storage overflow
        if batch_len + size > capacity {
            self.compact();
        }
        let size = self.data.len();

        // if theres not enough space even after a compaction, grow
        if batch_len + size > capacity {
            let diff = batch_len - capacity;
            self.data.reserve(diff);
        }

        self.data.extend(batch.iter().cloned());
    }

    // discard odd indices by iterating and replacing, then cutting the tail.
    // compaction is supposed to be extremely rare. it is a worst case scenario.
    pub fn compact(&mut self) {
        let mut idx: usize = 0;
        for i in 0..self.data.len() {
            // extract even numbered indices
            if i % 2 == 0 {
                let data = self.data[i].clone();
                self.data[idx] = data;
                idx += 1;
            }
        }
        self.data.truncate(idx);
    }

    // binary search to find the cut point (where tick > cut_tick), then cut off the prefix.
    pub fn discard_to(&mut self, cut_tick: TickInt) {
        let cut_idx = self.data.partition_point(|data| data.tick <= cut_tick);
        self.data.drain(0..cut_idx);
    }
}

pub struct Simulation {
    excess: u64,
    tick: u64,
    world: World,
    schedule: Schedule,

    // output buffers
    lossy_view_stage: Vec<ViewDataPayload>,
    pub lossy_view_buf: LossyViewBuffer,
}

impl Simulation {
    pub fn new() -> Self {
        let schedule = Schedule::default();

        Simulation {
            excess: 0,
            tick: 0,
            world: World::new(),
            lossy_view_stage: Vec::default(),
            lossy_view_buf: LossyViewBuffer::new(4096),
            schedule,
        }
    }

    // the identifier is a u64
    // the action execution is a simple conditional state mutation
    // the simulation is updated regardless of the outcome of that conditional mutation
    pub fn exec(&mut self, mut interaction: SimInput) -> SimResult {
        let res = interaction.payload.exec(self);
        self.tick_loop(interaction.dt);
        res
    }

    fn tick_loop(&mut self, total_dt: u64) {
        let mut remaining_time = total_dt + self.excess;
        loop {
            let tick_duration = self.get_tick_duration();
            if remaining_time < tick_duration {
                self.excess = remaining_time;
                break;
            }

            // movement_system(&mut self.world);
            // apply_forces(&mut self.world, tick_duration);
            // apply_velocity(&mut self.world, tick_duration);
            self.schedule.run(&mut self.world);

            remaining_time -= tick_duration;
            self.tick += 1;
        }
    }

    // TODO:
    // determine precision requirements based on cached max speeds and smallest distances
    fn get_tick_duration(&self) -> u64 {
        // placeholder. this isnt intended to be fixed timestep.
        1_000_000_000 / 120
    }
}
