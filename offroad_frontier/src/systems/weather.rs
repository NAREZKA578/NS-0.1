//! Weather system
//! 
//! Handles dynamic weather including:
//! - Weather type transitions
//! - Precipitation (rain, snow)
//! - Wind effects
//! - Temperature
//! - Time of day

use glam::Vec3;
use crate::core::constants::SurfaceType;

/// Type of precipitation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrecipitationType {
    None,
    Rain,
    Snow,
    Hail,
}

/// Weather type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeatherType {
    Clear,
    Cloudy,
    Overcast,
    Rain,
    Thunderstorm,
    Snow,
    Blizzard,
    Fog,
}

impl WeatherType {
    pub fn default_precipitation(&self) -> PrecipitationType {
        match self {
            WeatherType::Clear | WeatherType::Cloudy | WeatherType::Overcast | WeatherType::Fog => {
                PrecipitationType::None
            }
            WeatherType::Rain => PrecipitationType::Rain,
            WeatherType::Thunderstorm => PrecipitationType::Rain,
            WeatherType::Snow | WeatherType::Blizzard => PrecipitationType::Snow,
        }
    }
    
    pub fn base_visibility(&self) -> f32 {
        match self {
            WeatherType::Clear => 10000.0,
            WeatherType::Cloudy => 8000.0,
            WeatherType::Overcast => 5000.0,
            WeatherType::Rain => 3000.0,
            WeatherType::Thunderstorm => 2000.0,
            WeatherType::Snow => 2500.0,
            WeatherType::Blizzard => 500.0,
            WeatherType::Fog => 1000.0,
        }
    }
}

/// Current weather state
#[derive(Debug, Clone)]
pub struct WeatherState {
    /// Current weather type
    pub weather_type: WeatherType,
    
    /// Target weather type (for transitions)
    pub target_weather: WeatherType,
    
    /// Weather transition progress (0.0 - 1.0)
    pub transition_progress: f32,
    
    /// Precipitation intensity (0.0 - 1.0)
    pub precipitation_intensity: f32,
    
    /// Type of precipitation
    pub precipitation_type: PrecipitationType,
    
    /// Wind direction (normalized)
    pub wind_direction: Vec3,
    
    /// Wind speed in m/s
    pub wind_speed: f32,
    
    /// Temperature in Celsius
    pub temperature: f32,
    
    /// Humidity (0.0 - 1.0)
    pub humidity: f32,
    
    /// Time of day in hours (0.0 - 24.0)
    pub time_of_day: f32,
    
    /// Visibility distance in meters
    pub visibility: f32,
    
    /// Cloud coverage (0.0 - 1.0)
    pub cloud_coverage: f32,
}

impl Default for WeatherState {
    fn default() -> Self {
        Self {
            weather_type: WeatherType::Clear,
            target_weather: WeatherType::Clear,
            transition_progress: 1.0,
            precipitation_intensity: 0.0,
            precipitation_type: PrecipitationType::None,
            wind_direction: Vec3::new(1.0, 0.0, 0.0),
            wind_speed: 2.0,
            temperature: 20.0,
            humidity: 0.5,
            time_of_day: 12.0,
            visibility: 10000.0,
            cloud_coverage: 0.2,
        }
    }
}

impl WeatherState {
    /// Update weather state
    pub fn update(&mut self, delta_time: f32) {
        // Update time of day
        self.time_of_day += delta_time / 60.0; // 1 real second = 1 game minute
        if self.time_of_day >= 24.0 {
            self.time_of_day -= 24.0;
        }
        
        // Weather transition
        if self.weather_type != self.target_weather && self.transition_progress < 1.0 {
            self.transition_progress += delta_time * 0.1; // 10 seconds for full transition
            if self.transition_progress >= 1.0 {
                self.transition_progress = 1.0;
                self.weather_type = self.target_weather;
            }
            
            // Interpolate properties during transition
            let t = self.transition_progress;
            let from_precip = self.weather_type.default_precipitation();
            let to_precip = self.target_weather.default_precipitation();
            
            self.precipitation_type = if t > 0.5 { to_precip } else { from_precip };
            self.precipitation_intensity = self.lerp_precipitation_intensity(from_precip, to_precip, t);
            self.visibility = self.lerp_visibility(t);
            self.cloud_coverage = self.lerp_cloud_coverage(t);
        }
        
        // Update wind slightly
        let wind_variation = (self.time_of_day * 0.1).sin() * 0.5 + 0.5;
        self.wind_speed = 2.0 + wind_variation * 3.0;
    }
    
