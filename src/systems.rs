use std::time::Duration;

use crate::{
    components::{
        ActionRange, ActiveSprite, AnimationConfig, AnimationState, AnimationTimer, Boat, Building,
        Cloud, DayNightColor, DefaultColor, Direction, Land, Layer, Ocean, OnControl, Player,
        PlayerMenu, PlayerState, SpriteCollection, Sun, Velocity,
    },
    constants::{
        K_GROUND_LEVEL, K_HEIGHT, K_OCEAN_LAND_BORDER, K_SECS_IN_DAY, K_SIT_OFFSET, K_SPEED,
        K_WIDTH,
    },
    events::{Action, Catch, EndAction, Hit, Hook, Sell},
    items::{self, Value, Weight},
    states::GameState,
};
use bevy::app::AppExit;
use bevy::prelude::*;
use rand::Rng;

///
/// Observers
///

pub fn on_hook(_action: On<Hook>, player: Single<&mut PlayerState, With<Player>>) {
    info!("On Hook!");
    let mut player = player.into_inner();
    if *player == PlayerState::Fish {
        *player = PlayerState::Hook;
    } else if *player == PlayerState::Idle {
        *player = PlayerState::Attack;
    }
}

pub fn on_sell(_action: On<Sell>, player: Single<&mut Player>) {
    info!("On Sell!");
    let mut player = player.into_inner();
    let item = player.items.pop();
    if let Some(item) = item {
        info!("Sold item: {}", item.name());
        player.money += item.value();
    }
}

pub fn on_hit(
    _action: On<Hit>,
    player: Single<&GlobalTransform, With<Player>>,
    buildings: Query<(&GlobalTransform, &Name, &ActionRange), With<Building>>,
    mut commands: Commands,
) {
    info!("On Hit!");
    let position = player.translation();
    for (transform, name, action_range) in buildings.iter() {
        let distance = (position.x - transform.translation().x).abs();
        if distance <= action_range.range {
            info!(
                "Found building '{}' at distance {:.2} from action",
                name, distance
            );
            commands.trigger(Sell);
        }
    }
}

pub fn on_catch(_action: On<Catch>, mut player: Single<&mut Player>) {
    info!("On Catch!");
    let mut rng = rand::thread_rng();
    let chance: f32 = rng.gen_range(0.0..1.0);
    if chance < 0.1 {
        let weight = rng.gen_range(0.1..20.0);
        player.items.push(items::Item::Fish(items::Fish {
            t: items::FishType::Golden,
            weight,
        }));
        info!("Caught a Golden fish! Weight: {:.2}", weight);
    } else if chance < 0.5 {
        let weight = rng.gen_range(0.1..20.0);
        player.items.push(items::Item::Fish(items::Fish {
            t: items::FishType::Silver,
            weight,
        }));
        info!("Caught a Silver fish! Weight: {:.2}", weight);
    } else {
        info!("Nothing caught this time...");
    }
}

pub fn on_action(
    _action: On<Action>,
    player: Single<(&mut PlayerState, &GlobalTransform)>,
    oceans: Query<(&GlobalTransform, &Name, &ActionRange), With<Ocean>>,
) {
    info!("On Action!");
    let (mut state, player_transform) = player.into_inner();
    let position = player_transform.translation();

    *state = PlayerState::Idle;

    for (transform, name, action_range) in oceans.iter() {
        let distance = (position.x - transform.translation().x).abs();
        if distance <= action_range.range {
            info!(
                "Found ocean '{}' at distance {:.2} from action",
                name, distance
            );
            *state = PlayerState::Fish;
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

///
/// Input systems
///

pub fn menu_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) || keyboard_input.just_pressed(KeyCode::Tab) {
        info!("Back in Game!");
        next_state.set(GameState::InGame);
        return;
    }
}

pub fn game_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(&mut Velocity, Option<&mut Direction>), With<OnControl>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        info!("Quitting app!");
        app_exit_events.write(AppExit::Success);
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Tab) {
        info!("Player menu!");
        next_state.set(GameState::InPlayerMenu);
        return;
    }

    let mut vel = 0.0;
    if keyboard_input.pressed(KeyCode::KeyA) {
        vel += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        vel -= 1.0;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        info!("Stay, trigger action!");
        vel = 0.0;
        commands.trigger(Action);
        next_state.set(GameState::InAction);
    }

    for (mut velocity, direction) in query {
        velocity.value = vel;
        if vel != 0.0 {
            if let Some(mut direction) = direction {
                *direction = if vel > 0.0 {
                    Direction::Left
                } else {
                    Direction::Right
                };
            }
        }
    }
}

