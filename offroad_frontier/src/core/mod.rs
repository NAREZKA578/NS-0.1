//! Offroad Frontier - Core Game Module
//! 
//! This module contains the core game logic, state management,
//! world data, events, and integration with the RTGC engine.

pub mod game_state;
pub mod config;
pub mod constants;
pub mod events;
pub mod world;

pub use game_state::GameState;
pub use config::GameConfig;
pub use constants::*;
pub use events::*;
pub use world::*;
