use bevy::prelude::*;
use crate::{types::{ObjectType, PrimitiveType}, components::ObjectComponentType};

/// Layer System

#[derive(Debug, Clone, PartialEq)]
pub struct LayerObject {
    pub t: ObjectType,
    pub component: ObjectComponentType,
    pub position: Vec2,
    pub size: Vec2,
    pub color: Color,
    pub name: String,
}

pub struct Layer {
    pub objects: Vec<LayerObject>,
    pub depth: f32,
}

impl Layer {
    pub fn build(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        for obj in &self.objects {
            // Create the appropriate mesh based on primitive type
            let mesh = match obj.t {
                ObjectType::Primitive(PrimitiveType::Rectangle) => {
                    meshes.add(Rectangle::new(obj.size.x, obj.size.y))
                }
                ObjectType::Primitive(PrimitiveType::Circle) => {
                    meshes.add(Circle::new(f32::max(obj.size.x, obj.size.y) / 2.0))
                }
            };

            // Spawn entity with common components
            let mut entity = commands.spawn((
                Mesh2d(mesh),
                MeshMaterial2d(materials.add(obj.color.clone())),
                Transform::from_xyz(obj.position.x, obj.position.y, self.depth),
                Name::new(obj.name.clone()),
            ));

            // Add the specific component type
            match &obj.component {
                ObjectComponentType::Player(_) => {
                    entity.insert(crate::components::Player);
                }
                ObjectComponentType::Building(_) => {
                    entity.insert(crate::components::Building);
                }
                ObjectComponentType::Sun(_) => {
                    entity.insert(crate::components::Sun);
                }
            }
        }
    }
}