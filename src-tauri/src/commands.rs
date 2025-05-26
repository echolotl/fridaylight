use crate::download::download_gamebanana_mod;
use crate::filesystem::create_mod_info;
use crate::gamebanana::{ fetch_gamebanana_mods, get_mod_download_files, get_mod_info };
use crate::logger;
use crate::modutils::{
    find_mod_metadata_files,
    get_executable_directory,
    toggle_mod_enabled_state,
    check_mod_enabled_state,
};
use crate::models::{
    EngineModsResponse,
    GameBananaResponse,
    ModDisableResult,
    ModInfo,
    ModMetadataFile,
    ModsState,
};
use log::{ debug, error, info, warn };
use tauri::window::{ Effect, EffectsBuilder };
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
pub async fn select_mod_folder(app: tauri::AppHandle) -> Result<String, String> {
    debug!("Opening folder selection dialog");
    let folder = app.dialog().file().set_title("Select Mod Folder").blocking_pick_folder();

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
pub async fn select_settings_folder(app: tauri::AppHandle) -> Result<String, String> {
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
pub async fn select_executable(app: tauri::AppHandle) -> Result<String, String> {
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

    // Add to our state
    let mut mods = mods_state.0.lock().unwrap();
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
pub fn launch_mod(id: String, mods_state: State<'_, ModsState>) -> Result<(), String> {
    info!("Attempting to launch mod with ID: {}", id);
    let mut mods = mods_state.0.lock().unwrap();
    let mod_name: String;
    // Get required info from the mod
    let (executable_path, should_disable_other_mods, engine_mod_info) = match mods.get(&id) {
        Some(mod_info) => {
            // Check if the mod is already running
            if let Some(pid) = mod_info.process_id {
                warn!("Mod is already running with PID: {}", pid);
                return Err(format!("Mod is already running with PID: {}", pid));
            }

            mod_name = mod_info.name.clone();

            let exe_path = if let Some(exe_path) = &mod_info.executable_path {
                exe_path.clone()
            } else {
                let err_msg = format!("No executable found for mod: {}", mod_info.name);
                warn!("{}", err_msg);
                crate::terminaloutput::add_log(&id, &format!("[ERROR] {}", err_msg));
                return Err(err_msg);
            };

            // Check if this mod has an engine_mod field and should disable other mods
            let (should_disable, engine_mod_info) = if
                let Some(ref engine_mod) = mod_info.engine_mod
            {
                (
                    true,
                    Some((
                        mod_info.engine.as_ref().and_then(|e| e.engine_type.clone()),
                        mod_info.engine.as_ref().and_then(|e| e.mods_folder_path.clone()),
                        engine_mod.folder_path.clone(),
                    )),
                )
            } else {
                (false, None)
            };

            (exe_path, should_disable, engine_mod_info)
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

    // If this mod has an engine_mod field, disable all other engine mods before launching
    if should_disable_other_mods {
        if let Some((engine_type, mods_folder_path, exception_mod_path)) = engine_mod_info {
            if let Some(engine_type) = engine_type {
                // Determine the mods folder path
                let actual_mods_folder_path = if let Some(custom_path) = mods_folder_path {
                    // Use custom mods folder path
                    working_dir.join(custom_path).to_string_lossy().to_string()
                } else {
                    // Use default "mods" folder
                    working_dir.join("mods").to_string_lossy().to_string()
                };

                info!(
                    "Disabling other engine mods before launching {} (engine: {}, mods_folder: {})",
                    mod_name,
                    engine_type,
                    actual_mods_folder_path
                );

                match
                    crate::modutils::disable_all_mods_except(
                        &executable_path,
                        &actual_mods_folder_path,
                        &engine_type,
                        &exception_mod_path
                    )
                {
                    Ok(results) => {
                        let disabled_count = results
                            .iter()
                            .filter(|r| r.success && !r.enabled)
                            .count();
                        let kept_enabled_count = results
                            .iter()
                            .filter(|r| r.success && r.enabled)
                            .count();
                        info!(
                            "Successfully disabled {} other mods, kept {} enabled",
                            disabled_count,
                            kept_enabled_count
                        );
                        crate::terminaloutput::add_log(
                            &id,
                            &format!("Disabled {} other engine mods before launching", disabled_count)
                        );
                    }
                    Err(e) => {
                        warn!("Failed to disable other engine mods: {}", e);
                        crate::terminaloutput::add_log(
                            &id,
                            &format!("[WARNING] Failed to disable other engine mods: {}", e)
                        );
                        // Continue with launch despite this error
                    }
                }
            } else {
                warn!("Engine mod found but no engine type specified, cannot disable other mods");
                crate::terminaloutput::add_log(
                    &id,
                    &"[WARNING] Engine mod found but no engine type specified"
                );
            }
        }
    }

    // Clear previous logs for this mod
    crate::terminaloutput::clear_logs(&id);

    // Add initial log entry showing the startup
    crate::terminaloutput::add_log(
        &id,
        &format!("Starting mod: {} ({})", mod_name, executable_path)
    );
    crate::terminaloutput::add_log(&id, &format!("Working directory: {}", working_dir.display()));

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
            // Store the process ID in the ModInfo and update last_played timestamp
            if let Some(mod_info) = mods.get_mut(&id) {
                mod_info.process_id = Some(pid);

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
                if let Ok(mut global_mods) = crate::models::GLOBAL_MODS_STATE.lock() {
                    if let Some(global_mod_info) = global_mods.get_mut(&id) {
                        global_mod_info.process_id = Some(pid);
                        global_mod_info.last_played = Some(current_time);
                        info!(
                            "Updated global state for mod {}: pid={}, last_played={}",
                            mod_info.name,
                            pid,
                            current_time
                        );
                    } else {
                        warn!(
                            "Mod {} not found in global state when updating process info",
                            mod_info.name
                        );
                    }
                } else {
                    warn!("Failed to acquire lock on global state when updating process info");
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
            std::thread::spawn(move || {
                match child.wait() {
                    Ok(status) => {
                        let exit_message = format!("Process exited with status: {}", status);
                        info!("{}", exit_message);
                        crate::terminaloutput::add_log(&id_for_thread, &exit_message);

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
        if let Ok(global_mods) = crate::models::GLOBAL_MODS_STATE.lock() {
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
        } else {
            info!("Could not acquire lock on global state");
            false
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
pub fn stop_mod(id: String, mods_state: State<'_, ModsState>) -> Result<(), String> {
    info!("Attempting to stop mod with ID: {}", id);
    let mut mods = mods_state.0.lock().unwrap();

    if let Some(mod_info) = mods.get_mut(&id) {
        if let Some(pid) = mod_info.process_id {
            info!("Stopping mod: {} with PID: {}", mod_info.name, pid);

            #[cfg(target_os = "windows")]
            {
                use std::process::Command;
                // On Windows, use taskkill to terminate the process
                match Command::new("taskkill").args(&["/PID", &pid.to_string(), "/F"]).output() {
                    Ok(_) => {
                        info!("Successfully stopped process with PID: {}", pid);
                        mod_info.process_id = None;

                        // Also update the global state
                        if let Ok(mut global_mods) = crate::models::GLOBAL_MODS_STATE.lock() {
                            if let Some(global_mod_info) = global_mods.get_mut(&id) {
                                global_mod_info.process_id = None;
                                info!("Updated global state for mod {}: pid=None", mod_info.name);
                            } else {
                                warn!(
                                    "Mod {} not found in global state when stopping",
                                    mod_info.name
                                );
                            }
                        } else {
                            warn!("Failed to acquire lock on global state when stopping mod");
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
) -> Result<GameBananaResponse, String> {
    fetch_gamebanana_mods(query, page).await
}

#[tauri::command]
pub async fn get_mod_info_command(
    mod_id: i64,
    model_type: Option<String>
) -> Result<serde_json::Value, String> {
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
) -> Result<serde_json::Value, String> {
    // Default to "Mod" if not specified
    let model_type = model_type.unwrap_or_else(|| "Mod".to_string());
    crate::gamebanana::get_mod_posts(mod_id, page, &model_type).await
}

#[tauri::command]
pub async fn get_mod_updates_command(
    mod_id: i64,
    page: i64,
    model_type: Option<String>
) -> Result<serde_json::Value, String> {
    // Default to "Mod" if not specified
    let model_type = model_type.unwrap_or_else(|| "Mod".to_string());
    crate::gamebanana::get_mod_updates(mod_id, page, &model_type).await
}

// Command to download a mod from GameBanana
#[tauri::command]
pub async fn download_gamebanana_mod_command(
    url: String,
    name: String,
    mod_id: i64,
    install_location: Option<String>,
    model_type: Option<String>,
    app: tauri::AppHandle
) -> Result<String, String> {
    download_gamebanana_mod(url, name, mod_id, install_location, model_type, app).await
}

#[tauri::command]
pub async fn download_custom_mod_command(
    url: String,
    name: String,
    mod_id: i64,
    install_location: Option<String>,
    thumbnail_url: Option<String>,
    description: Option<String>,
    version: Option<String>,
    app: tauri::AppHandle
) -> Result<String, String> {
    info!("Starting custom mod download process for {} with ID: {}", name, mod_id);
    crate::download::download_custom_mod(
        url,
        name,
        mod_id,
        install_location,
        thumbnail_url,
        description,
        version,
        app
    ).await
}

#[tauri::command]
pub async fn download_engine_command(
    engine_type: String,
    install_location: Option<String>,
    custom_name: Option<String>,
    app: tauri::AppHandle
) -> Result<String, String> {
    info!("Starting direct download process for {} engine", engine_type);
    crate::download::download_engine(engine_type, install_location, custom_name, app).await
}

// Command to sync mods from database
#[tauri::command]
pub async fn sync_mods_from_database(
    mods_data: Vec<ModInfo>,
    mods_state: State<'_, ModsState>
) -> Result<(), String> {
    info!("Syncing {} mods from database to ModsState", mods_data.len());

    // First, capture the process_id from existing mods
    let process_ids: HashMap<String, Option<u32>> = {
        let current_mods = mods_state.0.lock().unwrap();
        current_mods
            .iter()
            .map(|(id, mod_info)| (id.clone(), mod_info.process_id))
            .collect()
    };

    // Clear current mods and add all the ones from the database
    let mut mods = mods_state.0.lock().unwrap();
    mods.clear(); // Remove existing mods

    // Insert all mods from the database, preserving process_id values
    for mut mod_info in mods_data {
        // If this mod was running, preserve its process_id
        if let Some(pid) = process_ids.get(&mod_info.id).and_then(|&pid| pid) {
            info!("Preserving process_id {} for mod {}", pid, mod_info.name);
            mod_info.process_id = Some(pid);
        }

        debug!("Syncing mod from database: {} ({})", mod_info.name, mod_info.id);
        mods.insert(mod_info.id.clone(), mod_info);
    }

    // Also sync with the global state
    {
        let mut global_mods = crate::models::GLOBAL_MODS_STATE.lock().unwrap();
        *global_mods = mods.clone();
        info!("Updated global mods state with {} mods", global_mods.len());
    }

    info!("Successfully synced {} mods from database", mods.len());
    Ok(())
}

// Command to check if a mod folder already exists
#[tauri::command]
pub fn check_mod_folder_exists(
    name: String,
    install_location: Option<String>,
    app: tauri::AppHandle
) -> Result<bool, String> {
    info!("Checking if mod folder exists for: {}", name);

    // Sanitize mod name for folder name (same process as in download.rs)
    let sanitized_name = name
        .replace(' ', "-")
        .replace('/', "_")
        .replace('\\', "_")
        .replace(':', "")
        .replace('*', "")
        .replace('?', "")
        .replace('"', "")
        .replace('<', "")
        .replace('>', "")
        .replace('|', "");

    // Get the install location - use provided location or fall back to default
    let install_dir = if let Some(location) = install_location {
        let path = std::path::PathBuf::from(&location);
        info!("Using provided install location: {}", path.display());
        path
    } else {
        let default_path = crate::download::get_default_install_location(&app);
        info!("Using default install location: {}", default_path.display());
        default_path
    };

    // Build the potential mod folder path
    let mod_folder = install_dir.join(&sanitized_name);
    debug!("Checking if folder exists: {}", mod_folder.display());

    // Check if the directory exists
    let exists = mod_folder.exists();
    info!(
        "Mod folder {} for mod '{}': {}",
        if exists {
            "exists"
        } else {
            "does not exist"
        },
        name,
        mod_folder.display()
    );

    Ok(exists)
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
        Err("Unsupported platform! 'apply_mica' is only supported on Windows".into())
    }
}

#[tauri::command]
fn remove_mica_theme(app_handle: tauri::AppHandle, window: String) -> Result<(), String> {
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
        Err("Unsupported platform! 'remove_mica' is only supported on Windows".into())
    }
}

// Command to search for engine-specific mod metadata files
#[tauri::command]
pub async fn find_engine_mod_files(
    executable_path: String,
    engine_type: String,
    mods_folder: String
) -> Result<EngineModsResponse, String> {
    info!("Searching for {} engine mod files for executable: {}", engine_type, executable_path);
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
        match check_mod_enabled_state(&executable_path, &mod_file.folder_path, &engine_type) {
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
    toggle_mod_enabled_state(&executable_path, &mod_folder_path, &engine_type, enable)
}

// Command to bulk toggle all mods in a folder
#[tauri::command]
pub async fn toggle_all_mods_enabled(
    executable_path: String,
    mods_folder_path: String,
    engine_type: String,
    enable: bool
) -> Result<Vec<ModDisableResult>, String> {
    info!(
        "Bulk toggling all mods (engine: {}, enable: {}, mods_folder: {})",
        engine_type,
        enable,
        mods_folder_path
    );
    crate::modutils::toggle_all_mods_enabled_state(
        &executable_path,
        &mods_folder_path,
        &engine_type,
        enable
    )
}

// Command to disable all mods except one specified mod
#[tauri::command]
pub async fn disable_all_mods_except(
    executable_path: String,
    mods_folder_path: String,
    engine_type: String,
    exception_mod_path: String
) -> Result<Vec<ModDisableResult>, String> {
    info!(
        "Disabling all mods except: {} (engine: {}, mods_folder: {})",
        exception_mod_path,
        engine_type,
        mods_folder_path
    );
    crate::modutils::disable_all_mods_except(
        &executable_path,
        &mods_folder_path,
        &engine_type,
        &exception_mod_path
    )
}

// Command to get all mod paths in a mods folder
#[tauri::command]
pub async fn get_all_mod_paths(
    mods_folder_path: String,
    engine_type: String
) -> Result<Vec<String>, String> {
    info!("Getting all mod paths (engine: {}, mods_folder: {})", engine_type, mods_folder_path);
    crate::modutils::get_all_mod_paths(&mods_folder_path, &engine_type)
}

// Command to get enabled status for all mods in a folder
#[tauri::command]
pub async fn get_all_mods_enabled_status(
    executable_path: String,
    mods_folder_path: String,
    engine_type: String
) -> Result<Vec<(String, String, bool)>, String> {
    info!(
        "Getting enabled status for all mods (engine: {}, mods_folder: {})",
        engine_type,
        mods_folder_path
    );
    crate::modutils::get_all_mods_enabled_status(&executable_path, &mods_folder_path, &engine_type)
}

// Command to get a file's content as base64
#[tauri::command]
pub async fn get_file_as_base64(file_path: String) -> Result<String, String> {
    info!("Loading file as base64: {}", file_path);
    crate::modutils::get_mod_icon_data(&file_path)
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
) -> Result<serde_json::Value, String> {
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
                    let msg = format!("Metadata is missing required metadata_version field");
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

// Command to get terminal logs for a specific mod
#[tauri::command]
pub fn get_mod_logs(id: String) -> Vec<String> {
    crate::terminaloutput::get_logs(&id)
}

// Command to clear terminal logs for a specific mod
#[tauri::command]
pub fn clear_mod_logs(id: String) -> Result<(), String> {
    info!("Clearing terminal logs for mod with ID: {}", id);
    crate::terminaloutput::clear_logs(&id);
    Ok(())
}

// Command to delete a mod by deleting its containing folder
#[tauri::command]
pub async fn super_delete_mod(id: String, mods_state: State<'_, ModsState>) -> Result<(), String> {
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
    crate::modutils::find_mod_dependency(mods_folder_path, dependency_name, required_version)?;
    Ok(())
}

// Command to convert an engine mod to an actual mod
#[tauri::command]
pub async fn convert_engine_mod_to_mod(
    engine_mod: ModMetadataFile,
    original_mod: ModInfo
) -> ModInfo {
    info!("Converting engine mod to mod at path: {}", engine_mod.folder_path);
    crate::modutils::convert_to_mod_info(engine_mod, original_mod).await
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
                    Ok(_) => info!("Successfully registered deep link protocol 'flmod://"),
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
                get_mod_info_command,
                download_gamebanana_mod_command,
                download_custom_mod_command,
                download_engine_command,
                sync_mods_from_database,
                select_mods_parent_folder,
                change_mica_theme,
                find_engine_mod_files,
                get_file_as_base64,
                toggle_mod_enabled,
                toggle_all_mods_enabled,
                disable_all_mods_except,
                get_all_mod_paths,
                get_all_mods_enabled_status,
                is_windows_11,
                get_mod_download_files_command,
                get_mod_metadata,
                remove_mica_theme,
                is_mod_running,
                stop_mod,
                get_mod_logs,
                clear_mod_logs,
                super_delete_mod,
                check_mod_folder_exists,
                get_mod_posts_command,
                get_mod_updates_command,
                check_mod_dependency,
                convert_engine_mod_to_mod
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
