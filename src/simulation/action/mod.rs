use hecs::Entity;

use crate::simulation::Simulation;

pub type ActionResult = Result<Option<Response>, ActionError>;

pub enum ActionError {
    EntityNotFound,
}

// responses will use generalized structures
// you must extract the correct response based on context
pub enum Response {
    Entity(Entity),
}

// before any action is executed, all the ticks within dt will be simulated.
// to implement time dilation, simply multiply the dt passed into the sim interaction by some factor.
pub enum Action {
    Null, // null actions are ephemeral on the timeline and are intended only to force a state update for rendering
    // or similar
    Jump { entity: Entity },
    StartMove { entity: Entity, dir: MoveDir },
    StopMove { entity: Entity, dir: MoveDir },
}

impl Action {
    // passing a mutable reference for both passes removes code duplication.
    // there may be actions which have conditional paths for rejection conditions, so if we were to
    // call two separate functions, we'd essentially just be duplicating that code for no reason.
    fn handle(&mut self, sim: &mut Simulation, mutate: bool) -> ActionResult {
        match self {
            Self::Null => Ok(None),
            Self::Jump { entity } => Ok(None),
            Self::StartMove { entity, dir } => Ok(None),
            Self::StopMove { entity, dir } => Ok(None),
        }
    }

    pub fn exec(&mut self, sim: &mut Simulation) -> ActionResult {
        self.handle(sim, true)
    }

    pub fn validate(&mut self, sim: &mut Simulation) -> ActionResult {
        self.handle(sim, false)
    }
}
