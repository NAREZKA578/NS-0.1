//! World module
//! 
//! Contains world management, chunk loading, and level data

use glam::Vec3;
use crate::core::constants::SurfaceType;
use crate::systems::terrain::TerrainChunk;

/// World region data
#[derive(Debug, Clone)]
pub struct WorldRegion {
    /// Region ID
    pub id: u32,
    /// Region name
    pub name: String,
    /// Region bounds (min_x, min_z, max_x, max_z)
    pub bounds: (f32, f32, f32, f32),
    /// Primary surface type
    pub primary_surface: SurfaceType,
    /// Difficulty multiplier
    pub difficulty_multiplier: f32,
    /// Is region unlocked
    pub is_unlocked: bool,
}

/// Checkpoint for missions
#[derive(Debug, Clone)]
pub struct Checkpoint {
    /// Checkpoint ID
    pub id: usize,
    /// Position in world space
    pub position: Vec3,
    /// Radius to trigger checkpoint
    pub radius: f32,
    /// Is checkpoint activated
    pub is_activated: bool,
    /// Next checkpoint ID (if any)
    pub next_checkpoint: Option<usize>,
}

/// Anchor point for winch
#[derive(Debug, Clone)]
pub struct WorldAnchor {
    /// Anchor ID
    pub id: u32,
    /// Position in world space
    pub position: Vec3,
    /// Anchor type
    pub anchor_type: AnchorType,
    /// Is anchor valid for use
    pub is_valid: bool,
    /// Display name
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnchorType {
    Tree,
    Rock,
    Building,
    Tower,
    Vehicle,
    Custom,
}

/// Fuel station / repair point
#[derive(Debug, Clone)]
pub struct ServicePoint {
    /// Service point ID
    pub id: u32,
    /// Position in world space
    pub position: Vec3,
    /// Radius of service area
    pub radius: f32,
    /// Has fuel available
    pub has_fuel: bool,
    /// Has repair available
    pub has_repair: bool,
    /// Cost multiplier for services
    pub cost_multiplier: f32,
    /// Service point name
    pub name: String,
}

/// Collectible item in the world
#[derive(Debug, Clone)]
pub struct Collectible {
    /// Collectible ID
    pub id: u32,
    /// Position in world space
    pub position: Vec3,
    /// Type of collectible
    pub collectible_type: CollectibleType,
    /// Is collectible already collected
    pub is_collected: bool,
    /// Value/score when collected
    pub value: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectibleType {
    Scrap,
    Parts,
    FuelCanister,
    Document,
    Photo,
    Secret,
}

/// Hazard in the world
#[derive(Debug, Clone)]
pub struct Hazard {
    /// Hazard ID
    pub id: u32,
    /// Position in world space
    pub position: Vec3,
    /// Type of hazard
    pub hazard_type: HazardType,
    /// Severity/damage potential
    pub severity: f32,
    /// Radius of effect
    pub radius: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HazardType {
    DeepWater,
    SteepCliff,
    MudPit,
    IcePatch,
    RockField,
    FallenTree,
    BrokenBridge,
}

/// World manager
pub struct World {
    /// All regions in the world
    pub regions: Vec<WorldRegion>,
    /// All checkpoints
    pub checkpoints: Vec<Checkpoint>,
    /// All anchor points
    pub anchors: Vec<WorldAnchor>,
    /// All service points
    pub service_points: Vec<ServicePoint>,
    /// All collectibles
    pub collectibles: Vec<Collectible>,
    /// All hazards
    pub hazards: Vec<Hazard>,
    /// World size
    pub world_size: f32,
    /// Current active region
    pub current_region: Option<u32>,
}

impl Default for World {
    fn default() -> Self {
        Self::new(2000.0)
    }
}

impl World {
    /// Create a new world
    pub fn new(world_size: f32) -> Self {
        let mut world = Self {
            regions: Vec::new(),
            checkpoints: Vec::new(),
            anchors: Vec::new(),
            service_points: Vec::new(),
            collectibles: Vec::new(),
            hazards: Vec::new(),
            world_size,
            current_region: None,
        };
        
        world.generate_default_content();
        world
    }
    
