use bevy::prelude::*;

/// Events

#[derive(Event)]
pub struct Action {
    pub position: Vec2,
}

#[derive(Event)]
pub struct EndAction;

#[derive(Event)]
pub struct Catch;
