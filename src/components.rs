use bevy::prelude::*;

/// Component Types

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Player;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Ocean;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Building;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Sun;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Layer {
    pub depth: f32,
    pub size: Vec2,
}
