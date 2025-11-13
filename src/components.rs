use crate::items::{self, FishType};
use bevy::prelude::*;

/// Component Types

#[derive(Component, Debug, Clone, PartialEq)]
pub struct DefaultColor {
    pub color: Color,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct DayNightColor;

#[derive(Default, Component, Debug, Clone, PartialEq)]
pub struct AnimationTimer {
    pub timer: Timer,
}

#[derive(Default, Component, Debug, Clone, PartialEq)]
pub enum AnimationState {
    Run,
    #[default]
    Finish,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct AnimationConfig {
    pub first_index: usize,
    pub last_index: usize,
    // Duration per frame in ms
    pub ms: u64,
    pub mode: TimerMode,
}

#[derive(Component, Debug, Clone)]
pub struct SpriteCollection {
    pub sprites: Vec<Sprite>,
    pub animations: Vec<AnimationConfig>,
}

#[derive(Component, Debug, Clone)]
pub struct ActiveSprite {
    pub index: usize,
}

#[derive(Default, Component, Debug, Clone, PartialEq)]
pub struct Velocity {
    pub value: f32,
}

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
pub struct Fish {
    pub t: FishType,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Land {
    pub size: Vec2,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Ocean {
    pub size: Vec2,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Building;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Sun;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Cloud {
    pub speed: f32,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Sky;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct ActionRange {
    pub range: f32,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Layer {
    pub depth: f32,
    pub speed: f32,
    pub size: Vec2,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct PlayerLayer;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct BoatLayer;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct CityLayer;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct SkyLayer;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct PlayerMenu;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct OnControl;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct OnAI;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum PlayerState {
    Idle,
    Walk,
    Row,
    Fish,
    Hook,
    Attack,
}
