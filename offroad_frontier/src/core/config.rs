//! Game configuration settings

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    /// Graphics settings
    pub graphics: GraphicsConfig,
    
    /// Physics settings
    pub physics: PhysicsConfig,
    
    /// Audio settings
    pub audio: AudioConfig,
    
    /// Gameplay settings
    pub gameplay: GameplayConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsConfig {
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
    pub shadow_quality: ShadowQuality,
    pub texture_quality: TextureQuality,
    pub anti_aliasing: bool,
    pub bloom_enabled: bool,
    pub motion_blur: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ShadowQuality {
    Low,
    Medium,
    High,
    Ultra,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TextureQuality {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsConfig {
    pub timestep: f32,
    pub substeps: u32,
    pub vehicle_physics_enabled: bool,
    pub terrain_deformation_enabled: bool,
    pub weather_effects_on_physics: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub engine_sound_enabled: bool,
    pub environmental_sounds: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplayConfig {
    pub difficulty: Difficulty,
    pub assists_enabled: bool,
    pub damage_model: DamageModel,
    pub fuel_consumption: bool,
    pub auto_save: bool,
    pub camera_mode: CameraMode,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Realistic,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DamageModel {
    None,
    Arcade,
    Simulation,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CameraMode {
    ThirdPerson,
    FirstPerson,
    Hood,
    Cinematic,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            graphics: GraphicsConfig {
                resolution_width: 1920,
                resolution_height: 1080,
                fullscreen: false,
                vsync: true,
                shadow_quality: ShadowQuality::High,
                texture_quality: TextureQuality::High,
                anti_aliasing: true,
                bloom_enabled: true,
                motion_blur: false,
            },
            physics: PhysicsConfig {
                timestep: 1.0 / 60.0,
                substeps: 4,
                vehicle_physics_enabled: true,
                terrain_deformation_enabled: true,
                weather_effects_on_physics: true,
            },
            audio: AudioConfig {
                master_volume: 0.8,
                music_volume: 0.5,
                sfx_volume: 0.7,
                engine_sound_enabled: true,
                environmental_sounds: true,
            },
            gameplay: GameplayConfig {
                difficulty: Difficulty::Normal,
                assists_enabled: true,
                damage_model: DamageModel::Arcade,
                fuel_consumption: false,
                auto_save: true,
                camera_mode: CameraMode::ThirdPerson,
            },
        }
    }
}

impl GameConfig {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: GameConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
