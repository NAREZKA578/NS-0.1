//! Notification system
//! 
//! Displays messages, warnings, and tutorials to the player

use std::collections::VecDeque;

/// Notification priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Notification type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationType {
    Info,
    Warning,
    Error,
    Success,
    Tutorial,
    Achievement,
}

/// A single notification
#[derive(Debug, Clone)]
pub struct Notification {
    /// Message text
    pub message: String,
    
    /// Optional title
    pub title: Option<String>,
    
    /// Notification type (affects icon/color)
    pub notification_type: NotificationType,
    
    /// Priority (affects order and display duration)
    pub priority: NotificationPriority,
    
    /// Display duration in seconds
    pub duration: f32,
    
    /// Time remaining until dismissal
    pub time_remaining: f32,
    
    /// Can player dismiss this manually?
    pub dismissible: bool,
    
    /// Unique ID for tracking
    pub id: u64,
}

impl Notification {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            title: None,
            notification_type: NotificationType::Info,
            priority: NotificationPriority::Normal,
            duration: 5.0,
            time_remaining: 5.0,
            dismissible: true,
            id: rand::random(),
        }
    }
    
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    
    pub fn with_type(mut self, notification_type: NotificationType) -> Self {
        self.notification_type = notification_type;
        self
    }
    
    pub fn with_priority(mut self, priority: NotificationPriority) -> Self {
        self.priority = priority;
        self.duration = match priority {
            NotificationPriority::Low => 3.0,
            NotificationPriority::Normal => 5.0,
            NotificationPriority::High => 8.0,
            NotificationPriority::Critical => 15.0,
        };
        self.time_remaining = self.duration;
        self
    }
    
    pub fn with_duration(mut self, duration: f32) -> Self {
        self.duration = duration;
        self.time_remaining = duration;
        self
    }
    
    pub fn non_dismissible(mut self) -> Self {
        self.dismissible = false;
        self
    }
}

/// Notification system manager
pub struct NotificationSystem {
    /// Active notifications (displayed on screen)
    pub active: Vec<Notification>,
    
    /// Queued notifications (waiting to be displayed)
    pub queue: VecDeque<Notification>,
    
    /// Maximum simultaneous notifications
    pub max_visible: usize,
    
    /// Notification counter for IDs
    next_id: u64,
}

impl Default for NotificationSystem {
    fn default() -> Self {
        Self {
            active: Vec::new(),
            queue: VecDeque::new(),
            max_visible: 3,
            next_id: 0,
        }
    }
}

impl NotificationSystem {
    /// Create new notification system
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a notification to be displayed
    pub fn add(&mut self, notification: Notification) {
        let mut notification = notification;
        notification.id = self.next_id;
        self.next_id += 1;
        
        // Sort queue by priority
        self.queue.push_back(notification);
        self.sort_queue();
    }
    
    /// Show an info message
    pub fn info(&mut self, message: impl Into<String>) {
        self.add(Notification::new(message).with_type(NotificationType::Info));
    }
    
    /// Show a warning message
    pub fn warning(&mut self, message: impl Into<String>) {
        self.add(Notification::new(message)
            .with_type(NotificationType::Warning)
            .with_priority(NotificationPriority::High));
    }
    
    /// Show an error message
    pub fn error(&mut self, message: impl Into<String>) {
        self.add(Notification::new(message)
            .with_type(NotificationType::Error)
            .with_priority(NotificationPriority::Critical));
    }
    
    /// Show a success message
    pub fn success(&mut self, message: impl Into<String>) {
        self.add(Notification::new(message).with_type(NotificationType::Success));
    }
    
    /// Show a tutorial tip
    pub fn tutorial(&mut self, message: impl Into<String>) {
        self.add(Notification::new(message)
            .with_type(NotificationType::Tutorial)
            .with_duration(8.0));
    }
    
    /// Show an achievement unlocked
    pub fn achievement(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.add(Notification::new(message)
            .with_title(title)
            .with_type(NotificationType::Achievement)
            .with_priority(NotificationPriority::High)
            .with_duration(10.0));
    }
    
    /// Sort queue by priority (highest first)
    fn sort_queue(&mut self) {
        let mut queue_vec: Vec<_> = self.queue.drain(..).collect();
        queue_vec.sort_by(|a, b| b.priority.cmp(&a.priority));
        self.queue = queue_vec.into_iter().collect();
    }
    
    /// Update notifications (call every frame)
    pub fn update(&mut self, delta_time: f32) {
        // Fill active slots from queue
        while self.active.len() < self.max_visible && !self.queue.is_empty() {
            if let Some(notification) = self.queue.pop_front() {
                self.active.push(notification);
            }
        }
        
        // Update active notifications
        self.active.retain_mut(|notification| {
            notification.time_remaining -= delta_time;
            notification.time_remaining > 0.0
        });
    }
    
    /// Dismiss a specific notification
    pub fn dismiss(&mut self, id: u64) {
        self.active.retain(|n| n.id != id);
    }
    
    /// Dismiss all dismissible notifications
    pub fn dismiss_all_dismissible(&mut self) {
        self.active.retain(|n| !n.dismissible);
    }
    
    /// Clear all notifications
    pub fn clear(&mut self) {
        self.active.clear();
        self.queue.clear();
    }
    
    /// Get active notifications for rendering
    pub fn get_active(&self) -> &[Notification] {
        &self.active
    }
}

/// Preset messages for common game events
pub mod presets {
    use super::*;
    
    pub fn vehicle_stuck() -> Notification {
        Notification::new("Vehicle is stuck! Try using the winch or differential lock.")
            .with_type(NotificationType::Warning)
            .with_title("Stuck")
    }
    
    pub fn low_fuel() -> Notification {
        Notification::new("Fuel level is critically low!")
            .with_type(NotificationType::Warning)
            .with_title("Low Fuel")
            .with_priority(NotificationPriority::High)
    }
    
    pub fn checkpoint_reached(checkpoint_name: &str) -> Notification {
        Notification::new(format!("Checkpoint reached: {}", checkpoint_name))
            .with_type(NotificationType::Success)
            .with_title("Checkpoint")
    }
    
    pub fn mission_complete(mission_name: &str) -> Notification {
        Notification::new(format!("Mission completed: {}", mission_name))
            .with_type(NotificationType::Success)
            .with_title("Mission Complete")
            .with_priority(NotificationPriority::High)
            .with_duration(10.0)
    }
    
    pub fn winch_attached() -> Notification {
        Notification::new("Winch attached")
            .with_type(NotificationType::Info)
            .with_duration(2.0)
    }
    
    pub fn winch_detached() -> Notification {
        Notification::new("Winch detached")
            .with_type(NotificationType::Info)
            .with_duration(2.0)
    }
    
    pub fn diff_lock_enabled() -> Notification {
        Notification::new("Differential lock engaged")
            .with_type(NotificationType::Info)
            .with_duration(2.0)
    }
    
    pub fn diff_lock_disabled() -> Notification {
        Notification::new("Differential lock disengaged")
            .with_type(NotificationType::Info)
            .with_duration(2.0)
    }
    
    pub fn four_wd_enabled() -> Notification {
        Notification::new("4WD engaged")
            .with_type(NotificationType::Info)
            .with_duration(2.0)
    }
    
    pub fn four_wd_disabled() -> Notification {
        Notification::new("4WD disengaged")
            .with_type(NotificationType::Info)
            .with_duration(2.0)
    }
}
