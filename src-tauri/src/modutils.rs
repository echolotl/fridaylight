use crate::models::{ModDisableResult, ModMetadataFile, ModsState, GLOBAL_MODS_STATE};
use log::{debug, error, info, warn};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::Manager;
use serde_json;
use roxmltree;

// Function to update a mod's running state from any thread
pub fn set_mod_not_running(mod_id: &str) {
    // First, update the global state if available
    if let Ok(mut mods) = GLOBAL_MODS_STATE.lock() {
        if let Some(mod_info) = mods.get_mut(mod_id) {
            mod_info.process_id = None;
            log::info!("Updated mod {} to not running status in global state", mod_id);
        }
    }
    
    // Add a log entry to note that the process has finished
    crate::terminaloutput::add_log(mod_id, "[Process terminated]");

    // Also update the ModsState in the Tauri state manager
    if let Some(app_handle) = crate::app_handle::get_global_app_handle() {
        if let Some(mods_state) = app_handle.try_state::<ModsState>() {
            if let Ok(mut mods) = mods_state.0.lock() {
                if let Some(mod_info) = mods.get_mut(mod_id) {
                    mod_info.process_id = None;
                    log::info!("Updated mod {} to not running status in ModsState", mod_id);
                }
            }
        }
    }

    // Emit an event to notify the UI about the process termination
    if let Err(e) = crate::app_handle::emit_event("mod-terminated", mod_id.to_string()) {
        log::error!("Failed to emit mod-terminated event: {}", e);
    } else {
        log::info!("Emitted mod-terminated event for mod: {}", mod_id);
    }
}

//
// FUNCTIONALITY FROM MODENABLER.RS
//

/// Toggle the enabled state of a mod based on engine type
pub fn toggle_mod_enabled_state(
    executable_path: &str,
    mod_folder_path: &str,
    engine_type: &str,
    enable: bool
) -> Result<ModDisableResult, String> {
    info!(
        "Toggling mod state for {} (engine: {}, enable: {}, executable: {})",
        mod_folder_path, engine_type, enable, executable_path
    );

    match engine_type.to_lowercase().as_str() {
        "psych" => toggle_psych_engine_mod(executable_path, mod_folder_path, enable),
        "vanilla" | "fps-plus" => toggle_polymod_mod(mod_folder_path, enable),
        // I don't use Codename Engine often so I'd have to look at how to do this
        "codename" => Err("Disabling Codename Engine mods is not currently supported".to_string()),
        _ => Err(format!("Unsupported engine type: {}", engine_type)),
    }
}

/// Toggle a Psych Engine mod by updating the modsList.txt file
fn toggle_psych_engine_mod(executable_path: &str, mod_folder_path: &str, enable: bool) -> Result<ModDisableResult, String> {
    debug!("Toggling Psych Engine mod: {}", mod_folder_path);
    
    // Get the executable directory
    let exe_dir = Path::new(executable_path).parent().ok_or_else(|| {
        "Could not determine executable's directory".to_string()
    })?;
    
    // Get the mods list file path
    let mods_list_path = exe_dir.join("modsList.txt");
    
    // Get the mod folder name (last path component)
    let mod_folder = Path::new(mod_folder_path)
        .file_name()
        .ok_or_else(|| format!("Invalid mod folder path: {}", mod_folder_path))?
        .to_string_lossy()
        .to_string();
    
    debug!("Mod folder name: {}", mod_folder);
    
    // Flag to track if the mod was found in the list
    let mut found_in_list = false;
    
    // Read existing mods list if available
    let mut mods_list = String::new();
    let mut updated_lines = Vec::new();
    
    if mods_list_path.exists() {
        debug!("Reading existing modsList.txt from: {}", mods_list_path.display());
        
        match fs::read_to_string(&mods_list_path) {
            Ok(content) => {
                mods_list = content;
            },
            Err(e) => {
                warn!("Failed to read modsList.txt, creating new one: {}", e);
                // Will create a new file below
            }
        };
        
        // Process each line
        for line in mods_list.lines() {
            if line.is_empty() {
                updated_lines.push(line.to_string());
                continue;
            }
            
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 1 && parts[0] == mod_folder {
                // Found the mod, update its enabled state
                updated_lines.push(format!("{}|{}", mod_folder, if enable { 1 } else { 0 }));
                found_in_list = true;
            } else {
                // Keep other lines unchanged
                updated_lines.push(line.to_string());
            }
        }
    }
    
    // If mod wasn't found in the list and we're enabling it, add it
    if !found_in_list {
        debug!("Mod not found in list, adding it");
        updated_lines.push(format!("{}|{}", mod_folder, if enable { 1 } else { 0 }));
    }
    
    // Write the updated list back to the file
    match fs::write(&mods_list_path, updated_lines.join("\n")) {
        Ok(_) => {
            info!("Successfully updated modsList.txt");
            Ok(ModDisableResult {
                success: true,
                enabled: enable,
                message: format!("Mod '{}' has been {}", mod_folder, if enable { "enabled" } else { "disabled" }),
            })
        },
        Err(e) => {
            error!("Failed to write modsList.txt: {}", e);
            Err(format!("Failed to update mods list: {}", e))
        }
    }
}

