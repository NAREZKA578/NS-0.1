//! Audio system
//!
//! Handles all game audio:
//! - Engine sounds (RPM-based)
//! - Environmental sounds (wind, rain)
//! - Collision and impact sounds
//! - UI feedback sounds
//! - Music system

use std::collections::HashMap;
use glam::Vec3;

/// Sound type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoundType {
    // Vehicle sounds
    EngineIdle,
    EngineRunning,
    EngineStart,
    EngineStop,
    GearShift,
    TireSlip,
    Suspension,
    
    // Environment sounds
    Wind,
    Rain,
    Thunder,
    WaterSplash,
    MudSquelch,
    
    // Collision sounds
    MetalImpact,
    WoodBreak,
    RockScrape,
    GlassBreak,
    
    // UI sounds
    MenuClick,
    MenuSelect,
    Notification,
    MissionComplete,
    MissionFail,
    
    // Winch sounds
    WinchExtend,
    WinchRetract,
    WinchTension,
    CableSnap,
}

/// Audio source configuration
#[derive(Debug, Clone)]
pub struct AudioSource {
    /// Sound file path
    pub path: String,
    /// Is sound looping
    pub is_looping: bool,
    /// Base volume (0.0 - 1.0)
    pub base_volume: f32,
    /// Pitch multiplier
    pub pitch: f32,
    /// 3D position (None = 2D sound)
    pub position: Option<Vec3>,
    /// Minimum distance for 3D attenuation
    pub min_distance: f32,
    /// Maximum distance for 3D attenuation
    pub max_distance: f32,
}

impl Default for AudioSource {
    fn default() -> Self {
        Self {
            path: String::new(),
            is_looping: false,
            base_volume: 1.0,
            pitch: 1.0,
            position: None,
            min_distance: 1.0,
            max_distance: 100.0,
        }
    }
}

/// Active sound instance
pub struct ActiveSound {
    /// Sound type
    pub sound_type: SoundType,
    /// Current volume (after attenuation)
    pub current_volume: f32,
    /// Is sound playing
    pub is_playing: bool,
    /// Playback progress (0.0 - 1.0)
    pub progress: f32,
    /// Source configuration
    pub source: AudioSource,
}

/// Audio category for volume control
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioCategory {
    Master,
    Music,
    SFX,
    Engine,
    Environment,
    UI,
}

/// Audio system manager
pub struct AudioSystem {
    /// Volume levels per category
    pub volumes: HashMap<AudioCategory, f32>,
    /// Loaded audio sources
    pub sound_library: HashMap<SoundType, AudioSource>,
    /// Currently active sounds
    pub active_sounds: Vec<ActiveSound>,
    /// Listener position (usually camera/vehicle)
    pub listener_position: Vec3,
    /// Listener velocity (for Doppler effect)
    pub listener_velocity: Vec3,
    /// Is audio muted
    pub is_muted: bool,
    /// Is environmental audio enabled
    pub environmental_enabled: bool,
}

impl Default for AudioSystem {
    fn default() -> Self {
        let mut volumes = HashMap::new();
        volumes.insert(AudioCategory::Master, 0.8);
        volumes.insert(AudioCategory::Music, 0.5);
        volumes.insert(AudioCategory::SFX, 0.7);
        volumes.insert(AudioCategory::Engine, 0.9);
        volumes.insert(AudioCategory::Environment, 0.6);
        volumes.insert(AudioCategory::UI, 0.8);

        Self {
            volumes,
            sound_library: HashMap::new(),
            active_sounds: Vec::new(),
            listener_position: Vec3::ZERO,
            listener_velocity: Vec3::ZERO,
            is_muted: false,
            environmental_enabled: true,
        }
    }
}

impl AudioSystem {
    /// Create new audio system
    pub fn new() -> Self {
        let mut system = Self::default();
        system.initialize_sound_library();
        system
    }

