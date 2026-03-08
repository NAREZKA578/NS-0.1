//! Particles module
//! 
//! Contains particle systems for visual effects:
//! - Mud splashes
//! - Water spray
//! - Smoke
//! - Dust
//! - Snow/rain

use glam::Vec3;
use std::time::Duration;

/// Particle type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleType {
    MudSplash,
    WaterSpray,
    Smoke,
    Dust,
    Snow,
    Rain,
    Spark,
    Exhaust,
    Steam,
}

/// Individual particle
#[derive(Debug, Clone)]
pub struct Particle {
    /// Position in world space
    pub position: Vec3,
    /// Velocity
    pub velocity: Vec3,
    /// Current lifetime
    pub age: f32,
    /// Maximum lifetime
    pub max_age: f32,
    /// Current size
    pub size: f32,
    /// Initial size
    pub initial_size: f32,
    /// Current color (RGBA)
    pub color: [f32; 4],
    /// Type of particle
    pub particle_type: ParticleType,
    /// Is particle affected by gravity
    pub affected_by_gravity: bool,
    /// Is particle affected by wind
    pub affected_by_wind: bool,
}

impl Particle {
    /// Create a new particle
    pub fn new(
        position: Vec3,
        velocity: Vec3,
        max_age: f32,
        size: f32,
        color: [f32; 4],
        particle_type: ParticleType,
    ) -> Self {
        Self {
            position,
            velocity,
            age: 0.0,
            max_age,
            size,
            initial_size: size,
            color,
            particle_type,
            affected_by_gravity: matches!(particle_type, ParticleType::MudSplash | ParticleType::WaterSpray),
            affected_by_wind: matches!(particle_type, ParticleType::Smoke | ParticleType::Dust | ParticleType::Snow),
        }
    }
    
    /// Update particle
    pub fn update(&mut self, delta_time: f32, gravity: Vec3, wind: Vec3) -> bool {
        self.age += delta_time;
        
        if self.age >= self.max_age {
            return false; // Particle died
        }
        
        // Apply forces
        if self.affected_by_gravity {
            self.velocity += gravity * delta_time;
        }
        
        if self.affected_by_wind {
            self.velocity += wind * delta_time * 0.5;
        }
        
        // Apply drag
        self.velocity *= 0.98;
        
        // Update position
        self.position += self.velocity * delta_time;
        
        // Shrink over time
        let life_ratio = 1.0 - (self.age / self.max_age);
        self.size = self.initial_size * life_ratio;
        
        // Fade out
        self.color[3] = life_ratio;
        
        true
    }
}

/// Particle emitter configuration
#[derive(Debug, Clone)]
pub struct EmitterConfig {
    /// Emission rate (particles per second)
    pub emission_rate: f32,
    /// Initial velocity range
    pub velocity_range: (f32, f32),
    /// Initial size range
    pub size_range: (f32, f32),
    /// Lifetime range
    pub lifetime_range: (f32, f32),
    /// Color gradient (start, end)
    pub color_gradient: ([f32; 4], [f32; 4]),
    /// Spread angle in radians
    pub spread_angle: f32,
    /// Local position offset
    pub local_offset: Vec3,
}

impl Default for EmitterConfig {
    fn default() -> Self {
        Self {
            emission_rate: 10.0,
            velocity_range: (1.0, 3.0),
            size_range: (0.1, 0.3),
            lifetime_range: (1.0, 2.0),
            color_gradient: ([1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 0.0]),
            spread_angle: std::f32::consts::PI / 4.0,
            local_offset: Vec3::ZERO,
        }
    }
}

/// Particle emitter
pub struct ParticleEmitter {
    /// Emitter position
    pub position: Vec3,
    /// Emitter rotation
    pub rotation: glam::Quat,
    /// Configuration
    pub config: EmitterConfig,
    /// Particle type
    pub particle_type: ParticleType,
    /// Is emitter active
    pub is_active: bool,
    /// Time since last emission
    pub emission_timer: f32,
    /// Maximum particles
    pub max_particles: usize,
}

impl ParticleEmitter {
    /// Create a new emitter
    pub fn new(position: Vec3, particle_type: ParticleType, config: EmitterConfig) -> Self {
        Self {
            position,
            rotation: glam::Quat::IDENTITY,
            config,
            particle_type,
            is_active: true,
            emission_timer: 0.0,
            max_particles: 1000,
        }
    }
    
    /// Emit a single particle
    pub fn emit_particle(&self) -> Option<Particle> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Random values within ranges
        let velocity_mag = rng.gen_range(self.config.velocity_range.0..=self.config.velocity_range.1);
        let size = rng.gen_range(self.config.size_range.0..=self.config.size_range.1);
        let lifetime = rng.gen_range(self.config.lifetime_range.0..=self.config.lifetime_range.1);
        
        // Random direction within spread cone
        let theta = rng.gen_range(0.0..=self.config.spread_angle);
        let phi = rng.gen_range(0.0..=std::f32::consts::PI * 2.0);
        
        let dir = Vec3::new(
            theta.sin() * phi.cos(),
            theta.cos(),
            theta.sin() * phi.sin(),
        ).normalize_or(Vec3::Y);
        
