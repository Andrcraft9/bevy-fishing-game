use std::time::Duration;

use bevy::{prelude::*, window::WindowResolution};

mod components;
mod constants;
mod events;
mod items;
mod layer;
mod resources;
mod states;
mod systems;

use constants::*;
use layer::*;
use states::*;

use crate::components::{Cloud, Fish, OnControl, Velocity};
use crate::items::Value;
use crate::resources::AITimer;

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
        .insert_resource(AITimer {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .init_state::<GameState>()
        // Observers.
        .add_observer(systems::on_action)
        .add_observer(systems::on_end_action)
        .add_observer(systems::on_hook)
        .add_observer(systems::on_sell)
        .add_observer(systems::on_catch)
        .add_observer(systems::on_hit)
        .add_systems(Startup, setup)
        // In-action update systems.
        .add_systems(
            Update,
            (
                systems::ai_timer,
                systems::fish_spawn,
                systems::added_animation,
                systems::action_input,
                systems::ai_input,
                systems::player_state_walk_or_row,
                systems::move_control,
                systems::move_layer,
                systems::move_sun,
                systems::move_cloud,
                systems::move_ai,
                systems::changed_animation_player,
                systems::changed_active_sprite,
                systems::changed_direction,
                systems::changed_player_state,
                systems::action_animation_control,
                systems::animation_ai,
                systems::animation,
                systems::color_day_night,
            )
                .chain()
                .run_if(in_state(GameState::InAction)),
        )
        // In-game update systems.
        .add_systems(
            Update,
            (
                systems::ai_timer,
                systems::fish_spawn,
                systems::added_animation,
                systems::game_input,
                systems::ai_input,
                systems::player_state_walk_or_row,
                systems::move_control,
                systems::move_layer,
                systems::move_sun,
                systems::move_cloud,
                systems::move_ai,
                systems::changed_active_sprite,
                systems::changed_direction,
                systems::changed_player_state,
                systems::game_animation_control,
                systems::animation_ai,
                systems::animation,
                systems::color_day_night,
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        )
        // In-Menu systems.
        .add_systems(
            Update,
            systems::menu_input.run_if(in_state(GameState::InPlayerMenu)),
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
    commands
        .spawn(Camera2d)
        .insert(Velocity { ..default() })
        .insert(OnControl);

    let layer_fauna = LayerDesc {
        objects: vec![
            LayerObjectDesc {
                t: ObjectType::SpriteAtlas(SpriteAtlasDesc {
                    sprite: SpriteDesc {
                        path: "fauna/fish/7.png".to_string(),
                        ..default()
                    },
                    tile: UVec2::new(30, 12),
                    cols: 2,
                    rows: 1,
                    index: 0,
                    mode: TimerMode::Repeating,
                    ms: K_ANIMATION_FRAME_MS,
                }),
                component: ObjectComponentType::Fish(Fish {
                    t: items::FishType::Fish,
                }),
                position: Vec2::new(K_FISH_AREA_BORDER + 128.0, K_GROUND_LEVEL - 16.0),
                size: Vec2::new(48.0, 16.0),
                color: Color::srgb(1.0, 1.0, 1.0),
                name: items::FishType::Fish.name(),
            },
            LayerObjectDesc {
                t: ObjectType::SpriteAtlas(SpriteAtlasDesc {
                    sprite: SpriteDesc {
                        path: "fauna/fish/8.png".to_string(),
                        ..default()
                    },
                    tile: UVec2::new(30, 12),
                    cols: 2,
                    rows: 1,
                    index: 0,
                    mode: TimerMode::Repeating,
                    ms: K_ANIMATION_FRAME_MS,
                }),
                component: ObjectComponentType::Fish(Fish {
                    t: items::FishType::Ray,
                }),
                position: Vec2::new(K_FISH_AREA_BORDER + 128.0, K_GROUND_LEVEL - 64.0),
                size: Vec2::new(64.0, 18.0),
                color: Color::srgb(1.0, 1.0, 1.0),
                name: items::FishType::Ray.name(),
            },
            LayerObjectDesc {
                t: ObjectType::SpriteAtlas(SpriteAtlasDesc {
                    sprite: SpriteDesc {
                        path: "fauna/fish/6.png".to_string(),
                        ..default()
                    },
                    tile: UVec2::new(54, 22),
                    cols: 2,
                    rows: 1,
                    index: 0,
                    mode: TimerMode::Repeating,
                    ms: K_ANIMATION_FRAME_MS,
                }),
                component: ObjectComponentType::Fish(Fish {
                    t: items::FishType::Shark,
                }),
                position: Vec2::new(K_FISH_AREA_BORDER + 128.0, K_GROUND_LEVEL - 128.0),
                size: Vec2::new(128.0, 48.0),
                color: Color::srgb(1.0, 1.0, 1.0),
                name: items::FishType::Shark.name(),
            },
        ],
        t: LayerType::City,
        depth: 1.0,
        speed: 0.0,
        size: Vec2::new(K_WIDTH, K_HEIGHT),
        name: "Fauna".to_string(),
    };

    let layer_terrain = LayerDesc {
        objects: vec![
            LayerObjectDesc {
                t: ObjectType::Primitive(PrimitiveType::Rectangle),
                component: ObjectComponentType::Land,
                position: Vec2::new(
                    K_OCEAN_LAND_BORDER - K_LAND_SIZE / 2.0,
                    K_GROUND_LEVEL - K_HEIGHT / 2.0,
                ),
                size: Vec2::new(K_LAND_SIZE, K_HEIGHT),
                color: Color::srgb_u8(60, 128, 60),
                name: "Land".to_string(),
            },
            LayerObjectDesc {
                t: ObjectType::Primitive(PrimitiveType::Rectangle),
                component: ObjectComponentType::Ocean,
                position: Vec2::new(
                    K_OCEAN_LAND_BORDER + K_OCEAN_SIZE / 2.0,
                    K_GROUND_LEVEL - K_HEIGHT / 2.0,
                ),
                size: Vec2::new(K_OCEAN_SIZE, K_HEIGHT),
                color: Color::srgb_u8(85, 128, 200),
                name: "Ocean".to_string(),
            },
        ],
        t: LayerType::City,
        depth: 1.0,
        speed: 0.0,
        size: Vec2::new(K_WIDTH, K_HEIGHT),
        name: "Terrain".to_string(),
    };

    let layer_city = LayerDesc {
        objects: vec![LayerObjectDesc {
            t: ObjectType::Sprite(SpriteDesc {
                path: "building/hut.png".to_string(),
                ..default()
            }),
            component: ObjectComponentType::Building,
            position: Vec2::new(K_OCEAN_LAND_BORDER, K_GROUND_LEVEL + 128.0 - 66.0),
            size: Vec2::new(480.0, 320.0),
            color: Color::srgb(1.0, 1.0, 1.0),
            name: "Hut".to_string(),
        }],
        t: LayerType::City,
        depth: 0.0,
        speed: 0.0,
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
        t: LayerType::Sky,
        depth: -9.0,
        speed: 1.0,
        size: Vec2::new(K_WIDTH, K_HEIGHT),
        name: "Sun".to_string(),
    };

    let layer_sky = LayerDesc {
        objects: vec![LayerObjectDesc {
            t: ObjectType::Sprite(SpriteDesc {
                path: "sky/sky.png".to_string(),
                ..default()
            }),
            component: ObjectComponentType::Sky,
            position: Vec2::new(0.0, 0.0),
            size: Vec2::new(K_WIDTH, K_HEIGHT),
            color: Color::srgb(1.0, 1.0, 1.0),
            name: "Sky".to_string(),
        }],
        t: LayerType::Sky,
        depth: -10.0,
        speed: 1.0,
        size: Vec2::new(K_WIDTH, K_HEIGHT),
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
            color: Color::srgb(1.0, 1.0, 1.0),
            name: "Mountain".to_string(),
        }],
        t: LayerType::Sky,
        depth: -8.0,
        speed: 0.75,
        size: Vec2::new(8.0 * K_WIDTH, K_HEIGHT),
        name: "Mountain".to_string(),
    };

    let layer_clouds = LayerDesc {
        objects: vec![
            LayerObjectDesc {
                t: ObjectType::Sprite(SpriteDesc {
                    path: "sky/clouds_mg_3.png".to_string(),
                    mode: SpriteImageMode::Tiled {
                        tile_x: true,
                        tile_y: false,
                        stretch_value: 3.33,
                    },
                }),
                component: ObjectComponentType::Cloud(Cloud { speed: 0.5 }),
                position: Vec2::new(0.0, 0.0),
                size: Vec2::new(4.0 * K_WIDTH, K_HEIGHT),
                color: Color::srgb(1.0, 1.0, 1.0),
                name: "Clouds-3".to_string(),
            },
            LayerObjectDesc {
                t: ObjectType::Sprite(SpriteDesc {
                    path: "sky/clouds_mg_2.png".to_string(),
                    mode: SpriteImageMode::Tiled {
                        tile_x: true,
                        tile_y: false,
                        stretch_value: 3.33,
                    },
                }),
                component: ObjectComponentType::Cloud(Cloud { speed: 0.75 }),
                position: Vec2::new(0.0, 0.0),
                size: Vec2::new(4.0 * K_WIDTH, K_HEIGHT),
                color: Color::srgb(1.0, 1.0, 1.0),
                name: "Clouds-2".to_string(),
            },
            LayerObjectDesc {
                t: ObjectType::Sprite(SpriteDesc {
                    path: "sky/clouds_mg_1.png".to_string(),
                    mode: SpriteImageMode::Tiled {
                        tile_x: true,
                        tile_y: false,
                        stretch_value: 3.33,
                    },
                }),
                component: ObjectComponentType::Cloud(Cloud { speed: 1.0 }),
                position: Vec2::new(0.0, 0.0),
                size: Vec2::new(4.0 * K_WIDTH, K_HEIGHT),
                color: Color::srgb(1.0, 1.0, 1.0),
                name: "Clouds-1".to_string(),
            },
        ],
        t: LayerType::Sky,
        depth: -5.0,
        speed: 1.0,
        size: Vec2::new(K_WIDTH, K_HEIGHT),
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
            color: Color::srgb(1.0, 1.0, 1.0),
            name: "Forest".to_string(),
        }],
        t: LayerType::Sky,
        depth: -4.0,
        speed: 0.5,
        size: Vec2::new(8.0 * K_WIDTH, K_HEIGHT),
        name: "Forest".to_string(),
    };

    let layer_play = LayerDesc {
        objects: vec![LayerObjectDesc {
            t: ObjectType::SpriteCollection(vec![
                SpriteAtlasDesc {
                    sprite: SpriteDesc {
                        path: "player/walk.png".to_string(),
                        ..default()
                    },
                    tile: UVec2::splat(48),
                    cols: 6,
                    rows: 1,
                    index: 0,
                    mode: TimerMode::Repeating,
                    ms: K_ANIMATION_FRAME_MS,
                },
                SpriteAtlasDesc {
                    sprite: SpriteDesc {
                        path: "player/row.png".to_string(),
                        ..default()
                    },
                    tile: UVec2::splat(48),
                    cols: 4,
                    rows: 1,
                    index: 0,
                    mode: TimerMode::Repeating,
                    ms: K_ANIMATION_FRAME_MS,
                },
                SpriteAtlasDesc {
                    sprite: SpriteDesc {
                        path: "player/fish.png".to_string(),
                        ..default()
                    },
                    tile: UVec2::splat(48),
                    cols: 4,
                    rows: 1,
                    index: 0,
                    mode: TimerMode::Repeating,
                    ms: K_ANIMATION_FRAME_MS,
                },
                SpriteAtlasDesc {
                    sprite: SpriteDesc {
                        path: "player/idle.png".to_string(),
                        ..default()
                    },
                    tile: UVec2::splat(48),
                    cols: 4,
                    rows: 1,
                    index: 0,
                    mode: TimerMode::Repeating,
                    ms: K_ANIMATION_FRAME_MS,
                },
                SpriteAtlasDesc {
                    sprite: SpriteDesc {
                        path: "player/hook.png".to_string(),
                        ..default()
                    },
                    tile: UVec2::splat(48),
                    cols: 6,
                    rows: 1,
                    index: 0,
                    mode: TimerMode::Once,
                    ms: K_ANIMATION_FRAME_MS / 2,
                },
                SpriteAtlasDesc {
                    sprite: SpriteDesc {
                        path: "player/attack.png".to_string(),
                        ..default()
                    },
                    tile: UVec2::splat(48),
                    cols: 6,
                    rows: 1,
                    index: 0,
                    mode: TimerMode::Once,
                    ms: K_ANIMATION_FRAME_MS / 2,
                },
            ]),
            component: ObjectComponentType::Player,
            position: Vec2::new(0.0, K_GROUND_LEVEL + 64.0),
            size: Vec2::new(128.0, 128.0),
            color: Color::srgb(1.0, 1.0, 1.0),
            name: "Player".to_string(),
        }],
        t: LayerType::Player,
        depth: 5.0,
        speed: 0.0,
        size: Vec2::new(K_WIDTH, K_HEIGHT),
        name: "Player".to_string(),
    };

    let layer_boat = LayerDesc {
        objects: vec![LayerObjectDesc {
            t: ObjectType::Sprite(SpriteDesc {
                path: "boat/boat.png".to_string(),
                ..default()
            }),
            component: ObjectComponentType::Boat,
            position: Vec2::new(K_OCEAN_LAND_BORDER, K_GROUND_LEVEL + 16.0),
            size: Vec2::new(160.0, 40.0),
            color: Color::srgb(1.0, 1.0, 1.0),
            name: "Boat".to_string(),
        }],
        t: LayerType::Boat,
        depth: 5.5,
        speed: 0.0,
        size: Vec2::new(K_WIDTH, K_HEIGHT),
        name: "Boat".to_string(),
    };

    layer_fauna.build(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut texture_atlas_layouts,
        &mut materials,
    );
    layer_terrain.build(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut texture_atlas_layouts,
        &mut materials,
    );
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
    layer_clouds.build(
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