/// Toggle a Polymod mod (Vanilla/FPS Plus) by renaming metadata file
fn toggle_polymod_mod(mod_folder_path: &str, enable: bool) -> Result<ModDisableResult, String> {
    debug!("Toggling Polymod mod: {}", mod_folder_path);
    
    let path = Path::new(mod_folder_path);
    let meta_path = path.join("_polymod_meta.json");
    let disabled_meta_path = path.join("_polymod_meta_disabled.json");
    
    if enable {
        // If enabling, rename _polymod_meta_disabled.json to _polymod_meta.json if it exists
        if disabled_meta_path.exists() {
            debug!("Renaming _polymod_meta_disabled.json to _polymod_meta.json");
            if let Err(e) = fs::rename(&disabled_meta_path, &meta_path) {
                error!("Failed to enable Polymod mod: {}", e);
                return Err(format!("Failed to enable mod: {}", e));
            }
            
            info!("Successfully enabled Polymod mod");
            return Ok(ModDisableResult {
                success: true,
                enabled: true,
                message: "Mod has been enabled".to_string(),
            });
        } else if meta_path.exists() {
            debug!("Mod is already enabled");
            return Ok(ModDisableResult {
                success: true,
                enabled: true,
                message: "Mod is already enabled".to_string(),
            });
        } else {
            return Err("No metadata file found for this mod".to_string());
        }
    } else {
        // If disabling, rename _polymod_meta.json to _polymod_meta_disabled.json if it exists
        if meta_path.exists() {
            debug!("Renaming _polymod_meta.json to _polymod_meta_disabled.json");
            if let Err(e) = fs::rename(&meta_path, &disabled_meta_path) {
                error!("Failed to disable Polymod mod: {}", e);
                return Err(format!("Failed to disable mod: {}", e));
            }
            
            info!("Successfully disabled Polymod mod");
            return Ok(ModDisableResult {
                success: true,
                enabled: false,
                message: "Mod has been disabled".to_string(),
            });
        } else if disabled_meta_path.exists() {
            debug!("Mod is already disabled");
            return Ok(ModDisableResult {
                success: true,
                enabled: false,
                message: "Mod is already disabled".to_string(),
            });
        } else {
            return Err("No metadata file found for this mod".to_string());
        }
    }
}

/// Check if a mod is enabled based on engine type
pub fn check_mod_enabled_state(
    executable_path: &str, 
    mod_folder_path: &str, 
    engine_type: &str
) -> Result<bool, String> {
    match engine_type.to_lowercase().as_str() {
        "psych" => check_psych_engine_mod_enabled(executable_path, mod_folder_path),
        "vanilla" | "fps-plus" => check_polymod_mod_enabled(mod_folder_path),
        "codename" => Ok(true), // Always enabled for Codename Engine
        _ => Err(format!("Unsupported engine type: {}", engine_type)),
    }
}

