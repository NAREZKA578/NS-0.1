//! Minimap system
//! 
//! Renders a top-down view of the surrounding terrain and objectives

use glam::Vec2;

/// Minimap configuration
pub struct Minimap {
    /// Is minimap enabled?
    pub enabled: bool,
    
    /// Minimap radius (world units)
    pub radius: f32,
    
    /// Minimap resolution (pixels)
    pub resolution: u32,
    
    /// Show player arrow
    pub show_player: bool,
    
    /// Show objectives
    pub show_objectives: bool,
    
    /// Show waypoints
    pub show_waypoints: bool,
    
    /// Zoom level
    pub zoom: f32,
    
    /// Rotation mode (follow player or north-up)
    pub rotation_mode: RotationMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationMode {
    /// Map rotates with player
    FollowPlayer,
    /// North is always up
    NorthUp,
}

impl Default for Minimap {
    fn default() -> Self {
        Self {
            enabled: true,
            radius: 500.0,
            resolution: 256,
            show_player: true,
            show_objectives: true,
            show_waypoints: true,
            zoom: 1.0,
            rotation_mode: RotationMode::FollowPlayer,
        }
    }
}

impl Minimap {
    /// Create new minimap
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Toggle minimap visibility
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
    
    /// Zoom in
    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom * 1.2).min(4.0);
        self.radius = (self.radius / 1.2).max(100.0);
    }
    
    /// Zoom out
    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom / 1.2).max(0.25);
        self.radius = (self.radius * 1.2).min(2000.0);
    }
    
    /// Reset zoom
    pub fn reset_zoom(&mut self) {
        self.zoom = 1.0;
        self.radius = 500.0;
    }
    
    /// Toggle rotation mode
    pub fn toggle_rotation_mode(&mut self) {
        self.rotation_mode = match self.rotation_mode {
            RotationMode::FollowPlayer => RotationMode::NorthUp,
            RotationMode::NorthUp => RotationMode::FollowPlayer,
        };
    }
    
    /// Convert world position to minimap coordinates
    pub fn world_to_minimap(&self, world_pos: Vec2, player_pos: Vec2, player_angle: f32) -> Vec2 {
        let relative = world_pos - player_pos;
        
        let rotated = match self.rotation_mode {
            RotationMode::FollowPlayer => {
                // Rotate relative to player direction
                let cos = player_angle.cos();
                let sin = player_angle.sin();
                Vec2::new(
                    relative.x * cos - relative.y * sin,
                    relative.x * sin + relative.y * cos,
                )
            }
            RotationMode::NorthUp => relative,
        };
        
        // Scale to minimap space (-1 to 1)
        let scale = 1.0 / self.radius * self.zoom;
        rotated * scale
    }
    
    /// Check if world position is visible on minimap
    pub fn is_visible(&self, world_pos: Vec2, player_pos: Vec2) -> bool {
        let distance = (world_pos - player_pos).length();
        distance < self.radius / self.zoom
    }
    
    /// Render minimap data (placeholder)
    pub fn render(&self, player_pos: Vec2, player_angle: f32) -> MinimapRenderData {
        MinimapRenderData {
            enabled: self.enabled,
            zoom: self.zoom,
            radius: self.radius,
            rotation_mode: self.rotation_mode,
            center_x: player_pos.x,
            center_y: player_pos.y,
            player_angle,
        }
    }
}

/// Data prepared for rendering
pub struct MinimapRenderData {
    pub enabled: bool,
    pub zoom: f32,
    pub radius: f32,
    pub rotation_mode: RotationMode,
    pub center_x: f32,
    pub center_y: f32,
    pub player_angle: f32,
}
