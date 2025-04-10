use crate::download::download_gamebanana_mod;
use crate::filesystem::create_mod_info;
use crate::gamebanana::fetch_gamebanana_mods;
use crate::logger;
use crate::modfiles::{find_mod_metadata_files, get_executable_directory};
use crate::modenabler::{toggle_mod_enabled_state, check_mod_enabled_state};
use crate::models::{ModInfo, ModsState, GameBananaResponse, EngineModsResponse, ModDisableResult};
use log::{debug, error, info, warn};
use tauri::utils::config::WindowEffectsConfig;
use tauri::window::{Effect, EffectsBuilder};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use tauri::{Manager, State};
use tauri_plugin_dialog::DialogExt;

// Command to open a folder dialog and get the selected folder path
#[tauri::command]
pub async fn select_mod_folder(app: tauri::AppHandle) -> Result<String, String> {
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
        },
        None => {
            warn!("No mod folder selected");
            Err("No folder selected".to_string())
        },
    }
}

// Command to open a folder dialog and select a folder containing multiple mods
#[tauri::command]
pub async fn select_mods_parent_folder(app: tauri::AppHandle, mods_state: State<'_, ModsState>) -> Result<Vec<ModInfo>, String> {
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
            
            // Read all subdirectories and add each as a mod
            match std::fs::read_dir(&path_str) {
                Ok(entries) => {
                    let mut added_mods = Vec::new();
                    
                    for entry in entries.filter_map(|e| e.ok()) {
                        let entry_path = entry.path();
                        
                        // Only process directories
                        if entry_path.is_dir() {
                            let subdir_path = entry_path.to_string_lossy().to_string();
                            debug!("Processing potential mod directory: {}", subdir_path);
                            
                            // Try to add this directory as a mod
                            match create_mod_info(&subdir_path) {
                                Ok(mod_info) => {
                                    let id = mod_info.id.clone();
                                    added_mods.push(mod_info.clone());
                                    
                                    // Add to our state
                                    let mut mods = mods_state.0.lock().unwrap();
                                    mods.insert(id, mod_info);
                                    info!("Added mod: {} ({})", entry_path.display(), subdir_path);
                                },
                                Err(e) => {
                                    warn!("Failed to add directory as mod: {} - {}", subdir_path, e);
                                    // Continue to next directory
                                }
                            }
                        }
                    }
                    
                    info!("Added {} mods from parent folder", added_mods.len());
                    Ok(added_mods)
                },
                Err(e) => {
                    let err_msg = format!("Failed to read directory contents: {}", e);
                    error!("{}", err_msg);
                    Err(err_msg)
                }
            }
        },
        None => {
            warn!("No folder selected");
            Err("No folder selected".to_string())
        },
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
        },
        None => {
            warn!("No settings folder selected");
            Err("No folder selected".to_string())
        },
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
        },
        None => {
            warn!("No executable selected");
            Err("No file selected".to_string())
        },
    }
}

// Command to add a mod to our list
#[tauri::command]
pub fn add_mod(path: String, mods_state: State<'_, ModsState>) -> Result<ModInfo, String> {
    info!("Adding mod from path: {}", path);
    
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
    let mods = mods_state.0.lock().unwrap();

    if let Some(mod_info) = mods.get(&id) {
        info!("Launching mod: {}", mod_info.name);

        if let Some(exe_path) = &mod_info.executable_path {
            debug!("Launching executable: {}", exe_path);
            
            // Get the executable's directory to use as the working directory
            let exe_path_obj = Path::new(exe_path);
            let working_dir = exe_path_obj.parent().map(|p| p.to_path_buf()).ok_or_else(|| {
                let err = "Could not determine executable's directory".to_string();
                error!("{}", err);
                err
            })?;
            
            debug!("Using working directory: {}", working_dir.display());
            
            // Launch the executable from its own directory
            match Command::new(exe_path)
                .current_dir(working_dir)  // Set the working directory to the mod's directory
                .spawn() 
            {
                Ok(_) => {
                    info!("Successfully launched: {}", exe_path);
                    Ok(())
                }
                Err(e) => {
                    let error_msg = format!("Failed to launch executable: {}", e);
                    error!("{}", error_msg);
                    Err(error_msg)
                }
            }
        } else {
            let err_msg = format!("No executable found for mod: {}", mod_info.name);
            warn!("{}", err_msg);
            Err(err_msg)
        }
    } else {
        let err_msg = format!("Mod not found with ID: {}", id);
        warn!("{}", err_msg);
        Err("Mod not found".to_string())
    }
}

