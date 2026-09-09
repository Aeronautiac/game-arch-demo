use bevy_ecs::component::Component;
use glam::Vec2;
use slotmap::new_key_type;

use crate::common::list_arena::ListArena;

new_key_type! {
    struct PhysicsModifierID;
}

pub enum ModifierKind {
    Velocity,
    AngularVelocity,
    Force,
    AngularForce,
}

pub struct PhysicsModifier {
    pub kind: ModifierKind,
    pub value: f32,
}

pub type PhysicsModifierArena = ListArena<PhysicsModifierID, PhysicsModifier>;

#[derive(Component, Default)]
pub struct Physics {
    pub velocity: Vec2,
    pub angular_velocity: f32,
    pub first_modifier: Option<PhysicsModifierID>,
}

impl Physics {
    pub fn add_mod(
        &mut self,
        mod_arena: &mut PhysicsModifierArena,
        modifier: PhysicsModifier,
    ) -> PhysicsModifierID {
        let node = PhysicsNode {
            next_sibling: self.first_modifier,
            prev_sibling: None,
            modifier,
        };
        let key = arena.nodes.insert(node);
        self.first_modifier = Some(key);
        key
    }

    pub fn remove_mod(&mut self, arena: &mut PhysicsArena, modifier: PhysicsNodeKey) {}
}
