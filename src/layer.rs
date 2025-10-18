use crate::{
    components::{self, Building, Ocean, Player, Sun},
    types::{ObjectType, PrimitiveType},
};
use bevy::prelude::*;

/// Layer System

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectComponentType {
    Player(Player),
    Ocean(Ocean),
    Building(Building),
    Sun(Sun),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayerObject {
    pub t: ObjectType,
    pub component: ObjectComponentType,
    pub position: Vec2,
    pub size: Vec2,
    pub color: Color,
    pub name: String,
}

pub struct LayerDesc {
    pub objects: Vec<LayerObject>,
    pub depth: f32,
    pub size: Vec2,
    pub name: String,
}

impl LayerDesc {
    pub fn build(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
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
            let mesh = match obj.t {
                ObjectType::Primitive(PrimitiveType::Rectangle) => {
                    meshes.add(Rectangle::new(obj.size.x, obj.size.y))
                }
                ObjectType::Primitive(PrimitiveType::Circle) => {
                    meshes.add(Circle::new(f32::max(obj.size.x, obj.size.y) / 2.0))
                }
            };

            // Spawn entity with common components
            let entity_id = commands
                .spawn((
                    Mesh2d(mesh),
                    MeshMaterial2d(materials.add(obj.color.clone())),
                    Transform::from_xyz(obj.position.x, obj.position.y, 0.0),
                    Name::new(obj.name.clone()),
                ))
                .id();

            // Add the specific component type
            match &obj.component {
                ObjectComponentType::Player(_) => {
                    commands.entity(entity_id).insert(crate::components::Player);
                }
                ObjectComponentType::Ocean(_) => {
                    commands.entity(entity_id).insert(crate::components::Ocean);
                }
                ObjectComponentType::Building(_) => {
                    commands
                        .entity(entity_id)
                        .insert(crate::components::Building);
                }
                ObjectComponentType::Sun(_) => {
                    commands.entity(entity_id).insert(crate::components::Sun);
                }
            }

            commands.entity(layer_entity).add_child(entity_id);
        }
    }
}
