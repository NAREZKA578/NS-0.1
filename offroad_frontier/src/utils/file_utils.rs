//! File utilities
//! 
//! Contains helper functions for file operations

use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, de::DeserializeOwned};

/// Get the game data directory
pub fn get_data_dir() -> PathBuf {
    // In production, this would use proper platform-specific paths
    // For now, use a relative path
    let mut path = std::env::current_dir().unwrap_or_default();
    path.push("game_data");
    path
}

/// Get the saves directory
pub fn get_saves_dir() -> PathBuf {
    let mut path = get_data_dir();
    path.push("saves");
    path
}

/// Get the config directory
pub fn get_config_dir() -> PathBuf {
    let mut path = get_data_dir();
    path.push("config");
    path
}

/// Ensure directories exist
pub fn ensure_directories() -> std::io::Result<()> {
    fs::create_dir_all(get_data_dir())?;
    fs::create_dir_all(get_saves_dir())?;
    fs::create_dir_all(get_config_dir())?;
    Ok(())
}

/// Save JSON data to file
pub fn save_json<T: Serialize>(path: &Path, data: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(data)?;
    fs::write(path, json)?;
    Ok(())
}

/// Load JSON data from file
pub fn load_json<T: DeserializeOwned>(path: &Path) -> Result<T, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let data: T = serde_json::from_str(&content)?;
    Ok(data)
}

/// Save binary data to file
pub fn save_binary<T: Serialize>(path: &Path, data: &T) -> Result<(), Box<dyn std::error::Error>> {
    let encoded = bincode::serialize(data)?;
    fs::write(path, encoded)?;
    Ok(())
}

/// Load binary data from file
pub fn load_binary<T: DeserializeOwned>(path: &Path) -> Result<T, Box<dyn std::error::Error>> {
    let content = fs::read(path)?;
    let data: T = bincode::deserialize(&content)?;
    Ok(data)
}

/// Check if file exists
pub fn file_exists(path: &Path) -> bool {
    path.exists()
}

/// Delete file
pub fn delete_file(path: &Path) -> std::io::Result<()> {
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

/// List files in directory with extension
pub fn list_files_with_extension(dir: &Path, extension: &str) -> Vec<PathBuf> {
    if !dir.exists() {
        return Vec::new();
    }
    
    fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .map(|ext| ext.to_string_lossy() == extension)
                .unwrap_or(false)
        })
        .map(|entry| entry.path())
        .collect()
}

/// Get file size in bytes
pub fn get_file_size(path: &Path) -> Option<u64> {
    fs::metadata(path).ok().map(|m| m.len())
}

/// Get file modified time
pub fn get_file_modified(path: &Path) -> Option<std::time::SystemTime> {
    fs::metadata(path).ok().and_then(|m| m.modified().ok())
}

/// Create backup of file
pub fn create_backup(path: &Path) -> std::io::Result<PathBuf> {
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        ));
    }
    
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let mut backup_path = path.to_path_buf();
    backup_path.set_extension(format!("bak.{}", timestamp));
    
    fs::copy(path, &backup_path)?;
    Ok(backup_path)
}

/// Read text file
pub fn read_text(path: &Path) -> std::io::Result<String> {
    fs::read_to_string(path)
}

/// Write text file
pub fn write_text(path: &Path, content: &str) -> std::io::Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}

/// Append to text file
pub fn append_text(path: &Path, content: &str) -> std::io::Result<()> {
    use std::io::Write;
    
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Hash file contents
pub fn hash_file(path: &Path) -> std::io::Result<String> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let content = fs::read(path)?;
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    Ok(format!("{:x}", hasher.finish()))
}

/// Log utility function
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        log::info!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        log::warn!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log::error!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        log::debug!($($arg)*);
    };
}
