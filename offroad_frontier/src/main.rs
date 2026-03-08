//! Offroad Frontier - Main Entry Point
//! 
//! This is the main entry point for the game.
//! It initializes the RTGC engine and runs the game loop.

mod core;
mod systems;
mod ui;

use core::{GameConfig, GameState};
use systems::{VehiclePhysics, TerrainSystem, WeatherState, CameraSystem, WinchSystem};
use ui::{HUD, Minimap, MainMenu, NotificationSystem};

use log::{info, warn};
use std::time::Instant;

fn main() {
    // Initialize logging
    env_logger::init();
    info!("Starting Offroad Frontier...");
    
    // Load configuration
    let config = GameConfig::default();
    
    // Initialize game state
    let mut game_state = GameState::new(&config);
    
    // Initialize systems
    let mut vehicle = VehiclePhysics::new();
    let mut terrain = TerrainSystem::default();
    let mut weather = WeatherState::default();
    let mut camera = CameraSystem::new();
    let mut winch = WinchSystem::new();
    
    // Initialize UI
    let mut hud = HUD::new();
    let mut minimap = Minimap::new();
    let mut menu = MainMenu::new();
    let mut notifications = NotificationSystem::new();
    
    info!("Game initialized successfully!");
    notifications.info("Welcome to Offroad Frontier!");
    notifications.tutorial("Use WASD or Arrow Keys to drive. Press C to change camera view.");
    
    // Main game loop timing
    let mut last_time = Instant::now();
    
    // Placeholder game loop (will be integrated with RTGC engine)
    println!("=== OFFROAD FRONTIER ===");
    println!("Game initialized. Press Ctrl+C to exit.");
    println!();
    println!("Controls:");
    println!("  W/Up    - Accelerate");
    println!("  S/Down  - Brake/Reverse");
    println!("  A/Left  - Steer Left");
    println!("  D/Right - Steer Right");
    println!("  Space   - Handbrake");
    println!("  C       - Change Camera");
    println!("  E       - Winch Extend/Retract");
    println!("  Q       - Toggle Differential Lock");
    println!("  F       - Toggle 4WD");
    println!("  M       - Toggle Minimap");
    println!("  ESC     - Pause Menu");
    println!();
    
    // Simple simulation loop for testing
    let mut running = true;
    let mut frame_count = 0;
    let start_time = Instant::now();
    
    while running && frame_count < 100 {
        let now = Instant::now();
        let delta_time = now.duration_since(last_time).as_secs_f32();
        last_time = now;
        
        // Update game state
        if !game_state.is_paused {
            // Update vehicle physics
            vehicle.step(delta_time);
            
            // Update terrain deformation based on wheel positions
            let vehicle_state = vehicle.get_state();
            for wheel in &vehicle_state.wheels {
                if wheel.is_in_contact || wheel.suspension_compression > 0.1 {
                    terrain.apply_deformation_at(
                        wheel.position.x,
                        wheel.position.z,
                        0.3, // Wheel radius
                        0.05 * wheel.suspension_compression,
                    );
                }
            }
            
            // Update winch
            winch.update(delta_time, vehicle_state.position, vehicle_state.rotation);
            
            // Apply winch force to vehicle if under tension
            if winch.state != systems::winch::WinchState::Idle {
                let winch_force = winch.get_force_on_vehicle();
                // In full implementation, this would affect vehicle physics
                let _ = winch_force;
            }
            
            // Update weather
            weather.update(delta_time);
            
            // Update camera
            camera.update(
                delta_time,
                vehicle_state.position,
                vehicle_state.rotation,
            );
            
            // Update game state
            game_state.update(delta_time);
            
            // Update notifications
            notifications.update(delta_time);
        }
        
        // Simple input handling placeholder
        // In real implementation, this would read from actual input system
        handle_placeholder_input(&mut vehicle, &mut camera, &mut winch, &mut menu, &mut game_state, &mut notifications);
        
        frame_count += 1;
        
        // Print status every 30 frames
        if frame_count % 30 == 0 {
            let elapsed = start_time.elapsed().as_secs_f32();
            let fps = frame_count as f32 / elapsed;
            
            let v = vehicle.get_state();
            println!("Frame: {} | FPS: {:.1} | Pos: ({:.1}, {:.1}, {:.1}) | Speed: {:.1} km/h",
                frame_count,
                fps,
                v.position.x,
                v.position.y,
                v.position.z,
                v.linear_velocity.length() * 3.6
            );
        }
        
        // Small delay to simulate frame time
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    
    info!("Game loop ended. Shutting down...");
    println!("\nGame session ended. Thanks for playing!");
}

/// Placeholder input handling
fn handle_placeholder_input(
    vehicle: &mut VehiclePhysics,
    camera: &mut CameraSystem,
    winch: &mut WinchSystem,
    menu: &mut MainMenu,
    game_state: &mut GameState,
    notifications: &mut NotificationSystem,
) {
    // This is a simulation - in real game, input comes from window events
    // For now, just do some automatic demo behavior
    
    let v = vehicle.get_state_mut();
    
    // Auto-accelerate forward
    v.set_throttle(0.5);
    
    // Occasional steering
    use std::time::{SystemTime, UNIX_EPOCH};
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f32;
    
    let steer = ((time / 1000.0).sin() * 0.5).to_radians();
    v.set_steering(steer);
    
    // Simulate key press for camera change every 10 seconds
    if (time as u64) % 10000 < 100 {
        camera.cycle_mode();
        notifications.info(format!("Camera mode: {:?}", camera.mode));
    }
}
