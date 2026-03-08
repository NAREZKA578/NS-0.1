//! Mission system
//!
//! Handles mission creation, tracking, and completion:
//! - Delivery missions
//! - Recovery missions
//! - Exploration missions
//! - Time trial missions

use glam::Vec3;
use crate::core::world::{Checkpoint, ServicePoint};

/// Mission type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionType {
    /// Deliver cargo from A to B
    Delivery,
    /// Recover a broken vehicle
    Recovery,
    /// Reach all checkpoints in order
    CheckpointRun,
    /// Explore and find collectibles
    Exploration,
    /// Complete course within time limit
    TimeTrial,
    /// Survive in harsh conditions
    Survival,
}

/// Mission difficulty
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionDifficulty {
    Easy,
    Normal,
    Hard,
    Extreme,
}

/// Cargo type for delivery missions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CargoType {
    Supplies,
    Fuel,
    Equipment,
    Logs,
    ConstructionMaterials,
    EmergencyAid,
}

/// Mission objective
#[derive(Debug, Clone)]
pub struct MissionObjective {
    /// Objective description
    pub description: String,
    /// Is objective completed
    pub is_completed: bool,
    /// Optional position target
    pub target_position: Option<Vec3>,
    /// Optional radius for completion
    pub completion_radius: f32,
    /// Progress towards completion (0.0 - 1.0)
    pub progress: f32,
}

/// Mission reward
#[derive(Debug, Clone, Default)]
pub struct MissionReward {
    /// Money reward
    pub money: u32,
    /// Experience points
    pub experience: u32,
    /// Unlock new region (region ID)
    pub unlock_region: Option<u32>,
    /// Unlock new vehicle
    pub unlock_vehicle: Option<String>,
    /// Reputation gain
    pub reputation: i32,
}

/// Complete mission data
#[derive(Debug, Clone)]
pub struct Mission {
    /// Unique mission ID
    pub id: usize,
    /// Mission name
    pub name: String,
    /// Mission description
    pub description: String,
    /// Type of mission
    pub mission_type: MissionType,
    /// Difficulty level
    pub difficulty: MissionDifficulty,
    /// Starting position
    pub start_position: Vec3,
    /// Start service point (if any)
    pub start_service_point: Option<u32>,
    /// End position
    pub end_position: Vec3,
    /// End service point (if any)
    pub end_service_point: Option<u32>,
    /// Checkpoints for the mission
    pub checkpoints: Vec<Checkpoint>,
    /// Mission objectives
    pub objectives: Vec<MissionObjective>,
    /// Cargo information (for delivery missions)
    pub cargo: Option<CargoInfo>,
    /// Time limit in seconds (None = no limit)
    pub time_limit: Option<f32>,
    /// Current elapsed time
    pub elapsed_time: f32,
    /// Is mission active
    pub is_active: bool,
    /// Is mission completed
    pub is_completed: bool,
    /// Is mission failed
    pub is_failed: bool,
    /// Failure reason
    pub failure_reason: Option<String>,
    /// Reward for completion
    pub reward: MissionReward,
}

/// Cargo information
#[derive(Debug, Clone)]
pub struct CargoInfo {
    /// Type of cargo
    pub cargo_type: CargoType,
    /// Weight in kg
    pub weight: f32,
    /// Volume in m³
    pub volume: f32,
    /// Fragile cargo (affects driving)
    pub is_fragile: bool,
    /// Damage taken (0.0 - 1.0)
    pub damage_taken: f32,
    /// Maximum allowed damage
    pub max_damage: f32,
}

impl Mission {
    /// Create a new delivery mission
    pub fn new_delivery(
        id: usize,
        name: String,
        start: Vec3,
        end: Vec3,
        cargo: CargoInfo,
        difficulty: MissionDifficulty,
        reward: MissionReward,
    ) -> Self {
        Self {
            id,
            name,
            description: format!("Deliver {:?} cargo to destination", cargo.cargo_type),
            mission_type: MissionType::Delivery,
            difficulty,
            start_position: start,
            end_position: end,
            start_service_point: None,
            end_service_point: None,
            checkpoints: Vec::new(),
            objectives: vec![
                MissionObjective {
                    description: "Pick up cargo".to_string(),
                    is_completed: false,
                    target_position: Some(start),
                    completion_radius: 10.0,
                    progress: 0.0,
                },
                MissionObjective {
                    description: "Deliver cargo".to_string(),
                    is_completed: false,
                    target_position: Some(end),
                    completion_radius: 15.0,
                    progress: 0.0,
                },
            ],
            cargo: Some(cargo),
            time_limit: None,
            elapsed_time: 0.0,
            is_active: false,
            is_completed: false,
            is_failed: false,
            failure_reason: None,
            reward,
        }
    }

