//! Vehicle physics and state management
//! 
//! This module handles the advanced vehicle physics including:
//! - Suspension simulation
//! - Wheel-ground interaction
//! - Engine and transmission
//! - Differential and torque distribution

use glam::{Vec3, Quat, Vec2};
use crate::core::constants::*;

/// State of a single wheel
#[derive(Debug, Clone)]
pub struct WheelState {
    /// Position in world space
    pub position: Vec3,
    
    /// Velocity in world space
    pub velocity: Vec3,
    
    /// Steering angle in radians
    pub steering_angle: f32,
    
    /// Rotation angle (for visual rotation)
    pub rotation_angle: f32,
    
    /// Angular velocity
    pub angular_velocity: f32,
    
    /// Suspension compression (0.0 = fully extended, 1.0 = fully compressed)
    pub suspension_compression: f32,
    
    /// Is the wheel in contact with ground?
    pub is_in_contact: bool,
    
    /// Contact point normal
    pub contact_normal: Vec3,
    
    /// Current surface friction coefficient
    pub surface_friction: f32,
    
    /// Slip ratio (for traction calculation)
    pub slip_ratio: f32,
    
    /// Slip angle (for lateral force calculation)
    pub slip_angle: f32,
}

impl Default for WheelState {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            steering_angle: 0.0,
            rotation_angle: 0.0,
            angular_velocity: 0.0,
            suspension_compression: 0.0,
            is_in_contact: false,
            contact_normal: Vec3::Y,
            surface_friction: 1.0,
            slip_ratio: 0.0,
            slip_angle: 0.0,
        }
    }
}

/// Complete vehicle state
#[derive(Debug, Clone)]
pub struct VehicleState {
    /// Position in world space
    pub position: Vec3,
    
    /// Rotation (orientation)
    pub rotation: Quat,
    
    /// Linear velocity
    pub linear_velocity: Vec3,
    
    /// Angular velocity
    pub angular_velocity: Vec3,
    
    /// State of each wheel
    pub wheels: [WheelState; WHEEL_COUNT],
    
    /// Engine RPM (0-1 normalized)
    pub engine_rpm: f32,
    
    /// Current gear (-1 = reverse, 0 = neutral, 1+ = forward gears)
    pub current_gear: i8,
    
    /// Throttle input (0.0 - 1.0)
    pub throttle: f32,
    
    /// Brake input (0.0 - 1.0)
    pub brake: f32,
    
    /// Clutch engagement (0.0 - 1.0)
    pub clutch: f32,
    
    /// Handbrake engaged
    pub handbrake: bool,
    
    /// Differential lock engaged
    pub diff_lock: bool,
    
    /// Four-wheel drive engaged
    pub four_wheel_drive: bool,
    
    /// Vehicle mass in kg
    pub mass: f32,
    
    /// Health/damage state (1.0 = perfect, 0.0 = destroyed)
    pub health: f32,
    
    /// Fuel level (0.0 - 1.0)
    pub fuel_level: f32,
}

impl Default for VehicleState {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 5.0, 0.0),
            rotation: Quat::IDENTITY,
            linear_velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            wheels: [WheelState::default(); WHEEL_COUNT],
            engine_rpm: 0.0,
            current_gear: 0,
            throttle: 0.0,
            brake: 0.0,
            clutch: 1.0,
            handbrake: false,
            diff_lock: false,
            four_wheel_drive: true,
            mass: 2500.0, // 2.5 tons typical off-road vehicle
            health: 1.0,
            fuel_level: 1.0,
        }
    }
}

impl VehicleState {
    /// Update vehicle physics
    pub fn update(&mut self, delta_time: f32) {
        // Apply gravity
        let gravity = Vec3::new(0.0, -GRAVITY, 0.0);
        self.linear_velocity += gravity * delta_time;
        
        // Update wheel positions based on vehicle transform
        self.update_wheel_positions();
        
        // Update engine RPM based on throttle and load
        self.update_engine(delta_time);
        
        // Integrate position
        self.position += self.linear_velocity * delta_time;
        
        // Simple ground collision (temporary - will be replaced by terrain system)
        if self.position.y < 0.0 {
            self.position.y = 0.0;
            self.linear_velocity.y = 0.0;
        }
    }
    
