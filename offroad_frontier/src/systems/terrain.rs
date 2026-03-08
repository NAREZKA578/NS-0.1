//! Terrain system
//! 
//! Handles terrain generation, deformation, and surface properties.

use glam::Vec3;

/// Terrain chunk data
#[derive(Debug, Clone)]
pub struct TerrainChunk {
    /// Chunk position in world space
    pub position: Vec3,
    
    /// Height map data (grid of heights)
    pub heights: Vec<f32>,
    
    /// Surface type for each vertex
    pub surface_types: Vec<u8>,
    
    /// Deformation depth for each vertex
    pub deformation: Vec<f32>,
    
    /// Resolution of the chunk
    pub resolution: u32,
    
    /// Size of the chunk in world units
    pub size: f32,
}

impl TerrainChunk {
    pub fn new(position: Vec3, resolution: u32, size: f32) -> Self {
        let vertex_count = (resolution + 1) * (resolution + 1);
        Self {
            position,
            heights: vec![0.0; vertex_count as usize],
            surface_types: vec![0; vertex_count as usize],
            deformation: vec![0.0; vertex_count as usize],
            resolution,
            size,
        }
    }
    
    /// Get height at local coordinates (0.0 - 1.0)
    pub fn get_height(&self, x: f32, z: f32) -> f32 {
        let x = x.clamp(0.0, 1.0);
        let z = z.clamp(0.0, 1.0);
        
        let fx = x * self.resolution as f32;
        let fz = z * self.resolution as f32;
        
        let x0 = fx.floor() as usize;
        let z0 = fz.floor() as usize;
        let x1 = (x0 + 1).min(self.resolution as usize);
        let z1 = (z0 + 1).min(self.resolution as usize);
        
        let tx = fx - x0 as f32;
        let tz = fz - z0 as f32;
        
        let idx = |x: usize, z: usize| z * (self.resolution as usize + 1) + x;
        
        let h00 = self.heights[idx(x0, z0)];
        let h10 = self.heights[idx(x1, z0)];
        let h01 = self.heights[idx(x0, z1)];
        let h11 = self.heights[idx(x1, z1)];
        
        // Bilinear interpolation
        let h0 = h00 + (h10 - h00) * tx;
        let h1 = h01 + (h11 - h01) * tx;
        h0 + (h1 - h0) * tz
    }
    
    /// Apply deformation (e.g., from vehicle tracks)
    pub fn apply_deformation(&mut self, center_x: f32, center_z: f32, radius: f32, depth: f32) {
        let step = self.size / self.resolution as f32;
        let center_world_x = center_x * self.size;
        let center_world_z = center_z * self.size;
        
        for z in 0..=self.resolution as usize {
            for x in 0..=self.resolution as usize {
                let wx = x as f32 * step;
                let wz = z as f32 * step;
                
                let dx = wx - center_world_x;
                let dz = wz - center_world_z;
                let dist_sq = dx * dx + dz * dz;
                
                if dist_sq < radius * radius {
                    let dist = dist_sq.sqrt();
                    let falloff = 1.0 - (dist / radius).powi(2);
                    let idx = z * (self.resolution as usize + 1) + x;
                    self.deformation[idx] = (self.deformation[idx] + depth * falloff)
                        .clamp(0.0, 0.5); // Max 50cm deformation
                }
            }
        }
    }
    
    /// Get final height including deformation
    pub fn get_final_height(&self, x: f32, z: f32) -> f32 {
        self.get_height(x, z) - self.get_deformation(x, z)
    }
    
    /// Get deformation at local coordinates
    pub fn get_deformation(&self, x: f32, z: f32) -> f32 {
        // Similar to get_height but for deformation
        let x = x.clamp(0.0, 1.0);
        let z = z.clamp(0.0, 1.0);
        
        let fx = x * self.resolution as f32;
        let fz = z * self.resolution as f32;
        
        let x0 = fx.floor() as usize;
        let z0 = fz.floor() as usize;
        let x1 = (x0 + 1).min(self.resolution as usize);
        let z1 = (z0 + 1).min(self.resolution as usize);
        
        let tx = fx - x0 as f32;
        let tz = fz - z0 as f32;
        
        let idx = |x: usize, z: usize| z * (self.resolution as usize + 1) + x;
        
        let d00 = self.deformation[idx(x0, z0)];
        let d10 = self.deformation[idx(x1, z0)];
        let d01 = self.deformation[idx(x0, z1)];
        let d11 = self.deformation[idx(x1, z1)];
        
        let d0 = d00 + (d10 - d00) * tx;
        let d1 = d01 + (d11 - d01) * tx;
        d0 + (d1 - d0) * tz
    }
}

/// Terrain system manager
pub struct TerrainSystem {
    /// All terrain chunks
    pub chunks: Vec<TerrainChunk>,
    
    /// Base terrain size
    pub terrain_size: f32,
    
    /// Chunk resolution
    pub chunk_resolution: u32,
    
    /// Number of chunks per side
    pub chunks_per_side: u32,
}

impl TerrainSystem {
    pub fn new(terrain_size: f32, chunk_resolution: u32, chunks_per_side: u32) -> Self {
        let mut system = Self {
            chunks: Vec::new(),
            terrain_size,
            chunk_resolution,
            chunks_per_side,
        };
        
        system.generate_chunks();
        system
    }
    
