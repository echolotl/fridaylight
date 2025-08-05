use crate::download::download_gamebanana_mod;
use crate::filesystem::create_mod_info;
use crate::gamebanana::{
  fetch_gamebanana_mods,
  fetch_featured_mods,
  get_mod_download_files,
  get_mod_info,
};
use crate::logger;
use crate::modutils::{
  find_mod_metadata_files,
  get_executable_directory,
  toggle_mod_enabled_state,
  check_mod_enabled_state,
};
use crate::models::{
  EngineModsResponse,
  GBDownloadPage,
  GBFile,
  GBModPosts,
  GBModUpdates,
  GBProfilePage,
  GBSubfeed,
  GBTopSubs,
  ModDisableResult,
  ModInfo,
  ModsState,
};
use log::{ debug, error, info, warn };
use serde_json;
use tauri::window::{ Effect, EffectsBuilder };
use tauri_plugin_sql::{ Migration, MigrationKind };
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use tauri::{ Manager, State, Emitter };
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_deep_link::DeepLinkExt;
use tauri_plugin_updater::UpdaterExt;

// Command to open a folder dialog and get the selected folder path
#[tauri::command]
pub async fn select_mod_folder(
  app: tauri::AppHandle
) -> Result<String, String> {
  debug!("Opening folder selection dialog");
  let folder = app
    .dialog()
    .file()
    .set_title("Select Mod Folder")
    .blocking_pick_folder();

  match folder {
    Some(path) => {
      info!("Mod folder selected: {}", path);
      Ok(path.to_string())
    }
    None => {
      warn!("No mod folder selected");
      Err("No folder selected".to_string())
    }
  }
}

// Command to open a folder dialog and select a folder containing multiple mods
#[tauri::command]
pub async fn select_mods_parent_folder(
  app: tauri::AppHandle,
  validate: Option<bool>,
  mods_state: State<'_, ModsState>
) -> Result<Vec<ModInfo>, String> {
  debug!("Opening parent folder selection dialog for multiple mods");
  let folder = app
    .dialog()
    .file()
    .set_title("Select Folder Containing Mods")
    .blocking_pick_folder();

  match folder {
    Some(path) => {
      info!("Parent folder selected: {}", path);
      let path_str = path.to_string();

      // Get validation setting (default to true if not specified)
      let should_validate = validate.unwrap_or(true);

      // Read all subdirectories and add each as a mod
      match std::fs::read_dir(&path_str) {
        Ok(entries) => {
          let mut added_mods = Vec::new();
          let mut invalid_mods = 0;

          for entry in entries.filter_map(|e| e.ok()) {
            let entry_path = entry.path();

            // Only process directories
            if entry_path.is_dir() {
              let subdir_path = entry_path.to_string_lossy().to_string();
              debug!("Processing potential mod directory: {}", subdir_path);

              // Check if it's a valid FNF mod if validation is enabled
              let is_valid =
                !should_validate ||
                crate::filesystem::is_valid_fnf_mod(&entry_path);

              if is_valid {
                // Add the mod
                match add_mod(subdir_path, Some(false), mods_state.clone()) {
                  Ok(mod_info) => {
                    added_mods.push(mod_info);
                    info!("Added mod: {}", entry_path.display());
                  }
                  Err(e) => {
                    warn!(
                      "Failed to add directory as mod: {} - {}",
                      entry_path.display(),
                      e
                    );
                    // Continue to next directory
                  }
                }
              } else {
                debug!("Skipping invalid FNF mod: {}", subdir_path);
                invalid_mods += 1;
              }
            }
          }

          info!(
            "Added {} mods from parent folder (skipped {} invalid mods)",
            added_mods.len(),
            invalid_mods
          );

          if added_mods.is_empty() && invalid_mods > 0 {
            return Err(
              format!("No valid FNF mods found. Skipped {} invalid folders that didn't contain required FNF mod structure.", invalid_mods)
            );
          }

          Ok(added_mods)
        }
        Err(e) => {
          let err_msg = format!("Failed to read directory contents: {}", e);
          error!("{}", err_msg);
          Err(err_msg)
        }
      }
    }
    None => {
      warn!("No folder selected");
      Err("No folder selected".to_string())
    }
  }
}

// Command to open a folder dialog and get the selected settings folder path
#[tauri::command]
pub async fn select_settings_folder(
  app: tauri::AppHandle
) -> Result<String, String> {
  debug!("Opening settings folder selection dialog");
  let folder = app
    .dialog()
    .file()
    .set_title("Select Default Install Location")
    .blocking_pick_folder();

  match folder {
    Some(path) => {
      info!("Settings folder selected: {}", path);
      Ok(path.to_string())
    }
    None => {
      warn!("No settings folder selected");
      Err("No folder selected".to_string())
    }
  }
}

// Command to open a file dialog and select an executable
#[tauri::command]
pub async fn select_executable(
  app: tauri::AppHandle
) -> Result<String, String> {
  debug!("Opening executable selection dialog");
  let file = app
    .dialog()
    .file()
    .set_title("Select Executable")
    .add_filter("Executable", &["exe"])
    .blocking_pick_file();

  match file {
    Some(path) => {
      info!("Executable selected: {}", path);
      Ok(path.to_string())
    }
    None => {
      warn!("No executable selected");
      Err("No file selected".to_string())
    }
  }
}

