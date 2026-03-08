//! Camera system
//! 
//! Handles multiple camera modes:
//! - Third person (chase cam)
//! - First person (driver view)
//! - Hood view
//! - Cinematic

use glam::{Vec3, Quat, Mat4};
use crate::core::constants::{CAMERA_SMOOTHING, FOV};
use crate::core::config::CameraMode;

/// Camera data
#[derive(Debug, Clone)]
pub struct Camera {
    /// Camera position in world space
    pub position: Vec3,
    
    /// Camera rotation (orientation)
    pub rotation: Quat,
    
    /// Field of view in degrees
    pub fov: f32,
    
    /// Near clipping plane
    pub near: f32,
    
    /// Far clipping plane
    pub far: f32,
    
    /// Aspect ratio
    pub aspect_ratio: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 5.0, -10.0),
            rotation: Quat::IDENTITY,
            fov: FOV,
            near: 0.1,
            far: 10000.0,
            aspect_ratio: 16.0 / 9.0,
        }
    }
}

impl Camera {
    /// Get view matrix
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::from_rotation_translation(self.rotation, self.position).inverse()
    }
    
    /// Get projection matrix
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh_gl(
            self.fov.to_radians(),
            self.aspect_ratio,
            self.near,
            self.far,
        )
    }
    
    /// Get view-projection matrix
    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
    
    /// Get forward direction
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }
    
    /// Get right direction
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }
    
    /// Get up direction
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }
}

/// Camera system manager
pub struct CameraSystem {
    /// Current active camera
    pub camera: Camera,
    
    /// Current camera mode
    pub mode: CameraMode,
    
    /// Target camera position (for smoothing)
    pub target_position: Vec3,
    
    /// Target camera rotation (for smoothing)
    pub target_rotation: Quat,
    
    /// Camera distance from vehicle (third person)
    pub distance: f32,
    
    /// Camera height offset
    pub height_offset: f32,
    
    /// Camera horizontal angle (for orbit)
    pub orbit_angle_h: f32,
    
    /// Camera vertical angle (for orbit)
    pub orbit_angle_v: f32,
    
    /// Camera shake intensity
    pub shake_intensity: f32,
    
    /// Camera shake duration
    pub shake_duration: f32,
}

impl Default for CameraSystem {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            mode: CameraMode::ThirdPerson,
            target_position: Vec3::new(0.0, 5.0, -10.0),
            target_rotation: Quat::IDENTITY,
            distance: 8.0,
            height_offset: 3.0,
            orbit_angle_h: 0.0,
            orbit_angle_v: 0.3,
            shake_intensity: 0.0,
            shake_duration: 0.0,
        }
    }
}

impl CameraSystem {
    /// Create a new camera system
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update camera based on vehicle position and mode
    pub fn update(&mut self, delta_time: f32, vehicle_position: Vec3, vehicle_rotation: Quat) {
        // Update target based on mode
        match self.mode {
            CameraMode::ThirdPerson => {
                self.update_third_person(vehicle_position, vehicle_rotation);
            }
            CameraMode::FirstPerson => {
                self.update_first_person(vehicle_position, vehicle_rotation);
            }
            CameraMode::Hood => {
                self.update_hood(vehicle_position, vehicle_rotation);
            }
            CameraMode::Cinematic => {
                self.update_cinematic(delta_time, vehicle_position, vehicle_rotation);
            }
        }
        
        // Apply smoothing
        let t = 1.0 - (-CAMERA_SMOOTHING * delta_time).exp();
        self.camera.position = self.camera.position.lerp(self.target_position, t);
        self.camera.rotation = self.camera.rotation.slerp(self.target_rotation, t);
        
        // Apply camera shake
        if self.shake_duration > 0.0 {
            self.apply_shake(delta_time);
        }
        
        // Update aspect ratio (should come from window resize events)
        // self.camera.aspect_ratio = ...;
    }
    
    /// Update third person camera
    fn update_third_person(&mut self, vehicle_pos: Vec3, vehicle_rot: Quat) {
        // Calculate orbit offset
        let horizontal_offset = Vec3::new(
            self.orbit_angle_h.sin(),
            self.orbit_angle_v.sin(),
            self.orbit_angle_h.cos(),
        );
        
        let offset = vehicle_rot * horizontal_offset * self.distance;
        self.target_position = vehicle_pos + offset + Vec3::Y * self.height_offset;
        
        // Look at vehicle
        let direction = (vehicle_pos + Vec3::Y * 1.5) - self.target_position;
        self.target_rotation = Quat::from_rotation_arc(Vec3::NEG_Z, direction.normalize());
    }
    