    /// Initialize sound library with default sounds
    fn initialize_sound_library(&mut self) {
        // Engine sounds
        self.sound_library.insert(SoundType::EngineIdle, AudioSource {
            path: "audio/vehicles/engine_idle.ogg".to_string(),
            is_looping: true,
            base_volume: 0.8,
            ..Default::default()
        });

        self.sound_library.insert(SoundType::EngineRunning, AudioSource {
            path: "audio/vehicles/engine_running.ogg".to_string(),
            is_looping: true,
            base_volume: 0.9,
            pitch: 1.0,
            ..Default::default()
        });

        self.sound_library.insert(SoundType::EngineStart, AudioSource {
            path: "audio/vehicles/engine_start.ogg".to_string(),
            is_looping: false,
            base_volume: 1.0,
            ..Default::default()
        });

        self.sound_library.insert(SoundType::GearShift, AudioSource {
            path: "audio/vehicles/gear_shift.ogg".to_string(),
            is_looping: false,
            base_volume: 0.5,
            ..Default::default()
        });

        self.sound_library.insert(SoundType::TireSlip, AudioSource {
            path: "audio/vehicles/tire_slip.ogg".to_string(),
            is_looping: true,
            base_volume: 0.6,
            ..Default::default()
        });

        // Environment sounds
        self.sound_library.insert(SoundType::Wind, AudioSource {
            path: "audio/environment/wind.ogg".to_string(),
            is_looping: true,
            base_volume: 0.4,
            ..Default::default()
        });

        self.sound_library.insert(SoundType::Rain, AudioSource {
            path: "audio/environment/rain.ogg".to_string(),
            is_looping: true,
            base_volume: 0.5,
            ..Default::default()
        });

        self.sound_library.insert(SoundType::Thunder, AudioSource {
            path: "audio/environment/thunder.ogg".to_string(),
            is_looping: false,
            base_volume: 0.8,
            ..Default::default()
        });

        self.sound_library.insert(SoundType::WaterSplash, AudioSource {
            path: "audio/environment/water_splash.ogg".to_string(),
            is_looping: false,
            base_volume: 0.7,
            ..Default::default()
        });

        self.sound_library.insert(SoundType::MudSquelch, AudioSource {
            path: "audio/environment/mud_squelch.ogg".to_string(),
            is_looping: false,
            base_volume: 0.6,
            ..Default::default()
        });

        // Collision sounds
        self.sound_library.insert(SoundType::MetalImpact, AudioSource {
            path: "audio/collisions/metal_impact.ogg".to_string(),
            is_looping: false,
            base_volume: 0.8,
            ..Default::default()
        });

        // UI sounds
        self.sound_library.insert(SoundType::MenuClick, AudioSource {
            path: "audio/ui/menu_click.ogg".to_string(),
            is_looping: false,
            base_volume: 0.6,
            ..Default::default()
        });

        self.sound_library.insert(SoundType::MissionComplete, AudioSource {
            path: "audio/ui/mission_complete.ogg".to_string(),
            is_looping: false,
            base_volume: 1.0,
            ..Default::default()
        });

        // Winch sounds
        self.sound_library.insert(SoundType::WinchExtend, AudioSource {
            path: "audio/vehicles/winch_extend.ogg".to_string(),
            is_looping: true,
            base_volume: 0.7,
            ..Default::default()
        });

        self.sound_library.insert(SoundType::WinchRetract, AudioSource {
            path: "audio/vehicles/winch_retract.ogg".to_string(),
            is_looping: true,
            base_volume: 0.7,
            ..Default::default()
        });
    }

    /// Update audio system
    pub fn update(&mut self, delta_time: f32, listener_pos: Vec3, listener_vel: Vec3) {
        self.listener_position = listener_pos;
        self.listener_velocity = listener_vel;

        if self.is_muted {
            return;
        }

        // Update active sounds
        self.active_sounds.retain_mut(|sound| {
            if !sound.is_playing {
                return false;
            }

            // Update 3D attenuation
            if let Some(pos) = sound.source.position {
                let distance = (pos - listener_pos).length();
                
                if distance > sound.source.max_distance {
                    sound.current_volume = 0.0;
                } else if distance < sound.source.min_distance {
                    sound.current_volume = sound.source.base_volume;
                } else {
                    // Linear attenuation
                    let t = (distance - sound.source.min_distance) 
                        / (sound.source.max_distance - sound.source.min_distance);
                    sound.current_volume = sound.source.base_volume * (1.0 - t);
                }
            } else {
                sound.current_volume = sound.source.base_volume;
            }

            // Apply category volume
            let category = self.get_sound_category(sound.sound_type);
            if let Some(&vol) = self.volumes.get(&category) {
                sound.current_volume *= vol;
            }

            // Update progress for non-looping sounds
            if !sound.source.is_looping {
                sound.progress += delta_time / 2.0; // Assume 2 second average sound
                if sound.progress >= 1.0 {
                    return false;
                }
            }

            true
        });
    }