    /// Create a checkpoint run mission
    pub fn new_checkpoint_run(
        id: usize,
        name: String,
        checkpoints: Vec<Checkpoint>,
        time_limit: Option<f32>,
        difficulty: MissionDifficulty,
        reward: MissionReward,
    ) -> Self {
        let start_pos = checkpoints.first().map(|c| c.position).unwrap_or(Vec3::ZERO);
        let end_pos = checkpoints.last().map(|c| c.position).unwrap_or(Vec3::ZERO);

        Self {
            id,
            name,
            description: format!("Reach all {} checkpoints", checkpoints.len()),
            mission_type: MissionType::CheckpointRun,
            difficulty,
            start_position: start_pos,
            end_position: end_pos,
            start_service_point: None,
            end_service_point: None,
            checkpoints,
            objectives: Vec::new(),
            cargo: None,
            time_limit,
            elapsed_time: 0.0,
            is_active: false,
            is_completed: false,
            is_failed: false,
            failure_reason: None,
            reward,
        }
    }

    /// Update mission progress
    pub fn update(&mut self, delta_time: f32, current_position: Vec3) {
        if !self.is_active || self.is_completed || self.is_failed {
            return;
        }

        // Update elapsed time
        self.elapsed_time += delta_time;

        // Check time limit
        if let Some(limit) = self.time_limit {
            if self.elapsed_time > limit {
                self.fail("Time limit exceeded".to_string());
                return;
            }
        }

        // Check cargo damage (for delivery missions)
        if let Some(ref mut cargo) = self.cargo {
            if cargo.damage_taken >= cargo.max_damage {
                self.fail("Cargo destroyed".to_string());
                return;
            }
        }

        // Update objectives
        for objective in &mut self.objectives {
            if !objective.is_completed {
                if let Some(target) = objective.target_position {
                    let distance = (current_position - target).length();
                    if distance <= objective.completion_radius {
                        objective.is_completed = true;
                        objective.progress = 1.0;
                    } else {
                        objective.progress = 1.0 - (distance / objective.completion_radius).min(1.0);
                    }
                }
            }
        }

        // Check checkpoints
        for checkpoint in &mut self.checkpoints {
            if !checkpoint.is_activated {
                let distance = (current_position - checkpoint.position).length();
                if distance <= checkpoint.radius {
                    checkpoint.is_activated = true;
                }
            }
        }

        // Check mission completion
        self.check_completion();
    }

    /// Check if mission is complete
    fn check_completion(&mut self) {
        let all_objectives_complete = self.objectives.iter().all(|o| o.is_completed);
        let all_checkpoints_complete = self.checkpoints.iter().all(|c| c.is_activated);

        match self.mission_type {
            MissionType::Delivery => {
                if all_objectives_complete {
                    self.complete();
                }
            }
            MissionType::CheckpointRun => {
                if all_checkpoints_complete {
                    self.complete();
                }
            }
            _ => {
                if all_objectives_complete {
                    self.complete();
                }
            }
        }
    }

    /// Complete the mission
    pub fn complete(&mut self) {
        self.is_completed = true;
        self.is_active = false;
    }

    /// Fail the mission
    pub fn fail(&mut self, reason: String) {
        self.is_failed = true;
        self.is_active = false;
        self.failure_reason = Some(reason);
    }

    /// Get overall mission progress (0.0 - 1.0)
    pub fn get_progress(&self) -> f32 {
        let objective_progress: f32 = self.objectives.iter()
            .map(|o| o.progress)
            .sum::<f32>() / self.objectives.len().max(1) as f32;

        let checkpoint_progress: f32 = self.checkpoints.iter()
            .filter(|c| c.is_activated)
            .count() as f32 / self.checkpoints.len().max(1) as f32;

        match self.mission_type {
            MissionType::CheckpointRun => checkpoint_progress,
            _ => objective_progress,
        }
    }

    /// Get formatted time display
    pub fn get_time_display(&self) -> String {
        let minutes = (self.elapsed_time / 60.0) as u32;
        let seconds = (self.elapsed_time % 60.0) as u32;

        if let Some(limit) = self.time_limit {
            let limit_minutes = (limit / 60.0) as u32;
            let limit_seconds = (limit % 60.0) as u32;
            format!("{:02}:{:02} / {:02}:{:02}", minutes, seconds, limit_minutes, limit_seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        }
    }
}

/// Mission manager
pub struct MissionManager {
    /// All available missions
    pub missions: Vec<Mission>,
    /// Currently active mission index
    pub active_mission_index: Option<usize>,
    /// Completed mission IDs
    pub completed_missions: Vec<usize>,
    /// Total money earned
    pub total_money: u32,
    /// Total experience earned
    pub total_experience: u32,
    /// Player reputation
    pub reputation: i32,
}

impl Default for MissionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MissionManager {
    /// Create new mission manager
    pub fn new() -> Self {
        let mut manager = Self {
            missions: Vec::new(),
            active_mission_index: None,
            completed_missions: Vec::new(),
            total_money: 0,
            total_experience: 0,
            reputation: 0,
        };

        // Generate some default missions
        manager.generate_default_missions();
        manager
    }

