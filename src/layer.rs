use crate::components::{
    self, ActionRange, ActiveSprite, AnimationConfig, AnimationTimer, Boat, Building, Cloud,
    DayNightColor, DefaultColor, Direction, Land, Layer, Ocean, OnControl, Player, PlayerState,
    Sky, SpriteCollection, Sun, Velocity,
};
use bevy::prelude::*;

/// Layer System

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveType {
    Rectangle,
    Circle,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SpriteDesc {
    pub path: String,
    pub mode: SpriteImageMode,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SpriteAtlasDesc {
    pub sprite: SpriteDesc,
    pub splat: u32,
    pub rows: u32,
    pub cols: u32,
    pub index: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    Primitive(PrimitiveType),
    Sprite(SpriteDesc),
    SpriteAtlas(SpriteAtlasDesc),
    SpriteCollection(Vec<SpriteAtlasDesc>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectComponentType {
    Player,
    Boat,
    Land,
    Ocean,
    Building,
    Sun,
    Cloud(Cloud),
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

#[derive(Debug, Clone, PartialEq)]
pub enum LayerType {
    Player,
    Boat,
    City,
    Sky,
}

pub struct LayerDesc {
    pub t: LayerType,
    pub objects: Vec<LayerObjectDesc>,
    pub depth: f32,
    pub speed: f32,
    pub size: Vec2,
    pub name: String,
}

impl LayerDesc {
    fn create_sprite_atlas_entity(
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        atlas: &SpriteAtlasDesc,
        size: Vec2,
        color: Color,
    ) -> (Sprite, AnimationConfig) {
        let texture = asset_server.load(atlas.sprite.path.clone());
        let layout = TextureAtlasLayout::from_grid(
            UVec2::splat(atlas.splat),
            atlas.cols,
            atlas.rows,
            None,
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let sprite = Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: atlas.index,
            }),
            custom_size: Some(Vec2::new(size.x, size.y)),
            color: color,
            image_mode: atlas.sprite.mode.clone(),
            ..default()
        };

        let animation_config = AnimationConfig {
            first_index: atlas.index,
            last_index: std::cmp::max(atlas.cols - 1, atlas.rows - 1) as usize,
        };

        (sprite, animation_config)
    }

    pub fn build(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let mut layer_entity = commands.spawn((
            Transform::from_xyz(0.0, 0.0, self.depth),
            Name::new(self.name.clone()),
            Layer {
                depth: self.depth,
                speed: self.speed,
                size: self.size,
            },
        ));

        let layer_entity = match self.t {
            LayerType::Player => layer_entity.insert(components::PlayerLayer).id(),
            LayerType::Boat => layer_entity.insert(components::BoatLayer).id(),
            LayerType::City => layer_entity.insert(components::CityLayer).id(),
            LayerType::Sky => layer_entity.insert(components::SkyLayer).id(),
        };

        for obj in &self.objects {
            let entity_id = match &obj.t {
                ObjectType::Primitive(PrimitiveType::Rectangle) => {
                    let mesh = meshes.add(Rectangle::new(obj.size.x, obj.size.y));
                    commands
                        .spawn((
                            Mesh2d(mesh),
                            MeshMaterial2d(materials.add(obj.color)),
                            DefaultColor { color: obj.color },
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
                            MeshMaterial2d(materials.add(obj.color)),
                            DefaultColor { color: obj.color },
                            Transform::from_xyz(obj.position.x, obj.position.y, 0.0),
                            Name::new(obj.name.clone()),
                        ))
                        .id()
                }
                ObjectType::Sprite(sprite) => {
                    let texture: Handle<Image> = asset_server.load(sprite.path.clone());
                    commands
                        .spawn((
                            Sprite {
                                image: texture.clone(),
                                custom_size: Some(Vec2::new(obj.size.x, obj.size.y)),
                                image_mode: sprite.mode.clone(),
                                color: obj.color,
                                ..default()
                            },
                            DefaultColor { color: obj.color },
                            Transform::from_xyz(obj.position.x, obj.position.y, 0.0),
                            Name::new(obj.name.clone()),
                        ))
                        .id()
                }
                ObjectType::SpriteAtlas(atlas) => {
                    let (sprite, animation_config) = Self::create_sprite_atlas_entity(
                        asset_server,
                        texture_atlas_layouts,
                        atlas,
                        obj.size,
                        obj.color,
                    );

                    let entity = commands.spawn((
                        sprite,
                        animation_config,
                        AnimationTimer {
                            timer: Timer::default(),
                            ms: 100,
                            reset: true,
                        },
                        Transform::from_xyz(obj.position.x, obj.position.y, 0.0),
                        Name::new(obj.name.clone()),
                    ));

                    entity.id()
                }
                ObjectType::SpriteCollection(collection) => {
                    let mut entity = commands.spawn((
                        Transform::from_xyz(obj.position.x, obj.position.y, 0.0),
                        Name::new(obj.name.clone()),
                    ));

                    let mut sprite_collection = SpriteCollection {
                        sprites: Vec::new(),
                        animations: Vec::new(),
                    };
                    for atlas in collection {
                        let (sprite, animation_config) = Self::create_sprite_atlas_entity(
                            asset_server,
                            texture_atlas_layouts,
                            atlas,
                            obj.size,
                            obj.color,
                        );

                        sprite_collection.sprites.push(sprite);
                        sprite_collection.animations.push(animation_config);
                    }

                    // Active sprite is always the first one
                    entity.insert(sprite_collection.sprites[0].clone());
                    entity.insert(DefaultColor { color: obj.color });
                    entity.insert(sprite_collection.animations[0].clone());
                    entity.insert(AnimationTimer {
                        timer: Timer::default(),
                        ms: 100,
                        reset: true,
                    });
                    entity.insert(ActiveSprite { index: 0 });

                    entity.insert(sprite_collection);

                    entity.id()
                }
            };

            // Add the specific component type
            match &obj.component {
                ObjectComponentType::Player => {
                    commands
                        .entity(entity_id)
                        .insert(Player {
                            money: 0.0,
                            items: Vec::new(),
                        })
                        .insert(Direction::Right)
                        .insert(PlayerState::Walk)
                        .insert(Velocity { ..default() })
                        .insert(OnControl);
                }
                ObjectComponentType::Boat => {
                    commands
                        .entity(entity_id)
                        .insert(Boat)
                        .insert(Velocity { ..default() })
                        .insert(DayNightColor);
                }
                ObjectComponentType::Land => {
                    commands.entity(entity_id).insert(Land {
                        size: obj.size.clone(),
                    });
                }
                ObjectComponentType::Ocean => {
                    commands
                        .entity(entity_id)
                        .insert(Ocean {
                            size: obj.size.clone(),
                        })
                        .insert(ActionRange {
                            range: obj.size.x / 2.0,
                        });
                }
                ObjectComponentType::Building => {
                    commands
                        .entity(entity_id)
                        .insert(Building)
                        .insert(ActionRange {
                            range: obj.size.x / 2.0,
                        })
                        .insert(DayNightColor);
                }
                ObjectComponentType::Sun => {
                    commands.entity(entity_id).insert(Sun);
                }
                ObjectComponentType::Cloud(cloud) => {
                    commands
                        .entity(entity_id)
                        .insert(cloud.clone())
                        .insert(DayNightColor);
                }
                ObjectComponentType::Sky => {
                    commands.entity(entity_id).insert(Sky).insert(DayNightColor);
                }
            }

            commands.entity(layer_entity).add_child(entity_id);
        }
    }
}
