use std::time::Duration;

use bevy::{prelude::*, window::WindowResolution};

mod components;
mod constants;
mod events;
mod layer;
mod states;
mod systems;
mod types;

use components::*;
use constants::*;
use layer::*;
use states::*;
use types::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(K_WIDTH as u32, K_HEIGHT as u32),
                title: "Bevy Game Project".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(1)))
        .init_state::<GameState>()
        .add_observer(systems::on_action)
        .add_systems(Startup, setup)
        .add_systems(Update, systems::global_action)
        .add_systems(
            Update,
            systems::layer_update.run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            systems::sun_update.run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            systems::game_player_action.run_if(in_state(GameState::InGame)),
        )
        .add_systems(OnEnter(GameState::InPlayerMenu), systems::enter_player_menu)
        .add_systems(OnExit(GameState::InPlayerMenu), systems::exit_player_menu)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let layer_city = LayerDesc {
        objects: vec![
            LayerObjectDesc {
                t: ObjectType::Primitive(PrimitiveType::Rectangle),
                component: ObjectComponentType::Building(Building),
                position: Vec2::new(-640.0, K_GROUND_LEVEL + 256.0),
                size: Vec2::new(256.0, 512.0),
                color: Color::srgb(1.0, 0.0, 0.0),
                name: "Red Building".to_string(),
            },
            LayerObjectDesc {
                t: ObjectType::Primitive(PrimitiveType::Rectangle),
                component: ObjectComponentType::Building(Building),
                position: Vec2::new(-256.0, K_GROUND_LEVEL + 128.0),
                size: Vec2::new(128.0, 256.0),
                color: Color::srgb(0.0, 1.0, 0.0),
                name: "Green Building".to_string(),
            },
            LayerObjectDesc {
                t: ObjectType::Primitive(PrimitiveType::Rectangle),
                component: ObjectComponentType::Building(Building),
                position: Vec2::new(140.0, K_GROUND_LEVEL + 64.0),
                size: Vec2::new(128.0, 128.0),
                color: Color::srgb(0.2, 0.0, 0.2),
                name: "Deck Building".to_string(),
            },
            LayerObjectDesc {
                t: ObjectType::Primitive(PrimitiveType::Rectangle),
                component: ObjectComponentType::Ocean(Ocean),
                position: Vec2::new(720.0, K_GROUND_LEVEL - 32.0),
                size: Vec2::new(1024.0, 64.0),
                color: Color::srgb(0.0, 0.0, 1.0),
                name: "Ocean".to_string(),
            },
        ],
        depth: 0.0,
        size: Vec2::new(K_WIDTH, K_HEIGHT),
        name: "City".to_string(),
    };

    let layer_sun = LayerDesc {
        objects: vec![LayerObjectDesc {
            t: ObjectType::Primitive(PrimitiveType::Circle),
            component: ObjectComponentType::Sun(Sun),
            position: Vec2::new(0.0, K_GROUND_LEVEL + 512.0),
            size: Vec2::new(128.0, 128.0),
            color: Color::srgb(1.0, 1.0, 0.0),
            name: "Sun".to_string(),
        }],
        depth: -1.0,
        size: Vec2::new(K_WIDTH, K_HEIGHT),
        name: "Sky".to_string(),
    };

    let layer_play = LayerDesc {
        objects: vec![LayerObjectDesc {
            t: ObjectType::Primitive(PrimitiveType::Rectangle),
            component: ObjectComponentType::Player(Player),
            position: Vec2::new(0.0, K_GROUND_LEVEL + 32.0),
            size: Vec2::new(8.0, 64.0),
            color: Color::srgb(1.0, 1.0, 1.0),
            name: "Player".to_string(),
        }],
        depth: 1.0,
        size: Vec2::new(K_WIDTH, K_HEIGHT),
        name: "Play".to_string(),
    };

    layer_city.build(&mut commands, &mut meshes, &mut materials);
    layer_sun.build(&mut commands, &mut meshes, &mut materials);
    layer_play.build(&mut commands, &mut meshes, &mut materials);
}