/// Check if a Psych Engine mod is enabled by reading the modsList.txt file
fn check_psych_engine_mod_enabled(executable_path: &str, mod_folder_path: &str) -> Result<bool, String> {
    // Get the executable directory
    let exe_dir = Path::new(executable_path).parent().ok_or_else(|| {
        "Could not determine executable's directory".to_string()
    })?;
    
    // Get the mods list file path
    let mods_list_path = exe_dir.join("modsList.txt");
    
    // Get the mod folder name
    let mod_folder = Path::new(mod_folder_path)
        .file_name()
        .ok_or_else(|| format!("Invalid mod folder path: {}", mod_folder_path))?
        .to_string_lossy()
        .to_string();
    
    // If the mods list file doesn't exist, all mods are enabled by default
    if !mods_list_path.exists() {
        return Ok(true);
    }
    
    // Read the mods list file
    let mods_list = match fs::read_to_string(&mods_list_path) {
        Ok(content) => content,
        Err(e) => {
            warn!("Failed to read modsList.txt: {}", e);
            return Ok(true); // Assume enabled if can't read file
        }
    };
    
    // Look for the mod in the list
    for line in mods_list.lines() {
        if line.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 2 && parts[0] == mod_folder {
            // Found the mod, check its enabled state
            return Ok(parts[1] == "1");
        }
    }
    
    // If mod not found in the list, it's enabled by default
    Ok(true)
}

/// Check if a Polymod mod is enabled by checking if the metadata file exists
fn check_polymod_mod_enabled(mod_folder_path: &str) -> Result<bool, String> {
    let path = Path::new(mod_folder_path);
    let meta_path = path.join("_polymod_meta.json");
    let disabled_meta_path = path.join("_polymod_meta_disabled.json");
    
    // If _polymod_meta.json exists, the mod is enabled
    if meta_path.exists() {
        return Ok(true);
    }
    
    // If _polymod_meta_disabled.json exists, the mod is disabled
    if disabled_meta_path.exists() {
        return Ok(false);
    }
    
    // If neither file exists, something is wrong
    Err("No metadata file found for this mod".to_string())
}

//
// FUNCTIONALITY FROM MODFILES.RS
//

/// Find and process mod metadata files based on engine type
pub fn find_mod_metadata_files(executable_folder: &Path, engine_type: &str, custom_mods_folder: &Path) -> Result<Vec<ModMetadataFile>, String> {
    let mods_folder: PathBuf; 
    // Check if custom mods folder is an empty string or not
    if custom_mods_folder.to_string_lossy().is_empty() {
        mods_folder = executable_folder.join("mods");
    } else {
        mods_folder = executable_folder.join(custom_mods_folder);
    }
    
    if !mods_folder.exists() || !mods_folder.is_dir() {
        return Err(format!("Mods folder not found: {}", mods_folder.display()));
    }
    
    info!("Searching for mods in {} with engine type: {}", mods_folder.display(), engine_type);
    let mut metadata_files = Vec::new();
    
    // Handle each engine type according to its specific file patterns
    match engine_type.to_lowercase().as_str() {
        "psych" => {
            // For Psych Engine: Look for pack.json and pack.png
            let result = find_psych_engine_mods(&mods_folder)?;
            metadata_files.extend(result);
        },
        "vanilla" | "fps-plus" => {
            // For Vanilla/FPS Plus: Look for _polymod_meta.json and _polymod_icon.png
            let result = find_polymod_mods(&mods_folder)?;
            metadata_files.extend(result);
        },
        "codename" => {
            // For Codename: Look for data/config/credits.xml
            let result = find_codename_mods(&mods_folder)?;
            metadata_files.extend(result);
        },
        _ => {
            return Err(format!("Unsupported engine type: {}", engine_type));
        }
    }
    
    info!("Found {} mod metadata files", metadata_files.len());
    Ok(metadata_files)
}