        let velocity = self.rotation * dir * velocity_mag;
        
        // Interpolate color
        let t = rng.gen_range(0.0..=0.2); // Start near beginning color
        let color = [
            self.config.color_gradient.0[0] + (self.config.color_gradient.1[0] - self.config.color_gradient.0[0]) * t,
            self.config.color_gradient.0[1] + (self.config.color_gradient.1[1] - self.config.color_gradient.0[1]) * t,
            self.config.color_gradient.0[2] + (self.config.color_gradient.1[2] - self.config.color_gradient.0[2]) * t,
            self.config.color_gradient.0[3] + (self.config.color_gradient.1[3] - self.config.color_gradient.0[3]) * t,
        ];
        
        Some(Particle::new(
            self.position + self.rotation * self.config.local_offset,
            velocity,
            lifetime,
            size,
            color,
            self.particle_type,
        ))
    }
    
    /// Update emitter and generate new particles
    pub fn update(&mut self, delta_time: f32) -> Vec<Particle> {
        if !self.is_active {
            return Vec::new();
        }
        
        let mut new_particles = Vec::new();
        
        self.emission_timer += delta_time;
        let emit_interval = 1.0 / self.config.emission_rate;
        
        while self.emission_timer >= emit_interval {
            self.emission_timer -= emit_interval;
            
            if let Some(particle) = self.emit_particle() {
                new_particles.push(particle);
            }
        }
        
        new_particles
    }
    
    /// Set emitter position
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }
    
    /// Set emitter rotation
    pub fn set_rotation(&mut self, rotation: glam::Quat) {
        self.rotation = rotation;
    }
    
    /// Activate/deactivate emitter
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}

/// Particle system manager
pub struct ParticleSystem {
    /// All active particles
    pub particles: Vec<Particle>,
    /// All emitters
    pub emitters: Vec<ParticleEmitter>,
    /// Gravity vector
    pub gravity: Vec3,
    /// Wind vector
    pub wind: Vec3,
    /// Maximum total particles
    pub max_total_particles: usize,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ParticleSystem {
    /// Create new particle system
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            emitters: Vec::new(),
            gravity: Vec3::new(0.0, -9.81, 0.0),
            wind: Vec3::ZERO,
            max_total_particles: 10000,
        }
    }
    
    /// Add an emitter
    pub fn add_emitter(&mut self, emitter: ParticleEmitter) {
        self.emitters.push(emitter);
    }
    
    /// Remove emitter by index
    pub fn remove_emitter(&mut self, index: usize) {
        if index < self.emitters.len() {
            self.emitters.remove(index);
        }
    }
    
    /// Update all particles and emitters
    pub fn update(&mut self, delta_time: f32) {
        // Update emitters and collect new particles
        for emitter in &mut self.emitters {
            let new_particles = emitter.update(delta_time);
            if self.particles.len() + new_particles.len() <= self.max_total_particles {
                self.particles.extend(new_particles);
            }
        }
        
        // Update existing particles
        self.particles.retain_mut(|particle| {
            particle.update(delta_time, self.gravity, self.wind)
        });
    }
    
    /// Spawn a quick effect at position (one-shot)
    pub fn spawn_effect(
        &mut self,
        position: Vec3,
        particle_type: ParticleType,
        count: usize,
        velocity_range: (f32, f32),
        size_range: (f32, f32),
        lifetime: f32,
        color: [f32; 4],
    ) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for _ in 0..count {
            if self.particles.len() >= self.max_total_particles {
                break;
            }
            
            let velocity_mag = rng.gen_range(velocity_range.0..=velocity_range.1);
            let size = rng.gen_range(size_range.0..=size_range.1);
            
            let dir = Vec3::new(
                rng.gen_range(-1.0..=1.0),
                rng.gen_range(0.0..=1.0),
                rng.gen_range(-1.0..=1.0),
            ).normalize_or(Vec3::Y);
            
            let velocity = dir * velocity_mag;
            
            self.particles.push(Particle::new(
                position,
                velocity,
                lifetime,
                size,
                color,
                particle_type,
            ));
        }
    }
    
    /// Spawn mud splash from wheel
    pub fn spawn_mud_splash(&mut self, position: Vec3, velocity: Vec3, intensity: f32) {
        let count = (intensity * 20.0) as usize;
        self.spawn_effect(
            position,
            ParticleType::MudSplash,
            count,
            (2.0, 5.0),
            (0.05, 0.15),
            1.5,
            [0.3, 0.2, 0.1, 0.8],
        );
    }
    
    /// Spawn exhaust smoke
    pub fn spawn_exhaust(&mut self, position: Vec3, engine_load: f32) {
        let count = (engine_load * 5.0) as usize;
        self.spawn_effect(
            position,
            ParticleType::Exhaust,
            count.max(1),
            (0.5, 1.5),
            (0.2, 0.4),
            2.0,
            [0.3, 0.3, 0.3, 0.6],
        );
    }
    
    /// Clear all particles
    pub fn clear(&mut self) {
        self.particles.clear();
    }
    
    /// Get particle count
    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }
}
