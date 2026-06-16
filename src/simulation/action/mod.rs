use bevy_ecs::entity::Entity;

use crate::simulation::{
    Simulation,
    ecs::{
        movement::{Movement, MovementIntent},
        transform::Transform,
        velocity::Velocity,
    },
};

pub type ActionResult = Result<ActionResponse, ActionError>;

#[derive(Debug)]
pub enum ActionError {
    EntityNotFound,
}

// responses will use generalized structures
// you must extract the correct response based on context
pub enum ActionResponse {
    Null,
    Entity(Entity),
}

// before any action is executed, all the ticks within dt will be simulated.
// to implement time dilation, simply multiply the dt passed into the sim interaction by some factor.
#[derive(Clone)]
pub enum Action {
    Null, // null actions are ephemeral on the timeline and are intended only to force a state update for rendering
    SetMovementIntent {
        target: Entity,
        intent: MovementIntent,
    },
    CreateShip,
}

impl Action {
    fn handle(&mut self, sim: &mut Simulation, mutate: bool) -> ActionResult {
        match self {
            Self::Null => Ok(ActionResponse::Null),
            Self::SetMovementIntent { target, intent } => {
                let mut entity = sim.world.entity_mut(*target);
                let mut mvmt = entity.get_mut::<Movement>().unwrap();
                if mutate {
                    mvmt.intent = *intent;
                    // dbg!(mvmt.intent);
                }
                Ok(ActionResponse::Null)
            }
            Self::CreateShip => {
                if mutate {
                    let ship = sim
                        .world
                        .spawn((
                            Movement {
                                intent: MovementIntent::EMPTY,
                                boost: 0.into(),
                            },
                            Velocity {
                                x: 0.into(),
                                y: 0.into(),
                            },
                            Transform {
                                x: 0.into(),
                                y: 0.into(),
                            },
                        ))
                        .id();
                    Ok(ActionResponse::Entity(ship))
                } else {
                    Ok(ActionResponse::Null)
                }
            }
        }
    }

    pub fn exec(&mut self, sim: &mut Simulation) -> ActionResult {
        self.handle(sim, true)
    }

    pub fn validate(&mut self, sim: &mut Simulation) -> ActionResult {
        self.handle(sim, false)
    }
}