/// Find Psych Engine mods (looks for pack.json and pack.png)
fn find_psych_engine_mods(mods_folder: &Path) -> Result<Vec<ModMetadataFile>, String> {
    debug!("Searching for Psych Engine mods in {}", mods_folder.display());
    let mut metadata_files = Vec::new();
    
    match fs::read_dir(mods_folder) {
        Ok(entries) => {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                
                if path.is_dir() {
                    debug!("Checking mod folder: {}", path.display());
                    
                    // Look for pack.json
                    let json_path = path.join("pack.json");
                    let mut json_content = None;
                    
                    if json_path.exists() && json_path.is_file() {
                        match fs::read_to_string(&json_path) {
                            Ok(content) => {
                                match serde_json::from_str::<serde_json::Value>(&content) {
                                    Ok(json) => {
                                        json_content = Some(json);
                                        debug!("Found pack.json in {}", path.display());
                                    },
                                    Err(e) => {
                                        warn!("Failed to parse pack.json in {}: {}", path.display(), e);
                                    }
                                }
                            },
                            Err(e) => {
                                warn!("Failed to read pack.json in {}: {}", path.display(), e);
                            }
                        }
                    }
                    
                    // Look for pack.png
                    let icon_path = path.join("pack.png");
                    let icon_exists = icon_path.exists() && icon_path.is_file();
                    
                    if json_content.is_some() || icon_exists {
                        // Get mod name from pack.json if available, otherwise use folder name
                        let name = json_content.as_ref()
                            .and_then(|json| json.get("name").and_then(|v| v.as_str()))
                            .map(|s| s.to_string()) 
                            .unwrap_or_else(|| { 
                                path.file_name()
                                    .map(|n| n.to_string_lossy().to_string()) 
                                    .unwrap_or_else(|| "Unknown Mod".to_string()) 
                            }); 
                        
                        // Extract description from pack.json if available
                        let description = json_content.as_ref().and_then(|json| {
                            json.get("description").and_then(|v| v.as_str()).map(|s| s.to_string())
                        });
                        
                        // Extract restart info if available
                        let restart_info = json_content.as_ref().and_then(|json| {
                            if let Some(restart) = json.get("restart").and_then(|v| v.as_bool()) {
                                if restart {
                                    Some("Requires restart".to_string())
                                } else {
                                    Some("No restart required".to_string())
                                }
                            } else {
                                None
                            }
                        });
                        
                        // Combine description with restart info if available
                        let full_description = match (description, restart_info) {
                            (Some(desc), Some(restart)) => Some(format!("{}\n{}", desc, restart)),
                            (Some(desc), None) => Some(desc),
                            (None, Some(restart)) => Some(restart),
                            (None, None) => None,
                        };
                          metadata_files.push(ModMetadataFile {
                            name,
                            description: full_description,
                            folder_path: path.to_string_lossy().to_string(),
                            config_file_path: if json_content.is_some() { 
                                Some(json_path.to_string_lossy().to_string()) 
                            } else { 
                                None 
                            },
                            icon_file_path: if icon_exists { 
                                Some(icon_path.to_string_lossy().to_string()) 
                            } else { 
                                None 
                            },
                            icon_data: if icon_exists { 
                                Some(get_mod_icon_data(&icon_path.to_string_lossy().to_string()).unwrap_or_default()) 
                            } else { 
                                None 
                            },
                            enabled: None, // Will be set by the command handler
                        });
                    }
                }
            }
        },
        Err(e) => {
            return Err(format!("Failed to read directory {}: {}", mods_folder.display(), e));
        }
    }
    
    Ok(metadata_files)
}