    /// Update wheel positions based on vehicle transform and suspension
    fn update_wheel_positions(&mut self) {
        // Temporary wheel positions - will be replaced by proper vehicle model
        let wheel_offsets = [
            Vec3::new(-1.0, -0.5, 1.5),   // Front-left
            Vec3::new(1.0, -0.5, 1.5),    // Front-right
            Vec3::new(-1.0, -0.5, -1.5),  // Rear-left
            Vec3::new(1.0, -0.5, -1.5),   // Rear-right
        ];
        
        for (i, offset) in wheel_offsets.iter().enumerate() {
            let rotated_offset = self.rotation * *offset;
            self.wheels[i].position = self.position + rotated_offset;
            
            // Update wheel rotation based on velocity
            let forward_velocity = self.linear_velocity.dot(self.rotation * Vec3::Z);
            self.wheels[i].rotation_angle += forward_velocity * delta_time / 0.4; // 0.4m wheel radius
        }
    }
    
    /// Update engine state
    fn update_engine(&mut self, delta_time: f32) {
        if self.current_gear == 0 || self.clutch < 0.5 {
            // Neutral or clutch disengaged - engine revs freely
            let target_rpm = 0.2 + self.throttle * 0.8; // 20% - 100% RPM
            let rpm_diff = target_rpm - self.engine_rpm;
            self.engine_rpm += rpm_diff * delta_time * 5.0;
        } else {
            // Engaged - RPM tied to wheel speed
            let avg_wheel_speed = self.get_average_wheel_speed();
            let gear_ratio = self.get_gear_ratio();
            let final_drive = 4.0;
            
            self.engine_rpm = (avg_wheel_speed * gear_ratio * final_drive * 0.05).clamp(0.1, 1.0);
        }
        
        self.engine_rpm = self.engine_rpm.clamp(0.1, 1.0);
    }
    
    /// Get average wheel speed
    fn get_average_wheel_speed(&self) -> f32 {
        let mut total = 0.0;
        let mut count = 0;
        
        for wheel in &self.wheels {
            if wheel.is_in_contact {
                total += wheel.angular_velocity;
                count += 1;
            }
        }
        
        if count > 0 {
            total / count as f32
        } else {
            0.0
        }
    }
    
    /// Get gear ratio for current gear
    fn get_gear_ratio(&self) -> f32 {
        let ratios = [3.5, 2.5, 1.8, 1.4, 1.0, 0.8]; // Reverse + 5 forward gears
        
        if self.current_gear < 0 {
            ratios[0] // Reverse
        } else if self.current_gear == 0 {
            0.0 // Neutral
        } else {
            ratios[self.current_gear as usize].min(ratios.len() as f32 - 1.0)
        }
    }
    
    /// Apply steering input
    pub fn set_steering(&mut self, angle: f32) {
        let clamped_angle = angle.clamp(-MAX_STEERING_ANGLE.to_radians(), MAX_STEERING_ANGLE.to_radians());
        
        // Front wheels steer
        self.wheels[0].steering_angle = clamped_angle;
        self.wheels[1].steering_angle = clamped_angle;
    }
    
    /// Apply throttle input
    pub fn set_throttle(&mut self, value: f32) {
        self.throttle = value.clamp(0.0, 1.0);
    }
    
    /// Apply brake input
    pub fn set_brake(&mut self, value: f32) {
        self.brake = value.clamp(0.0, 1.0);
    }
    
    /// Toggle handbrake
    pub fn toggle_handbrake(&mut self) {
        self.handbrake = !self.handbrake;
    }
    
    /// Shift gear up
    pub fn shift_up(&mut self) {
        if self.current_gear < 5 {
            self.current_gear += 1;
        }
    }
    
    /// Shift gear down
    pub fn shift_down(&mut self) {
        if self.current_gear > -1 {
            self.current_gear -= 1;
        }
    }
    
    /// Toggle differential lock
    pub fn toggle_diff_lock(&mut self) {
        self.diff_lock = !self.diff_lock;
    }
    
    /// Toggle 4WD
    pub fn toggle_4wd(&mut self) {
        self.four_wheel_drive = !self.four_wheel_drive;
    }
}

/// Vehicle physics system
pub struct VehiclePhysics {
    /// Current vehicle state
    pub vehicle: VehicleState,
}

impl VehiclePhysics {
    pub fn new() -> Self {
        Self {
            vehicle: VehicleState::default(),
        }
    }
    
    /// Step physics simulation
    pub fn step(&mut self, delta_time: f32) {
        self.vehicle.update(delta_time);
    }
    
    /// Get vehicle state
    pub fn get_state(&self) -> &VehicleState {
        &self.vehicle
    }
    
    /// Get mutable vehicle state
    pub fn get_state_mut(&mut self) -> &mut VehicleState {
        &mut self.vehicle
    }
}

impl Default for VehiclePhysics {
    fn default() -> Self {
        Self::new()
    }
}