pub fn action_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_released(KeyCode::KeyW) || keyboard_input.just_released(KeyCode::KeyS) {
        commands.trigger(Hook);
    }

    if keyboard_input.just_released(KeyCode::Space) {
        info!("End action!");
        commands.trigger(EndAction);
        next_state.set(GameState::InGame);
        return;
    }
}

///
/// Reaction systems
///

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
        commands.entity(entity).remove::<AnimationTimer>();

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
            .insert(collection.animations[active.index].clone())
            .insert(AnimationTimer { ..default() });
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
                transform.translation.z = 0.55;
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
            // TODO: Hard-coded values should be removed.
            active.index = 2;
            transform.translation.y = K_GROUND_LEVEL + 64.0;
            transform.translation.z = 0.0;
        }
        PlayerState::Idle => {
            let (_entity, mut active, mut transform) = set.p0().into_inner();
            info!("Idle!");
            active.index = 3;
            transform.translation.y = K_GROUND_LEVEL + 64.0;
            transform.translation.z = 0.0;
        }
        PlayerState::Hook => {
            info!("Hook!");
            let (_entity, mut active, mut transform) = set.p0().into_inner();
            // TODO: Hard-coded values should be removed.
            active.index = 4;
            transform.translation.y = K_GROUND_LEVEL + 64.0;
            transform.translation.z = 0.0;
        }
        PlayerState::Attack => {
            info!("Attack!");
            let (_entity, mut active, mut transform) = set.p0().into_inner();
            // TODO: Hard-coded values should be removed.
            active.index = 5;
            transform.translation.y = K_GROUND_LEVEL + 64.0;
            transform.translation.z = 0.0;
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

pub fn added_animation(
    animations: Query<(Entity, &mut AnimationTimer, &AnimationConfig), Added<AnimationTimer>>,
    mut commands: Commands,
) {
    for (entity, mut animation, config) in animations.into_iter() {
        let size = config.last_index - config.first_index + 1;
        animation.timer = Timer::new(
            Duration::from_millis(size as u64 * config.ms),
            TimerMode::Once,
        );
        commands.entity(entity).insert(AnimationState::Run);
    }
}

pub fn changed_animation_player(
    query: Single<(&mut PlayerState, &AnimationState), Changed<AnimationState>>,
    mut commands: Commands,
) {
    let (mut player_state, state) = query.into_inner();
    //info!("Changed State! {:?}", state);
    match *player_state {
        PlayerState::Hook => {
            if *state == AnimationState::Finish {
                commands.trigger(Catch);
                *player_state = PlayerState::Fish;
            }
        }
        PlayerState::Attack => {
            if *state == AnimationState::Finish {
                commands.trigger(Hit);
                *player_state = PlayerState::Idle;
            }
        }
        _ => {}
    }
}

///
/// Update systems
///

pub fn animation(
    animations: Query<(
        &mut AnimationTimer,
        &mut Sprite,
        &mut AnimationState,
        &AnimationConfig,
    )>,
) {
    for (mut animation, mut sprite, mut state, config) in animations.into_iter() {
        let size = config.last_index - config.first_index + 1;

        if animation.timer.is_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = config.first_index;
            }
            *state = AnimationState::Finish;
            if config.mode == TimerMode::Repeating {
                animation.timer = Timer::new(
                    Duration::from_millis(size as u64 * config.ms),
                    TimerMode::Once,
                );
                *state = AnimationState::Run;
            }
        } else {
            let current = animation.timer.elapsed();
            let index = current.as_millis() as u64 / config.ms;
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = (config.first_index + index as usize)
                    .clamp(config.first_index, config.last_index);
            }
        }
    }
}