// Command to add a mod to our list
#[tauri::command]
pub fn add_mod(
  path: String,
  validate: Option<bool>,
  mods_state: State<'_, ModsState>
) -> Result<ModInfo, String> {
  info!("Adding mod from path: {}", path);

  // Check if the folder is a valid FNF mod if validation is enabled
  // Default to true if not specified (should be specified, but just in case)
  let should_validate = validate.unwrap_or(true);

  if should_validate {
    let path_obj = Path::new(&path);
    if !crate::filesystem::is_valid_fnf_mod(path_obj) {
      let err_msg =
        format!("Invalid FNF mod: '{}' doesn't contain required folders (assets and manifest)", path);
      warn!("{}", err_msg);
      return Err(err_msg);
    }
  }

  // Create ModInfo from path
  let mod_info = create_mod_info(&path)?;
  let id = mod_info.id.clone();

  // Check if the mod's path already exists in the state
  let mut mods = mods_state.0.lock().unwrap();
  if mods.values().any(|existing_mod| existing_mod.path == path) {
    let err_msg = format!("Mod with path '{}' already exists", path);
    warn!("{}", err_msg);
    return Err(err_msg);
  }

  // Add to our state
  mods.insert(id, mod_info.clone());
  info!("Mod added successfully: {} ({})", mod_info.name, mod_info.id);

  Ok(mod_info)
}

// Command to get all mods
#[tauri::command]
pub fn get_mods(mods_state: State<'_, ModsState>) -> Vec<ModInfo> {
  let mods = mods_state.0.lock().unwrap();
  mods.values().cloned().collect()
}

// Command to launch a mod
#[tauri::command]
pub fn launch_mod(
  id: String,
  mods_state: State<'_, ModsState>
) -> Result<(), String> {
  info!("Attempting to launch mod with ID: {}", id);
  let mut mods = mods_state.0.lock().unwrap();
  let mod_name: String;

  // Get required info from the mod
  let executable_path = match mods.get(&id) {
    Some(mod_info) => {
      // Check if the mod is already running
      if let Some(pid) = mod_info.process_id {
        warn!("Mod is already running with PID: {}", pid);
        return Err(format!("Mod is already running with PID: {}", pid));
      }

      mod_name = mod_info.name.clone();

      if let Some(exe_path) = &mod_info.executable_path {
        exe_path.clone()
      } else {
        let err_msg = format!("No executable found for mod: {}", mod_info.name);
        warn!("{}", err_msg);
        crate::terminaloutput::add_log(&id, &format!("[ERROR] {}", err_msg));
        return Err(err_msg);
      }
    }
    None => {
      let err_msg = format!("Mod not found with ID: {}", id);
      warn!("{}", err_msg);
      return Err("Mod not found".to_string());
    }
  };

  debug!("Launching executable: {}", executable_path);

  // Get the executable's directory to use as the working directory
  let exe_path_obj = Path::new(&executable_path);
  let working_dir = exe_path_obj
    .parent()
    .map(|p| p.to_path_buf())
    .ok_or_else(|| {
      let err = "Could not determine executable's directory".to_string();
      error!("{}", err);
      err
    })?;

  debug!("Using working directory: {}", working_dir.display());

  // Start a new session for this mod
  let session_id = crate::terminaloutput::start_session(&id, &mod_name);
  info!("Started session {} for mod {}", session_id, mod_name);

  // Add initial log entry showing the startup
  crate::terminaloutput::add_log(
    &id,
    &format!("Starting mod: {} ({})", mod_name, executable_path)
  );
  crate::terminaloutput::add_log(
    &id,
    &format!("Working directory: {}", working_dir.display())
  );
  crate::terminaloutput::add_log(&id, &format!("Session ID: {}", session_id));

  // Launch the executable with output capture
  match
    Command::new(&executable_path)
      .current_dir(&working_dir) // Set the working directory to the mod's directory
      .stdout(std::process::Stdio::piped()) // Capture stdout
      .stderr(std::process::Stdio::piped()) // Capture stderr
      .spawn()
  {
    Ok(mut child) => {
      let pid = child.id();
      info!("Successfully launched: {} with PID: {}", executable_path, pid);
      // Store the process ID and session info in the ModInfo and update last_played timestamp
      if let Some(mod_info) = mods.get_mut(&id) {
        mod_info.process_id = Some(pid);
        mod_info.current_session_id = Some(session_id.clone());
        mod_info.session_start_time = Some(chrono::Utc::now().timestamp());

        // Update last_played timestamp
        let current_time = std::time::SystemTime
          ::now()
          .duration_since(std::time::UNIX_EPOCH)
          .map(|d| d.as_secs() as i64)
          .unwrap_or(0);
        mod_info.last_played = Some(current_time);

        info!(
          "Updated last_played timestamp for mod {} to {}",
          mod_info.name,
          current_time
        );

        // Also update the global state
        match crate::models::GLOBAL_MODS_STATE.lock() {
          Ok(mut global_mods) => {
            if let Some(global_mod_info) = global_mods.get_mut(&id) {
              global_mod_info.process_id = Some(pid);
              global_mod_info.current_session_id = Some(session_id.clone());
              global_mod_info.session_start_time = Some(
                chrono::Utc::now().timestamp()
              );
              global_mod_info.last_played = Some(current_time);
              info!(
                "Updated global state for mod {}: pid={}, session_id={}, last_played={}",
                mod_info.name,
                pid,
                session_id,
                current_time
              );
            } else {
              warn!(
                "Mod {} not found in global state when updating process info",
                mod_info.name
              );
            }
          }
          _ => {
            warn!(
              "Failed to acquire lock on global state when updating process info"
            );
          }
        }
      }

      // Clone the id for use in threads
      let id_clone = id.clone();
      let id_clone2 = id.clone();

      // Set up stdout and stderr capturing in background threads
      if let Some(stdout) = child.stdout.take() {
        std::thread::spawn(move || {
          use std::io::{ BufRead, BufReader };
          let reader = BufReader::new(stdout);
          for line in reader.lines() {
            if let Ok(line) = line {
              crate::terminaloutput::add_log(&id_clone, &line);
            }
          }
        });
      }

      if let Some(stderr) = child.stderr.take() {
        std::thread::spawn(move || {
          use std::io::{ BufRead, BufReader };
          let reader = BufReader::new(stderr);
          for line in reader.lines() {
            if let Ok(line) = line {
              crate::terminaloutput::add_log(
                &id_clone2,
                &format!("[ERROR] {}", line)
              );
            }
          }
        });
      }

      // Monitor the process in a background thread to update state when it exits
      let id_for_thread = id.to_string();
      let session_id_for_thread = session_id.clone();
      std::thread::spawn(move || {
        match child.wait() {
          Ok(status) => {
            let exit_code = status.code();
            let exit_message =
              format!("Process exited with status: {}", status);
            info!("{}", exit_message);
            crate::terminaloutput::add_log(&id_for_thread, &exit_message);

            // End the session with the exit code
            crate::terminaloutput::end_session(
              &id_for_thread,
              &session_id_for_thread,
              exit_code
            );

            // The process has exited, so we need to update the mod's running state
            // We do this by directly invoking the set_mod_not_running function
            // which will handle the state update in a thread-safe way
            crate::modutils::set_mod_not_running(&id_for_thread);
          }
          Err(e) => {
            let error_msg = format!("Failed to wait for process: {}", e);
            error!("{}", error_msg);
            crate::terminaloutput::add_log(
              &id_for_thread,
              &format!("[ERROR] {}", error_msg)
            );

            // End the session even on error
            crate::terminaloutput::end_session(
              &id_for_thread,
              &session_id_for_thread,
              None
            );
          }
        }
      });

      Ok(())
    }
    Err(e) => {
      let error_msg = format!("Failed to launch executable: {}", e);
      error!("{}", error_msg);
      crate::terminaloutput::add_log(&id, &format!("[ERROR] {}", error_msg));
      Err(error_msg)
    }
  }
}

