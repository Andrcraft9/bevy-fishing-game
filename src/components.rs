use bevy::prelude::*;

/// Component Types

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Player;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Building;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Sun;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Layer {
    pub size: Vec2,
}