    /// Update first person camera (driver's eye view)
    fn update_first_person(&mut self, vehicle_pos: Vec3, vehicle_rot: Quat) {
        // Position inside cabin (approximate)
        let offset = Vec3::new(0.0, 1.7, -0.5); // Driver seat position
        self.target_position = vehicle_pos + vehicle_rot * offset;
        
        // Look forward with slight down angle
        let look_direction = vehicle_rot * (Vec3::NEG_Z * 0.95 + Vec3::NEG_Y * 0.1).normalize();
        self.target_rotation = Quat::from_rotation_arc(Vec3::NEG_Z, look_direction);
    }
    
    /// Update hood camera (looking over hood)
    fn update_hood(&mut self, vehicle_pos: Vec3, vehicle_rot: Quat) {
        // Position on hood
        let offset = Vec3::new(0.0, 1.2, 1.5);
        self.target_position = vehicle_pos + vehicle_rot * offset;
        
        // Look forward
        let look_direction = vehicle_rot * Vec3::NEG_Z;
        self.target_rotation = Quat::from_rotation_arc(Vec3::NEG_Z, look_direction);
    }
    
    /// Update cinematic camera (dynamic angles)
    fn update_cinematic(&mut self, delta_time: f32, vehicle_pos: Vec3, vehicle_rot: Quat) {
        // Rotate around vehicle slowly
        self.orbit_angle_h += delta_time * 0.2;
        
        let horizontal_offset = Vec3::new(
            self.orbit_angle_h.sin(),
            0.3,
            self.orbit_angle_h.cos(),
        );
        
        let offset = vehicle_rot * horizontal_offset * self.distance * 1.5;
        self.target_position = vehicle_pos + offset + Vec3::Y * 2.0;
        
        // Look at vehicle
        let direction = vehicle_pos - self.target_position;
        self.target_rotation = Quat::from_rotation_arc(Vec3::NEG_Z, direction.normalize());
    }
    
    /// Set camera mode
    pub fn set_mode(&mut self, mode: CameraMode) {
        self.mode = mode;
        
        // Adjust settings per mode
        match mode {
            CameraMode::ThirdPerson => {
                self.distance = 8.0;
                self.height_offset = 3.0;
            }
            CameraMode::FirstPerson | CameraMode::Hood => {
                self.shake_intensity = 0.02; // Slight engine vibration
            }
            CameraMode::Cinematic => {
                self.distance = 12.0;
            }
        }
    }
    
    /// Cycle to next camera mode
    pub fn cycle_mode(&mut self) {
        let next_mode = match self.mode {
            CameraMode::ThirdPerson => CameraMode::FirstPerson,
            CameraMode::FirstPerson => CameraMode::Hood,
            CameraMode::Hood => CameraMode::Cinematic,
            CameraMode::Cinematic => CameraMode::ThirdPerson,
        };
        self.set_mode(next_mode);
    }
    
    /// Add camera shake (e.g., from engine or impact)
    pub fn add_shake(&mut self, intensity: f32, duration: f32) {
        self.shake_intensity = self.shake_intensity.max(intensity);
        self.shake_duration = self.shake_duration.max(duration);
    }
    
    /// Apply camera shake
    fn apply_shake(&mut self, delta_time: f32) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        let shake_amount = self.shake_intensity * (self.shake_duration / 1.0);
        let offset = Vec3::new(
            rng.gen_range(-shake_amount..=shake_amount),
            rng.gen_range(-shake_amount..=shake_amount),
            rng.gen_range(-shake_amount..=shake_amount),
        );
        
        self.camera.position += offset;
        
        self.shake_duration -= delta_time;
        if self.shake_duration <= 0.0 {
            self.shake_intensity = 0.0;
            self.shake_duration = 0.0;
        }
    }
    
    /// Zoom in/out (for third person)
    pub fn zoom(&mut self, delta: f32) {
        self.distance = (self.distance - delta).clamp(3.0, 20.0);
    }
    
    /// Orbit camera horizontally
    pub fn orbit_horizontal(&mut self, delta: f32) {
        self.orbit_angle_h += delta;
    }
    
    /// Orbit camera vertically
    pub fn orbit_vertical(&mut self, delta: f32) {
        self.orbit_angle_v = (self.orbit_angle_v + delta).clamp(-0.5, 0.8);
    }
}