pub fn move_control(
    time: Res<Time<Virtual>>,
    query: Query<(&mut Transform, &Velocity, Option<&Camera>), With<OnControl>>,
) {
    for (mut transform, velocity, camera) in query {
        transform.translation.x -= K_SPEED * time.delta_secs() * velocity.value;
        if let Some(_) = camera {
            let sig = (transform.translation.x - (K_OCEAN_LAND_BORDER + K_WIDTH / 2.0)).signum();
            transform.translation.y -= sig * K_SPEED * time.delta_secs();
            transform.translation.y = transform.translation.y.clamp(-K_HEIGHT / 2.0, 0.0);
        }
    }
}

pub fn move_layer(
    time: Res<Time<Virtual>>,
    velocity: Single<&Velocity, With<Camera>>,
    query: Query<(&mut Transform, &Layer)>,
) {
    for (mut transform, layer) in query {
        transform.translation.x -= K_SPEED * time.delta_secs() * velocity.value * layer.speed;
    }
}

pub fn game_animation_control(
    time: Res<Time<Virtual>>,
    query: Query<(&mut AnimationTimer, &Velocity), With<OnControl>>,
) {
    for (mut animation, velocity) in query {
        if velocity.value != 0.0 {
            animation.timer.tick(time.delta());
        } else {
            animation.timer.finish();
        }
    }
}

pub fn action_animation_control(
    time: Res<Time<Virtual>>,
    animations: Query<&mut AnimationTimer, With<OnControl>>,
) {
    for mut animation in animations.into_iter() {
        animation.timer.tick(time.delta());
    }
}

pub fn move_sun(time: Res<Time<Virtual>>, mut sun_query: Single<&mut Transform, With<Sun>>) {
    let day_time = (24.0 * time.elapsed_secs() / K_SECS_IN_DAY) % 24.0;
    // Map to [0, 1] range.
    let day = (1.0 + (3.14 * day_time / 12.0 - 3.14 / 2.0).sin()) / 2.0;

    sun_query.translation.y = day * K_HEIGHT - K_HEIGHT / 2.0;
}

pub fn move_cloud(time: Res<Time<Virtual>>, query: Query<(&mut Transform, &Cloud, &Sprite)>) {
    for (mut transform, cloud, sprite) in query.into_iter() {
        if let Some(_) = sprite.custom_size {
            match sprite.image_mode {
                SpriteImageMode::Tiled { .. } => {
                    transform.translation.x += K_SPEED * cloud.speed * 1.0 * time.delta_secs();
                    transform.translation.x = transform.translation.x % K_WIDTH;
                }
                _ => {}
            }
        }
    }
}

pub fn player_state_walk_or_row(
    player: Single<(&mut PlayerState, &GlobalTransform), With<Player>>,
    oceans: Query<(&Ocean, &GlobalTransform)>,
    lands: Query<(&Land, &GlobalTransform)>,
) {
    let (mut state, transform) = player.into_inner();

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

pub fn color_day_night(
    time: Res<Time<Virtual>>,
    query: Query<(&mut Sprite, &DefaultColor), With<DayNightColor>>,
) {
    //info!("Time = {}", time.elapsed_secs());
    let day_time = (24.0 * time.elapsed_secs() / K_SECS_IN_DAY) % 24.0;
    // Map to [0, 1] range.
    let day = (1.0 + (3.14 * day_time / 12.0 - 3.14 / 2.0).sin()) / 2.0;
    //info!("Date = {}, light={}", day_time, day);

    for (mut sprite, color) in query {
        sprite.color = color.color.darker(0.8 * (1.0 - day));
    }
}

///
/// Game state transition systems
///

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
