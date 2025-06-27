use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use std::fs::{ File, OpenOptions };
use std::io::{ BufWriter, Write, BufReader, BufRead };
use std::path::{ Path, PathBuf };
use lazy_static::lazy_static;
use log::{ debug, info, error };
use chrono::Utc;
use tauri::Manager;
// Stores logs for each mod
pub type LogStore = Arc<Mutex<HashMap<String, Vec<String>>>>;

// Config
const MAX_LOGS_IN_MEMORY: usize = 500;
lazy_static! {
  static ref APP_HANDLE: tauri::AppHandle = crate::app_handle
    ::get_global_app_handle()
    .expect("Failed to get global app handle");
}
lazy_static! {
  static ref LOGS_DIR: Result<PathBuf, tauri::Error> = {
    match APP_HANDLE.path().app_data_dir() {
      Ok(app_data_dir) => Ok(app_data_dir.join("mod_logs")),
      Err(e) => {
        error!("Failed to get app data directory: {}", e);
        Err(e)
      }
    }
  };
}

// Create a global singleton for storing logs
lazy_static! {
  pub static ref MOD_LOGS: LogStore = Arc::new(Mutex::new(HashMap::new()));
}

/// Ensure the logs directory exists for a specific mod
fn ensure_mod_logs_dir(mod_id: &str) -> std::io::Result<()> {
  let logs_path = match &*LOGS_DIR {
    Ok(path_buf) => path_buf.join(mod_id),
    Err(e) => {
      error!("Failed to resolve LOGS_DIR: {}", e);
      return Err(
        std::io::Error::new(std::io::ErrorKind::Other, "Invalid LOGS_DIR")
      );
    }
  };
  if !logs_path.exists() {
    std::fs::create_dir_all(logs_path)?;
  }
  Ok(())
}

/// Create a log file path for a specific mod with timestamp
fn get_log_file_path(mod_id: &str) -> String {
  let timestamp = Utc::now().format("%Y%m%d_%H%M%S_%3f").to_string();
  let logs_dir = match &*LOGS_DIR {
    Ok(path_buf) => path_buf.display().to_string(),
    Err(e) => {
      error!("Failed to resolve LOGS_DIR: {}", e);
      String::new()
    }
  };
  format!("{}/{}/{}.log", logs_dir, mod_id, timestamp)
}

/// Save logs to disk for a specific mod
fn save_logs_to_disk(
  mod_id: &str,
  logs_to_save: &[String]
) -> std::io::Result<()> {
  if let Err(e) = ensure_mod_logs_dir(mod_id) {
    error!("Failed to create logs directory for mod {}: {}", mod_id, e);
    return Err(e);
  }

  let file_path = get_log_file_path(mod_id);
  let file = OpenOptions::new().create(true).append(true).open(&file_path)?;

  let mut writer = BufWriter::new(file);

  for log in logs_to_save {
    writeln!(writer, "{}", log)?;
  }

  writer.flush()?;
  debug!(
    "Saved {} logs to disk for mod {} at {}",
    logs_to_save.len(),
    mod_id,
    file_path
  );
  Ok(())
}

/// Load all logs from disk for a specific mod
fn load_logs_from_disk(mod_id: &str) -> Vec<String> {
  let mod_logs_dir = match &*LOGS_DIR {
    Ok(path_buf) => path_buf.join(mod_id),
    Err(e) => {
      error!("Failed to resolve LOGS_DIR: {}", e);
      return Vec::new();
    }
  };

  if !mod_logs_dir.exists() {
    return Vec::new();
  }

  let mut all_logs = Vec::new();

  match std::fs::read_dir(&mod_logs_dir) {
    Ok(entries) => {
      let mut log_files: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
          entry
            .path()
            .extension()
            .map_or(false, |ext| ext == "log")
        })
        .collect();

      // Sort log files by filename (which includes timestamp) to maintain chronological order
      log_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

      let file_count = log_files.len();

      for entry in log_files {
        match File::open(entry.path()) {
          Ok(file) => {
            let reader = BufReader::new(file);
            let file_logs: Vec<String> = reader
              .lines()
              .filter_map(|line| line.ok())
              .collect();
            all_logs.extend(file_logs);
          }
          Err(e) => {
            error!("Failed to read log file {:?}: {}", entry.path(), e);
          }
        }
      }

      debug!(
        "Loaded {} logs from disk for mod {} from {} files",
        all_logs.len(),
        mod_id,
        file_count
      );
    }
    Err(e) => {
      error!("Failed to read logs directory for mod {}: {}", mod_id, e);
    }
  }

  all_logs
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

    // If the log count exceeds the memory limit, save older logs to disk
    if mod_logs.len() > MAX_LOGS_IN_MEMORY {
      debug!("Mod {} has too many log entries in memory, archiving oldest to disk", mod_id);

      // Calculate how many logs to move to disk
      let logs_to_archive = MAX_LOGS_IN_MEMORY / 2; // Only archive half of the logs at a time
      let logs_to_save: Vec<String> = mod_logs
        .drain(0..logs_to_archive)
        .collect();

      // Save the archived logs to disk
      if let Err(e) = save_logs_to_disk(mod_id, &logs_to_save) {
        error!("Failed to save logs to disk for mod {}: {}", mod_id, e);
        // If we can't save to disk, put the logs back to avoid data loss
        for (i, log) in logs_to_save.into_iter().enumerate() {
          mod_logs.insert(i, log);
        }
      }
    }

    if mod_logs.len() % 10 == 0 {
      debug!("Mod {} now has {} log entries in memory", mod_id, mod_logs.len());
    }
  }
}