    /// Set target weather (will transition over time)
    pub fn set_weather(&mut self, weather: WeatherType) {
        if self.weather_type != weather {
            self.target_weather = weather;
            self.transition_progress = 0.0;
        }
    }
    
    /// Get sun direction based on time of day
    pub fn get_sun_direction(&self) -> Vec3 {
        let hour_angle = (self.time_of_day - 12.0) / 24.0 * std::f32::consts::PI * 2.0;
        let elevation = (self.time_of_day - 6.0).abs();
        let elevation_angle = if elevation < 12.0 {
            (1.0 - elevation / 12.0) * std::f32::consts::FRAC_PI_2
        } else {
            -std::f32::consts::FRAC_PI_2 * 0.2
        };
        
        Vec3::new(
            hour_angle.cos() * elevation_angle.cos(),
            elevation_angle.sin(),
            hour_angle.sin() * elevation_angle.cos(),
        ).normalize()
    }
    
    /// Get sun intensity based on time and weather
    pub fn get_sun_intensity(&self) -> f32 {
        let time_factor = {
            let hour = self.time_of_day;
            if hour >= 6.0 && hour <= 20.0 {
                let noon_dist = (hour - 12.0).abs();
                1.0 - (noon_dist / 6.0).powi(2)
            } else {
                0.0
            }
        };
        
        let weather_factor = match self.weather_type {
            WeatherType::Clear => 1.0,
            WeatherType::Cloudy => 0.7,
            WeatherType::Overcast => 0.4,
            WeatherType::Rain => 0.2,
            WeatherType::Thunderstorm => 0.1,
            WeatherType::Snow => 0.3,
            WeatherType::Blizzard => 0.15,
            WeatherType::Fog => 0.3,
        };
        
        time_factor * weather_factor
    }
    
    /// Check if surface should be wet/icy based on weather
    pub fn get_surface_condition(&self, base_surface: SurfaceType) -> SurfaceType {
        match self.precipitation_type {
            PrecipitationType::None => base_surface,
            PrecipitationType::Rain => {
                if self.precipitation_intensity > 0.3 {
                    match base_surface {
                        SurfaceType::Dirt => SurfaceType::Mud,
                        SurfaceType::Sand => SurfaceType::WetSand,
                        _ => base_surface,
                    }
                } else {
                    base_surface
                }
            }
            PrecipitationType::Snow => {
                if self.temperature < 0.0 {
                    match base_surface {
                        SurfaceType::Asphalt | SurfaceType::Dirt | SurfaceType::Grass => SurfaceType::Snow,
                        _ => base_surface,
                    }
                } else {
                    base_surface
                }
            }
            PrecipitationType::Hail => base_surface,
        }
    }
    
    fn lerp_precipitation_intensity(&self, from: PrecipitationType, to: PrecipitationType, t: f32) -> f32 {
        let from_intensity = match from {
            PrecipitationType::None => 0.0,
            _ => 0.5,
        };
        let to_intensity = match to {
            PrecipitationType::None => 0.0,
            PrecipitationType::Rain | PrecipitationType::Hail => 0.7,
            PrecipitationType::Snow => 0.6,
        };
        from_intensity + (to_intensity - from_intensity) * t
    }
    
    fn lerp_visibility(&self, t: f32) -> f32 {
        let from_vis = self.weather_type.base_visibility();
        let to_vis = self.target_weather.base_visibility();
        from_vis + (to_vis - from_vis) * t
    }
    
    fn lerp_cloud_coverage(&self, t: f32) -> f32 {
        let from_clouds = match self.weather_type {
            WeatherType::Clear => 0.1,
            WeatherType::Cloudy => 0.5,
            WeatherType::Overcast => 0.9,
            WeatherType::Rain | WeatherType::Thunderstorm => 0.95,
            WeatherType::Snow | WeatherType::Blizzard => 0.9,
            WeatherType::Fog => 0.7,
        };
        let to_clouds = match self.target_weather {
            WeatherType::Clear => 0.1,
            WeatherType::Cloudy => 0.5,
            WeatherType::Overcast => 0.9,
            WeatherType::Rain | WeatherType::Thunderstorm => 0.95,
            WeatherType::Snow | WeatherType::Blizzard => 0.9,
            WeatherType::Fog => 0.7,
        };
        from_clouds + (to_clouds - from_clouds) * t
    }
}

// Add WetSand to SurfaceType
// This is a workaround - in real implementation, add it to constants.rs
impl SurfaceType {
    pub const WetSand: SurfaceType = SurfaceType::Sand; // Placeholder
}
