use bevy_ecs::entity::Entity;

use crate::simulation::{Simulation, ecs::movement::MovementIntent};

pub type ActionResult = Result<Option<Response>, ActionError>;

#[derive(Clone)]
pub enum ActionError {
    EntityNotFound,
}

// responses will use generalized structures
// you must extract the correct response based on context
#[derive(Clone)]
pub enum Response {
    Entity(Entity),
}

// before any action is executed, all the ticks within dt will be simulated.
// to implement time dilation, simply multiply the dt passed into the sim interaction by some factor.
#[derive(Clone)]
pub enum Action {
    Null, // null actions are ephemeral on the timeline and are intended only to force a state update for rendering
    SetMovementIntent { entity: Entity, dir: MovementIntent },
}

impl Action {
    fn handle(&mut self, sim: &mut Simulation, mutate: bool) -> ActionResult {
        match self {
            Self::Null => Ok(None),
            Self::SetMovementIntent { entity: _, dir: _ } => Ok(None),
        }
    }

    pub fn exec(&mut self, sim: &mut Simulation) -> ActionResult {
        self.handle(sim, true)
    }

    pub fn validate(&mut self, sim: &mut Simulation) -> ActionResult {
        self.handle(sim, false)
    }
}