// Command to fetch mods from GameBanana
#[tauri::command]
pub async fn fetch_gamebanana_mods_command(query: String, page: i64) -> Result<GameBananaResponse, String> {
    fetch_gamebanana_mods(query, page).await
}

// Command to download a mod from GameBanana
#[tauri::command]
pub async fn download_gamebanana_mod_command(
    url: String, 
    name: String,
    mod_id: i64,
    install_location: Option<String>,
    app: tauri::AppHandle
) -> Result<String, String> {
    download_gamebanana_mod(url, name, mod_id, install_location, app).await
}

// Command to sync mods from database
#[tauri::command]
pub async fn sync_mods_from_database(mods_data: Vec<ModInfo>, mods_state: State<'_, ModsState>) -> Result<(), String> {
    info!("Syncing {} mods from database to ModsState", mods_data.len());
    
    // Clear current mods and add all the ones from the database
    let mut mods = mods_state.0.lock().unwrap();
    mods.clear(); // Remove existing mods
    
    // Insert all mods from the database
    for mod_info in mods_data {
        debug!("Syncing mod from database: {} ({})", mod_info.name, mod_info.id);
        mods.insert(mod_info.id.clone(), mod_info);
    }
    
    info!("Successfully synced {} mods from database", mods.len());
    Ok(())
}

#[tauri::command]
fn change_mica_theme(app_handle: tauri::AppHandle, window: String, dark: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let window = app_handle.get_webview_window(&window).ok_or_else(|| {
            let err = format!("Window with ID {} not found", window);
            error!("{}", err);
            err
        })?;
        if dark {
            window.set_effects(
                EffectsBuilder::new()
                .effect(Effect::MicaDark)
                .build()
            ).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            window.set_effects(
                EffectsBuilder::new()
                .effect(Effect::MicaLight)
                .build()
            ).map_err(|e| e.to_string())?;
            Ok(())
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err("Unsupported platform! 'apply_mica' is only supported on Windows".into())
    }
}

// Command to search for engine-specific mod metadata files
#[tauri::command]
pub async fn find_engine_mod_files(executable_path: String, engine_type: String) -> Result<EngineModsResponse, String> {
    info!("Searching for {} engine mod files for executable: {}", engine_type, executable_path);
    
    // Get the directory where the executable is located
    let exe_dir = get_executable_directory(&executable_path)?;
    
    // Find metadata files based on engine type
    let mut metadata_files = find_mod_metadata_files(&exe_dir, &engine_type)?;
    
    // Process each mod to load its icon data and check if it's enabled
    for mod_file in &mut metadata_files {
        // Load icon data if available
        if let Some(icon_path) = &mod_file.icon_file_path {
            match crate::modfiles::get_mod_icon_data(icon_path) {
                Ok(icon_data) => {
                    mod_file.icon_data = Some(icon_data);
                },
                Err(e) => {
                    warn!("Failed to load icon for mod {}: {}", mod_file.name, e);
                }
            }
        }
        
        // Check if the mod is enabled
        match check_mod_enabled_state(&executable_path, &mod_file.folder_path, &engine_type) {
            Ok(enabled) => {
                mod_file.enabled = Some(enabled);
            },
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
pub async fn toggle_mod_enabled(executable_path: String, mod_folder_path: String, engine_type: String, enable: bool) -> Result<ModDisableResult, String> {
    info!("Toggling mod enabled state for {} (engine: {}, enable: {})", mod_folder_path, engine_type, enable);
    toggle_mod_enabled_state(&executable_path, &mod_folder_path, &engine_type, enable)
}

// Command to get a file's content as base64
#[tauri::command]
pub async fn get_file_as_base64(file_path: String) -> Result<String, String> {
    info!("Loading file as base64: {}", file_path);
    crate::modfiles::get_mod_icon_data(&file_path)
}

// Command to check if the OS is Windows 11 or greater
#[tauri::command]
pub fn is_windows_11() -> bool {
    crate::utils::is_windows_11_or_greater()
}

// Setup function for Tauri application
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(ModsState(Mutex::new(HashMap::new())))
        .setup(|app| {
            Ok(logger::init(&app.handle()).map_err(|e| {
                error!("Logger initialization failed: {}", e);
                e
            })?)
        })        .invoke_handler(tauri::generate_handler![
            select_mod_folder,
            select_settings_folder,
            select_executable,
            add_mod,
            get_mods,
            launch_mod,
            fetch_gamebanana_mods_command,
            download_gamebanana_mod_command,
            sync_mods_from_database,
            select_mods_parent_folder,
            change_mica_theme,
            find_engine_mod_files,
            get_file_as_base64,
            toggle_mod_enabled,
            is_windows_11
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}