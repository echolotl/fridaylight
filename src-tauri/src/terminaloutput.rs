use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use lazy_static::lazy_static;
use log::{ debug, info };

// Stores logs for each mod
pub type LogStore = Arc<Mutex<HashMap<String, Vec<String>>>>;

// Create a global singleton for storing logs
lazy_static! {
    pub static ref MOD_LOGS: LogStore = Arc::new(Mutex::new(HashMap::new()));
}

/// Add a log entry to a specific mod's logs
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

/// Get all logs for a specific mod
pub fn get_logs(mod_id: &str) -> Vec<String> {
    let logs = MOD_LOGS.lock().unwrap();

    // Return a copy of the logs for this mod, or empty vector if not found
    let result = logs.get(mod_id).cloned().unwrap_or_else(Vec::new);
    result
}

/// Clear logs for a specific mod
pub fn clear_logs(mod_id: &str) {
    info!("Clearing logs for mod {}", mod_id);
    let mut logs = MOD_LOGS.lock().unwrap();

    if logs.contains_key(mod_id) {
        logs.insert(mod_id.to_string(), Vec::new());
        debug!("Logs cleared for mod {}", mod_id);
    } else {
        debug!("No logs found to clear for mod {}", mod_id);
    }
}

/// Clear logs for all mods
pub fn clear_all_logs() {
    info!("Clearing all mod logs");
    let mut logs = MOD_LOGS.lock().unwrap();
    logs.clear();
    debug!("All mod logs cleared");
}