// Command to check if a mod is running
#[tauri::command]
pub fn is_mod_running(id: String, mods_state: State<'_, ModsState>) -> bool {
  // First check in the Tauri state
  let running_in_tauri_state = {
    let mods = mods_state.0.lock().unwrap();

    if let Some(mod_info) = mods.get(&id) {
      let running = mod_info.process_id.is_some();
      info!(
        "Mod {} is running in Tauri state: {} (pid: {:?})",
        mod_info.name,
        running,
        mod_info.process_id
      );
      running
    } else {
      info!("Mod with ID {} not found in Tauri state", id);
      false
    }
  };

  // Also check in the global state for debugging
  let running_in_global_state = {
    match crate::models::GLOBAL_MODS_STATE.lock() {
      Ok(global_mods) => {
        if let Some(mod_info) = global_mods.get(&id) {
          let running = mod_info.process_id.is_some();
          info!(
            "Mod {} is running in global state: {} (pid: {:?})",
            mod_info.name,
            running,
            mod_info.process_id
          );
          running
        } else {
          info!("Mod with ID {} not found in global state", id);
          false
        }
      }
      _ => {
        info!("Could not acquire lock on global state");
        false
      }
    }
  };

  // If there's a discrepancy between the states, log it
  if running_in_tauri_state != running_in_global_state {
    warn!(
      "State inconsistency detected! Tauri state: {}, Global state: {}",
      running_in_tauri_state,
      running_in_global_state
    );
  }

  // Return the Tauri state value (as before)
  running_in_tauri_state
}

