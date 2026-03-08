//! Vehicle types definitions
//! 
//! Contains predefined vehicle configurations and specifications

use crate::core::constants::{WHEEL_COUNT, SUSPENSION_STIFFNESS, SUSPENSION_DAMPING};
use glam::Vec3;

/// Vehicle configuration
#[derive(Debug, Clone)]
pub struct VehicleConfig {
    /// Vehicle name
    pub name: String,
    /// Vehicle type
    pub vehicle_type: VehicleType,
    /// Mass in kg
    pub mass: f32,
    /// Engine power in HP
    pub engine_power: f32,
    /// Engine torque in Nm
    pub engine_torque: f32,
    /// Maximum speed in m/s
    pub max_speed: f32,
    /// Gear ratios (including reverse)
    pub gear_ratios: Vec<f32>,
    /// Final drive ratio
    pub final_drive: f32,
    /// Wheel base (distance between front and rear axles)
    pub wheel_base: f32,
    /// Track width (distance between left and right wheels)
    pub track_width: f32,
    /// Ground clearance
    pub ground_clearance: f32,
    /// Suspension stiffness
    pub suspension_stiffness: f32,
    /// Suspension damping
    pub suspension_damping: f32,
    /// Suspension travel (max compression)
    pub suspension_travel: f32,
    /// Tire radius
    pub tire_radius: f32,
    /// Tire width
    pub tire_width: f32,
    /// Differential lock available
    pub has_diff_lock: bool,
    /// Four-wheel drive
    pub has_4wd: bool,
    /// Winch mounted
    pub has_winch: bool,
    /// Fuel capacity in liters
    pub fuel_capacity: f32,
    /// Fuel consumption rate (liters per km)
    pub fuel_consumption: f32,
    /// Damage resistance multiplier
    pub damage_resistance: f32,
    /// Model path
    pub model_path: String,
    /// Interior model path (for first person view)
    pub interior_model_path: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleType {
    LightTruck,
    MediumTruck,
    HeavyTruck,
    SUV,
    Jeep,
    Buggy,
    Military,
    Specialized,
}

impl Default for VehicleConfig {
    fn default() -> Self {
        Self {
            name: "Default Truck".to_string(),
            vehicle_type: VehicleType::MediumTruck,
            mass: 2500.0,
            engine_power: 150.0,
            engine_torque: 450.0,
            max_speed: 40.0, // ~144 km/h
            gear_ratios: vec![3.5, 2.5, 1.8, 1.4, 1.0, 0.8],
            final_drive: 4.0,
            wheel_base: 2.8,
            track_width: 1.8,
            ground_clearance: 0.35,
            suspension_stiffness: SUSPENSION_STIFFNESS,
            suspension_damping: SUSPENSION_DAMPING,
            suspension_travel: 0.25,
            tire_radius: 0.4,
            tire_width: 0.3,
            has_diff_lock: true,
            has_4wd: true,
            has_winch: true,
            fuel_capacity: 80.0,
            fuel_consumption: 0.15,
            damage_resistance: 1.0,
            model_path: "models/vehicles/default_truck.glb".to_string(),
            interior_model_path: Some("models/vehicles/default_truck_interior.glb".to_string()),
        }
    }
}

impl VehicleConfig {
    /// Get wheel positions in local space
    pub fn get_wheel_positions(&self) -> [Vec3; WHEEL_COUNT] {
        let half_base = self.wheel_base / 2.0;
        let half_track = self.track_width / 2.0;
        let y_offset = -self.ground_clearance - self.tire_radius;
        
        [
            // Front-left
            Vec3::new(-half_track, y_offset, half_base),
            // Front-right
            Vec3::new(half_track, y_offset, half_base),
            // Rear-left
            Vec3::new(-half_track, y_offset, -half_base),
            // Rear-right
            Vec3::new(half_track, y_offset, -half_base),
        ]
    }
    
    /// Get gear count (excluding reverse)
    pub fn get_gear_count(&self) -> usize {
        self.gear_ratios.len().saturating_sub(1)
    }
}

/// Predefined vehicle configurations
pub mod presets {
    use super::*;
    
    /// Light off-road truck
    pub fn light_truck() -> VehicleConfig {
        VehicleConfig {
            name: "Scout 4x4".to_string(),
            vehicle_type: VehicleType::LightTruck,
            mass: 1800.0,
            engine_power: 120.0,
            engine_torque: 320.0,
            max_speed: 45.0,
            gear_ratios: vec![3.2, 2.2, 1.6, 1.2, 0.95, 0.75],
            final_drive: 3.8,
            wheel_base: 2.5,
            track_width: 1.6,
            ground_clearance: 0.3,
            suspension_stiffness: 15000.0,
            suspension_damping: 1200.0,
            suspension_travel: 0.22,
            tire_radius: 0.35,
            tire_width: 0.28,
            has_diff_lock: false,
            has_4wd: true,
            has_winch: false,
            fuel_capacity: 60.0,
            fuel_consumption: 0.12,
            damage_resistance: 0.8,
            model_path: "models/vehicles/scout.glb".to_string(),
            interior_model_path: Some("models/vehicles/scout_interior.glb".to_string()),
            ..Default::default()
        }
    }
    
