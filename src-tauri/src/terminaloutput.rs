use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use std::fs::{ File, OpenOptions };
use std::io::{ BufWriter, Write, BufReader, BufRead };
use std::path::{ Path, PathBuf };
use lazy_static::lazy_static;
use log::{ debug, info, error, warn };
use chrono::Utc;
use tauri::Manager;
use crate::models::SessionInfo;
use uuid;

// Stores logs for each mod session
// mod_id -> session_id -> logs
pub type SessionLogStore = Arc<
  Mutex<HashMap<String, HashMap<String, Vec<String>>>>
>;

// Config
const MAX_LOGS_IN_MEMORY: usize = 500;
lazy_static! {
  static ref APP_HANDLE: tauri::AppHandle = crate::app_handle
    ::get_global_app_handle()
    .expect("Failed to get global app handle");
}
lazy_static! {
  pub static ref LOGS_DIR: Result<PathBuf, tauri::Error> = {
    match APP_HANDLE.path().app_data_dir() {
      Ok(app_data_dir) => Ok(app_data_dir.join("mod_logs")),
      Err(e) => {
        error!("Failed to get app data directory: {}", e);
        Err(e)
      }
    }
  };
}

// Create a global singleton for storing session-based logs
lazy_static! {
  pub static ref SESSION_LOGS: SessionLogStore = Arc::new(
    Mutex::new(HashMap::new())
  );
}