// Command to stop a running mod
#[tauri::command]
pub fn stop_mod(
  id: String,
  mods_state: State<'_, ModsState>
) -> Result<(), String> {
  info!("Attempting to stop mod with ID: {}", id);
  let mut mods = mods_state.0.lock().unwrap();

  if let Some(mod_info) = mods.get_mut(&id) {
    if let Some(pid) = mod_info.process_id {
      info!("Stopping mod: {} with PID: {}", mod_info.name, pid);

      #[cfg(target_os = "windows")]
      {
        use std::process::Command;
        // On Windows, use taskkill to terminate the process
        match
          Command::new("taskkill")
            .args(&["/PID", &pid.to_string(), "/F"])
            .output()
        {
          Ok(_) => {
            info!("Successfully stopped process with PID: {}", pid);
            mod_info.process_id = None;

            // Also update the global state
            match crate::models::GLOBAL_MODS_STATE.lock() {
              Ok(mut global_mods) => {
                if let Some(global_mod_info) = global_mods.get_mut(&id) {
                  global_mod_info.process_id = None;
                  info!(
                    "Updated global state for mod {}: pid=None",
                    mod_info.name
                  );
                } else {
                  warn!(
                    "Mod {} not found in global state when stopping",
                    mod_info.name
                  );
                }
              }
              _ => {
                warn!(
                  "Failed to acquire lock on global state when stopping mod"
                );
              }
            }

            Ok(())
          }
          Err(e) => {
            let error_msg = format!("Failed to stop process: {}", e);
            error!("{}", error_msg);
            Err(error_msg)
          }
        }
      }
    } else {
      let msg = format!("Mod {} is not running", mod_info.name);
      warn!("{}", msg);
      Err(msg)
    }
  } else {
    let err_msg = format!("Mod not found with ID: {}", id);
    warn!("{}", err_msg);
    Err(err_msg)
  }
}

// Command to fetch mods from GameBanana
#[tauri::command]
pub async fn fetch_gamebanana_mods_command(
  query: String,
  page: i64
) -> Result<GBSubfeed, String> {
  fetch_gamebanana_mods(query, page).await
}

#[tauri::command]
pub async fn get_featured_mods_command() -> Result<GBTopSubs, String> {
  fetch_featured_mods().await
}

#[tauri::command]
pub async fn get_mod_info_command(
  mod_id: i64,
  model_type: Option<String>
) -> Result<GBProfilePage, String> {
  // Default to "Mod" if not specified
  let model_type = model_type.unwrap_or_else(|| "Mod".to_string());
  get_mod_info(mod_id, &model_type).await
}

// Command to get mod posts
#[tauri::command]
pub async fn get_mod_posts_command(
  mod_id: i64,
  page: i64,
  model_type: Option<String>
) -> Result<GBModPosts, String> {
  // Default to "Mod" if not specified
  let model_type = model_type.unwrap_or_else(|| "Mod".to_string());
  crate::gamebanana::get_mod_posts(mod_id, page, &model_type).await
}

#[tauri::command]
pub async fn get_mod_updates_command(
  mod_id: i64,
  page: i64,
  model_type: Option<String>
) -> Result<GBModUpdates, String> {
  // Default to "Mod" if not specified
  let model_type = model_type.unwrap_or_else(|| "Mod".to_string());
  crate::gamebanana::get_mod_updates(mod_id, page, &model_type).await
}

// Command to download a mod from GameBanana
#[tauri::command]
pub async fn download_gamebanana_mod_command(
  file: GBFile,
  info: GBProfilePage,
  install_location: Option<String>,
  folder_name: Option<String>,
  update_existing: Option<bool>,
  app: tauri::AppHandle
) -> Result<ModInfo, String> {
  download_gamebanana_mod(
    file,
    info,
    install_location,
    folder_name,
    update_existing,
    app
  ).await
}

#[tauri::command]
pub async fn download_engine_command(
  engine_type: String,
  install_location: Option<String>,
  custom_name: Option<String>,
  update_existing: Option<bool>,
  download_id: i64,
  app: tauri::AppHandle
) -> Result<ModInfo, String> {
  info!("Starting direct download process for {} engine", engine_type);
  crate::download::download_engine(
    engine_type,
    install_location,
    custom_name,
    update_existing,
    download_id,
    app
  ).await
}

// Command to sync/update mods from database
#[tauri::command]
pub async fn sync_mods_from_database(
  mods_data: Vec<ModInfo>,
  mods_state: State<'_, ModsState>
) -> Result<(), String> {
  info!("Syncing {} mods from database to ModsState", mods_data.len());

  let mut mods = mods_state.0.lock().unwrap();
  let mut updated_count = 0;
  let mut added_count = 0;

  // For each mod in the database, check if it exists in the state
  for mut new_mod_info in mods_data {
    match mods.get(&new_mod_info.id) {
      Some(existing_mod) => {
        // Check if the mod has actually changed (excluding process_id)
        let mut existing_mod_copy = existing_mod.clone();
        existing_mod_copy.process_id = None; // Ignore process_id for comparison
        let mut new_mod_copy = new_mod_info.clone();
        new_mod_copy.process_id = None; // Ignore process_id for comparison

        if existing_mod_copy != new_mod_copy {
          // Mod has changed, preserve the process_id and update
          new_mod_info.process_id = existing_mod.process_id;

          if let Some(pid) = new_mod_info.process_id {
            info!(
              "Updating mod {} (preserving process_id {})",
              new_mod_info.name,
              pid
            );
          } else {
            info!("Updating mod {}", new_mod_info.name);
          }

          mods.insert(new_mod_info.id.clone(), new_mod_info);
          updated_count += 1;
        } else {
          // Mod hasn't changed, keep the existing one (which preserves process_id)
          debug!("Mod {} unchanged, keeping existing", existing_mod.name);
        }
      }
      None => {
        // This is a new mod, add it
        info!("Adding new mod from database: {}", new_mod_info.name);
        mods.insert(new_mod_info.id.clone(), new_mod_info);
        added_count += 1;
      }
    }
  }

  // Also sync with the global state
  {
    let mut global_mods = crate::models::GLOBAL_MODS_STATE.lock().unwrap();
    *global_mods = mods.clone();
    info!("Updated global mods state with {} mods", global_mods.len());
  }

  info!(
    "Successfully synced mods from database: {} total, {} added, {} updated",
    mods.len(),
    added_count,
    updated_count
  );
  Ok(())
}

