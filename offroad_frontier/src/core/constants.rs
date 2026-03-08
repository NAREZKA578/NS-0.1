//! Game constants and configuration values

/// Physics constants
pub const GRAVITY: f32 = 9.81;
pub const MAX_STEERING_ANGLE: f32 = 35.0; // degrees
pub const BRAKE_FORCE: f32 = 15000.0;
pub const ENGINE_TORQUE: f32 = 450.0;

/// Terrain constants
pub const TERRAIN_CHUNK_SIZE: u32 = 64;
pub const MAX_DEFORMATION_DEPTH: f32 = 0.5; // meters
pub const DEFORMATION_RECOVERY_RATE: f32 = 0.01; // meters per second

/// Vehicle constants
pub const WHEEL_COUNT: usize = 4;
pub const SUSPENSION_STIFFNESS: f32 = 18000.0;
pub const SUSPENSION_DAMPING: f32 = 1500.0;
pub const TIRE_FRICTION_DRY: f32 = 1.2;
pub const TIRE_FRICTION_WET: f32 = 0.7;
pub const TIRE_FRICTION_MUD: f32 = 0.3;
pub const TIRE_FRICTION_SNOW: f32 = 0.25;
pub const TIRE_FRICTION_ICE: f32 = 0.1;

/// Game settings
pub const FOV: f32 = 75.0;
pub const CAMERA_SMOOTHING: f32 = 5.0;
pub const UI_SCALE: f32 = 1.0;

/// Surface types enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceType {
    Asphalt,
    Dirt,
    Mud,
    Sand,
    Snow,
    Ice,
    Grass,
    Rock,
}

impl SurfaceType {
    pub fn friction_coefficient(&self) -> f32 {
        match self {
            SurfaceType::Asphalt => TIRE_FRICTION_DRY,
            SurfaceType::Dirt => 0.8,
            SurfaceType::Mud => TIRE_FRICTION_MUD,
            SurfaceType::Sand => 0.5,
            SurfaceType::Snow => TIRE_FRICTION_SNOW,
            SurfaceType::Ice => TIRE_FRICTION_ICE,
            SurfaceType::Grass => 0.6,
            SurfaceType::Rock => 0.9,
        }
    }

    pub fn deformation_factor(&self) -> f32 {
        match self {
            SurfaceType::Asphalt => 0.0,
            SurfaceType::Dirt => 0.7,
            SurfaceType::Mud => 1.0,
            SurfaceType::Sand => 0.8,
            SurfaceType::Snow => 0.9,
            SurfaceType::Ice => 0.1,
            SurfaceType::Grass => 0.3,
            SurfaceType::Rock => 0.0,
        }
    }
}
