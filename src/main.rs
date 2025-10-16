use bevy::{prelude::*, window::WindowResolution};

mod components;
mod constants;
mod events;
mod layer;
mod systems;
mod types;

use components::*;
use constants::*;
use layer::*;
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
        .add_observer(systems::action::on_action)
        .add_systems(Startup, setup)
        .add_systems(Update, systems::player::player_control)
        .add_systems(Update, systems::player::player_action)
        .add_systems(Update, systems::sun::sun_update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let layer_city = Layer {
        objects: vec![
            LayerObject {
                t: ObjectType::Primitive(PrimitiveType::Rectangle),
                component: ObjectComponentType::Building(Building),
                position: Vec2::new(0.0, K_GROUND_LEVEL + 256.0),
                size: Vec2::new(128.0, 512.0),
                color: Color::srgb(1.0, 0.0, 0.0),
                name: "Red Building".to_string(),
            },
            LayerObject {
                t: ObjectType::Primitive(PrimitiveType::Rectangle),
                component: ObjectComponentType::Building(Building),
                position: Vec2::new(256.0, K_GROUND_LEVEL + 128.0),
                size: Vec2::new(64.0, 256.0),
                color: Color::srgb(0.0, 1.0, 0.0),
                name: "Green Building".to_string(),
            },
        ],
        depth: 0.0,
    };

    let layer_sun = Layer {
        objects: vec![LayerObject {
            t: ObjectType::Primitive(PrimitiveType::Circle),
            component: ObjectComponentType::Sun(Sun),
            position: Vec2::new(-512.0, K_GROUND_LEVEL + 512.0),
            size: Vec2::new(128.0, 128.0),
            color: Color::srgb(1.0, 1.0, 0.0),
            name: "Sun".to_string(),
        }],
        depth: -1.0,
    };

    let layer_player = Layer {
        objects: vec![LayerObject {
            t: ObjectType::Primitive(PrimitiveType::Rectangle),
            component: ObjectComponentType::Player(Player),
            position: Vec2::new(0.0, K_GROUND_LEVEL + 16.0),
            size: Vec2::new(4.0, 32.0),
            color: Color::srgb(1.0, 1.0, 1.0),
            name: "Player".to_string(),
        }],
        depth: 1.0,
    };

    layer_city.build(&mut commands, &mut meshes, &mut materials);
    layer_sun.build(&mut commands, &mut meshes, &mut materials);
    layer_player.build(&mut commands, &mut meshes, &mut materials);
}
