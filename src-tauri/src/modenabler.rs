use crate::models::ModDisableResult;
use log::{debug, error, info, warn};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

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
