use bevy_ecs::{
    component::Component,
    system::{Query, Res, ResMut},
};
use glam::Vec2;
use slotmap::new_key_type;

use crate::{
    common::list_arena::ListArena,
    simulation::{Dt, PhysicsArenas, ecs::transform::Transform},
};

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
    pub struct ModifierID;
}

pub enum Modifier {
    Velocity(Vec2),
    AngularVelocity(f32),
    Force(Vec2),
    AngularForce(f32),
}

pub type ModifierArena = ListArena<ModifierID, Modifier>;

new_key_type! {
    pub struct ConstraintID;
}

pub enum Constraint {}

pub type ConstraintArena = ListArena<ConstraintID, Constraint>;

#[derive(Default)]
pub struct KinematicData {
    base_vel: Vec2,
    var_vel: Vec2,
    base_ang_vel: f32,
    var_ang_vel: f32,
}

impl KinematicData {
    pub fn vel(&self) -> Vec2 {
        self.base_vel + self.var_vel
    }

    pub fn ang_vel(&self) -> f32 {
        self.base_ang_vel + self.var_ang_vel
    }
}

#[derive(Component, Default)]
pub struct Physics {
    pub kinematic_data: KinematicData,
    first_modifier: Option<ModifierID>,
    first_constraint: Option<ConstraintID>,
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

struct ModifierUpdate {
    pub mod_id: ModifierID,
    pub new_val: Modifier,
}

// modifies only physics values, does not apply the values to position or similar.
pub fn physics_values(
    dt: Res<Dt>,
    mut physics_arenas: ResMut<PhysicsArenas>,
    mut query: Query<&mut Physics>,
) {
    // TODO:
    // use something like a scratchpad arena instead to avoid an alloc on every tick
    //
    // let mut constraint_stage: Vec<ModifierUpdate> = Vec::new();

    // constraint building pass
    // for (mut physics) in &mut query {}

    // constraint application pass

    // modifier application pass
    for mut physics in &mut query {
        physics.kinematic_data.base_vel = Vec2::new(0.0, 0.0);
        physics.kinematic_data.base_ang_vel = 0.0;
        physics_arenas
            .modifiers
            .for_each_mut(physics.first_modifier, |m| match *m {
                Modifier::Velocity(val) => physics.kinematic_data.base_vel += val,
                Modifier::AngularVelocity(val) => physics.kinematic_data.base_ang_vel += val,
                Modifier::Force(val) => physics.kinematic_data.var_vel += val * dt.0,
                Modifier::AngularForce(val) => physics.kinematic_data.var_ang_vel += val * dt.0,
            });
    }
}

// applies physics to transforms and handles things like collision detection (does not technically
// need to be run in serial, we can likely optimize)
pub fn physics_application(dt: Res<Dt>, mut query: Query<(&mut Transform, &Physics)>) {
    for (mut transform, physics) in &mut query {
        transform.position += physics.kinematic_data.vel() * dt.0;
        let rot = transform.get_rotation();
        transform.set_rotation(rot + physics.kinematic_data.ang_vel() * dt.0);
    }
}

pub fn collision_detection() {}
