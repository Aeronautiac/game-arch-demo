use std::collections::HashMap;

use bevy_ecs::world::World;

use crate::{
    common::Time,
    simulation::{
        action::{Action, ActionResult},
        ecs::{movement::movement_system, velocity::apply_velocity},
    },
};

pub mod action;

mod ecs;

#[derive(Clone)]
pub struct SimInteraction {
    pub action: Action,
    pub dt: Time, // a simple time increment. using this rather than timestamps prevents
                  // events that go back in time from even being a possibility enforcing the invariant on a
                  // structural level.
}

// TODO: world serialization/snapshot for the transport layer
#[derive(Clone, Default)]
pub struct SimViewport;

// this will include viewports, responses, and a "consumed" flag
// for now viewports will just be world snapshots after the entire ticking loop
// responses will also just use a map right now to make things simple,
// but this will hurt performance in serious games
#[derive(Clone)]
pub struct SimOutput {
    pub viewport: SimViewport,
    pub results: HashMap<u64, ActionResult>,
    pub consumed: bool,
}

pub struct Simulation {
    pub excess: Time,
    pub world: World,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            excess: 0,
            world: World::new(),
        }
    }

    // the identifier is a u64
    pub fn exec(&mut self, mut interaction: SimInteraction) -> ActionResult {
        // TODO:
        // the action execution is a simple conditional state mutation
        // the simulation is updated regardless of the outcome of that conditional mutation

        self.tick_loop(interaction.dt);

        // TODO:
        // handle rejections later
        let _ = interaction.action.exec(self);

        Ok(None)
    }

    fn tick_loop(&mut self, total_dt: Time) {
        loop {
            let remaining_time = total_dt + self.excess;
            let dt = self.get_tick_duration();
            if remaining_time < dt {
                self.excess = remaining_time;
                break;
            }

            movement_system(&mut self.world);
            apply_velocity(&mut self.world, dt);
        }
    }

    // for now fix at 60hz, later this is chosen based on current state
    fn get_tick_duration(&self) -> Time {
        1_000_000 / 60
    }
}