/// Get all logs for a specific mod (both in-memory and from disk)
pub fn get_logs(mod_id: &str) -> Vec<String> {
  let logs = MOD_LOGS.lock().unwrap();

  // Get in-memory logs
  let memory_logs = logs.get(mod_id).cloned().unwrap_or_else(Vec::new);

  // Get logs from disk
  let disk_logs = load_logs_from_disk(mod_id);

  // Combine disk logs (older) with memory logs (newer)
  let mut all_logs = disk_logs.clone();
  all_logs.extend(memory_logs.clone());

  debug!(
    "Retrieved {} total logs for mod {} ({} from disk, {} from memory)",
    all_logs.len(),
    mod_id,
    disk_logs.len(),
    memory_logs.len()
  );

  all_logs
}

/// Get only the logs currently in memory for a specific mod
pub fn get_memory_logs(mod_id: &str) -> Vec<String> {
  let logs = MOD_LOGS.lock().unwrap();
  logs.get(mod_id).cloned().unwrap_or_else(Vec::new)
}

/// Clear logs for a specific mod (both in-memory and on disk)
pub fn clear_logs(mod_id: &str) {
  info!("Clearing all logs for mod {}", mod_id);
  let mut logs = MOD_LOGS.lock().unwrap();

  // Clear in-memory logs
  if logs.contains_key(mod_id) {
    logs.insert(mod_id.to_string(), Vec::new());
    debug!("In-memory logs cleared for mod {}", mod_id);
  }

  // Clear disk logs by deleting the mod's entire log directory
  let mod_logs_dir = match &*LOGS_DIR {
    Ok(path_buf) => path_buf.join(mod_id),
    Err(e) => {
      error!("Failed to resolve LOGS_DIR: {}", e);
      return;
    }
  };
  if mod_logs_dir.exists() {
    match std::fs::remove_dir_all(&mod_logs_dir) {
      Ok(_) => debug!("Disk log directory deleted for mod {}", mod_id),
      Err(e) =>
        error!("Failed to delete log directory for mod {}: {}", mod_id, e),
    }
  }
}

/// Clear logs for all mods (both in-memory and on disk)
pub fn clear_all_logs() {
  info!("Clearing all mod logs");
  let mut logs = MOD_LOGS.lock().unwrap();

  // Clear in-memory logs
  logs.clear();
  debug!("All in-memory mod logs cleared");

  // Clear disk logs by removing the entire logs directory
  let logs_path = match &*LOGS_DIR {
    Ok(path_buf) => Path::new(path_buf),
    Err(e) => {
      error!("Failed to resolve LOGS_DIR: {}", e);
      return;
    }
  };
  if logs_path.exists() {
    match std::fs::remove_dir_all(logs_path) {
      Ok(_) => {
        debug!("All disk log files deleted");
        // Recreate the main logs directory for future use
        if let Err(e) = std::fs::create_dir_all(logs_path) {
          error!("Failed to recreate logs directory: {}", e);
        }
      }
      Err(e) => error!("Failed to delete logs directory: {}", e),
    }
  }
}

/// Clear only in-memory logs for a specific mod (save them to disk first)
pub fn clear_memory_logs(mod_id: &str) {
  info!("Clearing in-memory logs for mod {} (saving to disk first)", mod_id);
  let mut logs = MOD_LOGS.lock().unwrap();

  if let Some(mod_logs) = logs.get(mod_id) {
    // Only save if there are logs in memory to save
    if !mod_logs.is_empty() {
      // Save all current in-memory logs to disk
      if let Err(e) = save_logs_to_disk(mod_id, mod_logs) {
        error!(
          "Failed to save in-memory logs to disk for mod {}: {}",
          mod_id,
          e
        );
        return; // Don't clear memory if we couldn't save to disk
      }

      debug!(
        "Saved {} in-memory logs to disk for mod {}",
        mod_logs.len(),
        mod_id
      );
    }

    // Clear the in-memory logs
    logs.insert(mod_id.to_string(), Vec::new());
    debug!("In-memory logs cleared for mod {}", mod_id);
  } else {
    debug!("No in-memory logs found for mod {}", mod_id);
  }
}

/// Clear in-memory logs for all mods (save them to disk first)
pub fn clear_all_memory_logs() {
  info!("Clearing all in-memory logs (saving to disk first)");
  let mut logs = MOD_LOGS.lock().unwrap();

  // Save all in-memory logs to disk before clearing
  for (mod_id, mod_logs) in logs.iter() {
    if !mod_logs.is_empty() {
      if let Err(e) = save_logs_to_disk(mod_id, mod_logs) {
        error!(
          "Failed to save in-memory logs to disk for mod {}: {}",
          mod_id,
          e
        );
      } else {
        debug!(
          "Saved {} in-memory logs to disk for mod {}",
          mod_logs.len(),
          mod_id
        );
      }
    }
  }

  // Clear all in-memory logs
  logs.clear();
  debug!("All in-memory logs cleared");
}
