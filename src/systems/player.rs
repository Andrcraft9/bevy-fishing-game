use bevy::prelude::*;
use crate::{components::Player, events::Action};

pub fn player_action(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Single<&Transform, With<Player>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("Action!");
        commands.trigger(Action {
            position: Vec2::new(player_query.translation.x, player_query.translation.y),
        });
    }
}

pub fn player_control(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Single<&mut Transform, With<Player>>
) {
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
        println!("Moving left!");
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
        println!("Moving right!");
    }

    let speed = 150.0;
    player_query.translation += direction * speed * time.delta_secs();

    if direction != Vec3::ZERO {
        println!("Player position: {:?}", player_query.translation);
    }
}