use crate::download::download_gamebanana_mod;
use crate::filesystem::create_mod_info;
use crate::gamebanana::fetch_gamebanana_mods;
use crate::logger;
use crate::models::{ModInfo, ModsState, GameBananaResponse};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use tauri::State;
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

// Simple hello world command
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            select_mod_folder,
            select_settings_folder,
            select_executable,
            add_mod,
            get_mods,
            launch_mod,
            fetch_gamebanana_mods_command,
            download_gamebanana_mod_command,
            sync_mods_from_database,
            select_mods_parent_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}