    /// Play a sound
    pub fn play(&mut self, sound_type: SoundType, position: Option<Vec3>) {
        if self.is_muted {
            return;
        }

        if let Some(source) = self.sound_library.get(&sound_type).cloned() {
            let mut sound = ActiveSound {
                sound_type,
                current_volume: source.base_volume,
                is_playing: true,
                progress: 0.0,
                source,
            };

            if let Some(pos) = position {
                sound.source.position = Some(pos);
            }

            self.active_sounds.push(sound);
        }
    }

    /// Stop all sounds of a type
    pub fn stop(&mut self, sound_type: SoundType) {
        for sound in &mut self.active_sounds {
            if sound.sound_type == sound_type {
                sound.is_playing = false;
            }
        }
    }

    /// Stop all sounds
    pub fn stop_all(&mut self) {
        for sound in &mut self.active_sounds {
            sound.is_playing = false;
        }
    }

    /// Set volume for a category
    pub fn set_volume(&mut self, category: AudioCategory, volume: f32) {
        self.volumes.insert(category, volume.clamp(0.0, 1.0));
    }

    /// Get volume for a category
    pub fn get_volume(&self, category: AudioCategory) -> f32 {
        *self.volumes.get(&category).unwrap_or(&1.0)
    }

    /// Toggle mute
    pub fn toggle_mute(&mut self) {
        self.is_muted = !self.is_muted;
    }

    /// Get sound category for a sound type
    fn get_sound_category(&self, sound_type: SoundType) -> AudioCategory {
        match sound_type {
            SoundType::EngineIdle | SoundType::EngineRunning | SoundType::EngineStart 
            | SoundType::EngineStop | SoundType::GearShift | SoundType::TireSlip 
            | SoundType::Suspension | SoundType::WinchExtend | SoundType::WinchRetract 
            | SoundType::WinchTension | SoundType::CableSnap => AudioCategory::Engine,
            
            SoundType::Wind | SoundType::Rain | SoundType::Thunder 
            | SoundType::WaterSplash | SoundType::MudSquelch => AudioCategory::Environment,
            
            SoundType::MenuClick | SoundType::MenuSelect | SoundType::Notification 
            | SoundType::MissionComplete | SoundType::MissionFail => AudioCategory::UI,
            
            _ => AudioCategory::SFX,
        }
    }

    /// Play engine sound based on RPM
    pub fn update_engine_sound(&mut self, rpm: f32, position: Vec3) {
        // Stop idle sound when running
        if rpm > 0.2 {
            self.stop(SoundType::EngineIdle);
            
            // Adjust pitch based on RPM
            if let Some(sound) = self.active_sounds.iter_mut()
                .find(|s| s.sound_type == SoundType::EngineRunning) 
            {
                sound.source.pitch = 0.8 + rpm * 0.6; // Pitch varies with RPM
            } else {
                self.play(SoundType::EngineRunning, Some(position));
            }
        } else {
            self.stop(SoundType::EngineRunning);
            self.play(SoundType::EngineIdle, Some(position));
        }
    }

    /// Play tire slip sound based on slip ratio
    pub fn update_tire_sound(&mut self, slip_ratio: f32, position: Vec3) {
        if slip_ratio > 0.3 {
            if !self.active_sounds.iter().any(|s| s.sound_type == SoundType::TireSlip) {
                self.play(SoundType::TireSlip, Some(position));
            }
        } else {
            self.stop(SoundType::TireSlip);
        }
    }

    /// Play collision sound
    pub fn play_collision(&mut self, intensity: f32, position: Vec3, material: &str) {
        let sound_type = match material {
            "metal" => SoundType::MetalImpact,
            "wood" => SoundType::WoodBreak,
            "rock" => SoundType::RockScrape,
            "glass" => SoundType::GlassBreak,
            _ => SoundType::MetalImpact,
        };

        if let Some(source) = self.sound_library.get_mut(&sound_type) {
            source.base_volume = intensity.clamp(0.0, 1.0);
        }

        self.play(sound_type, Some(position));
    }
}
