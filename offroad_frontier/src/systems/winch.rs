//! Winch system
//! 
//! Implements winch mechanics for vehicle recovery:
//! - Attach/detach from anchor points
//! - Cable tension simulation
//! - Force application
//! - UI feedback

use glam::{Vec3, Quat};

/// Winch state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinchState {
    /// Winch is not in use
    Idle,
    /// Cable is being extended
    Extending,
    /// Cable is being retracted
    Retracting,
    /// Cable is attached and under tension
    UnderTension,
}

/// Winch attachment point
#[derive(Debug, Clone)]
pub struct AnchorPoint {
    /// World position of the anchor
    pub position: Vec3,
    
    /// Is this anchor valid (e.g., tree, rock)?
    pub is_valid: bool,
    
    /// Anchor type (for visual/audio feedback)
    pub anchor_type: AnchorType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnchorType {
    Tree,
    Rock,
    Building,
    AnotherVehicle,
    GroundStake,
}

/// Winch system for vehicle recovery
pub struct WinchSystem {
    /// Current winch state
    pub state: WinchState,
    
    /// Cable length in meters
    pub cable_length: f32,
    
    /// Maximum cable length
    pub max_cable_length: f32,
    
    /// Minimum cable length
    pub min_cable_length: f32,
    
    /// Cable extension/retraction speed (m/s)
    pub cable_speed: f32,
    
    /// Maximum cable tension (Newtons)
    pub max_tension: f32,
    
    /// Current cable tension
    pub current_tension: f32,
    
    /// Attached anchor point (if any)
    pub attached_anchor: Option<AnchorPoint>,
    
    /// Vehicle attach point (local space)
    pub vehicle_attach_point: Vec3,
    
    /// World position of vehicle attach point
    pub vehicle_attach_world: Vec3,
    
    /// Cable stiffness (spring constant)
    pub cable_stiffness: f32,
    
    /// Cable damping
    pub cable_damping: f32,
}

impl Default for WinchSystem {
    fn default() -> Self {
        Self {
            state: WinchState::Idle,
            cable_length: 2.0,
            max_cable_length: 50.0,
            min_cable_length: 2.0,
            cable_speed: 3.0,
            max_tension: 10000.0, // 10 kN
            current_tension: 0.0,
            attached_anchor: None,
            vehicle_attach_point: Vec3::new(0.0, 0.5, -2.0), // Rear of vehicle
            vehicle_attach_world: Vec3::ZERO,
            cable_stiffness: 50000.0,
            cable_damping: 1000.0,
        }
    }
}

impl WinchSystem {
    /// Create a new winch system
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update winch physics
    pub fn update(&mut self, delta_time: f32, vehicle_position: Vec3, vehicle_rotation: Quat) {
        // Update vehicle attach point world position
        self.vehicle_attach_world = vehicle_position + vehicle_rotation * self.vehicle_attach_point;
        
        match self.state {
            WinchState::Idle => {
                self.current_tension = 0.0;
            }
            
            WinchState::Extending => {
                self.cable_length = (self.cable_length + self.cable_speed * delta_time)
                    .min(self.max_cable_length);
                self.current_tension = 0.0;
                
                if self.cable_length >= self.max_cable_length {
                    self.state = WinchState::Idle;
                }
            }
            
            WinchState::Retracting => {
                self.cable_length = (self.cable_length - self.cable_speed * delta_time)
                    .max(self.min_cable_length);
                self.current_tension = 0.0;
                
                if self.cable_length <= self.min_cable_length {
                    self.state = WinchState::Idle;
                }
            }
            
            WinchState::UnderTension => {
                if let Some(anchor) = &self.attached_anchor {
                    // Calculate distance between anchor and vehicle
                    let direction = anchor.position - self.vehicle_attach_world;
                    let distance = direction.length();
                    
                    // Spring force based on cable stretch
                    let stretch = distance - self.cable_length;
                    
                    if stretch > 0.0 {
                        // Cable is taut - apply spring force
                        let spring_force = stretch * self.cable_stiffness;
                        let damping_force = self.current_tension * self.cable_damping / self.max_tension;
                        
                        self.current_tension = (spring_force - damping_force)
                            .clamp(0.0, self.max_tension);
                        
                        // Auto-retract if tension is too low (vehicle pulled to anchor)
                        if distance < self.min_cable_length + 0.5 {
                            self.detach();
                        }
                    } else {
                        // Cable is slack
                        self.current_tension = 0.0;
                    }
                } else {
                    self.state = WinchState::Idle;
                }
            }
        }
    }
    
    /// Extend cable
    pub fn extend(&mut self) {
        if self.state == WinchState::Idle || self.state == WinchState::Retracting {
            self.state = WinchState::Extending;
        }
    }
    
    /// Retract cable
    pub fn retract(&mut self) {
        if self.state == WinchState::Idle || self.state == WinchState::Extending {
            self.state = WinchState::Retracting;
        }
    }
    
    /// Stop winch
    pub fn stop(&mut self) {
        if self.state == WinchState::Extending || self.state == WinchState::Retracting {
            self.state = WinchState::Idle;
        }
    }
    
    /// Try to attach to an anchor point
    pub fn attach(&mut self, anchor: AnchorPoint) -> bool {
        if self.state != WinchState::Idle && self.state != WinchState::UnderTension {
            return false;
        }
        
        if !anchor.is_valid {
            return false;
        }
        
        // Check if anchor is within range
        let distance = (anchor.position - self.vehicle_attach_world).length();
        if distance > self.cable_length + 5.0 {
            return false; // Too far to attach
        }
        
        self.attached_anchor = Some(anchor);
        self.cable_length = distance.min(self.max_cable_length);
        self.state = WinchState::UnderTension;
        
        true
    }
    
    /// Detach from current anchor
    pub fn detach(&mut self) {
        self.attached_anchor = None;
        self.state = WinchState::Idle;
        self.current_tension = 0.0;
    }
    
    /// Get force to apply to vehicle
    pub fn get_force_on_vehicle(&self) -> Vec3 {
        if self.state != WinchState::UnderTension {
            return Vec3::ZERO;
        }
        
        if let Some(anchor) = &self.attached_anchor {
            let direction = anchor.position - self.vehicle_attach_world;
            let normalized = direction.normalize_or_zero();
            
            // Force pulls vehicle toward anchor
            normalized * self.current_tension
        } else {
            Vec3::ZERO
        }
    }
    
    /// Get cable direction vector
    pub fn get_cable_direction(&self) -> Vec3 {
        if let Some(anchor) = &self.attached_anchor {
            (anchor.position - self.vehicle_attach_world).normalize_or_zero()
        } else {
            Vec3::ZERO
        }
    }
    
    /// Get cable end position (where it would attach)
    pub fn get_cable_end_position(&self) -> Vec3 {
        if let Some(anchor) = &self.attached_anchor {
            anchor.position
        } else {
            // Return position at max cable length in camera direction
            self.vehicle_attach_world
        }
    }
    
    /// Is winch currently active (extending, retracting, or under tension)?
    pub fn is_active(&self) -> bool {
        self.state != WinchState::Idle
    }
    
    /// Get cable length as percentage of max
    pub fn get_cable_percentage(&self) -> f32 {
        self.cable_length / self.max_cable_length
    }
    
    /// Get tension as percentage of max
    pub fn get_tension_percentage(&self) -> f32 {
        self.current_tension / self.max_tension
    }
}