    /// Generate default missions
    fn generate_default_missions(&mut self) {
        // Delivery mission
        self.missions.push(Mission::new_delivery(
            0,
            "First Delivery".to_string(),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(500.0, 0.0, 300.0),
            CargoInfo {
                cargo_type: CargoType::Supplies,
                weight: 500.0,
                volume: 2.0,
                is_fragile: false,
                damage_taken: 0.0,
                max_damage: 0.5,
            },
            MissionDifficulty::Easy,
            MissionReward {
                money: 500,
                experience: 100,
                unlock_region: None,
                unlock_vehicle: None,
                reputation: 10,
            },
        ));

        // Checkpoint run mission
        let checkpoints = vec![
            Checkpoint {
                id: 0,
                position: Vec3::new(0.0, 0.0, 0.0),
                radius: 10.0,
                is_activated: false,
                next_checkpoint: Some(1),
            },
            Checkpoint {
                id: 1,
                position: Vec3::new(200.0, 0.0, 100.0),
                radius: 15.0,
                is_activated: false,
                next_checkpoint: Some(2),
            },
            Checkpoint {
                id: 2,
                position: Vec3::new(400.0, 0.0, -100.0),
                radius: 15.0,
                is_activated: false,
                next_checkpoint: None,
            },
        ];

        self.missions.push(Mission::new_checkpoint_run(
            1,
            "Valley Sprint".to_string(),
            checkpoints,
            Some(300.0), // 5 minutes
            MissionDifficulty::Normal,
            MissionReward {
                money: 800,
                experience: 200,
                unlock_region: None,
                unlock_vehicle: None,
                reputation: 20,
            },
        ));
    }

    /// Start a mission
    pub fn start_mission(&mut self, mission_id: usize) -> bool {
        if let Some(mission) = self.missions.get_mut(mission_id) {
            if mission.is_active || mission.is_completed {
                return false;
            }

            // Reset mission state
            mission.is_active = true;
            mission.is_completed = false;
            mission.is_failed = false;
            mission.elapsed_time = 0.0;
            mission.failure_reason = None;

            for objective in &mut mission.objectives {
                objective.is_completed = false;
                objective.progress = 0.0;
            }

            for checkpoint in &mut mission.checkpoints {
                checkpoint.is_activated = false;
            }

            self.active_mission_index = Some(mission_id);
            true
        } else {
            false
        }
    }

    /// Cancel active mission
    pub fn cancel_mission(&mut self) {
        if let Some(index) = self.active_mission_index {
            if let Some(mission) = self.missions.get_mut(index) {
                mission.is_active = false;
            }
            self.active_mission_index = None;
        }
    }

    /// Update active mission
    pub fn update(&mut self, delta_time: f32, current_position: Vec3) {
        if let Some(index) = self.active_mission_index {
            if let Some(mission) = self.missions.get_mut(index) {
                mission.update(delta_time, current_position);

                if mission.is_completed {
                    // Apply rewards
                    self.total_money += mission.reward.money;
                    self.total_experience += mission.reward.experience;
                    self.reputation += mission.reward.reputation;
                    self.completed_missions.push(mission.id);
                    self.active_mission_index = None;
                }
            }
        }
    }

    /// Get active mission
    pub fn get_active_mission(&self) -> Option<&Mission> {
        self.active_mission_index.and_then(|i| self.missions.get(i))
    }

    /// Get mutable active mission
    pub fn get_active_mission_mut(&mut self) -> Option<&mut Mission> {
        self.active_mission_index.and_then(|i| self.missions.get_mut(i))
    }

    /// Get available (not started) missions
    pub fn get_available_missions(&self) -> Vec<&Mission> {
        self.missions.iter()
            .filter(|m| !m.is_active && !m.is_completed && !m.is_failed)
            .collect()
    }

    /// Add damage to cargo (for active delivery mission)
    pub fn add_cargo_damage(&mut self, damage: f32) {
        if let Some(mission) = self.get_active_mission_mut() {
            if let Some(ref mut cargo) = mission.cargo {
                cargo.damage_taken = (cargo.damage_taken + damage).min(cargo.max_damage);
            }
        }
    }
}
