use crate::{
    components::{ActionRange, Building, Layer, Ocean, Player, PlayerMenu, Sun},
    constants::{K_HEIGHT, K_SPEED, K_WIDTH},
    events::Action,
    items::{self, Value, Weight},
    states::GameState,
};
use bevy::app::AppExit;
use bevy::prelude::*;
use rand::Rng;

pub fn on_action(
    action: On<Action>,
    mut player: Single<&mut Player>,
    oceans: Query<(&GlobalTransform, &Name, &ActionRange), With<Ocean>>,
    buildings: Query<(&GlobalTransform, &Name, &ActionRange), With<Building>>,
) {
    let action_position = action.event().position;
    info!("On Action!");

    for (transform, name, action_range) in oceans.iter() {
        let distance = (action_position.x - transform.translation().x).abs();
        if distance <= action_range.range {
            info!(
                "Found ocean '{}' at distance {:.2} from action",
                name, distance
            );
            info!("Fishing...");
            // Random fishing logic
            let mut rng = rand::thread_rng();
            let chance: f32 = rng.gen_range(0.0..1.0);
            if chance < 0.1 {
                // 10% chance for golden fish
                let weight = rng.gen_range(0.1..20.0);
                player.items.push(items::Item::Fish(items::Fish {
                    t: items::FishType::Golden,
                    weight,
                }));
                info!("Caught a Golden fish! Weight: {:.2}", weight);
            } else if chance < 0.5 {
                // 40% chance for silver fish (0.1 to 0.5 range)
                let weight = rng.gen_range(0.1..20.0);
                player.items.push(items::Item::Fish(items::Fish {
                    t: items::FishType::Silver,
                    weight,
                }));
                info!("Caught a Silver fish! Weight: {:.2}", weight);
            } else {
                // 50% chance for nothing (0.5 to 1.0 range)
                info!("Nothing caught this time...");
            }
        }
    }

    for (transform, name, action_range) in buildings.iter() {
        let distance = (action_position.x - transform.translation().x).abs();
        if distance <= action_range.range {
            info!(
                "Found building '{}' at distance {:.2} from action",
                name, distance
            );
            info!("Selling...");
            let item = player.items.pop();
            if let Some(item) = item {
                info!("Sold item: {}", item.name());
                player.money += item.value();
            }
        }
    }
}

pub fn global_action(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: MessageWriter<AppExit>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if *state.get() != GameState::InGame {
            info!("Back in Game!");
            next_state.set(GameState::InGame);
        } else {
            info!("Quitting app!");
            app_exit_events.write(AppExit::Success);
        }
    }
}

pub fn game_player_action(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Single<&Transform, With<Player>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        info!("Action!");
        commands.trigger(Action {
            position: Vec2::new(player_query.translation.x, player_query.translation.y),
        });
    }

    if keyboard_input.just_pressed(KeyCode::Tab) {
        info!("Player menu!");
        next_state.set(GameState::InPlayerMenu);
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

pub fn sun_update(time: Res<Time<Virtual>>, mut sun_query: Single<&mut Transform, With<Sun>>) {
    sun_query.translation.y = (0.5 * time.elapsed_secs()).sin() * K_HEIGHT / 2.0;
}

pub fn enter_player_menu(
    mut commands: Commands,
    mut time: ResMut<Time<Virtual>>,
    player: Single<&Player>,
) {
    info!("Creating player menu");
    time.pause();

    let money = format!("Money: {}", player.money.to_string());
    commands.spawn((Text::new(money), TextFont::from_font_size(48.0), PlayerMenu));

    commands
        .spawn((
            Node {
                width: percent(50),
                height: percent(50),
                top: percent(25),
                left: percent(25),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            PlayerMenu,
            BackgroundColor(Color::srgb(0.50, 0.50, 0.50)),
        ))
        .with_children(|parent| {
            for item in &player.items {
                parent.spawn((
                    Text::new(format!(
                        "Item: {}; Weight: {:.2}",
                        item.name(),
                        item.weight()
                    )),
                    Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                ));
            }
        });
}

pub fn exit_player_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<PlayerMenu>>,
    mut time: ResMut<Time<Virtual>>,
) {
    info!("Removing player menu");
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
    time.unpause();
}
