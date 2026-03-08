//! Game systems module
//! 
//! Contains all game systems: vehicle physics, weather, terrain, etc.

pub mod vehicle;
pub mod weather;
pub mod terrain;
pub mod winch;
pub mod camera;
pub mod mission;
pub mod audio;

pub use vehicle::{VehicleState, VehiclePhysics, WheelState};
pub use weather::{WeatherState, WeatherType, PrecipitationType};
pub use terrain::TerrainSystem;
pub use winch::WinchSystem;
pub use camera::CameraSystem;
pub use mission::{MissionManager, Mission, MissionType, MissionDifficulty, CargoType};
