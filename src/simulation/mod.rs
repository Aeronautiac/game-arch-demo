use bevy_ecs::world::World;
use fixed::types::I32F32;

use crate::{
    common::{Tick, Time},
    simulation::{
        action::{Action, ActionResult},
        ecs::{movement::movement_system, transform::Transform, velocity::apply_velocity},
    },
};

pub mod action;

pub mod ecs;

#[derive(Clone)]
pub struct SimInteraction {
    pub action: Action,
    pub dt: Time, // a simple time increment. using this rather than timestamps prevents
                  // events that go back in time from even being a possibility enforcing the invariant on a
                  // structural level.
}

#[derive(Clone)]
pub struct SimView {
    pub x: I32F32,
    pub y: I32F32,
    pub tick: Tick,
}

pub struct SimOutput {
    pub view: SimView,
    pub action_result: ActionResult,
}

pub struct Simulation {
    pub excess: Time,
    pub tick: Tick,
    pub world: World,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            excess: 0,
            tick: 0,
            world: World::new(),
        }
    }

    // the identifier is a u64
    pub fn exec(&mut self, mut interaction: SimInteraction) -> SimOutput {
        // TODO:
        // the action execution is a simple conditional state mutation
        // the simulation is updated regardless of the outcome of that conditional mutation

        let tick_out = self.tick_loop(interaction.dt);

        // TODO:
        // handle rejections later
        let action_result = interaction.action.exec(self);

        SimOutput {
            view: tick_out,
            action_result,
        }
    }

    fn tick_loop(&mut self, total_dt: Time) -> SimView {
        let mut remaining_time = total_dt + self.excess;
        // dbg!(remaining_time);
        // dbg!(self.excess);
        loop {
            let tick_duration = self.get_tick_duration();
            if remaining_time < tick_duration {
                self.excess = remaining_time;
                break;
            }

            movement_system(&mut self.world);
            apply_velocity(&mut self.world, tick_duration);

            remaining_time -= tick_duration;
            self.tick += 1;
        }
        // dbg!(self.tick);

        let mut query = self.world.query::<&Transform>();
        let position = query.iter(&self.world).next();
        if let Some(pos) = position {
            // dbg!(pos.x, pos.y);
            SimView {
                x: pos.x,
                y: pos.y,
                tick: self.tick,
            }
        } else {
            SimView {
                x: 0.into(),
                y: 0.into(),
                tick: self.tick,
            }
        }
    }

    fn get_tick_duration(&self) -> Time {
        1_000_000_000 / 240
    }
}
