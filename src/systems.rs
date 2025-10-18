use crate::{
    components::{Building, Layer, Player, Sun},
    constants::{K_ACTION_RADIUS, K_HEIGHT, K_SPEED, K_WIDTH},
    events::Action,
};
use bevy::prelude::*;

pub fn on_action(action: On<Action>, query: Query<(&GlobalTransform, &Name), With<Building>>) {
    let action_position = action.event().position;
    info!("On Action!");
    for (transform, name) in query.iter() {
        let distance = (action_position.x - transform.translation().x).abs();
        if distance <= K_ACTION_RADIUS {
            info!(
                "Found building '{}' at distance {:.2} from action",
                name, distance
            );
        }
    }
}

pub fn player_action(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Single<&Transform, With<Player>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        info!("Action!");
        commands.trigger(Action {
            position: Vec2::new(player_query.translation.x, player_query.translation.y),
        });
    }
}

pub fn player_control(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Single<&mut Transform, With<Player>>,
) {
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
        debug!("Moving left!");
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
        debug!("Moving right!");
    }

    player_query.translation += direction * K_SPEED * time.delta_secs();

    if direction != Vec3::ZERO {
        debug!("Player position: {:?}", player_query.translation);
    }
}

pub fn layer_update(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Layer)>,
) {
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction -= 1.0;
    }

    for (mut transform, layer) in player_query.iter_mut() {
        if layer.depth <= 0.0 {
            transform.translation.x += direction * K_SPEED * time.delta_secs();
            if direction != 0.0 {
                debug!("Layer transform: {:?}", transform.translation);
            }
        }
    }
}

pub fn sun_update(time: Res<Time>, mut sun_query: Single<&mut Transform, With<Sun>>) {
    sun_query.translation.y = (0.5 * time.elapsed_secs()).sin() * K_HEIGHT / 2.0;
}
