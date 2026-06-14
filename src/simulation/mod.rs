use hecs::World;

use crate::{
    common::Time,
    simulation::action::{Action, ActionResult},
};

pub mod action;

mod systems;

pub struct SimInteraction {
    pub action: Action,
    pub dt: Time, // a simple time increment. using this rather than timestamps prevents
                  // events that go back in time from even being a possibility enforcing the invariant on a
                  // structural level.
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

    pub fn exec(&mut self, mut interaction: SimInteraction) -> ActionResult {
        // TODO:
        // physics loop and shit.
        // the action execution is a simple conditional state mutation
        // the simulation is updated regardless of the outcome of that conditional mutation

        let real_dt = interaction.dt + self.excess;

        // TODO:
        // handle rejections
        let _ = interaction.action.exec(self);

        Ok(None)
    }
}
