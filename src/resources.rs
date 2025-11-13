use bevy::prelude::*;

#[derive(Resource, Debug, Clone, PartialEq)]
pub struct AITimer {
    pub timer: Timer,
}
