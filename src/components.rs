use bevy::prelude::*;

/// Component Types

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Player;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Building;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Sun;

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectComponentType {
    Player(Player),
    Building(Building),
    Sun(Sun),
}