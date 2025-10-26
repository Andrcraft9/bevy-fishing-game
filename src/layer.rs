use std::time::Duration;

use crate::components::{self, AnimationConfig, Building, Ocean, Player, Sun};
use bevy::{asset::AssetPath, prelude::*};

/// Layer System

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveType {
    Rectangle,
    Circle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpriteDesc {
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpriteAtlasDesc {
    pub path: String,
    pub splat: u32,
    pub cols: u32,
    pub rows: u32,
    pub index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectType {
    Primitive(PrimitiveType),
    Sprite(SpriteDesc),
    SpriteAtlas(SpriteAtlasDesc),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectComponentType {
    Player,
    Boat,
    Land,
    Ocean,
    Building,
    Sun,
    Sky,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayerObjectDesc {
    pub t: ObjectType,
    pub component: ObjectComponentType,
    pub position: Vec2,
    pub size: Vec2,
    pub color: Color,
    pub name: String,
}

pub struct LayerDesc {
    pub objects: Vec<LayerObjectDesc>,
    pub depth: f32,
    pub size: Vec2,
    pub name: String,
}

impl LayerDesc {
    pub fn build(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let layer_entity = commands
            .spawn((
                Transform::from_xyz(0.0, 0.0, self.depth),
                Name::new(self.name.clone()),
                components::Layer {
                    depth: self.depth,
                    size: self.size,
                },
            ))
            .id();

        for obj in &self.objects {
            let entity_id = match &obj.t {
                ObjectType::Primitive(PrimitiveType::Rectangle) => {
                    let mesh = meshes.add(Rectangle::new(obj.size.x, obj.size.y));
                    commands
                        .spawn((
                            Mesh2d(mesh),
                            MeshMaterial2d(materials.add(obj.color.clone())),
                            Transform::from_xyz(obj.position.x, obj.position.y, 0.0),
                            Name::new(obj.name.clone()),
                        ))
                        .id()
                }
                ObjectType::Primitive(PrimitiveType::Circle) => {
                    let mesh = meshes.add(Circle::new(f32::max(obj.size.x, obj.size.y) / 2.0));
                    commands
                        .spawn((
                            Mesh2d(mesh),
                            MeshMaterial2d(materials.add(obj.color.clone())),
                            Transform::from_xyz(obj.position.x, obj.position.y, 0.0),
                            Name::new(obj.name.clone()),
                        ))
                        .id()
                }
                ObjectType::Sprite(sprite) => {
                    let texture = asset_server.load(sprite.path.clone());
                    commands
                        .spawn((
                            Sprite {
                                image: texture.clone(),
                                custom_size: Some(Vec2::new(obj.size.x, obj.size.y)),
                                ..default()
                            },
                            Transform::from_xyz(obj.position.x, obj.position.y, 0.0),
                            Name::new(obj.name.clone()),
                        ))
                        .id()
                }
                ObjectType::SpriteAtlas(sprite) => {
                    let texture = asset_server.load(sprite.path.clone());
                    let layout = TextureAtlasLayout::from_grid(
                        UVec2::splat(sprite.splat),
                        sprite.cols,
                        sprite.rows,
                        None,
                        None,
                    );
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    commands
                        .spawn((
                            Sprite {
                                image: texture.clone(),
                                texture_atlas: Some(TextureAtlas {
                                    layout: texture_atlas_layout.clone(),
                                    index: sprite.index,
                                }),
                                custom_size: Some(Vec2::new(obj.size.x, obj.size.y)),
                                ..default()
                            },
                            AnimationConfig {
                                first_index: sprite.index,
                                last_index: std::cmp::max(sprite.cols - 1, sprite.rows - 1)
                                    as usize,
                                timer: Timer::new(Duration::from_secs(0), TimerMode::Once),
                            },
                            Transform::from_xyz(obj.position.x, obj.position.y, 0.0),
                            Name::new(obj.name.clone()),
                        ))
                        .id()
                }
            };

            // Add the specific component type
            match &obj.component {
                ObjectComponentType::Player => {
                    commands
                        .entity(entity_id)
                        .insert(components::Player {
                            money: 0.0,
                            items: Vec::new(),
                        })
                        .insert(components::Direction::Right);
                }
                ObjectComponentType::Boat => {
                    commands.entity(entity_id).insert(components::Boat);
                }
                ObjectComponentType::Land => {
                    commands.entity(entity_id).insert(components::Land);
                }
                ObjectComponentType::Ocean => {
                    commands.entity(entity_id).insert(components::Ocean).insert(
                        components::ActionRange {
                            range: obj.size.x / 2.0,
                        },
                    );
                }
                ObjectComponentType::Building => {
                    commands
                        .entity(entity_id)
                        .insert(components::Building)
                        .insert(components::ActionRange {
                            range: obj.size.x / 2.0,
                        });
                }
                ObjectComponentType::Sun => {
                    commands.entity(entity_id).insert(components::Sun);
                }
                ObjectComponentType::Sky => {
                    commands.entity(entity_id).insert(components::Sky);
                }
            }

            commands.entity(layer_entity).add_child(entity_id);
        }
    }
}
