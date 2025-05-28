use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use lazy_static::lazy_static;
use log::{ debug, info };

// Stores logs for each mod
pub type LogStore = Arc<Mutex<HashMap<String, Vec<String>>>>;

// Create a global singleton for storing logs
#[cfg(feature = "terminal-output")]
lazy_static! {
    pub static ref MOD_LOGS: LogStore = Arc::new(Mutex::new(HashMap::new()));
}

/// Add a log entry to a specific mod's logs
#[cfg(feature = "terminal-output")]
pub fn add_log(mod_id: &str, log_entry: &str) {
    debug!("Adding log for mod {}: {}", mod_id, log_entry);
    let mut logs = MOD_LOGS.lock().unwrap();

    // If the mod_id doesn't exist in the map, create a new entry
    if !logs.contains_key(mod_id) {
        debug!("Creating new log entry for mod {}", mod_id);
        logs.insert(mod_id.to_string(), Vec::new());
    }

    // Get the log vector for this mod and push the new entry
    if let Some(mod_logs) = logs.get_mut(mod_id) {
        mod_logs.push(log_entry.to_string());

        // Keep the log size manageable (max 500 entries)
        if mod_logs.len() > 500 {
            debug!("Truncating logs for mod {} (size: {})", mod_id, mod_logs.len());
            *mod_logs = mod_logs.split_off(mod_logs.len() - 500);
        }

        if mod_logs.len() % 10 == 0 {
            debug!("Mod {} now has {} log entries", mod_id, mod_logs.len());
        }
    }
}

/// Add a log entry to a specific mod's logs (no-op when feature disabled)
#[cfg(not(feature = "terminal-output"))]
pub fn add_log(_mod_id: &str, _log_entry: &str) {
    // No-op when terminal output feature is disabled
}

/// Get all logs for a specific mod
#[cfg(feature = "terminal-output")]
pub fn get_logs(mod_id: &str) -> Vec<String> {
    let logs = MOD_LOGS.lock().unwrap();

    // Return a copy of the logs for this mod, or empty vector if not found
    let result = logs.get(mod_id).cloned().unwrap_or_else(Vec::new);    result
}

/// Get all logs for a specific mod (returns empty vector when feature disabled)
#[cfg(not(feature = "terminal-output"))]
pub fn get_logs(_mod_id: &str) -> Vec<String> {
    Vec::new()
}

/// Clear logs for a specific mod
#[cfg(feature = "terminal-output")]
pub fn clear_logs(mod_id: &str) {
    info!("Clearing logs for mod {}", mod_id);
    let mut logs = MOD_LOGS.lock().unwrap();

    if logs.contains_key(mod_id) {
        logs.insert(mod_id.to_string(), Vec::new());
        debug!("Logs cleared for mod {}", mod_id);
    } else {
        debug!("No logs found to clear for mod {}", mod_id);    }
}

/// Clear logs for a specific mod (no-op when feature disabled)
#[cfg(not(feature = "terminal-output"))]
pub fn clear_logs(_mod_id: &str) {
    // No-op when terminal output feature is disabled
}

/// Clear logs for all mods
#[cfg(feature = "terminal-output")]
pub fn clear_all_logs() {
    info!("Clearing all mod logs");
    let mut logs = MOD_LOGS.lock().unwrap();    logs.clear();
    debug!("All mod logs cleared");
}

/// Clear logs for all mods (no-op when feature disabled)
#[cfg(not(feature = "terminal-output"))]
pub fn clear_all_logs() {
    // No-op when terminal output feature is disabled
}
