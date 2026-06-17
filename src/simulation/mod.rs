use std::collections::VecDeque;

use bevy_ecs::world::World;

use crate::{
    common::{
        Fixed, Tick,
        time::{Time, TimeBase},
    },
    simulation::{
        action::{Action, ActionResult},
        ecs::{
            movement::movement_system,
            physics::{
                forces::apply_forces,
                velocity::{Velocity, apply_velocity},
            },
            transform::Transform,
        },
    },
};

pub mod action;
pub mod ecs;

#[derive(Clone)]
pub struct SimInteraction {
    pub action: Action,
    pub dt: TimeBase, // a simple time increment. using this rather than timestamps prevents
                      // events that go back in time from even being a possibility enforcing the invariant on a
                      // structural level.
}

#[derive(Clone)]
pub struct VpEntity {
    pub pos: Transform,
    pub vel: Velocity,
}

#[derive(Clone)]
pub struct Viewport {
    pub entities: Vec<VpEntity>, // find a way to make this more performant later
}

#[derive(Clone)]
pub struct TickView {
    pub viewports: Vec<Viewport>,
    pub tick: Tick,
}

#[derive(Clone)]
pub struct SimView {
    pub tick_views: VecDeque<TickView>,
}

// to merge sim views, just append the one with the latest first tick time to the other
// simviews will not have overlapping ticks
impl SimView {
    // for now just loop through to find the tick, but we can use a binary search later to find the
    // cut point
    pub fn prune_to(&mut self, tick: Tick) {
        while !self.tick_views.is_empty() {
            if self.tick_views[0].tick < tick {
                self.tick_views.pop_front();
            } else {
                break;
            }
        }
    }

    pub fn merge_with(&mut self, mut other: Self) {
        if other.tick_views.is_empty() {
            return;
        }
        if self.tick_views.is_empty() {
            self.tick_views = other.tick_views;
            return;
        }
        if other.tick_views[0].tick > self.tick_views[0].tick {
            self.tick_views.append(&mut other.tick_views);
        } else {
            other.tick_views.append(&mut self.tick_views);
            self.tick_views = other.tick_views;
        }
    }
}

pub struct SimOutput {
    pub view: SimView,
    pub action_result: ActionResult,
}

pub struct Simulation {
    pub excess: TimeBase,
    pub tick: Tick,
    pub world: World,
    pub view_stagger: u64, // views are sampled at the start and end of the tick loop, and
                           // within the loop, views are pulled every x ticks
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            excess: 0,
            tick: 0,
            world: World::new(),
            view_stagger: 10,
        }
    }

    // the identifier is a u64
    // the action execution is a simple conditional state mutation
    // the simulation is updated regardless of the outcome of that conditional mutation
    pub fn exec(&mut self, mut interaction: SimInteraction) -> SimOutput {
        let tick_out = self.tick_loop(interaction.dt);
        let action_result = interaction.action.exec(self);
        SimOutput {
            view: tick_out,
            action_result,
        }
    }

    fn get_viewport(&mut self) -> Viewport {
        let mut vp = Viewport { entities: vec![] };
        let mut query = self.world.query::<(&Velocity, &Transform)>();
        for (vel, transform) in query.iter(&self.world) {
            vp.entities.push(VpEntity {
                vel: vel.clone(),
                pos: transform.clone(),
            });
        }
        vp
    }

    fn tick_loop(&mut self, total_dt: TimeBase) -> SimView {
        let mut view = SimView {
            tick_views: VecDeque::new(),
        };

        let mut remaining_time = total_dt + self.excess;
        let mut first_tick = true;
        let mut stagger: u64 = self.view_stagger;
        loop {
            let tick_duration = self.get_tick_duration();
            if remaining_time < tick_duration.pure() {
                view.tick_views.push_back(TickView {
                    viewports: vec![self.get_viewport()],
                    tick: self.tick,
                });
                self.excess = remaining_time;
                break;
            }

            movement_system(&mut self.world);
            apply_forces(&mut self.world, tick_duration);
            apply_velocity(&mut self.world, tick_duration);

            if first_tick || stagger == 0 {
                view.tick_views.push_back(TickView {
                    viewports: vec![self.get_viewport()],
                    tick: self.tick,
                });
            }

            remaining_time -= tick_duration.pure();
            self.tick += 1;

            if stagger == 0 {
                stagger = self.view_stagger;
            } else {
                stagger -= 1;
            }

            first_tick = false;
        }

        view
    }

    // TODO:
    // determine precision requirements based on cached max speeds and smallest distances
    fn get_tick_duration(&self) -> Time {
        // placeholder. this isnt intended to be fixed timestep.
        Time::from_sec(Fixed::from_num(1) / Fixed::from_num(240))
    }
}
