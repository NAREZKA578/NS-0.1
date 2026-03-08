//! Game state management

use crate::core::{GameConfig, Difficulty, CameraMode};
use crate::systems::vehicle::VehicleState;
use crate::systems::weather::WeatherState;

/// Current state of the game
#[derive(Debug, Clone)]
pub struct GameState {
    /// Is the game running?
    pub is_running: bool,
    
    /// Is the game paused?
    pub is_paused: bool,
    
    /// Current difficulty
    pub difficulty: Difficulty,
    
    /// Current camera mode
    pub camera_mode: CameraMode,
    
    /// Current vehicle state
    pub vehicle: VehicleState,
    
    /// Current weather state
    pub weather: WeatherState,
    
    /// Current mission index
    pub current_mission: Option<usize>,
    
    /// Player position in world coordinates
    pub player_position: glam::Vec3,
    
    /// Player rotation
    pub player_rotation: glam::Quat,
    
    /// Game time in seconds
    pub game_time: f32,
    
    /// Real time delta for the last frame
    pub delta_time: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            is_running: false,
            is_paused: false,
            difficulty: Difficulty::Normal,
            camera_mode: CameraMode::ThirdPerson,
            vehicle: VehicleState::default(),
            weather: WeatherState::default(),
            current_mission: None,
            player_position: glam::Vec3::ZERO,
            player_rotation: glam::Quat::IDENTITY,
            game_time: 0.0,
            delta_time: 0.0,
        }
    }
}

impl GameState {
    pub fn new(config: &GameConfig) -> Self {
        Self {
            is_running: true,
            is_paused: false,
            difficulty: config.gameplay.difficulty,
            camera_mode: config.gameplay.camera_mode,
            vehicle: VehicleState::default(),
            weather: WeatherState::default(),
            current_mission: None,
            player_position: glam::Vec3::new(0.0, 10.0, 0.0),
            player_rotation: glam::Quat::IDENTITY,
            game_time: 0.0,
            delta_time: 0.0,
        }
    }

    /// Update game state with delta time
    pub fn update(&mut self, delta_time: f32) {
        self.delta_time = delta_time;
        self.game_time += delta_time;
        
        // Update vehicle physics
        self.vehicle.update(delta_time);
        
        // Update weather
        self.weather.update(delta_time);
        
        // Update player position from vehicle
        self.player_position = self.vehicle.position;
        self.player_rotation = self.vehicle.rotation;
    }

    /// Pause the game
    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    /// Resume the game
    pub fn resume(&mut self) {
        self.is_paused = false;
    }

    /// Toggle pause state
    pub fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    /// Stop the game
    pub fn stop(&mut self) {
        self.is_running = false;
    }

    /// Set camera mode
    pub fn set_camera_mode(&mut self, mode: CameraMode) {
        self.camera_mode = mode;
    }

    /// Cycle through camera modes
    pub fn cycle_camera_mode(&mut self) {
        self.camera_mode = match self.camera_mode {
            CameraMode::ThirdPerson => CameraMode::FirstPerson,
            CameraMode::FirstPerson => CameraMode::Hood,
            CameraMode::Hood => CameraMode::Cinematic,
            CameraMode::Cinematic => CameraMode::ThirdPerson,
        };
    }
}
