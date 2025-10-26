use crate::items;
use bevy::prelude::*;

/// Component Types

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Player {
    pub money: f32,
    pub items: Vec<items::Item>,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Boat;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Land;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Ocean;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Building;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Sun;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Sky;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct ActionRange {
    pub range: f32,
}

#[derive(Component)]
pub struct AnimationConfig {
    pub first_index: usize,
    pub last_index: usize,
    pub timer: Timer,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Layer {
    pub depth: f32,
    pub size: Vec2,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct PlayerMenu;
