use std::time::Duration;

use crate::{
    components::{
        ActionRange, ActiveSprite, AnimationConfig, AnimationTimer, Boat, Building, Cloud,
        Direction, Land, Layer, Ocean, OnControl, Player, PlayerMenu, PlayerState,
        SpriteCollection, Sun,
    },
    constants::{K_GROUND_LEVEL, K_HEIGHT, K_OCEAN_LAND_BORDER, K_SIT_OFFSET, K_SPEED, K_WIDTH},
    events::{Action, Catch, EndAction},
    items::{self, Value, Weight},
    states::GameState,
};
use bevy::app::AppExit;
use bevy::prelude::*;
use rand::Rng;

pub fn on_catch(_action: On<Catch>, mut player: Single<&mut Player>) {
    info!("On Catch!");

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

pub fn on_action(
    action: On<Action>,
    player: Single<(&mut Player, &mut PlayerState)>,
    oceans: Query<(&GlobalTransform, &Name, &ActionRange), With<Ocean>>,
    buildings: Query<(&GlobalTransform, &Name, &ActionRange), With<Building>>,
) {
    let (mut player, mut state) = player.into_inner();
    let action_position = action.event().position;

    info!("On Action!");
    *state = PlayerState::Idle;

    for (transform, name, action_range) in oceans.iter() {
        let distance = (action_position.x - transform.translation().x).abs();
        if distance <= action_range.range {
            info!(
                "Found ocean '{}' at distance {:.2} from action",
                name, distance
            );
            *state = PlayerState::Fish;
            return;
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
            return;
        }
    }
}

pub fn on_end_action(_action: On<EndAction>, player: Single<(&mut Player, &mut PlayerState)>) {
    let (_, mut state) = player.into_inner();

    info!("On EndAction!");

    if *state == PlayerState::Fish {
        *state = PlayerState::Row;
    } else {
        *state = PlayerState::Walk;
    }
}

pub fn global_action(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: MessageWriter<AppExit>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        info!("Player menu!");
        next_state.set(GameState::InPlayerMenu);
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        if *state.get() != GameState::InGame {
            info!("Back in Game!");
            next_state.set(GameState::InGame);
        } else {
            info!("Quitting app!");
            app_exit_events.write(AppExit::Success);
        }
        return;
    }
}

pub fn game_player_action(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_transform: Single<&GlobalTransform, With<Player>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        info!("Trigger action!");
        commands.trigger(Action {
            position: Vec2::new(
                player_transform.translation().x,
                player_transform.translation().y,
            ),
        });
        next_state.set(GameState::InAction);
        return;
    }

    if keyboard_input.just_released(KeyCode::Space) {
        info!("End action!");
        commands.trigger(EndAction);
        next_state.set(GameState::InGame);
    }
}

pub fn changed_active_sprite(
    query: Query<
        (Entity, &ActiveSprite, &SpriteCollection, Option<&Direction>),
        Changed<ActiveSprite>,
    >,
    mut commands: Commands,
) {
    for (entity, active, collection, direction) in query {
        commands.entity(entity).remove::<Sprite>();
        commands.entity(entity).remove::<AnimationConfig>();

        let mut sprite = collection.sprites[active.index].clone();
        if let Some(direction) = direction {
            match direction {
                Direction::Left => {
                    sprite.flip_x = true;
                }
                Direction::Right => {
                    sprite.flip_x = false;
                }
            }
        }

        commands.entity(entity).insert(sprite);
        commands
            .entity(entity)
            .insert(collection.animations[active.index].clone());
    }
}

pub fn changed_player_state(
    state: Single<&PlayerState, (With<Player>, Changed<PlayerState>)>,
    mut set: ParamSet<(
        Single<(Entity, &mut ActiveSprite, &mut Transform), With<Player>>,
        Single<(Entity, &mut Transform), With<Boat>>,
    )>,
    mut commands: Commands,
) {
    match *state {
        PlayerState::Walk => {
            {
                let (_entity, mut active, mut transform) = set.p0().into_inner();
                info!("Stand and Walk!");
                active.index = 0;
                transform.translation.y = K_GROUND_LEVEL + 64.0;
                transform.translation.z = 0.0;
            }
            {
                let (boat_entity, mut boat_transform) = set.p1().into_inner();
                info!("Leave boat!");
                commands.entity(boat_entity).remove::<OnControl>();
                boat_transform.translation.x = K_OCEAN_LAND_BORDER;
            }
        }
        PlayerState::Row => {
            {
                let (_entity, mut active, mut transform) = set.p0().into_inner();
                info!("Sit and Row!");
                active.index = 1;
                transform.translation.y = K_GROUND_LEVEL + 64.0 + K_SIT_OFFSET;
                transform.translation.z = 0.0;
            }
            {
                let (boat_entity, _) = set.p1().into_inner();
                info!("Take boat!");
                commands.entity(boat_entity).insert(OnControl);
            }
        }
        PlayerState::Fish => {
            let (_entity, mut active, mut transform) = set.p0().into_inner();
            info!("Fish!");
            active.index = 2;
            transform.translation.y = K_GROUND_LEVEL + 64.0;
            transform.translation.z = -0.5;
        }
        PlayerState::Idle => {
            let (_entity, mut active, mut transform) = set.p0().into_inner();
            info!("Idle!");
            active.index = 3;
            transform.translation.y = K_GROUND_LEVEL + 64.0;
            transform.translation.z = 0.0;
        }
        PlayerState::Catch => {
            info!("Catch!");
        }
    }
}

pub fn player_on_land_ocean(
    player: Single<(&GlobalTransform, &mut PlayerState), With<Player>>,
    oceans: Query<(&Ocean, &GlobalTransform)>,
    lands: Query<(&Land, &GlobalTransform)>,
) {
    let (transform, mut state) = player.into_inner();

    if *state == PlayerState::Row {
        for (land, land_transform) in lands.iter() {
            if transform.translation().x > land_transform.translation().x - land.size.x / 2.0
                && transform.translation().x < land_transform.translation().x + land.size.x / 2.0
            {
                info!("Hit land!");
                *state = PlayerState::Walk;
            }
            return;
        }
    }

    if *state == PlayerState::Walk {
        for (ocean, ocean_transform) in oceans.iter() {
            if transform.translation().x > ocean_transform.translation().x - ocean.size.x / 2.0
                && transform.translation().x < ocean_transform.translation().x + ocean.size.x / 2.0
            {
                info!("Hit ocean!");
                *state = PlayerState::Row;
                return;
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

pub fn time_control(time: Res<Time>, animations: Query<&mut AnimationTimer, With<OnControl>>) {
    for mut animation in animations.into_iter() {
        animation.timer.tick(time.delta());
        animation.reset = false;
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
            //info!("OnControl transform: {:?}", transform.translation);
        }
    }

    for (mut transform, layer) in layers.into_iter() {
        if mov != 0.0 {
            transform.translation.x -= mov * K_SPEED * layer.speed * time.delta_secs();
            //info!("Layer transform: {:?}", transform.translation);
        }
    }
}

pub fn sun_update(time: Res<Time<Virtual>>, mut sun_query: Single<&mut Transform, With<Sun>>) {
    sun_query.translation.y = (0.5 * time.elapsed_secs()).sin() * K_HEIGHT / 2.0;
}

pub fn cloud_update(time: Res<Time<Virtual>>, query: Query<(&Cloud, &mut Transform)>) {
    for (cloud, mut transform) in query.into_iter() {
        // TODO: Fix, remove hard-coded sprite size.
        transform.translation.x = K_SPEED * cloud.speed * time.elapsed_secs() % (2.0 * 384.0);
    }
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
