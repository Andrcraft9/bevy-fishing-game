use bevy::prelude::*;

/// Events

#[derive(Event)]
pub struct Action;

#[derive(Event)]
pub struct EndAction;

#[derive(Event)]
pub struct Hook;

#[derive(Event)]
pub struct Catch;

#[derive(Event)]
pub struct Hit;

#[derive(Event)]
pub struct Sell;