    /// Medium expedition truck
    pub fn medium_truck() -> VehicleConfig {
        VehicleConfig {
            name: "Expedition Pro".to_string(),
            vehicle_type: VehicleType::MediumTruck,
            mass: 2800.0,
            engine_power: 180.0,
            engine_torque: 520.0,
            max_speed: 38.0,
            gear_ratios: vec![3.8, 2.7, 1.9, 1.5, 1.1, 0.85],
            final_drive: 4.2,
            wheel_base: 3.0,
            track_width: 1.9,
            ground_clearance: 0.4,
            suspension_stiffness: 20000.0,
            suspension_damping: 1800.0,
            suspension_travel: 0.28,
            tire_radius: 0.42,
            tire_width: 0.32,
            has_diff_lock: true,
            has_4wd: true,
            has_winch: true,
            fuel_capacity: 100.0,
            fuel_consumption: 0.18,
            damage_resistance: 1.1,
            model_path: "models/vehicles/expedition.glb".to_string(),
            interior_model_path: Some("models/vehicles/expedition_interior.glb".to_string()),
            ..Default::default()
        }
    }
    
    /// Heavy cargo truck
    pub fn heavy_truck() -> VehicleConfig {
        VehicleConfig {
            name: "Titan Hauler".to_string(),
            vehicle_type: VehicleType::HeavyTruck,
            mass: 4500.0,
            engine_power: 280.0,
            engine_torque: 850.0,
            max_speed: 30.0,
            gear_ratios: vec![4.5, 3.2, 2.4, 1.8, 1.4, 1.1, 0.9],
            final_drive: 5.0,
            wheel_base: 3.8,
            track_width: 2.2,
            ground_clearance: 0.45,
            suspension_stiffness: 35000.0,
            suspension_damping: 3000.0,
            suspension_travel: 0.32,
            tire_radius: 0.5,
            tire_width: 0.4,
            has_diff_lock: true,
            has_4wd: true,
            has_winch: true,
            fuel_capacity: 150.0,
            fuel_consumption: 0.28,
            damage_resistance: 1.3,
            model_path: "models/vehicles/titan.glb".to_string(),
            interior_model_path: Some("models/vehicles/titan_interior.glb".to_string()),
            ..Default::default()
        }
    }
    
    /// Agile buggy
    pub fn buggy() -> VehicleConfig {
        VehicleConfig {
            name: "Sand Runner".to_string(),
            vehicle_type: VehicleType::Buggy,
            mass: 900.0,
            engine_power: 100.0,
            engine_torque: 180.0,
            max_speed: 50.0,
            gear_ratios: vec![2.8, 2.0, 1.5, 1.1, 0.85],
            final_drive: 3.5,
            wheel_base: 2.2,
            track_width: 1.7,
            ground_clearance: 0.35,
            suspension_stiffness: 12000.0,
            suspension_damping: 1000.0,
            suspension_travel: 0.35,
            tire_radius: 0.38,
            tire_width: 0.35,
            has_diff_lock: false,
            has_4wd: false,
            has_winch: false,
            fuel_capacity: 40.0,
            fuel_consumption: 0.1,
            damage_resistance: 0.6,
            model_path: "models/vehicles/buggy.glb".to_string(),
            interior_model_path: None,
            ..Default::default()
        }
    }
    
    /// Military vehicle
    pub fn military() -> VehicleConfig {
        VehicleConfig {
            name: "Armored Patrol".to_string(),
            vehicle_type: VehicleType::Military,
            mass: 3500.0,
            engine_power: 220.0,
            engine_torque: 680.0,
            max_speed: 35.0,
            gear_ratios: vec![4.0, 2.9, 2.1, 1.6, 1.2, 0.95],
            final_drive: 4.5,
            wheel_base: 3.2,
            track_width: 2.0,
            ground_clearance: 0.5,
            suspension_stiffness: 28000.0,
            suspension_damping: 2500.0,
            suspension_travel: 0.3,
            tire_radius: 0.45,
            tire_width: 0.38,
            has_diff_lock: true,
            has_4wd: true,
            has_winch: true,
            fuel_capacity: 120.0,
            fuel_consumption: 0.25,
            damage_resistance: 1.5,
            model_path: "models/vehicles/military.glb".to_string(),
            interior_model_path: Some("models/vehicles/military_interior.glb".to_string()),
            ..Default::default()
        }
    }
}

/// Get all available vehicle presets
pub fn get_all_presets() -> Vec<VehicleConfig> {
    vec![
        presets::light_truck(),
        presets::medium_truck(),
        presets::heavy_truck(),
        presets::buggy(),
        presets::military(),
    ]
}
