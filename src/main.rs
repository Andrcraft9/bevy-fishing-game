use std::time::Duration;

use bevy::{prelude::*, window::WindowResolution};

mod components;
mod constants;
mod events;
mod items;
mod layer;
mod states;
mod systems;

use constants::*;
use layer::*;
use states::*;

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
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let layer_city = LayerDesc {
        objects: vec![
            LayerObjectDesc {
                t: ObjectType::Sprite(SpriteDesc {
                    path: "building/hut.png".to_string(),
                    ..default()
                }),
                component: ObjectComponentType::Building,
                position: Vec2::new(512.0, K_GROUND_LEVEL + 128.0 - 79.0),
                size: Vec2::new(384.0, 256.0),
                color: Color::srgb_u8(128, 128, 128),
                name: "Hut".to_string(),
            },
            LayerObjectDesc {
                t: ObjectType::Primitive(PrimitiveType::Rectangle),
                component: ObjectComponentType::Land,
                position: Vec2::new(512.0 - 2024.0, K_GROUND_LEVEL - 32.0),
                size: Vec2::new(4096.0, 64.0),
                color: Color::srgb_u8(60, 128, 60),
                name: "Land".to_string(),
            },
            LayerObjectDesc {
                t: ObjectType::Primitive(PrimitiveType::Rectangle),
                component: ObjectComponentType::Ocean,
                position: Vec2::new(512.0 + 2024.0, K_GROUND_LEVEL - 32.0),
                size: Vec2::new(4096.0, 64.0),
                color: Color::srgb_u8(85, 128, 200),
                name: "Ocean".to_string(),
            },
        ],
        depth: 0.0,
        speed: 1.0,
        size: Vec2::new(K_WIDTH, K_HEIGHT),
        name: "City".to_string(),
    };

    let layer_sun = LayerDesc {
        objects: vec![LayerObjectDesc {
            t: ObjectType::Primitive(PrimitiveType::Circle),
            component: ObjectComponentType::Sun,
            position: Vec2::new(0.0, K_GROUND_LEVEL + 512.0),
            size: Vec2::new(128.0, 128.0),
            color: Color::srgb(1.0, 1.0, 0.0),
            name: "Sun".to_string(),
        }],
        depth: -9.0,
        speed: 0.0,
        size: Vec2::new(K_WIDTH, K_HEIGHT),
        name: "Sun".to_string(),
    };

    let layer_sky = LayerDesc {
        objects: vec![LayerObjectDesc {
            t: ObjectType::Primitive(PrimitiveType::Rectangle),
            component: ObjectComponentType::Sky,
            position: Vec2::new(0.0, 0.0),
            size: Vec2::new(8.0 * K_WIDTH, K_HEIGHT),
            color: Color::srgb_u8(0, 180, 250),
            name: "Sky".to_string(),
        }],
        depth: -10.0,
        speed: 0.0,
        size: Vec2::new(8.0 * K_WIDTH, K_HEIGHT),
        name: "Sky".to_string(),
    };

    let layer_mountain = LayerDesc {
        objects: vec![LayerObjectDesc {
            t: ObjectType::Sprite(SpriteDesc {
                path: "sky/glacial_mountains.png".to_string(),
                mode: SpriteImageMode::Tiled {
                    tile_x: true,
                    tile_y: false,
                    stretch_value: 3.0,
                },
            }),
            component: ObjectComponentType::Sky,
            position: Vec2::new(0.0, 0.0),
            size: Vec2::new(8.0 * K_WIDTH, K_HEIGHT),
            color: Color::srgb_u8(0, 180, 0),
            name: "Mountain".to_string(),
        }],
        depth: -8.0,
        speed: 0.15,
        size: Vec2::new(8.0 * K_WIDTH, K_HEIGHT),
        name: "Mountain".to_string(),
    };

    let layer_clouds1 = LayerDesc {
        objects: vec![LayerObjectDesc {
            t: ObjectType::Sprite(SpriteDesc {
                path: "sky/clouds_mg_1.png".to_string(),
                mode: SpriteImageMode::Tiled {
                    tile_x: true,
                    tile_y: false,
                    stretch_value: 3.0,
                },
            }),
            component: ObjectComponentType::Sky,
            position: Vec2::new(0.0, K_GROUND_LEVEL + K_HEIGHT / 2.0),
            size: Vec2::new(8.0 * K_WIDTH, K_HEIGHT),
            color: Color::srgb_u8(0, 180, 0),
            name: "Clouds-1".to_string(),
        }],
        depth: -5.0,
        speed: 0.15,
        size: Vec2::new(8.0 * K_WIDTH, K_HEIGHT),
        name: "Clouds".to_string(),
    };

    let layer_clouds2 = LayerDesc {
        objects: vec![LayerObjectDesc {
            t: ObjectType::Sprite(SpriteDesc {
                path: "sky/clouds_mg_2.png".to_string(),
                mode: SpriteImageMode::Tiled {
                    tile_x: true,
                    tile_y: false,
                    stretch_value: 3.0,
                },
            }),
            component: ObjectComponentType::Sky,
            position: Vec2::new(0.0, K_GROUND_LEVEL + K_HEIGHT / 2.0),
            size: Vec2::new(8.0 * K_WIDTH, K_HEIGHT),
            color: Color::srgb_u8(0, 180, 0),
            name: "Clouds-2".to_string(),
        }],
        depth: -6.0,
        speed: 0.15,
        size: Vec2::new(8.0 * K_WIDTH, K_HEIGHT),
        name: "Clouds".to_string(),
    };

    let layer_clouds3 = LayerDesc {
        objects: vec![LayerObjectDesc {
            t: ObjectType::Sprite(SpriteDesc {
                path: "sky/clouds_mg_3.png".to_string(),
                mode: SpriteImageMode::Tiled {
                    tile_x: true,
                    tile_y: false,
                    stretch_value: 2.0,
                },
            }),
            component: ObjectComponentType::Sky,
            position: Vec2::new(0.0, K_GROUND_LEVEL + K_HEIGHT / 2.0),
            size: Vec2::new(8.0 * K_WIDTH, K_HEIGHT),
            color: Color::srgb_u8(0, 180, 0),
            name: "Clouds-3".to_string(),
        }],
        depth: -7.0,
        speed: 0.15,
        size: Vec2::new(8.0 * K_WIDTH, K_HEIGHT),
        name: "Clouds".to_string(),
    };

    let layer_forest = LayerDesc {
        objects: vec![LayerObjectDesc {
            t: ObjectType::Sprite(SpriteDesc {
                path: "forest/forest.png".to_string(),
                mode: SpriteImageMode::Tiled {
                    tile_x: true,
                    tile_y: false,
                    stretch_value: 2.0,
                },
            }),
            component: ObjectComponentType::Sky,
            position: Vec2::new(0.0, 104.0 - K_HEIGHT / 2.0),
            size: Vec2::new(8.0 * K_WIDTH, K_HEIGHT / 2.0),
            color: Color::srgb_u8(0, 180, 0),
            name: "Forest".to_string(),
        }],
        depth: -4.0,
        speed: 0.5,
        size: Vec2::new(8.0 * K_WIDTH, K_HEIGHT),
        name: "Forest".to_string(),
    };

    let layer_play = LayerDesc {
        objects: vec![LayerObjectDesc {
            t: ObjectType::SpriteAtlas(SpriteAtlasDesc {
                path: "player/walk.png".to_string(),
                splat: 48,
                cols: 6,
                rows: 1,
                index: 0,
            }),
            component: ObjectComponentType::Player,
            position: Vec2::new(0.0, K_GROUND_LEVEL + 64.0),
            size: Vec2::new(128.0, 128.0),
            color: Color::srgb(1.0, 1.0, 1.0),
            name: "Player".to_string(),
        }],
        depth: 0.5,
        speed: 1.0,
        size: Vec2::new(K_WIDTH, K_HEIGHT),
        name: "Play".to_string(),
    };

    let layer_boat = LayerDesc {
        objects: vec![LayerObjectDesc {
            t: ObjectType::Sprite(SpriteDesc {
                path: "boat/boat.png".to_string(),
                ..default()
            }),
            component: ObjectComponentType::Boat,
            position: Vec2::new(512.0, K_GROUND_LEVEL + 16.0),
            size: Vec2::new(128.0, 32.0),
            color: Color::srgb(1.0, 1.0, 1.0),
            name: "Boat".to_string(),
        }],
        depth: 1.0,
        speed: 1.0,
        size: Vec2::new(K_WIDTH, K_HEIGHT),
        name: "Boat".to_string(),
    };

    layer_city.build(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut texture_atlas_layouts,
        &mut materials,
    );
    layer_sun.build(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut texture_atlas_layouts,
        &mut materials,
    );
    layer_sky.build(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut texture_atlas_layouts,
        &mut materials,
    );
    layer_mountain.build(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut texture_atlas_layouts,
        &mut materials,
    );
    layer_clouds3.build(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut texture_atlas_layouts,
        &mut materials,
    );
    layer_clouds2.build(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut texture_atlas_layouts,
        &mut materials,
    );
    layer_clouds1.build(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut texture_atlas_layouts,
        &mut materials,
    );
    layer_forest.build(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut texture_atlas_layouts,
        &mut materials,
    );
    layer_play.build(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut texture_atlas_layouts,
        &mut materials,
    );
    layer_boat.build(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut texture_atlas_layouts,
        &mut materials,
    );
}
