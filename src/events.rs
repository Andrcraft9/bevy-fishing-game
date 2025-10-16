use bevy::prelude::*;

/// Events

#[derive(Event)]
pub struct Action {
    pub position: Vec2,
}