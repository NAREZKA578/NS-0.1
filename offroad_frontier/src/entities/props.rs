//! Props module
//! 
//! Contains world props: trees, rocks, buildings, decorations

use glam::Vec3;

/// Prop type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropType {
    Tree,
    Rock,
    Bush,
    Log,
    Stump,
    Fence,
    Building,
    Bridge,
    Tower,
    Sign,
    Barrel,
    Crate,
    Vehicle wreck,
    Custom,
}

/// Prop data
#[derive(Debug, Clone)]
pub struct Prop {
    /// Unique prop ID
    pub id: u32,
    /// Position in world space
    pub position: Vec3,
    /// Rotation (yaw angle in radians)
    pub rotation: f32,
    /// Scale multiplier
    pub scale: f32,
    /// Type of prop
    pub prop_type: PropType,
    /// Model path
    pub model_path: String,
    /// Is prop destructible
    pub is_destructible: bool,
    /// Health (for destructible props)
    pub health: f32,
    /// Can be used as winch anchor
    pub can_anchor: bool,
    /// Anchor strength (0.0 - 1.0)
    pub anchor_strength: f32,
    /// Collision radius
    pub collision_radius: f32,
    /// Collision height
    pub collision_height: f32,
}

impl Default for Prop {
    fn default() -> Self {
        Self {
            id: 0,
            position: Vec3::ZERO,
            rotation: 0.0,
            scale: 1.0,
            prop_type: PropType::Tree,
            model_path: String::new(),
            is_destructible: false,
            health: 1.0,
            can_anchor: false,
            anchor_strength: 0.5,
            collision_radius: 0.5,
            collision_height: 1.0,
        }
    }
}

impl Prop {
    /// Create a tree prop
    pub fn tree(id: u32, position: Vec3, scale: f32) -> Self {
        Self {
            id,
            position,
            rotation: rand::random::<f32>() * std::f32::consts::PI * 2.0,
            scale,
            prop_type: PropType::Tree,
            model_path: format!("models/props/tree_{}.glb", rand::random::<u8>() % 5 + 1),
            is_destructible: true,
            health: 1.0,
            can_anchor: true,
            anchor_strength: 0.8,
            collision_radius: 0.5 * scale,
            collision_height: 8.0 * scale,
        }
    }
    
    /// Create a rock prop
    pub fn rock(id: u32, position: Vec3, scale: f32) -> Self {
        Self {
            id,
            position,
            rotation: rand::random::<f32>() * std::f32::consts::PI * 2.0,
            scale,
            prop_type: PropType::Rock,
            model_path: format!("models/props/rock_{}.glb", rand::random::<u8>() % 4 + 1),
            is_destructible: false,
            health: 1.0,
            can_anchor: true,
            anchor_strength: 1.0,
            collision_radius: 1.0 * scale,
            collision_height: 2.0 * scale,
        }
    }
    
    /// Create a building prop
    pub fn building(id: u32, position: Vec3, rotation: f32, scale: f32) -> Self {
        Self {
            id,
            position,
            rotation,
            scale,
            prop_type: PropType::Building,
            model_path: "models/props/building_warehouse.glb".to_string(),
            is_destructible: false,
            health: 1.0,
            can_anchor: true,
            anchor_strength: 1.0,
            collision_radius: 5.0 * scale,
            collision_height: 6.0 * scale,
        }
    }
    
    /// Apply damage to prop
    pub fn take_damage(&mut self, damage: f32) -> bool {
        if !self.is_destructible {
            return false;
        }
        
        self.health -= damage;
        if self.health <= 0.0 {
            self.health = 0.0;
            return true; // Destroyed
        }
        false
    }
    
    /// Get anchor strength considering damage
    pub fn get_current_anchor_strength(&self) -> f32 {
        if !self.can_anchor {
            return 0.0;
        }
        self.anchor_strength * self.health
    }
}

/// Prop manager
pub struct PropManager {
    /// All props in the world
    pub props: Vec<Prop>,
    /// Next prop ID
    next_id: u32,
}

impl Default for PropManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PropManager {
    /// Create new prop manager
    pub fn new() -> Self {
        Self {
            props: Vec::new(),
            next_id: 1,
        }
    }
    
    /// Add a prop
    pub fn add_prop(&mut self, prop: Prop) -> u32 {
        let id = prop.id;
        self.props.push(prop);
        id
    }
    
    /// Spawn a tree at position
    pub fn spawn_tree(&mut self, position: Vec3, scale: f32) -> u32 {
        let prop = Prop::tree(self.next_id, position, scale);
        self.next_id += 1;
        self.add_prop(prop)
    }
    
    /// Spawn a rock at position
    pub fn spawn_rock(&mut self, position: Vec3, scale: f32) -> u32 {
        let prop = Prop::rock(self.next_id, position, scale);
        self.next_id += 1;
        self.add_prop(prop)
    }
    
    /// Remove destroyed props
    pub fn cleanup_destroyed(&mut self) -> usize {
        let before = self.props.len();
        self.props.retain(|p| p.health > 0.0 || !p.is_destructible);
        before - self.props.len()
    }
    
    /// Get props in radius
    pub fn get_props_in_radius(&self, center: Vec3, radius: f32) -> Vec<&Prop> {
        self.props
            .iter()
            .filter(|p| (p.position - center).length() <= radius)
            .collect()
    }
    
    /// Get potential anchor points near position
    pub fn get_anchor_points(&self, position: Vec3, max_distance: f32) -> Vec<&Prop> {
        self.props
            .iter()
            .filter(|p| {
                p.can_anchor && 
                p.get_current_anchor_strength() > 0.3 &&
                (p.position - position).length() <= max_distance
            })
            .collect()
    }
    
    /// Apply area damage
    pub fn apply_area_damage(&mut self, center: Vec3, radius: f32, damage: f32) -> Vec<u32> {
        let mut destroyed_ids = Vec::new();
        
        for prop in &mut self.props {
            if (prop.position - center).length() <= radius {
                if prop.take_damage(damage) {
                    destroyed_ids.push(prop.id);
                }
            }
        }
        
        destroyed_ids
    }
}
