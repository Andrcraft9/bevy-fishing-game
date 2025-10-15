use bevy::prelude::*;

/// Layer Description / Types

#[derive(Debug, Clone, PartialEq, Eq)]
enum PrimitiveType {
    Rectangle,
    Circle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ObjectType {
    Primitive(PrimitiveType),
}

#[derive(Component, Debug, Clone, PartialEq)]
struct Player;

#[derive(Component, Debug, Clone, PartialEq)]
struct Building;

#[derive(Component, Debug, Clone, PartialEq)]
struct Sun;

#[derive(Debug, Clone, PartialEq)]
enum ObjectComponentType {
    Player(Player),
    Building(Building),
    Sun(Sun),
}

#[derive(Debug, Clone, PartialEq)]
struct LayerObject {
    pub t: ObjectType,
    pub component: ObjectComponentType,
    pub position: Vec2,
    pub size: Vec2,
    pub color: Color,
    pub name: String,
}

struct Layer {
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
                    entity.insert(Player);
                }
                ObjectComponentType::Building(_) => {
                    entity.insert(Building);
                }
                ObjectComponentType::Sun(_) => {
                    entity.insert(Sun);
                }
            }
        }
    }
}

/// Game

const K_WIDTH: f32 = 1280.0;
const K_HEIGHT: f32 = 720.0;
const K_GROUND_LEVEL: f32 = -350.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (K_WIDTH, K_HEIGHT).into(),
                title: "Bevy Game Project".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, player_control)
        .add_systems(Update, sun_update)
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

fn player_control(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Single<&mut Transform, With<Player>>,
) {
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
        println!("Moving left!");
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
        println!("Moving right!");
    }

    let speed = 150.0;
    player_query.translation += direction * speed * time.delta_secs();

    if direction != Vec3::ZERO {
        println!("Player position: {:?}", player_query.translation);
    }
}

fn sun_update(time: Res<Time>, mut sun_query: Single<&mut Transform, With<Sun>>) {
    sun_query.translation.x = (0.5 * time.elapsed_secs()).sin() * K_WIDTH / 2.0;
    println!("Sun position: {:?}", sun_query.translation);
}