    /// Generate default world content
    fn generate_default_content(&mut self) {
        // Create initial region
        self.regions.push(WorldRegion {
            id: 0,
            name: "Starting Valley".to_string(),
            bounds: (-500.0, -500.0, 500.0, 500.0),
            primary_surface: SurfaceType::Dirt,
            difficulty_multiplier: 1.0,
            is_unlocked: true,
        });
        
        // Add some anchor points
        for i in 0..20 {
            let angle = (i as f32 / 20.0) * std::f32::consts::PI * 2.0;
            let radius = 100.0 + (i as f32 * 17.0) % 400.0;
            self.anchors.push(WorldAnchor {
                id: i,
                position: Vec3::new(
                    angle.cos() * radius,
                    0.0,
                    angle.sin() * radius,
                ),
                anchor_type: if i % 3 == 0 { AnchorType::Rock } else { AnchorType::Tree },
                is_valid: true,
                name: format!("Anchor {}", i),
            });
        }
        
        // Add service points
        self.service_points.push(ServicePoint {
            id: 0,
            position: Vec3::new(0.0, 0.0, 0.0),
            radius: 10.0,
            has_fuel: true,
            has_repair: true,
            cost_multiplier: 1.0,
            name: "Base Camp".to_string(),
        });
        
        // Add some collectibles
        for i in 0..10 {
            self.collectibles.push(Collectible {
                id: i,
                position: Vec3::new(
                    (i as f32 * 123.4).sin() * 200.0,
                    0.0,
                    (i as f32 * 567.8).cos() * 200.0,
                ),
                collectible_type: if i % 2 == 0 { CollectibleType::Scrap } else { CollectibleType::Parts },
                is_collected: false,
                value: 100 + i as u32 * 50,
            });
        }
        
        // Add some hazards
        self.hazards.push(Hazard {
            id: 0,
            position: Vec3::new(-200.0, 0.0, 150.0),
            hazard_type: HazardType::MudPit,
            severity: 0.8,
            radius: 15.0,
        });
        
        self.hazards.push(Hazard {
            id: 1,
            position: Vec3::new(300.0, 0.0, -100.0),
            hazard_type: HazardType::DeepWater,
            severity: 1.0,
            radius: 25.0,
        });
    }
    
    /// Get region at position
    pub fn get_region_at(&self, x: f32, z: f32) -> Option<&WorldRegion> {
        for region in &self.regions {
            if x >= region.bounds.0 && x <= region.bounds.2 &&
               z >= region.bounds.1 && z <= region.bounds.3 {
                return Some(region);
            }
        }
        None
    }
    
    /// Get nearest anchor point
    pub fn get_nearest_anchor(&self, position: Vec3, max_distance: f32) -> Option<&WorldAnchor> {
        self.anchors
            .iter()
            .filter(|a| a.is_valid)
            .min_by(|a, b| {
                let dist_a = (a.position - position).length_squared();
                let dist_b = (b.position - position).length_squared();
                dist_a.partial_cmp(&dist_b).unwrap()
            })
            .filter(|a| (a.position - position).length() <= max_distance)
    }
    
    /// Check if position is in service area
    pub fn is_in_service_area(&self, position: Vec3) -> Option<&ServicePoint> {
        for sp in &self.service_points {
            if (sp.position - position).length() <= sp.radius {
                return Some(sp);
            }
        }
        None
    }
    
    /// Collect item at position
    pub fn collect_at(&mut self, position: Vec3, radius: f32) -> Option<Collectible> {
        for collectible in &mut self.collectibles {
            if !collectible.is_collected &&
               (collectible.position - position).length() <= radius {
                collectible.is_collected = true;
                return Some(collectible.clone());
            }
        }
        None
    }
    
    /// Check if position is in hazard zone
    pub fn get_hazard_at(&self, position: Vec3) -> Option<&Hazard> {
        for hazard in &self.hazards {
            if (hazard.position - position).length() <= hazard.radius {
                return Some(hazard);
            }
        }
        None
    }
    
    /// Activate checkpoint
    pub fn activate_checkpoint(&mut self, checkpoint_id: usize) -> bool {
        if let Some(checkpoint) = self.checkpoints.get_mut(checkpoint_id) {
            if !checkpoint.is_activated {
                checkpoint.is_activated = true;
                return true;
            }
        }
        false
    }
    
    /// Get total collectibles value
    pub fn get_total_collectibles_value(&self) -> u32 {
        self.collectibles
            .iter()
            .filter(|c| c.is_collected)
            .map(|c| c.value)
            .sum()
    }
}