// Command to remove all mods from the state
#[tauri::command]
pub async fn remove_all_mods_command(
  mods_state: State<'_, ModsState>
) -> Result<(), String> {
  info!("Removing all mods from state");
  let mut mods = mods_state.0.lock().unwrap();
  mods.clear();
  info!("All mods removed from state");
  Ok(())
}

#[tauri::command]
fn change_mica_theme(
  app_handle: tauri::AppHandle,
  window: String,
  dark: bool
) -> Result<(), String> {
  #[cfg(target_os = "windows")]
  {
    let window = app_handle.get_webview_window(&window).ok_or_else(|| {
      let err = format!("Window with ID {} not found", window);
      error!("{}", err);
      err
    })?;
    if dark {
      window
        .set_effects(EffectsBuilder::new().effect(Effect::MicaDark).build())
        .map_err(|e| e.to_string())?;
      Ok(())
    } else {
      window
        .set_effects(EffectsBuilder::new().effect(Effect::MicaLight).build())
        .map_err(|e| e.to_string())?;
      Ok(())
    }
  }
  #[cfg(not(target_os = "windows"))]
  {
    Err(
      "Unsupported platform! 'apply_mica' is only supported on Windows".into()
    )
  }
}

#[tauri::command]
fn remove_mica_theme(
  app_handle: tauri::AppHandle,
  window: String
) -> Result<(), String> {
  #[cfg(target_os = "windows")]
  {
    let window = app_handle.get_webview_window(&window).ok_or_else(|| {
      let err = format!("Window with ID {} not found", window);
      error!("{}", err);
      err
    })?;
    // You can't really disable the Mica effect once you enable it, but since this won't effect anything
    // if not on Windows 11, we can do the best we can by setting it to light theme to mimic the normal titlebar.
    // The only reason this is a separate function is to allow for if Tauri adds the ability to disable Mica in the future.
    window
      .set_effects(EffectsBuilder::new().effect(Effect::MicaLight).build())
      .map_err(|e| e.to_string())?;
    Ok(())
  }
  #[cfg(not(target_os = "windows"))]
  {
    Err(
      "Unsupported platform! 'remove_mica' is only supported on Windows".into()
    )
  }
}

// Command to search for engine-specific mod metadata files
#[tauri::command]
pub async fn find_engine_mod_files(
  executable_path: String,
  engine_type: String,
  mods_folder: String
) -> Result<EngineModsResponse, String> {
  info!(
    "Searching for {} engine mod files for executable: {}",
    engine_type,
    executable_path
  );
  if mods_folder.is_empty() {
    info!("No mods folder provided, using default location");
    // Use the default mods folder location if none is provided
  } else {
    info!("Using provided mods folder: {}", mods_folder);
  }

  // Get the directory where the executable is located
  let exe_dir = get_executable_directory(&executable_path)?;

  // Find metadata files based on engine type
  let mut metadata_files = find_mod_metadata_files(
    &exe_dir,
    &engine_type,
    Path::new(&mods_folder)
  )?;

  // Process each mod to load its icon data and check if it's enabled
  for mod_file in &mut metadata_files {
    // Load icon data if available
    if let Some(icon_path) = &mod_file.icon_file_path {
      match crate::modutils::get_mod_icon_data(icon_path) {
        Ok(icon_data) => {
          mod_file.icon_data = Some(icon_data);
        }
        Err(e) => {
          warn!("Failed to load icon for mod {}: {}", mod_file.name, e);
        }
      }
    }

    // Check if the mod is enabled
    match
      check_mod_enabled_state(
        &executable_path,
        &mod_file.folder_path,
        &engine_type
      )
    {
      Ok(enabled) => {
        mod_file.enabled = Some(enabled);
      }
      Err(e) => {
        warn!("Failed to check if mod {} is enabled: {}", mod_file.name, e);
        // Default to enabled if we can't check
        mod_file.enabled = Some(true);
      }
    }
  }

  Ok(EngineModsResponse {
    engine_type: engine_type,
    executable_path: executable_path,
    mods: metadata_files,
  })
}

// Command to toggle a mod's enabled state
#[tauri::command]
pub async fn toggle_mod_enabled(
  executable_path: String,
  mod_folder_path: String,
  engine_type: String,
  enable: bool
) -> Result<ModDisableResult, String> {
  info!(
    "Toggling mod enabled state for {} (engine: {}, enable: {})",
    mod_folder_path,
    engine_type,
    enable
  );
  toggle_mod_enabled_state(
    &executable_path,
    &mod_folder_path,
    &engine_type,
    enable
  )
}

// Command to get a file's content as base64
#[tauri::command]
pub async fn get_file_as_base64(file_path: String) -> Result<String, String> {
  info!("Loading file as base64: {}", file_path);
  crate::modutils::get_mod_icon_data(&file_path)
}

