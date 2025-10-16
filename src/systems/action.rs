use bevy::prelude::*;
use crate::{components::Building, events::Action, constants::K_ACTION_RADIUS};

pub fn on_action(action: On<Action>, query: Query<(&Transform, &Name), With<Building>>) {
    let action_position = action.event().position;
    println!("On Action!");
    for (transform, name) in query.iter() {
        let distance = (action_position.x - transform.translation.x).abs();
        if distance <= K_ACTION_RADIUS {
            println!(
                "Found building '{}' at distance {:.2} from action",
                name, distance
            );
        }
    }
}