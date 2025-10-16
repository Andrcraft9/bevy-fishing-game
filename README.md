# Bevy Game Project

A modular 2D game prototype built with the Bevy game engine, demonstrating clean architecture and separation of concerns following Rust best practices.

## 🎮 Game Overview

This is a simple 2D side-scrolling prototype featuring:
- **Player Character**: White rectangle controlled with A/D keys
- **Buildings**: Interactive red and green buildings
- **Sun Animation**: Yellow circle that moves across the sky
- **Action System**: Space bar triggers interactions with nearby buildings

## 🏗️ Architecture

The project follows a modular architecture with clear separation of concerns:

```
src/
├── main.rs          # Entry point and game setup
├── components.rs    # ECS Components
├── events.rs        # Game events
├── types.rs         # Type definitions
├── layer.rs         # Object spawning system
├── constants.rs     # Game constants
└── systems/         # Game systems organized by domain
    ├── mod.rs       # Module declarations
    ├── player.rs    # Player movement and actions
    ├── sun.rs       # Sun animation
    └── action.rs    # Action handling and building interaction
```

### 📦 Module Breakdown

#### `main.rs`
- Application entry point
- Bevy app configuration and plugin setup
- Scene setup with layers (city, sun, player)

#### `components.rs`
- **Player**: Component for the player character
- **Building**: Component for interactive buildings
- **Sun**: Component for the animated sun
- **ObjectComponentType**: Enum for layer system integration

#### `events.rs`
- **Action**: Event triggered when player presses space bar

#### `types.rs`
- **PrimitiveType**: Shape definitions (Rectangle, Circle)
- **ObjectType**: Object type wrapper for primitives

#### `layer.rs`
- **LayerObject**: Definition for objects in the scene
- **Layer**: Container for objects with depth ordering
- Automated entity spawning with proper components

#### `constants.rs`
- Game dimensions (1280x720)
- Ground level positioning
- Action interaction radius

#### `systems/`
- **player.rs**: Movement controls and action triggering
- **sun.rs**: Animated sun movement across the sky
- **action.rs**: Building interaction detection and feedback

## 🚀 Getting Started

### Prerequisites
- Rust (latest stable version)
- Cargo package manager

### Running the Game
```bash
# Clone and navigate to the project
cd game-project

# Run the game
cargo run

# Or for optimized build
cargo run --release
```

### Controls
- **A**: Move left
- **D**: Move right  
- **Space**: Interact with nearby buildings (within 25 units)

## 🔧 Development

### Adding New Components
1. Define the component in `src/components.rs`
2. Add it to `ObjectComponentType` enum if needed for layers
3. Update layer building logic in `src/layer.rs`

### Adding New Systems
1. Create a new file in `src/systems/`
2. Add the module declaration to `src/systems/mod.rs`
3. Register the system in `main.rs`

### Adding New Events
1. Define the event in `src/events.rs`
2. Trigger events using `commands.trigger(YourEvent)`
3. Handle events with observer systems

## 🎯 Design Principles

### Separation of Concerns
- Each module has a single, well-defined responsibility
- Systems are organized by domain (player, sun, actions)
- Components are pure data structures

### Modularity
- Clear module boundaries with explicit imports
- Easy to add, remove, or modify individual systems
- Minimal coupling between modules

### Bevy Best Practices
- Proper use of ECS (Entity-Component-System) pattern
- Event-driven architecture for loose coupling
- System organization following Bevy conventions

### Rust Best Practices
- Clear module structure with `mod.rs` files
- Public interfaces explicitly defined
- Consistent naming conventions

## 🔮 Future Enhancements

Potential areas for expansion:
- **Physics System**: Add collision detection and physics
- **Asset Loading**: Sprite-based graphics instead of primitive shapes
- **Game States**: Menu, gameplay, pause states
- **Audio System**: Sound effects and background music
- **Save System**: Player progress persistence
- **Level System**: Multiple levels or procedural generation

## 📊 Performance Considerations

- Uses Bevy's efficient ECS for optimal performance
- Minimal allocations in hot paths (game loops)
- Clear system scheduling for predictable frame timing
- Modular architecture allows for easy profiling and optimization

## 🛠️ Build Configuration

The project uses standard Cargo configuration:
- **Debug builds**: Fast compilation, debug symbols included
- **Release builds**: Optimized performance, smaller binary size

```bash
# Debug build (default)
cargo build

# Release build
cargo build --release
```

## 📝 License

This project is a prototype/educational example. Feel free to use it as a starting point for your own Bevy projects.