#[tauri::command]
pub async fn get_url_as_base64(url: String) -> Result<String, String> {
  info!("Loading URL as base64: {}", url);
  match crate::utils::fetch_image_as_data_url(&url).await {
    Some(data) => Ok(data),
    None => Err("Failed to fetch image as data URL".to_string()),
  }
}

// Command to check if the OS is Windows 11 or greater
#[tauri::command]
pub fn is_windows_11() -> bool {
  crate::utils::is_windows_11_or_greater()
}

// Command to get download file options
#[tauri::command]
pub async fn get_mod_download_files_command(
  mod_id: i64,
  model_type: Option<String>
) -> Result<GBDownloadPage, String> {
  // Default to "Mod" if not specified
  let model_type = model_type.unwrap_or_else(|| "Mod".to_string());
  get_mod_download_files(mod_id, &model_type).await
}

// Command to get a mod's metadata JSON
#[tauri::command]
pub fn get_mod_metadata(
  mod_path: String,
  app: tauri::AppHandle
) -> Result<serde_json::Value, String> {
  info!("Getting metadata for mod at path: {}", mod_path);

  let path_obj = Path::new(&mod_path);

  // Check if the path exists
  if !path_obj.exists() {
    let err_msg = format!("Mod path does not exist: {}", mod_path);
    error!("{}", err_msg);

    // Emit an event to the frontend to notify about the missing mod path
    if let Err(emit_err) = app.emit("mod-path-missing", mod_path.clone()) {
      error!("Failed to emit mod-path-missing event: {}", emit_err);
    }

    return Err(err_msg);
  }

  // Call the filesystem function to read metadata
  match crate::filesystem::get_mod_metadata(path_obj) {
    Some(metadata) => {
      // Validate the metadata version
      let metadata_version = metadata
        .get("metadata_version")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32);

      match metadata_version {
        None => {
          let msg = format!(
            "Metadata is missing required metadata_version field"
          );
          warn!("{}", msg);
          return Err(msg);
        }
        Some(version) if version < crate::models::MIN_METADATA_VERSION => {
          let msg = format!(
            "Metadata version {} is too old. Minimum supported version is {}",
            version,
            crate::models::MIN_METADATA_VERSION
          );
          warn!("{}", msg);
          return Err(msg);
        }
        Some(_) => {
          debug!("Successfully retrieved metadata with valid version for mod at: {}", mod_path);
          Ok(metadata)
        }
      }
    }
    None => {
      let msg = format!("No metadata found for mod at: {}", mod_path);
      warn!("{}", msg);
      Err(msg)
    }
  }
}

// Command to save a mod's metadata JSON
#[tauri::command]
pub fn save_mod_metadata(
  mod_path: String,
  metadata: serde_json::Value
) -> Result<(), String> {
  info!("Saving metadata for mod at path: {}", mod_path);

  let path_obj = Path::new(&mod_path);

  // Check if the path exists
  if !path_obj.exists() {
    let err_msg = format!("Mod path does not exist: {}", mod_path);
    error!("{}", err_msg);
    return Err(err_msg);
  }

  // Create a new save function that overwrites existing metadata
  let flight_folder = path_obj.join(".flight");

  // Ensure the .flight folder exists
  if !flight_folder.exists() {
    debug!("Creating .flight directory: {}", flight_folder.display());
    if let Err(e) = std::fs::create_dir_all(&flight_folder) {
      let error_msg = format!("Failed to create .flight directory: {}", e);
      error!("{}", error_msg);
      return Err(error_msg);
    }
  }

  // Prepare metadata.json file path
  let metadata_path = flight_folder.join("metadata.json");

  // Format the JSON with pretty-printing
  let json_string = match serde_json::to_string_pretty(&metadata) {
    Ok(json) => json,
    Err(e) => {
      let error_msg = format!("Failed to serialize metadata to JSON: {}", e);
      error!("{}", error_msg);
      return Err(error_msg);
    }
  };

  // Write the JSON string to the file (overwrites existing file)
  if let Err(e) = std::fs::write(&metadata_path, json_string) {
    let error_msg = format!("Failed to write metadata.json: {}", e);
    error!("{}", error_msg);
    return Err(error_msg);
  }

  info!("Successfully saved metadata.json to: {}", metadata_path.display());
  Ok(())
}

// Command to get terminal logs for a specific mod's current session
#[tauri::command]
pub fn get_mod_logs(id: String) -> Vec<String> {
  crate::terminaloutput::get_logs(&id)
}

// Command to get terminal logs for a specific mod session
#[tauri::command]
pub fn get_mod_session_logs(id: String, session_id: String) -> Vec<String> {
  crate::terminaloutput::get_session_logs(&id, &session_id)
}

// Command to get current session logs for a mod
#[tauri::command]
pub fn get_mod_current_session_logs(id: String) -> Vec<String> {
  crate::terminaloutput::get_logs(&id)
}