/// Find Polymod mods (Vanilla/FPS Plus) (looks for _polymod_meta.json and _polymod_icon.png)
fn find_polymod_mods(mods_folder: &Path) -> Result<Vec<ModMetadataFile>, String> {
    debug!("Searching for Polymod mods in {}", mods_folder.display());
    let mut metadata_files = Vec::new();
    
    match fs::read_dir(mods_folder) {
        Ok(entries) => {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                
                if path.is_dir() {
                    debug!("Checking mod folder: {}", path.display());
                    
                    // Look for _polymod_meta.json
                    let json_path = path.join("_polymod_meta.json");
                    let mut json_content = None;
                    
                    if json_path.exists() && json_path.is_file() {
                        match fs::read_to_string(&json_path) {
                            Ok(content) => {
                                match serde_json::from_str::<serde_json::Value>(&content) {
                                    Ok(json) => {
                                        json_content = Some(json);
                                        debug!("Found _polymod_meta.json in {}", path.display());
                                    },
                                    Err(e) => {
                                        warn!("Failed to parse _polymod_meta.json in {}: {}", path.display(), e);
                                    }
                                }
                            },
                            Err(e) => {
                                warn!("Failed to read _polymod_meta.json in {}: {}", path.display(), e);
                            }
                        }
                    }
                    
                    // Look for _polymod_icon.png
                    let icon_path = path.join("_polymod_icon.png");
                    let icon_exists = icon_path.exists() && icon_path.is_file();
                    
                    if json_content.is_some() || icon_exists {
                        let title = json_content.as_ref()
                            .and_then(|json| json.get("title").and_then(|v| v.as_str()).map(|s| s.to_string())) // Convert Option<&str> to Option<String>
                            .unwrap_or_else(|| { 
                                path.file_name().map_or_else(
                                    || "Unknown Mod".to_string(), 
                                    |n| n.to_string_lossy().to_string() 
                                )
                            }); // title is now String
                        
                        // Get version information
                        let version = json_content.as_ref()
                            .and_then(|json| json.get("mod_version").and_then(|v| v.as_str()))
                            .map(|v| v.to_string());
                            
                        // Format name with version if available
                        let name = if let Some(v) = &version {
                            format!("{} (v{})", title, v)
                        } else {
                            title.to_string()
                        };
                        
                        // Extract description
                        let base_description = json_content.as_ref()
                            .and_then(|json| json.get("description").and_then(|v| v.as_str()))
                            .map(|s| s.to_string());
                        
                        // Get homepage if available
                        let homepage = json_content.as_ref()
                            .and_then(|json| json.get("homepage").and_then(|v| v.as_str()))
                            .map(|s| s.to_string());
                        
                        // Get license if available
                        let license = json_content.as_ref()
                            .and_then(|json| json.get("license").and_then(|v| v.as_str()))
                            .map(|s| s.to_string());
                        
                        // Format contributor information
                        let mut contributors_info = Vec::new();
                        
                        if let Some(contributors) = json_content.as_ref()
                            .and_then(|json| json.get("contributors").and_then(|v| v.as_array())) {
                            for contributor in contributors {
                                if let Some(name) = contributor.get("name").and_then(|v| v.as_str()) {
                                    let role = contributor.get("role")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("Contributor");
                                        
                                    contributors_info.push(format!("{}: {}", role, name));
                                }
                            }
                        }
                        
                        // Build a comprehensive description
                        let mut description_parts = Vec::new();
                        
                        if let Some(desc) = base_description {
                            description_parts.push(desc);
                        }
                        
                        if let Some(site) = homepage {
                            description_parts.push(format!("Website: {}", site));
                        }
                        
                        if !contributors_info.is_empty() {
                            description_parts.push(format!("Credits: {}", contributors_info.join(", ")));
                        }
                        
                        if let Some(lic) = license {
                            description_parts.push(format!("License: {}", lic));
                        }
                        
                        // Join all parts into a single description
                        let full_description = if !description_parts.is_empty() {
                            Some(description_parts.join("\n"))
                        } else {
                            None
                        };
                          metadata_files.push(ModMetadataFile {
                            name,
                            description: full_description,
                            folder_path: path.to_string_lossy().to_string(),
                            config_file_path: if json_content.is_some() { 
                                Some(json_path.to_string_lossy().to_string()) 
                            } else { 
                                None 
                            },
                            icon_file_path: if icon_exists { 
                                Some(icon_path.to_string_lossy().to_string()) 
                            } else { 
                                None 
                            },
                            icon_data: if icon_exists { 
                                Some(get_mod_icon_data(&icon_path.to_string_lossy().to_string())?) 
                            } else { 
                                None 
                            },
                            enabled: None, // Will be set by the command handler
                        });
                    }
                }
            }
        },
        Err(e) => {
            return Err(format!("Failed to read directory {}: {}", mods_folder.display(), e));
        }
    }
    
    Ok(metadata_files)
}

