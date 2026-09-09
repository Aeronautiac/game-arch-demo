use bevy_ecs::{component::Component, entity::Entity};
use glam::Vec2;
use slotmap::new_key_type;

use crate::common::list_arena::ListArena;

/*
* The physics system is composed of two layers and two sweeps.
*
* Constraints are engine level orchestrators of physics modifiers, for example, you might have some
* kind of rope constraint between two entities that owns some force that must be updated every tick
* based on current simulation state.
*
* Physics modifiers are the base physics primitives. They are things like constant factors of
* velocity and forces.
*
* Both kinds of primitives are stored in a ListArena, which is a custom slotmap + doubly linked list
* hybrid data structure.
* It is designed this way to keep everything physics related contiguous.
* Individual entities do not own heap allocations. All physics objects are just data stored within one
* of two contiguous buffers.
*
* The first sweep is a constraint sweep. For every entity with a physics component, iterate through
* that entity's constraint list, and prepare a batch of deferred updates, then apply those updates
* to the constraint's owned physics modifiers.
* Batch building is necessary to avoid iteration order dependency.
*
* The second sweep is a raw physics sweep. For every entity with a physics component, iterate
* through the entity's modifier list, and directly mutate its values.
*/

new_key_type! {
    struct ModifierID;
}

pub enum ModifierKind {
    Velocity,
    AngularVelocity,
    Force,
    AngularForce,
}

pub struct Modifier {
    pub kind: ModifierKind,
    pub value: f32,
}

pub type ModifierArena = ListArena<ModifierID, Modifier>;

new_key_type! {
    struct ConstraintID;
}

pub enum Constraint {
    // THIS IS AN EXAMPLE CONSTRAINT
    // simulate gravity between two objects
    // objects must have mass
    GravityLink {
        g: f32,
        obj_a: Entity,
        obj_b: Entity,
    },
}

pub type ConstraintArena = ListArena<ConstraintID, Constraint>;

#[derive(Default)]
pub struct KinematicData {
    pub velocity: Vec2,
    pub angular_velocity: f32,
}

#[derive(Component, Default)]
pub struct Physics {
    pub kinematic_data: KinematicData,
    pub first_modifier: Option<ModifierID>,
    pub first_constraint: Option<ConstraintID>,
}

impl Physics {
    pub fn add_mod(&mut self, mod_arena: &mut ModifierArena, modifier: Modifier) -> ModifierID {
        mod_arena.add(&mut self.first_modifier, modifier)
    }

    pub fn remove_mod(
        &mut self,
        mod_arena: &mut ModifierArena,
        mod_id: ModifierID,
    ) -> Option<Modifier> {
        mod_arena.remove(&mut self.first_modifier, mod_id)
    }

    pub fn add_constraint(
        &mut self,
        constraint_arena: &mut ConstraintArena,
        constraint: Constraint,
    ) -> ConstraintID {
        constraint_arena.add(&mut self.first_constraint, constraint)
    }

    pub fn remove_constraint(
        &mut self,
        constraint_arena: &mut ConstraintArena,
        constraint_id: ConstraintID,
    ) -> Option<Constraint> {
        constraint_arena.remove(&mut self.first_constraint, constraint_id)
    }
}
