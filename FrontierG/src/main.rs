//! FrontierG - Off-road Simulation Game
//! 
//! A realistic off-road driving simulator featuring:
//! - Advanced vehicle physics with suspension and tire simulation
//! - Deformable terrain with mud, snow, ice surfaces
//! - Dynamic weather affecting traction and visibility
//! - Winch mechanics for recovery
//! - Mission-based gameplay

mod game;
mod systems;
mod ui;

use rtgc_engine::engine::Engine;
use rtgc_engine::game::Game as EngineGame;
use log::{info, error};

fn main() {
    env_logger::init();
    info!("Starting FrontierG...");

    let mut engine = Engine::new();
    
    // Initialize game systems
    let game = game::FrontierGame::new();
    
    info!("FrontierG initialized successfully");
    
    // Run the main loop
    if let Err(e) = engine.run(game) {
        error!("Engine error: {}", e);
    }
}