/// Find Codename Engine mods (looks for data/config/credits.xml)
fn find_codename_mods(mods_folder: &Path) -> Result<Vec<ModMetadataFile>, String> {
    debug!("Searching for Codename Engine mods in {}", mods_folder.display());
    let mut metadata_files = Vec::new();
    
    match fs::read_dir(mods_folder) {
        Ok(entries) => {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                
                if path.is_dir() {
                    debug!("Checking mod folder: {}", path.display());
                    
                    // Look for data/config/credits.xml
                    let credits_path = path.join("data").join("config").join("credits.xml");
                    
                    if credits_path.exists() && credits_path.is_file() {
                        match fs::read_to_string(&credits_path) {
                            Ok(xml_content) => {
                                // Parse XML to extract name and description
                                match roxmltree::Document::parse(&xml_content) {
                                    Ok(doc) => {
                                        // Look for menu element with name and desc attributes
                                        if let Some(menu_elem) = doc.root_element().children().find(|n| n.has_tag_name("menu")) {
                                            let name = menu_elem.attribute("name")
                                                .unwrap_or("Unknown Mod").to_string();
                                            
                                            let description = menu_elem.attribute("desc")
                                                .map(|s| s.to_string());
                                            
                                            debug!("Found credits.xml in {} with name: {}", path.display(), name);
                                              metadata_files.push(ModMetadataFile {
                                                name,
                                                description,
                                                folder_path: path.to_string_lossy().to_string(),
                                                config_file_path: Some(credits_path.to_string_lossy().to_string()),
                                                icon_file_path: None, // Codename doesn't have a standard icon file
                                                icon_data: None,
                                                enabled: None, // Will be set by the command handler
                                            });
                                        } else {
                                            warn!("credits.xml found but no menu element in {}", path.display());
                                            
                                            // Still add the mod but with the folder name
                                            let name = path.file_name().map_or_else(
                                                || "Unknown Mod".to_string(), 
                                                |n| n.to_string_lossy().to_string()
                                            );
                                            
                                            metadata_files.push(ModMetadataFile {
                                                name,
                                                description: None,
                                                folder_path: path.to_string_lossy().to_string(),
                                                config_file_path: Some(credits_path.to_string_lossy().to_string()),
                                                icon_file_path: None,
                                                icon_data: None,
                                                enabled: None, // Will be set by the command handler
                                            });
                                        }
                                    },
                                    Err(e) => {
                                        warn!("Failed to parse credits.xml in {}: {}", path.display(), e);
                                        
                                        // Still add the mod but with the folder name and no description
                                        let name = path.file_name().map_or_else(
                                            || "Unknown Mod".to_string(), 
                                            |n| n.to_string_lossy().to_string()
                                        );
                                        
                                        metadata_files.push(ModMetadataFile {
                                            name,
                                            description: None,
                                            folder_path: path.to_string_lossy().to_string(),
                                            config_file_path: Some(credits_path.to_string_lossy().to_string()),
                                            icon_file_path: None,
                                            icon_data: None,
                                            enabled: None, // Will be set by the command handler
                                        });
                                    }
                                }
                            },
                            Err(e) => {
                                warn!("Failed to read credits.xml in {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
        },
        Err(e) => {
            return Err(format!("Failed to read directory {}: {}", mods_folder.display(), e));
        }
    }
    
    Ok(metadata_files)
}

/// Extract the executable's directory path
pub fn get_executable_directory(executable_path: &str) -> Result<PathBuf, String> {
    let path = Path::new(executable_path);
    
    match path.parent() {
        Some(dir) => Ok(dir.to_path_buf()),
        None => Err(format!("Cannot determine parent directory for {}", executable_path)),
    }
}

/// Fetch an image and encode it as base64
pub fn get_mod_icon_data(file_path: &str) -> Result<String, String> {
    debug!("Loading mod icon from: {}", file_path);
    let path = Path::new(file_path);
    match crate::utils::encode_file_to_data_url(path) {
        Some(data_url) => Ok(data_url),
        None => {
            warn!("Failed to read image file: {}", file_path);
            Err(format!("Failed to read image file: {}", file_path))
        }
    }
}