/// Game Constants

/// Coordinate space:
///
/// (-w/2, h/2)       (w/2, h/2)
///      +----------------+
///      |     (0,0)      |
///      |       |        |
///      +----------------+
/// (-w/2, -h/2)       (w/2, -h/2)
///
/// TODO: Move to Resources: handle resizes and speed change.
pub const K_WIDTH: f32 = 1280.0;
pub const K_HEIGHT: f32 = 720.0;
pub const K_GROUND_LEVEL: f32 = 32.0 - K_HEIGHT / 2.0;
pub const K_SPEED: f32 = 150.0;
pub const K_ANIMATION_FRAME_MS: u64 = 100;
pub const K_SECS_IN_DAY: f32 = 30.0;
pub const K_OCEAN_LAND_BORDER: f32 = 512.0;
pub const K_OCEAN_SIZE: f32 = 4096.0;
pub const K_LAND_SIZE: f32 = 4096.0;
pub const K_SIT_OFFSET: f32 = -22.0;
