use std::time::Duration;

use crate::{
    components::{
        ActionRange, AnimationConfig, AnimationTimer, Boat, Building, Direction, Land, Layer,
        Ocean, OnControl, OnLand, OnOcean, Player, PlayerMenu, Sun,
    },
    constants::{K_GROUND_LEVEL, K_HEIGHT, K_SPEED, K_WIDTH},
    events::{Action, SwitchSprite},
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
    player_transform: Single<&GlobalTransform, With<Player>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        info!("Player menu!");
        next_state.set(GameState::InPlayerMenu);
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        info!("Action!");
        commands.trigger(Action {
            position: Vec2::new(
                player_transform.translation().x,
                player_transform.translation().y,
            ),
        });
        return;
    }
}

pub fn player_on_ocean(
    player: Single<(Entity, &mut Transform), (With<Player>, Added<OnOcean>)>,
    boat: Single<Entity, With<Boat>>,
    mut commands: Commands,
) {
    let (entity, mut transform) = player.into_inner();
    commands.trigger(SwitchSprite {
        entity: entity,
        index: 1,
    });

    // TODO: Abstract away from the system.
    info!("Sit!");
    transform.translation.y = K_GROUND_LEVEL + 40.0;

    // TODO: Abstract away from the system.
    commands.entity(boat.entity()).insert(OnControl);
}

pub fn player_on_land(
    player: Single<(Entity, &mut Transform), (With<Player>, Added<OnLand>)>,
    boat: Single<(Entity, &mut Transform), (Without<Player>, With<Boat>)>,
    mut commands: Commands,
) {
    let (entity, mut transform) = player.into_inner();
    commands.trigger(SwitchSprite {
        entity: entity,
        index: 0,
    });

    // TODO: Abstract away from the system.
    info!("Stand!");
    transform.translation.y = K_GROUND_LEVEL + 64.0;

    // TODO: Abstract away from the system.
    let (boat_entity, mut boat_transform) = boat.into_inner();
    commands.entity(boat_entity).remove::<OnControl>();

    // TODO: Fix.
    boat_transform.translation.x = 512.0;
}

pub fn player_on_land_ocean(
    player: Single<(Entity, &GlobalTransform, Option<&OnLand>, Option<&OnOcean>), With<Player>>,
    oceans: Query<(&Ocean, &GlobalTransform)>,
    lands: Query<(&Land, &GlobalTransform)>,
    mut commands: Commands,
) {
    let (entity, transform, on_land, on_ocean) = player.into_inner();

    if let Some(_) = on_ocean {
        for (land, land_transform) in lands.iter() {
            if transform.translation().x > land_transform.translation().x - land.size.x / 2.0
                && transform.translation().x < land_transform.translation().x + land.size.x / 2.0
            {
                info!("Hit land!");
                commands.entity(entity).remove::<OnOcean>();
                commands.entity(entity).insert(OnLand);
            }
            break;
        }
    }

    if let Some(_) = on_land {
        for (ocean, ocean_transform) in oceans.iter() {
            if transform.translation().x > ocean_transform.translation().x - ocean.size.x / 2.0
                && transform.translation().x < ocean_transform.translation().x + ocean.size.x / 2.0
            {
                info!("Hit ocean!");
                commands.entity(entity).remove::<OnLand>();
                commands.entity(entity).insert(OnOcean);
                break;
            }
        }
    }
}

pub fn changed_direction(sprites: Query<(&mut Sprite, &Direction), Changed<Direction>>) {
    for (mut sprite, direction) in sprites.into_iter() {
        if *direction == Direction::Left {
            sprite.flip_x = true;
        } else {
            sprite.flip_x = false;
        }
    }
}

pub fn animation_control(animations: Query<(&mut AnimationTimer, &AnimationConfig, &mut Sprite)>) {
    for (mut animation, config, mut sprite) in animations.into_iter() {
        if animation.timer.is_finished() {
            animation.timer = Timer::new(Duration::from_millis(animation.ms), TimerMode::Once);
            if let Some(atlas) = &mut sprite.texture_atlas {
                if animation.reset {
                    atlas.index = config.first_index;
                } else if atlas.index >= config.last_index {
                    atlas.index = config.first_index;
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}

pub fn movement_control(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    objects: Query<(&mut Transform, Option<&mut Direction>), With<OnControl>>,
    animations: Query<&mut AnimationTimer, With<OnControl>>,
    layers: Query<(&mut Transform, &Layer), Without<OnControl>>,
) {
    let mut mov = 0.0;
    if keyboard_input.pressed(KeyCode::KeyA) {
        mov += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        mov -= 1.0;
    }

    for mut animation in animations.into_iter() {
        if mov != 0.0 {
            animation.timer.tick(time.delta());
            animation.reset = false;
        } else {
            animation.timer.finish();
            animation.reset = true;
        }
    }

    if mov != 0.0 {
        for (mut transform, direction) in objects.into_iter() {
            transform.translation.x -= mov * K_SPEED * time.delta_secs();
            if let Some(mut direction) = direction {
                let new_direction = if mov > 0.0 {
                    Direction::Left
                } else {
                    Direction::Right
                };
                if *direction != new_direction {
                    *direction = new_direction;
                }
            }
            info!("OnControl transform: {:?}", transform.translation);
        }
    }

    for (mut transform, layer) in layers.into_iter() {
        if mov != 0.0 {
            transform.translation.x -= mov * K_SPEED * layer.speed * time.delta_secs();
            info!("Layer transform: {:?}", transform.translation);
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
