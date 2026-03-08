//! Menu system
//! 
//! Main menu, pause menu, and settings

use crate::core::config::GameConfig;

/// Main menu state
pub struct MainMenu {
    /// Is menu visible?
    pub visible: bool,
    
    /// Current menu screen
    pub current_screen: MenuScreen,
    
    /// Selected menu item
    pub selected_item: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuScreen {
    Main,
    Play,
    Settings,
    Controls,
    LoadGame,
    SaveGame,
    Credits,
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            visible: true,
            current_screen: MenuScreen::Main,
            selected_item: 0,
        }
    }
}

impl MainMenu {
    /// Create new main menu
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Show menu
    pub fn show(&mut self) {
        self.visible = true;
    }
    
    /// Hide menu
    pub fn hide(&mut self) {
        self.visible = false;
    }
    
    /// Toggle menu visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
    
    /// Navigate to a screen
    pub fn navigate_to(&mut self, screen: MenuScreen) {
        self.current_screen = screen;
        self.selected_item = 0;
    }
    
    /// Go back to previous screen
    pub fn go_back(&mut self) {
        self.current_screen = MenuScreen::Main;
        self.selected_item = 0;
    }
    
    /// Move selection up
    pub fn select_up(&mut self) {
        if self.selected_item > 0 {
            self.selected_item -= 1;
        }
    }
    
    /// Move selection down
    pub fn select_down(&mut self) {
        self.selected_item += 1;
        // Will be bounded by actual menu item count
    }
    
    /// Confirm selection
    pub fn confirm(&mut self) -> MenuAction {
        match self.current_screen {
            MenuScreen::Main => self.handle_main_menu_selection(),
            MenuScreen::Play => MenuAction::StartGame,
            MenuScreen::Settings => MenuAction::None,
            MenuScreen::Controls => MenuAction::None,
            MenuScreen::LoadGame => MenuAction::None,
            MenuScreen::SaveGame => MenuAction::None,
            MenuScreen::Credits => MenuAction::None,
        }
    }
    
    fn handle_main_menu_selection(&mut self) -> MenuAction {
        match self.selected_item {
            0 => MenuAction::NavigateTo(MenuScreen::Play),
            1 => MenuAction::NavigateTo(MenuScreen::Settings),
            2 => MenuAction::NavigateTo(MenuScreen::Controls),
            3 => MenuAction::NavigateTo(MenuScreen::LoadGame),
            4 => MenuAction::Quit,
            _ => MenuAction::None,
        }
    }
    
    /// Get menu items for current screen
    pub fn get_items(&self) -> Vec<&str> {
        match self.current_screen {
            MenuScreen::Main => vec![
                "Play",
                "Settings",
                "Controls",
                "Load Game",
                "Quit",
            ],
            MenuScreen::Play => vec![
                "New Game",
                "Quick Drive",
                "Back",
            ],
            MenuScreen::Settings => vec![
                "Graphics",
                "Audio",
                "Gameplay",
                "Back",
            ],
            MenuScreen::Controls => vec![
                "Keyboard/Mouse",
                "Gamepad",
                "Reset to Default",
                "Back",
            ],
            MenuScreen::LoadGame => vec![], // Will be populated with save files
            MenuScreen::SaveGame => vec![],
            MenuScreen::Credits => vec!["Back"],
        }
    }
}

/// Menu action result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuAction {
    None,
    StartGame,
    LoadGame(String),
    SaveGame(String),
    NavigateTo(MenuScreen),
    ApplySettings,
    Quit,
}

/// Pause menu
pub struct PauseMenu {
    /// Is pause menu visible?
    pub visible: bool,
    
    /// Selected item
    pub selected_item: usize,
}

impl Default for PauseMenu {
    fn default() -> Self {
        Self {
            visible: false,
            selected_item: 0,
        }
    }
}

impl PauseMenu {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn show(&mut self) {
        self.visible = true;
    }
    
    pub fn hide(&mut self) {
        self.visible = false;
    }
    
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
    
    pub fn get_items(&self) -> Vec<&str> {
        vec![
            "Resume",
            "Settings",
            "Save Game",
            "Load Game",
            "Main Menu",
            "Quit",
        ]
    }
}
