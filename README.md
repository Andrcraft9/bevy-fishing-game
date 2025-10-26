# Bevy Game Project

A modular 2D game prototype built with the Bevy game engine, demonstrating clean architecture and separation of concerns following Rust best practices.

## Architecture

The project follows a modular architecture with clear separation of concerns:

```
src/
├── main.rs          # Entry point and game setup
├── components.rs    # ECS Components
├── events.rs        # Game events
├── types.rs         # Type definitions
├── layer.rs         # Object spawning system
├── constants.rs     # Game constants
└── systems.rs       # Game systems
```

## Getting Started

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
- **Space**: Interact with nearby building

## License

This project is a prototype/educational example. Feel free to use it as a starting point for your own Bevy projects.

## Credits
- Free Fishing Game Assets Pixel Art Pack by https://craftpix.net, license: https://craftpix.net/file-licenses/