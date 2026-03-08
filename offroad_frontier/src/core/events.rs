//! Core events module
//! 
//! Contains event types for game communication between systems

use glam::Vec3;
use crate::core::constants::SurfaceType;
use crate::systems::weather::WeatherType;

/// Base event trait
pub trait GameEvent: Send + Sync {
    fn event_type(&self) -> EventType;
}

/// Event types enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    // Vehicle events
    VehicleEngineStart,
    VehicleEngineStop,
    VehicleGearChange,
    VehicleDamage,
    VehicleStuck,
    VehicleRecovered,
    
    // Terrain events
    TerrainDeformation,
    TerrainRecovery,
    
    // Weather events
    WeatherChange,
    PrecipitationStart,
    PrecipitationStop,
    
    // Winch events
    WinchAttach,
    WinchDetach,
    WinchCableBreak,
    
    // Mission events
    MissionStart,
    MissionComplete,
    MissionFail,
    CheckpointReach,
    
    // UI events
    UINotification,
    UIPause,
    UIResume,
    
    // Audio events
    AudioPlay,
    AudioStop,
}

/// Vehicle-related events
#[derive(Debug, Clone)]
pub struct VehicleEvent {
    pub event_type: VehicleEventType,
    pub vehicle_id: u32,
    pub position: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleEventType {
    EngineStarted,
    EngineStopped,
    GearChanged(i8),
    DamageTaken(f32),
    Stuck,
    Recovered,
    Refueled,
}

impl GameEvent for VehicleEvent {
    fn event_type(&self) -> EventType {
        match self.event_type {
            VehicleEventType::EngineStarted => EventType::VehicleEngineStart,
            VehicleEventType::EngineStopped => EventType::VehicleEngineStop,
            VehicleEventType::GearChanged(_) => EventType::VehicleGearChange,
            VehicleEventType::DamageTaken(_) => EventType::VehicleDamage,
            VehicleEventType::Stuck => EventType::VehicleStuck,
            VehicleEventType::Recovered => EventType::VehicleRecovered,
            VehicleEventType::Refueled => EventType::UINotification,
        }
    }
}

/// Terrain deformation event
#[derive(Debug, Clone)]
pub struct TerrainDeformationEvent {
    pub position: Vec3,
    pub radius: f32,
    pub depth: f32,
    pub surface_type: SurfaceType,
}

impl GameEvent for TerrainDeformationEvent {
    fn event_type(&self) -> EventType {
        EventType::TerrainDeformation
    }
}

/// Weather change event
#[derive(Debug, Clone)]
pub struct WeatherChangeEvent {
    pub old_weather: WeatherType,
    pub new_weather: WeatherType,
    pub transition_duration: f32,
}

impl GameEvent for WeatherChangeEvent {
    fn event_type(&self) -> EventType {
        EventType::WeatherChange
    }
}

/// Winch event
#[derive(Debug, Clone)]
pub struct WinchEvent {
    pub event_type: WinchEventType,
    pub vehicle_id: u32,
    pub anchor_position: Option<Vec3>,
    pub tension: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinchEventType {
    Attached,
    Detached,
    CableBroken,
    MaxTensionReached,
}

impl GameEvent for WinchEvent {
    fn event_type(&self) -> EventType {
        match self.event_type {
            WinchEventType::Attached => EventType::WinchAttach,
            WinchEventType::Detached => EventType::WinchDetach,
            WinchEventType::CableBroken => EventType::WinchCableBreak,
            WinchEventType::MaxTensionReached => EventType::UINotification,
        }
    }
}

/// Mission event
#[derive(Debug, Clone)]
pub struct MissionEvent {
    pub event_type: MissionEventType,
    pub mission_id: usize,
    pub mission_name: String,
    pub completion_percentage: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionEventType {
    Started,
    Completed,
    Failed,
    CheckpointReached(usize),
    ProgressUpdated(f32),
}

impl GameEvent for MissionEvent {
    fn event_type(&self) -> EventType {
        match self.event_type {
            MissionEventType::Started => EventType::MissionStart,
            MissionEventType::Completed => EventType::MissionComplete,
            MissionEventType::Failed => EventType::MissionFail,
            MissionEventType::CheckpointReached(_) => EventType::CheckpointReach,
            MissionEventType::ProgressUpdated(_) => EventType::UINotification,
        }
    }
}

/// Generic notification event
#[derive(Debug, Clone)]
pub struct NotificationEvent {
    pub message: String,
    pub notification_type: NotificationType,
    pub duration: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationType {
    Info,
    Warning,
    Error,
    Success,
    Tutorial,
}

impl GameEvent for NotificationEvent {
    fn event_type(&self) -> EventType {
        EventType::UINotification
    }
}

/// Event bus for system communication
pub struct EventBus {
    subscribers: std::collections::HashMap<EventType, Vec<Box<dyn Fn(&dyn GameEvent) + Send + Sync>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscribers: std::collections::HashMap::new(),
        }
    }
    
    /// Subscribe to an event type
    pub fn subscribe<F>(&mut self, event_type: EventType, handler: F)
    where
        F: Fn(&dyn GameEvent) + Send + Sync + 'static,
    {
        self.subscribers
            .entry(event_type)
            .or_insert_with(Vec::new)
            .push(Box::new(handler));
    }
    
    /// Publish an event to all subscribers
    pub fn publish(&self, event: &dyn GameEvent) {
        if let Some(handlers) = self.subscribers.get(&event.event_type()) {
            for handler in handlers {
                handler(event);
            }
        }
        
        // Also notify generic UI listeners
        if let Some(handlers) = self.subscribers.get(&EventType::UINotification) {
            for handler in handlers {
                handler(event);
            }
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
