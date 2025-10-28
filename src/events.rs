use bevy::prelude::*;

/// Events

#[derive(Event)]
pub struct Action {
    pub position: Vec2,
}

#[derive(EntityEvent)]
pub struct SwitchSprite {
    pub entity: Entity,
    pub index: usize,
}