// Command to list all sessions for a specific mod
#[tauri::command]
pub fn list_mod_sessions(
  id: String
) -> Result<Vec<crate::models::SessionInfo>, String> {
  // This will search through all session directories to find sessions for this mod
  let logs_dir = match crate::terminaloutput::LOGS_DIR.as_ref() {
    Ok(path_buf) => path_buf,
    Err(e) => {
      return Err(format!("Failed to resolve logs directory: {}", e));
    }
  };

  let mut sessions = Vec::new();

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
                if let Ok(file) = std::fs::File::open(&session_info_path) {
                  if
                    let Ok(session_info) = serde_json::from_reader::<
                      _,
                      crate::models::SessionInfo
                    >(file)
                  {
                    if session_info.mod_id == id {
                      sessions.push(session_info);
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

  // Sort sessions by start time (newest first)
  sessions.sort_by(|a, b| b.start_time.cmp(&a.start_time));

  Ok(sessions)
}

// Command to clear in-memory terminal logs for a specific mod's current session, while saving them to a file
#[tauri::command]
pub fn clear_mod_logs(id: String) -> Result<(), String> {
  info!("Clearing current session terminal logs for mod with ID: {}", id);
  crate::terminaloutput::clear_memory_logs(&id);
  Ok(())
}

// Command to clear a specific session's logs
#[tauri::command]
pub fn clear_session_logs(
  id: String,
  session_id: String
) -> Result<(), String> {
  info!("Clearing logs for mod {} session {}", id, session_id);

  // Remove from memory if it's there
  {
    let mut logs = crate::terminaloutput::SESSION_LOGS.lock().unwrap();
    if let Some(mod_sessions) = logs.get_mut(&id) {
      mod_sessions.remove(&session_id);
    }
  }

  // Remove from disk
  let logs_dir = match crate::terminaloutput::LOGS_DIR.as_ref() {
    Ok(path_buf) => path_buf,
    Err(e) => {
      return Err(format!("Failed to resolve logs directory: {}", e));
    }
  };

  if let Ok(entries) = std::fs::read_dir(logs_dir) {
    for entry in entries.filter_map(|e| e.ok()) {
      if entry.path().is_dir() {
        let session_dir = entry.path().join(&session_id);
        if session_dir.exists() {
          // Check if this session belongs to our mod
          let session_info_path = session_dir.join("session_info.json");
          if session_info_path.exists() {
            if let Ok(file) = std::fs::File::open(&session_info_path) {
              if
                let Ok(session_info) = serde_json::from_reader::<
                  _,
                  crate::models::SessionInfo
                >(file)
              {
                if session_info.mod_id == id {
                  // This is our session, delete it
                  if let Err(e) = std::fs::remove_dir_all(&session_dir) {
                    return Err(
                      format!("Failed to delete session directory: {}", e)
                    );
                  }
                  break;
                }
              }
            }
          }
        }
      }
    }
  }

  Ok(())
}

// Command to clear all logs for a specific mod, including all sessions
#[tauri::command]
pub fn clear_all_mod_logs(id: String) -> Result<(), String> {
  info!("Clearing all sessions for mod with ID: {}", id);
  crate::terminaloutput::clear_logs(&id);
  Ok(())
}

// Command to get the logs folder path for a specific mod
#[tauri::command]
pub fn get_mod_logs_folder_path(id: String) -> Result<String, String> {
  debug!("Getting logs folder path for mod with ID: {}", id);
  crate::terminaloutput::get_mod_logs_folder_path(&id)
}

#[tauri::command]
pub fn compare_update_semver(
  current_version: String,
  latest_version: String
) -> bool {
  info!(
    "Comparing current version '{}' with latest version '{}'",
    current_version,
    latest_version
  );

  // Use the new semver comparison function
  match crate::modutils::compare_semver(&current_version, &latest_version) {
    Ok(result) => {
      // Return false if we need to update (current_version < latest_version)
      if result < 0 {
        info!(
          "Current version '{}' is less than latest version '{}', update needed",
          current_version,
          latest_version
        );
        false // Indicates we need to update
      } else {
        info!(
          "Current version '{}' is up-to-date with latest version '{}'",
          current_version,
          latest_version
        );
        true // Indicates no update needed
      }
    }
    Err(e) => {
      warn!("Failed to compare semver versions: {}", e);
      // Fall back to the old method if semver parsing fails
      let latest_version_format = format!(">={}", latest_version);
      crate::modutils::is_version_compatible(
        &current_version,
        &latest_version_format
      )
    }
  }
}

// Command to delete a mod by deleting its containing folder
#[tauri::command]
pub async fn super_delete_mod(
  id: String,
  mods_state: State<'_, ModsState>
) -> Result<(), String> {
  info!("Attempting to super delete mod with ID: {}", id);
  let mut mods = mods_state.0.lock().unwrap();

  // First check if the mod is running, can't delete a running mod
  let mod_path = if let Some(mod_info) = mods.get(&id) {
    if mod_info.process_id.is_some() {
      let err_msg = format!(
        "Cannot delete mod {} while it's running. Please stop it first.",
        mod_info.name
      );
      warn!("{}", err_msg);
      return Err(err_msg);
    }

    // Store the path for deletion
    mod_info.path.clone()
  } else {
    let err_msg = format!("Mod not found with ID: {}", id);
    warn!("{}", err_msg);
    return Err(err_msg);
  };

  // Check if path exists and is a directory
  let path = Path::new(&mod_path);
  if !path.exists() {
    let err_msg = format!("Path doesn't exist: {}", mod_path);
    error!("{}", err_msg);
    return Err(err_msg);
  }

  if !path.is_dir() {
    let err_msg = format!("Path is not a directory: {}", mod_path);
    error!("{}", err_msg);
    return Err(err_msg);
  }

  // Delete the directory
  info!("Deleting directory: {}", mod_path);
  match std::fs::remove_dir_all(path) {
    Ok(_) => {
      info!("Successfully deleted directory: {}", mod_path);

      // Remove the mod from our state
      mods.remove(&id);

      Ok(())
    }
    Err(e) => {
      let err_msg = format!("Failed to delete directory: {}", e);
      error!("{}", err_msg);
      Err(err_msg)
    }
  }
}

// Command to check for mod dependency
#[tauri::command]
pub fn check_mod_dependency(
  mods_folder_path: &str,
  dependency_name: &str,
  required_version: &str
) -> Result<(), String> {
  info!("Checking mod dependency for mod at: {}", mods_folder_path);
  crate::modutils::find_mod_dependency(
    mods_folder_path,
    dependency_name,
    required_version
  )?;
  Ok(())
}

#[tauri::command]
pub async fn check_gamebanana_mod_version(
  id: i64,
  model_type: String
) -> Result<String, String> {
  crate::gamebanana::get_mod_version(id, &model_type).await
}

#[tauri::command]
pub fn check_mod_folder_exists(
  info: GBProfilePage,
  install_location: Option<String>,
  folder_name: Option<String>,
  app: tauri::AppHandle
) -> Result<bool, String> {
  Ok(
    crate::download::simulate_mod_folder_creation(
      info,
      install_location,
      folder_name,
      &app
    )
  )
}

#[tauri::command]
pub fn check_engine_folder_exists(
  engine_type: String,
  install_location: Option<String>,
  custom_name: Option<String>,
  app: tauri::AppHandle
) -> Result<bool, String> {
  Ok(
    crate::download::simulate_engine_folder_creation(
      engine_type,
      install_location,
      custom_name,
      &app
    )
  )
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
  if let Some(update) = app.updater()?.check().await? {
    let mut downloaded = 0;

    update.download_and_install(
      |chunk_length, content_length| {
        downloaded += chunk_length;
        println!("downloaded {downloaded} from {content_length:?}");
      },
      || {
        println!("download finished");
      }
    ).await?;

    println!("update installed");
    app.restart();
  }

  Ok(())
}

// Setup function for Tauri application
pub fn run() {
  // Create a shared mods state that will be used throughout the application
  let mods_state = ModsState(Mutex::new(HashMap::new()));

  // Store a reference to this state in our global state for background thread access
  {
    let mut global_state = crate::models::GLOBAL_MODS_STATE.lock().unwrap();
    *global_state = mods_state.0.lock().unwrap().clone();
    info!("Initialized global mods state");
  }

  tauri::Builder
    ::default()
    .plugin(
      tauri_plugin_single_instance::init(|_app, argv, _cwd| {
        info!(
          "a new app instance was opened with {argv:?} and the deep link event was already triggered"
        );
        // when defining deep link schemes at runtime, you must also check `argv` here
      })
    )
    .plugin(tauri_plugin_sql::Builder::new().build())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_deep_link::init())
    .plugin(tauri_plugin_store::Builder::new().build())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_fs::init())
    .manage(mods_state)
    .setup(|app| {
      // Store the app handle for event emission
      crate::app_handle::set_global_app_handle(app.handle().clone());

      let handle = app.handle().clone();
      tauri::async_runtime::spawn(async move {
        update(handle).await.unwrap();
      });
      #[cfg(desktop)]
      {
        match app.deep_link().register("flmod") {
          Ok(_) =>
            info!("Successfully registered deep link protocol 'flmod://"),
          Err(e) => error!("Failed to register deep link protocol: {}", e),
        }
      }
      Ok(
        logger::init(&app.handle()).map_err(|e| {
          error!("Logger initialization failed: {}", e);
          e
        })?
      )
    })
    .invoke_handler(
      tauri::generate_handler![
        select_mod_folder,
        select_settings_folder,
        select_executable,
        add_mod,
        get_mods,
        launch_mod,
        fetch_gamebanana_mods_command,
        get_featured_mods_command,
        get_mod_info_command,
        download_gamebanana_mod_command,
        download_engine_command,
        sync_mods_from_database,
        select_mods_parent_folder,
        change_mica_theme,
        find_engine_mod_files,
        get_file_as_base64,
        toggle_mod_enabled,
        is_windows_11,
        get_mod_download_files_command,
        get_mod_metadata,
        remove_mica_theme,
        is_mod_running,
        stop_mod,
        get_mod_logs,
        get_mod_session_logs,
        get_mod_current_session_logs,
        list_mod_sessions,
        clear_mod_logs,
        clear_session_logs,
        clear_all_mod_logs,
        super_delete_mod,
        check_mod_folder_exists,
        check_engine_folder_exists,
        get_mod_posts_command,
        get_mod_updates_command,
        check_mod_dependency,
        check_gamebanana_mod_version,
        compare_update_semver,
        save_mod_metadata,
        get_url_as_base64,
        remove_all_mods_command
      ]
    )
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