    /// Generate all terrain chunks
    fn generate_chunks(&mut self) {
        let chunk_size = self.terrain_size / self.chunks_per_side as f32;
        
        for cz in 0..self.chunks_per_side {
            for cx in 0..self.chunks_per_side {
                let position = Vec3::new(
                    cx as f32 * chunk_size - self.terrain_size / 2.0,
                    0.0,
                    cz as f32 * chunk_size - self.terrain_size / 2.0,
                );
                
                let mut chunk = TerrainChunk::new(position, self.chunk_resolution, chunk_size);
                self.generate_heightmap(&mut chunk, cx, cz);
                self.generate_surface_types(&mut chunk, cx, cz);
                
                self.chunks.push(chunk);
            }
        }
    }
    
    /// Generate heightmap for a chunk using noise
    fn generate_heightmap(&mut self, chunk: &mut TerrainChunk, chunk_x: u32, chunk_z: u32) {
        use noise::{NoiseFn, Perlin};
        
        let perlin = Perlin::new();
        let chunk_size = self.terrain_size / self.chunks_per_side as f32;
        let scale = 0.01; // Noise scale
        
        for z in 0..=chunk.resolution as usize {
            for x in 0..=chunk.resolution as usize {
                let wx = chunk.position.x + x as f32 * chunk_size / chunk.resolution as f32;
                let wz = chunk.position.z + z as f32 * chunk_size / chunk.resolution as f32;
                
                // Multi-octave noise for realistic terrain
                let mut height = 0.0;
                let mut amplitude = 1.0;
                let mut frequency = scale;
                let mut max_amplitude = 0.0;
                
                for _ in 0..4 {
                    height += perlin.get([wx * frequency, wz * frequency]) * amplitude;
                    max_amplitude += amplitude;
                    amplitude *= 0.5;
                    frequency *= 2.0;
                }
                
                height /= max_amplitude;
                height = (height + 1.0) / 2.0; // Normalize to 0-1
                
                // Scale to actual height range (0-50 meters)
                height *= 50.0;
                
                let idx = z * (chunk.resolution as usize + 1) + x;
                chunk.heights[idx] = height;
            }
        }
    }
    
    /// Generate surface types based on height and slope
    fn generate_surface_types(&mut self, chunk: &mut TerrainChunk, chunk_x: u32, chunk_z: u32) {
        for z in 0..=chunk.resolution as usize {
            for x in 0..=chunk.resolution as usize {
                let idx = z * (chunk.resolution as usize + 1) + x;
                let height = chunk.heights[idx];
                
                // Simple surface type assignment based on height
                let surface_type = if height > 40.0 {
                    6u8 // Rock
                } else if height > 30.0 {
                    5u8 // Grass
                } else if height > 15.0 {
                    1u8 // Dirt
                } else if height > 5.0 {
                    3u8 // Sand
                } else {
                    2u8 // Mud (low areas)
                };
                
                chunk.surface_types[idx] = surface_type;
            }
        }
    }
    
    /// Get height at world position
    pub fn get_height_at(&self, world_x: f32, world_z: f32) -> Option<f32> {
        let chunk_idx_x = ((world_x + self.terrain_size / 2.0) / self.terrain_size * self.chunks_per_side as f32) as usize;
        let chunk_idx_z = ((world_z + self.terrain_size / 2.0) / self.terrain_size * self.chunks_per_side as f32) as usize;
        
        if chunk_idx_x >= self.chunks.len() || chunk_idx_z >= self.chunks.len() {
            return None;
        }
        
        let chunk = &self.chunks[chunk_idx_z * self.chunks_per_side as usize + chunk_idx_x];
        let chunk_size = self.terrain_size / self.chunks_per_side as f32;
        
        let local_x = (world_x - chunk.position.x) / chunk_size;
        let local_z = (world_z - chunk.position.z) / chunk_size;
        
        Some(chunk.get_final_height(local_x, local_z) + chunk.position.y)
    }
    
    /// Apply deformation at world position
    pub fn apply_deformation_at(&mut self, world_x: f32, world_z: f32, radius: f32, depth: f32) {
        let chunk_size = self.terrain_size / self.chunks_per_side as f32;
        
        // Find affected chunks
        let min_chunk_x = (((world_x - radius + self.terrain_size / 2.0) / self.terrain_size * self.chunks_per_side as f32) as usize).max(0);
        let max_chunk_x = (((world_x + radius + self.terrain_size / 2.0) / self.terrain_size * self.chunks_per_side as f32) as usize).min(self.chunks_per_side as usize - 1);
        let min_chunk_z = (((world_z - radius + self.terrain_size / 2.0) / self.terrain_size * self.chunks_per_side as f32) as usize).max(0);
        let max_chunk_z = (((world_z + radius + self.terrain_size / 2.0) / self.terrain_size * self.chunks_per_side as f32) as usize).min(self.chunks_per_side as usize - 1);
        
        for cz in min_chunk_z..=max_chunk_z {
            for cx in min_chunk_x..=max_chunk_x {
                let chunk_idx = cz * self.chunks_per_side as usize + cx;
                if let Some(chunk) = self.chunks.get_mut(chunk_idx) {
                    let local_x = (world_x - chunk.position.x) / chunk_size;
                    let local_z = (world_z - chunk.position.z) / chunk_size;
                    chunk.apply_deformation(local_x, local_z, radius / chunk_size, depth);
                }
            }
        }
    }
    
    /// Update terrain (called every frame)
    pub fn update(&mut self, delta_time: f32) {
        // Recovery logic could be added here
        // For now, just a placeholder
    }
}

impl Default for TerrainSystem {
    fn default() -> Self {
        Self::new(1000.0, 64, 4)
    }
}