// Track active sessions for each mod
lazy_static! {
  pub static ref ACTIVE_SESSIONS: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// Sanitize mod name for use in filesystem paths
fn sanitize_mod_name_for_filesystem(mod_name: &str) -> String {
  let invalid_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
  let mut sanitized = mod_name
    .chars()
    .map(|c| if invalid_chars.contains(&c) || c.is_control() { '_' } else { c })
    .collect::<String>();

  sanitized = sanitized.trim().trim_matches('.').to_string();

  // Limit length to prevent filesystem issues
  if sanitized.len() > 100 {
    sanitized.truncate(100);
  }
  if sanitized.is_empty() {
    sanitized = "unnamed_mod".to_string();
  }

  sanitized
}

/// Generate a date-based session folder name
fn generate_session_folder_name(session_id: &str, start_time: i64) -> String {
  let datetime = chrono::DateTime::<Utc>
    ::from_timestamp(start_time, 0)
    .unwrap_or_else(|| Utc::now());

  // YYYY-MM-DD_HH-MM-SS_{first_8_chars_of_session_id}
  format!(
    "{}_{}",
    datetime.format("%Y-%m-%d_%H-%M-%S"),
    &session_id[..(8).min(session_id.len())]
  )
}

/// Check if a mod has save_terminal_output enabled
fn get_mod_save_terminal_output_setting(mod_id: &str) -> bool {
  // Check in global state first
  if let Ok(global_mods) = crate::models::GLOBAL_MODS_STATE.lock() {
    if let Some(mod_info) = global_mods.get(mod_id) {
      return mod_info.save_terminal_output;
    }
  }

  // If not found in global state, try to get app handle and check ModsState
  if let Some(app_handle) = crate::app_handle::get_global_app_handle() {
    if
      let Some(mods_state) = app_handle.try_state::<crate::models::ModsState>()
    {
      if let Ok(mods) = mods_state.0.lock() {
        if let Some(mod_info) = mods.get(mod_id) {
          return mod_info.save_terminal_output;
        }
      }
    }
  }

  // Default to false if we can't find the mod
  warn!("Could not find mod {} to check save_terminal_output setting, defaulting to false", mod_id);
  false
}

/// Ensure the logs directory exists for a specific mod session
fn ensure_session_logs_dir(
  mod_name: &str,
  session_id: &str,
  start_time: i64
) -> std::io::Result<()> {
  let sanitized_name = sanitize_mod_name_for_filesystem(mod_name);
  let session_folder = generate_session_folder_name(session_id, start_time);
  let logs_path = match &*LOGS_DIR {
    Ok(path_buf) => path_buf.join(&sanitized_name).join(&session_folder),
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

/// Create a log file path for a specific mod session
fn get_session_log_file_path(
  mod_name: &str,
  session_id: &str,
  start_time: i64
) -> String {
  let sanitized_name = sanitize_mod_name_for_filesystem(mod_name);
  let session_folder = generate_session_folder_name(session_id, start_time);
  let logs_dir = match &*LOGS_DIR {
    Ok(path_buf) => path_buf.display().to_string(),
    Err(e) => {
      error!("Failed to resolve LOGS_DIR: {}", e);
      String::new()
    }
  };
  format!("{}/{}/{}/output.log", logs_dir, sanitized_name, session_folder)
}

/// Create a session info file path for a specific mod session
fn get_session_info_file_path(
  mod_name: &str,
  session_id: &str,
  start_time: i64
) -> String {
  let sanitized_name = sanitize_mod_name_for_filesystem(mod_name);
  let session_folder = generate_session_folder_name(session_id, start_time);
  let logs_dir = match &*LOGS_DIR {
    Ok(path_buf) => path_buf.display().to_string(),
    Err(e) => {
      error!("Failed to resolve LOGS_DIR: {}", e);
      String::new()
    }
  };
  format!(
    "{}/{}/{}/session_info.json",
    logs_dir,
    sanitized_name,
    session_folder
  )
}

/// Start a new session for a mod
pub fn start_session(mod_id: &str, mod_name: &str) -> String {
  let session_id = uuid::Uuid::new_v4().to_string();
  let start_time = Utc::now().timestamp();

  info!(
    "Starting new session {} for mod {} ({})",
    session_id,
    mod_name,
    mod_id
  );

  // Create session info
  let session_info = SessionInfo {
    session_id: session_id.clone(),
    mod_id: mod_id.to_string(),
    mod_name: mod_name.to_string(),
    start_time,
    end_time: None,
    exit_code: None,
  };

  // Check if this mod wants to save terminal output
  let should_save_output = get_mod_save_terminal_output_setting(mod_id);

  // Only create session directory and save session info if saving is enabled
  if should_save_output {
    if let Err(e) = ensure_session_logs_dir(mod_name, &session_id, start_time) {
      error!("Failed to create session logs directory: {}", e);
    } else {
      // Save session info file
      let session_info_path = get_session_info_file_path(
        mod_name,
        &session_id,
        start_time
      );
      if let Ok(file) = File::create(&session_info_path) {
        if let Err(e) = serde_json::to_writer_pretty(file, &session_info) {
          error!("Failed to write session info file: {}", e);
        } else {
          debug!("Created session info file: {}", session_info_path);
        }
      }
    }
  } else {
    debug!("Skipping session directory creation for mod {} - save_terminal_output is disabled", mod_name);
  }

  // Initialize session logs in memory
  {
    let mut logs = SESSION_LOGS.lock().unwrap();
    logs
      .entry(mod_id.to_string())
      .or_insert_with(HashMap::new)
      .insert(session_id.clone(), Vec::new());
  }

  // Track active session
  {
    let mut active_sessions = ACTIVE_SESSIONS.lock().unwrap();
    active_sessions.insert(mod_id.to_string(), session_id.clone());
  }

  session_id
}

/// End a session for a mod
pub fn end_session(mod_id: &str, session_id: &str, exit_code: Option<i32>) {
  info!(
    "Ending session {} for mod {} with exit code {:?}",
    session_id,
    mod_id,
    exit_code
  );

  // Remove from active sessions
  {
    let mut active_sessions = ACTIVE_SESSIONS.lock().unwrap();
    active_sessions.remove(mod_id);
  }

  // Save any remaining in-memory logs to disk if mod wants to save output
  if get_mod_save_terminal_output_setting(mod_id) {
    let mut logs = SESSION_LOGS.lock().unwrap();
    if let Some(mod_sessions) = logs.get_mut(mod_id) {
      if let Some(session_logs) = mod_sessions.get(session_id) {
        if !session_logs.is_empty() {
          // Save logs to disk if there are any
          if !session_logs.is_empty() {
            if
              let Err(e) = save_session_logs_to_disk(
                mod_id,
                session_id,
                session_logs
              )
            {
              error!("Failed to save session logs to disk: {}", e);
            }
          }
        }
        // Remove from memory after saving
        mod_sessions.remove(session_id);
      }
    }
  } else {
    // If not saving output, just clear from memory without writing to disk
    let mut logs = SESSION_LOGS.lock().unwrap();
    if let Some(mod_sessions) = logs.get_mut(mod_id) {
      mod_sessions.remove(session_id);
    }
  }

  // Update session info file with end time and exit code (only if saving is enabled)
  if get_mod_save_terminal_output_setting(mod_id) {
    if
      let Ok(mut session_info) = load_session_info_from_disk(mod_id, session_id)
    {
      session_info.end_time = Some(Utc::now().timestamp());
      session_info.exit_code = exit_code;

      let session_info_path = get_session_info_file_path(
        &session_info.mod_name,
        session_id,
        session_info.start_time
      );
      if let Ok(file) = File::create(&session_info_path) {
        if let Err(e) = serde_json::to_writer_pretty(file, &session_info) {
          error!("Failed to update session info file: {}", e);
        }
      }
    }
  }
}

/// Get the active session ID for a mod
pub fn get_active_session(mod_id: &str) -> Option<String> {
  let active_sessions = ACTIVE_SESSIONS.lock().unwrap();
  active_sessions.get(mod_id).cloned()
}

/// Load session info from disk
fn load_session_info_from_disk(
  mod_id: &str,
  session_id: &str
) -> Result<SessionInfo, Box<dyn std::error::Error>> {
  // We need to find the session info file, but we need the mod name for the path
  // Try to find it by searching in the logs directory
  let logs_dir = match &*LOGS_DIR {
    Ok(path_buf) => path_buf,
    Err(e) => {
      return Err(format!("Failed to resolve LOGS_DIR: {}", e).into());
    }
  };

  // Search for session info file in all mod directories
  if let Ok(entries) = std::fs::read_dir(logs_dir) {
    for entry in entries.filter_map(|e| e.ok()) {
      if entry.path().is_dir() {
        // Look through all date-based session folders in this mod directory
        if let Ok(session_entries) = std::fs::read_dir(entry.path()) {
          for session_entry in session_entries.filter_map(|e| e.ok()) {
            if session_entry.path().is_dir() {
              let session_info_path = session_entry
                .path()
                .join("session_info.json");

              if session_info_path.exists() {
                if let Ok(file) = File::open(&session_info_path) {
                  if
                    let Ok(session_info) = serde_json::from_reader::<
                      _,
                      SessionInfo
                    >(file)
                  {
                    if
                      session_info.mod_id == mod_id &&
                      session_info.session_id == session_id
                    {
                      return Ok(session_info);
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }

  Err("Session info not found".into())
}

/// Save logs to disk for a specific mod session
/// IMPORTANT: This function requires that session_info.json already exists on disk
/// to ensure consistent folder naming based on the original session start_time
fn save_session_logs_to_disk(
  mod_id: &str,
  session_id: &str,
  logs_to_save: &[String]
) -> std::io::Result<()> {
  // Load session info to get start_time and mod_name for path generation
  let session_info = load_session_info_from_disk(mod_id, session_id).map_err(
    |e| {
      error!(
        "Could not load session info for mod {} session {}, cannot save logs - session info must exist first: {}",
        mod_id,
        session_id,
        e
      );
      std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Session info file not found - cannot determine correct log path"
      )
    }
  )?;

  let start_time = session_info.start_time;
  let mod_name = &session_info.mod_name;

  if let Err(e) = ensure_session_logs_dir(mod_name, session_id, start_time) {
    error!(
      "Failed to create session logs directory for mod {}: {}",
      mod_name,
      e
    );
    return Err(e);
  }

  let file_path = get_session_log_file_path(mod_name, session_id, start_time);
  let file = OpenOptions::new().create(true).append(true).open(&file_path)?;

  let mut writer = BufWriter::new(file);

  for log in logs_to_save {
    writeln!(writer, "{}", log)?;
  }

  writer.flush()?;
  debug!(
    "Saved {} logs to disk for mod {} session {} at {}",
    logs_to_save.len(),
    mod_name,
    session_id,
    file_path
  );
  Ok(())
}

/// Load all logs from disk for a specific session
fn load_session_logs_from_disk(
  mod_name: &str,
  session_id: &str
) -> Vec<String> {
  // Load session info to get start_time for path generation
  let start_time = if
    let Ok(session_info) = load_session_info_from_disk(mod_name, session_id)
  {
    session_info.start_time
  } else {
    // If we can't load session info, try to find the session folder by searching
    warn!(
      "Could not load session info for {}/{}, trying to find session folder",
      mod_name,
      session_id
    );
    return Vec::new(); // Return empty if we can't find session info
  };

  let session_log_file = get_session_log_file_path(
    mod_name,
    session_id,
    start_time
  );

  if !Path::new(&session_log_file).exists() {
    return Vec::new();
  }

  let mut all_logs = Vec::new();

  match File::open(&session_log_file) {
    Ok(file) => {
      let reader = BufReader::new(file);
      let file_logs: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect();
      all_logs.extend(file_logs);

      debug!(
        "Loaded {} logs from disk for session {} of mod {}",
        all_logs.len(),
        session_id,
        mod_name
      );
    }
    Err(e) => {
      error!("Failed to read session log file {}: {}", session_log_file, e);
    }
  }

  all_logs
}

/// Add a log entry to a specific mod's current session
pub fn add_log(mod_id: &str, log_entry: &str) {
  debug!("Adding log for mod {}: {}", mod_id, log_entry);

  // Get active session ID
  let session_id = match get_active_session(mod_id) {
    Some(id) => id,
    None => {
      warn!("No active session for mod {}, cannot add log", mod_id);
      return;
    }
  };

  add_session_log(mod_id, &session_id, log_entry);
}

/// Add a log entry to a specific mod session
pub fn add_session_log(mod_id: &str, session_id: &str, log_entry: &str) {
  debug!("Adding log for mod {} session {}: {}", mod_id, session_id, log_entry);
  let mut logs = SESSION_LOGS.lock().unwrap();

  // Ensure mod entry exists
  let mod_sessions = logs
    .entry(mod_id.to_string())
    .or_insert_with(HashMap::new);

  // Ensure session entry exists
  let session_logs = mod_sessions
    .entry(session_id.to_string())
    .or_insert_with(Vec::new);

  // Add the log entry
  session_logs.push(log_entry.to_string());

  // If the log count exceeds the memory limit AND mod wants to save output, save older logs to disk
  if
    session_logs.len() > MAX_LOGS_IN_MEMORY &&
    get_mod_save_terminal_output_setting(mod_id)
  {
    debug!(
      "Session {} for mod {} has too many log entries in memory, archiving oldest to disk",
      session_id,
      mod_id
    );

    // Calculate how many logs to move to disk
    let logs_to_archive = MAX_LOGS_IN_MEMORY / 2; // Only archive half of the logs at a time
    let logs_to_save: Vec<String> = session_logs
      .drain(0..logs_to_archive)
      .collect();

    // Save the archived logs to disk
    if let Ok(_) = load_session_info_from_disk(mod_id, session_id) {
      // Save the archived logs to disk
      if
        let Err(e) = save_session_logs_to_disk(
          mod_id,
          session_id,
          &logs_to_save
        )
      {
        error!("Failed to save session logs to disk for mod {}: {}", mod_id, e);
        // If we can't save to disk, put the logs back to avoid data loss
        for (i, log) in logs_to_save.into_iter().enumerate() {
          session_logs.insert(i, log);
        }
      }
    } else {
      // If we can't get session info, put the logs back
      for (i, log) in logs_to_save.into_iter().enumerate() {
        session_logs.insert(i, log);
      }
    }
  }

  if session_logs.len() % 10 == 0 {
    debug!(
      "Session {} for mod {} now has {} log entries in memory",
      session_id,
      mod_id,
      session_logs.len()
    );
  }
}

/// Get all logs for a specific mod's current session (both in-memory and from disk)
pub fn get_logs(mod_id: &str) -> Vec<String> {
  // Get active session
  let session_id = match get_active_session(mod_id) {
    Some(id) => id,
    None => {
      debug!("No active session for mod {}", mod_id);
      return Vec::new();
    }
  };

  get_session_logs(mod_id, &session_id)
}

/// Get all logs for a specific session (both in-memory and from disk)
pub fn get_session_logs(mod_id: &str, session_id: &str) -> Vec<String> {
  let logs = SESSION_LOGS.lock().unwrap();

  // Get in-memory logs
  let memory_logs = logs
    .get(mod_id)
    .and_then(|sessions| sessions.get(session_id))
    .cloned()
    .unwrap_or_else(Vec::new);

  // Get logs from disk - we need mod name for this
  let disk_logs = if
    let Ok(session_info) = load_session_info_from_disk(mod_id, session_id)
  {
    load_session_logs_from_disk(&session_info.mod_name, session_id)
  } else {
    Vec::new()
  };

  // Combine disk logs (older) with memory logs (newer)
  let mut all_logs = disk_logs.clone();
  all_logs.extend(memory_logs.clone());

  debug!(
    "Retrieved {} total logs for mod {} session {} ({} from disk, {} from memory)",
    all_logs.len(),
    mod_id,
    session_id,
    disk_logs.len(),
    memory_logs.len()
  );

  all_logs
}

/// Get only the logs currently in memory for the current session of a mod
pub fn get_memory_logs(mod_id: &str) -> Vec<String> {
  let session_id = match get_active_session(mod_id) {
    Some(id) => id,
    None => {
      return Vec::new();
    }
  };

  let logs = SESSION_LOGS.lock().unwrap();
  logs
    .get(mod_id)
    .and_then(|sessions| sessions.get(&session_id))
    .cloned()
    .unwrap_or_else(Vec::new)
}

/// Clear all sessions for a specific mod (both in-memory and on disk)
pub fn clear_logs(mod_id: &str) {
  info!("Clearing all sessions for mod {}", mod_id);

  // Clear in-memory logs
  {
    let mut logs = SESSION_LOGS.lock().unwrap();
    logs.remove(mod_id);
    debug!("In-memory session logs cleared for mod {}", mod_id);
  }

  // Clear active session tracking
  {
    let mut active_sessions = ACTIVE_SESSIONS.lock().unwrap();
    active_sessions.remove(mod_id);
  }

  // Clear disk logs by finding and deleting all mod directories
  let logs_dir = match &*LOGS_DIR {
    Ok(path_buf) => path_buf,
    Err(e) => {
      error!("Failed to resolve LOGS_DIR: {}", e);
      return;
    }
  };

  // Search for directories containing sessions for this mod
  if let Ok(entries) = std::fs::read_dir(logs_dir) {
    for entry in entries.filter_map(|e| e.ok()) {
      if entry.path().is_dir() {
        // Check each session directory for this mod
        if let Ok(session_entries) = std::fs::read_dir(entry.path()) {
          for session_entry in session_entries.filter_map(|e| e.ok()) {
            if session_entry.path().is_dir() {
              let session_info_path = session_entry
                .path()
                .join("session_info.json");
              if session_info_path.exists() {
                if let Ok(file) = File::open(&session_info_path) {
                  if
                    let Ok(session_info) = serde_json::from_reader::<
                      _,
                      SessionInfo
                    >(file)
                  {
                    if session_info.mod_id == mod_id {
                      // This session belongs to our mod, remove the entire session directory
                      if
                        let Err(e) = std::fs::remove_dir_all(
                          session_entry.path()
                        )
                      {
                        error!(
                          "Failed to delete session directory {:?}: {}",
                          session_entry.path(),
                          e
                        );
                      } else {
                        debug!(
                          "Deleted session directory for mod {}: {:?}",
                          mod_id,
                          session_entry.path()
                        );
                      }
                    }
                  }
                }
              }
            }
          }
        }

        // If the mod directory is now empty, remove it
        if let Ok(mut mod_entries) = std::fs::read_dir(entry.path()) {
          if mod_entries.next().is_none() {
            if let Err(e) = std::fs::remove_dir(entry.path()) {
              error!(
                "Failed to delete empty mod directory {:?}: {}",
                entry.path(),
                e
              );
            }
          }
        }
      }
    }
  }
}

/// Clear all logs for all mods (both in-memory and on disk)
pub fn clear_all_logs() {
  info!("Clearing all mod session logs");

  // Clear in-memory logs
  {
    let mut logs = SESSION_LOGS.lock().unwrap();
    logs.clear();
    debug!("All in-memory session logs cleared");
  }

  // Clear active sessions
  {
    let mut active_sessions = ACTIVE_SESSIONS.lock().unwrap();
    active_sessions.clear();
  }

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

/// Clear only in-memory logs for the current session (save them to disk first)
pub fn clear_memory_logs(mod_id: &str) {
  info!("Clearing in-memory logs for mod {} current session (saving to disk first)", mod_id);

  let session_id = match get_active_session(mod_id) {
    Some(id) => id,
    None => {
      debug!("No active session for mod {}", mod_id);
      return;
    }
  };

  let mut logs = SESSION_LOGS.lock().unwrap();

  if let Some(mod_sessions) = logs.get_mut(mod_id) {
    if let Some(session_logs) = mod_sessions.get(session_id.as_str()) {
      // Only save if there are logs in memory to save
      if !session_logs.is_empty() {
        // Save all current in-memory logs to disk
        if
          let Err(e) = save_session_logs_to_disk(
            mod_id,
            &session_id,
            session_logs
          )
        {
          error!(
            "Failed to save in-memory logs to disk for mod {} session {}: {}",
            mod_id,
            session_id,
            e
          );
          return; // Don't clear memory if we couldn't save to disk
        }

        debug!(
          "Saved {} in-memory logs to disk for mod {} session {}",
          session_logs.len(),
          mod_id,
          session_id
        );
      }

      // Clear the in-memory logs for this session
      mod_sessions.insert(session_id.clone(), Vec::new());
      debug!(
        "In-memory logs cleared for mod {} session {}",
        mod_id,
        session_id
      );
    } else {
      debug!(
        "No in-memory logs found for mod {} session {}",
        mod_id,
        session_id
      );
    }
  } else {
    debug!("No sessions found for mod {}", mod_id);
  }
}

/// Clear in-memory logs for all active sessions (save them to disk first)
pub fn clear_all_memory_logs() {
  info!("Clearing all in-memory session logs (saving to disk first)");
  let mut logs = SESSION_LOGS.lock().unwrap();

  // Save all in-memory logs to disk before clearing
  for (mod_id, mod_sessions) in logs.iter() {
    for (session_id, session_logs) in mod_sessions.iter() {
      if !session_logs.is_empty() {
        // Save logs to disk
        if
          let Err(e) = save_session_logs_to_disk(
            mod_id,
            session_id,
            session_logs
          )
        {
          error!(
            "Failed to save in-memory logs to disk for mod {} session {}: {}",
            mod_id,
            session_id,
            e
          );
        } else {
          debug!(
            "Saved {} in-memory logs to disk for mod {} session {}",
            session_logs.len(),
            mod_id,
            session_id
          );
        }
      }
    }
  }

  // Clear all in-memory logs
  logs.clear();
  debug!("All in-memory session logs cleared");
}

/// Get the logs folder path for a specific mod
pub fn get_mod_logs_folder_path(mod_id: &str) -> Result<String, String> {
  // Try to get mod info to get the mod name
  let mod_name = if
    let Ok(global_mods) = crate::models::GLOBAL_MODS_STATE.lock()
  {
    if let Some(mod_info) = global_mods.get(mod_id) {
      mod_info.name.clone()
    } else {
      return Err(format!("Mod with ID {} not found", mod_id));
    }
  } else if let Some(app_handle) = crate::app_handle::get_global_app_handle() {
    if
      let Some(mods_state) = app_handle.try_state::<crate::models::ModsState>()
    {
      if let Ok(mods) = mods_state.0.lock() {
        if let Some(mod_info) = mods.get(mod_id) {
          mod_info.name.clone()
        } else {
          return Err(format!("Mod with ID {} not found", mod_id));
        }
      } else {
        return Err("Failed to access mods state".to_string());
      }
    } else {
      return Err("Failed to get mods state".to_string());
    }
  } else {
    return Err("Failed to access mod information".to_string());
  };

  let sanitized_name = sanitize_mod_name_for_filesystem(&mod_name);
  let logs_dir = match &*LOGS_DIR {
    Ok(path_buf) => path_buf.join(&sanitized_name),
    Err(e) => {
      return Err(format!("Failed to resolve logs directory: {}", e));
    }
  };

  Ok(logs_dir.display().to_string())
}
