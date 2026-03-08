//! HUD (Heads-Up Display)
//! 
//! Renders vehicle information and game status

use crate::systems::vehicle::VehicleState;
use crate::systems::winch::WinchSystem;
use crate::systems::weather::WeatherState;

/// HUD display data
pub struct HUD {
    /// Show speedometer
    pub show_speedometer: bool,
    
    /// Show gear indicator
    pub show_gear: bool,
    
    /// Show fuel gauge
    pub show_fuel: bool,
    
    /// Show winch status
    pub show_winch: bool,
    
    /// Show weather info
    pub show_weather: bool,
    
    /// Show coordinates
    pub show_coordinates: bool,
    
    /// Speed unit (km/h or mph)
    pub speed_unit: SpeedUnit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeedUnit {
    Kmh,
    Mph,
}

impl Default for HUD {
    fn default() -> Self {
        Self {
            show_speedometer: true,
            show_gear: true,
            show_fuel: true,
            show_winch: true,
            show_weather: false,
            show_coordinates: false,
            speed_unit: SpeedUnit::Kmh,
        }
    }
}

impl HUD {
    /// Create new HUD
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Get speed string with units
    pub fn get_speed_string(&self, speed: f32) -> String {
        let display_speed = match self.speed_unit {
            SpeedUnit::Kmh => speed * 3.6, // m/s to km/h
            SpeedUnit::Mph => speed * 2.237, // m/s to mph
        };
        
        let unit = match self.speed_unit {
            SpeedUnit::Kmh => "km/h",
            SpeedUnit::Mph => "mph",
        };
        
        format!("{:.0} {}", display_speed, unit)
    }
    
    /// Get gear display string
    pub fn get_gear_string(&self, gear: i8) -> String {
        match gear {
            -1 => "R".to_string(),
            0 => "N".to_string(),
            g => format!("{}", g),
        }
    }
    
    /// Get fuel percentage string
    pub fn get_fuel_string(&self, fuel_level: f32) -> String {
        format!("{:.0}%", fuel_level * 100.0)
    }
    
    /// Get RPM as percentage
    pub fn get_rpm_percentage(&self, rpm: f32) -> f32 {
        rpm // Already normalized 0-1
    }
    
    /// Render HUD (placeholder - will be implemented with actual rendering)
    pub fn render(&self, vehicle: &VehicleState, winch: &WinchSystem, weather: &WeatherState) -> HUDRenderData {
        HUDRenderData {
            speed: if self.show_speedometer {
                Some(self.get_speed_string(vehicle.linear_velocity.length()))
            } else {
                None
            },
            
            gear: if self.show_gear {
                Some(self.get_gear_string(vehicle.current_gear))
            } else {
                None
            },
            
            fuel: if self.show_fuel {
                Some((vehicle.fuel_level * 100.0) as u8)
            } else {
                None
            },
            
            rpm: if self.show_speedometer {
                Some(self.get_rpm_percentage(vehicle.engine_rpm))
            } else {
                None
            },
            
            winch_active: if self.show_winch {
                Some(winch.is_active())
            } else {
                None
            },
            
            winch_tension: if self.show_winch && winch.is_active() {
                Some(winch.get_tension_percentage() * 100.0)
            } else {
                None
            },
            
            weather_type: if self.show_weather {
                Some(format!("{:?}", weather.weather_type))
            } else {
                None
            },
            
            temperature: if self.show_weather {
                Some(weather.temperature)
            } else {
                None
            },
            
            coordinates: if self.show_coordinates {
                Some(format!(
                    "X: {:.1} Y: {:.1} Z: {:.1}",
                    vehicle.position.x,
                    vehicle.position.y,
                    vehicle.position.z
                ))
            } else {
                None
            },
            
            health: Some((vehicle.health * 100.0) as u8),
        }
    }
    
    /// Toggle speedometer visibility
    pub fn toggle_speedometer(&mut self) {
        self.show_speedometer = !self.show_speedometer;
    }
    
    /// Toggle fuel gauge visibility
    pub fn toggle_fuel(&mut self) {
        self.show_fuel = !self.show_fuel;
    }
    
    /// Toggle winch display
    pub fn toggle_winch(&mut self) {
        self.show_winch = !self.show_winch;
    }
    
    /// Toggle minimap (not implemented in HUD directly)
    pub fn toggle_minimap(&mut self) {
        // Handled by minimap system
    }
    
    /// Cycle speed units
    pub fn cycle_speed_units(&mut self) {
        self.speed_unit = match self.speed_unit {
            SpeedUnit::Kmh => SpeedUnit::Mph,
            SpeedUnit::Mph => SpeedUnit::Kmh,
        };
    }
}

/// Data prepared for rendering
pub struct HUDRenderData {
    pub speed: Option<String>,
    pub gear: Option<String>,
    pub fuel: Option<u8>,
    pub rpm: Option<f32>,
    pub winch_active: Option<bool>,
    pub winch_tension: Option<f32>,
    pub weather_type: Option<String>,
    pub temperature: Option<f32>,
    pub coordinates: Option<String>,
    pub health: Option<u8>,
}
