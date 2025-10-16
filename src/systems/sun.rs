use bevy::prelude::*;
use crate::{components::Sun, constants::K_WIDTH};

pub fn sun_update(time: Res<Time>, mut sun_query: Single<&mut Transform, With<Sun>>) {
    sun_query.translation.x = (0.5 * time.elapsed_secs()).sin() * K_WIDTH / 2.0;
}