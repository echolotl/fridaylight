use crate::models::{
  ModDisableResult,
  ModInfo,
  ModMetadataFile,
  ModsState,
  GLOBAL_MODS_STATE,
};
use log::{ debug, error, info, warn };
use std::fs;
use std::io::Read;
use std::path::{ Path, PathBuf };
use std::collections::HashMap;
use tauri::Manager;
use serde_json;
use roxmltree;
use zip::ZipArchive;
use std::fs::File;
use regex::Regex;

// Function to update a mod's running state from any thread
pub fn set_mod_not_running(mod_id: &str) {
  // First, update the global state if available
  if let Ok(mut mods) = GLOBAL_MODS_STATE.lock() {
    if let Some(mod_info) = mods.get_mut(mod_id) {
      mod_info.process_id = None;
      log::info!(
        "Updated mod {} to not running status in global state",
        mod_id
      );
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
          log::info!(
            "Updated mod {} to not running status in ModsState",
            mod_id
          );
        }
      }
    }
  }

  // Emit an event to notify the UI about the process termination
  if
    let Err(e) = crate::app_handle::emit_event(
      "mod-terminated",
      mod_id.to_string()
    )
  {
    log::error!("Failed to emit mod-terminated event: {}", e);
  } else {
    log::info!("Emitted mod-terminated event for mod: {}", mod_id);
  }
}

/// Toggle the enabled state of a mod based on engine type
pub fn toggle_mod_enabled_state(
  executable_path: &str,
  mod_folder_path: &str,
  engine_type: &str,
  enable: bool
) -> Result<ModDisableResult, String> {
  info!(
    "Toggling mod state for {} (engine: {}, enable: {}, executable: {})",
    mod_folder_path,
    engine_type,
    enable,
    executable_path
  );

  match engine_type.to_lowercase().as_str() {
    "psych" =>
      toggle_psych_engine_mod(executable_path, mod_folder_path, enable),
    "vanilla" => toggle_polymod_mod(mod_folder_path, enable),
    "fps-plus" => toggle_fpsplus_polymod_mod(mod_folder_path, enable),
    // TODO: Implement Codename Engine mod toggling (and launching??)
    "codename" =>
      Err(
        "Disabling Codename Engine mods is not currently supported".to_string()
      ),
    _ => Err(format!("Unsupported engine type: {}", engine_type)),
  }
}

/// Toggle a Psych Engine mod by updating the modsList.txt file
fn toggle_psych_engine_mod(
  executable_path: &str,
  mod_folder_path: &str,
  enable: bool
) -> Result<ModDisableResult, String> {
  debug!("Toggling Psych Engine mod: {}", mod_folder_path);

  // Get the executable directory
  let exe_dir = Path::new(executable_path)
    .parent()
    .ok_or_else(|| {
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
      }
      Err(e) => {
        warn!("Failed to read modsList.txt, creating new one: {}", e);
        // Will create a new file below
      }
    }

    // Process each line
    for line in mods_list.lines() {
      if line.is_empty() {
        updated_lines.push(line.to_string());
        continue;
      }

      let parts: Vec<&str> = line.split('|').collect();
      if parts.len() >= 1 && parts[0] == mod_folder {
        // Found the mod, update its enabled state
        updated_lines.push(
          format!("{}|{}", mod_folder, if enable { 1 } else { 0 })
        );
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
    updated_lines.push(
      format!("{}|{}", mod_folder, if enable { 1 } else { 0 })
    );
  }

  // Write the updated list back to the file
  match fs::write(&mods_list_path, updated_lines.join("\n")) {
    Ok(_) => {
      info!("Successfully updated modsList.txt");
      Ok(ModDisableResult {
        success: true,
        enabled: enable,
        message: format!("Mod '{}' has been {}", mod_folder, if enable {
          "enabled"
        } else {
          "disabled"
        }),
      })
    }
    Err(e) => {
      error!("Failed to write modsList.txt: {}", e);
      Err(format!("Failed to update mods list: {}", e))
    }
  }
}

/// Toggle a Polymod mod (Vanilla/FPS Plus) by renaming metadata file
fn toggle_polymod_mod(
  mod_folder_path: &str,
  enable: bool
) -> Result<ModDisableResult, String> {
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

/// Toggle a FPS Plus mod (similar to Polymod but with different file names)
fn toggle_fpsplus_polymod_mod(
  mod_folder_path: &str,
  enable: bool
) -> Result<ModDisableResult, String> {
  debug!("Toggling FPS Plus mod: {}", mod_folder_path);

  let path = Path::new(mod_folder_path);
  let meta_path = path.join("meta.json");
  let disabled_meta_path = path.join("meta_disabled.json");

  if enable {
    // If enabling, rename meta_disabled.json to meta.json if it exists
    if disabled_meta_path.exists() {
      debug!("Renaming meta_disabled.json to meta.json");
      if let Err(e) = fs::rename(&disabled_meta_path, &meta_path) {
        error!("Failed to enable FPS Plus mod: {}", e);
        return Err(format!("Failed to enable mod: {}", e));
      }

      info!("Successfully enabled FPS Plus mod");
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
    // If disabling, rename meta.json to meta_disabled.json if it exists
    if meta_path.exists() {
      debug!("Renaming meta.json to meta_disabled.json");
      if let Err(e) = fs::rename(&meta_path, &disabled_meta_path) {
        error!("Failed to disable FPS Plus mod: {}", e);
        return Err(format!("Failed to disable mod: {}", e));
      }

      info!("Successfully disabled FPS Plus mod");
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
    "vanilla" => check_polymod_mod_enabled(mod_folder_path),
    "fps-plus" => check_fpsplus_polymod_mod_enabled(mod_folder_path),
    "codename" => Ok(true), // Always enabled for Codename Engine
    _ => Err(format!("Unsupported engine type: {}", engine_type)),
  }
}

/// Check if a Psych Engine mod is enabled by reading the modsList.txt file
fn check_psych_engine_mod_enabled(
  executable_path: &str,
  mod_folder_path: &str
) -> Result<bool, String> {
  // Get the executable directory
  let exe_dir = Path::new(executable_path)
    .parent()
    .ok_or_else(|| {
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

/// Check if a FPS Plus mod is enabled by checking if the metadata file exists
/// This is similar to the Polymod function but with different file names
fn check_fpsplus_polymod_mod_enabled(
  mod_folder_path: &str
) -> Result<bool, String> {
  let path = Path::new(mod_folder_path);
  let meta_path = path.join("meta.json");
  let disabled_meta_path = path.join("meta_disabled.json");

  // If meta.json exists, the mod is enabled
  if meta_path.exists() {
    return Ok(true);
  }

  // If meta_disabled.json exists, the mod is disabled
  if disabled_meta_path.exists() {
    return Ok(false);
  }

  // If neither file exists, something is wrong
  Err("No metadata file found for this mod".to_string())
}

/// Find and process mod metadata files based on engine type
pub fn find_mod_metadata_files(
  executable_folder: &Path,
  engine_type: &str,
  custom_mods_folder: &Path
) -> Result<Vec<ModMetadataFile>, String> {
  let mods_folder: PathBuf;
  // Check if custom mods folder is an empty string or not
  if custom_mods_folder.to_string_lossy().is_empty() {
    mods_folder = executable_folder.join("mods");
  } else {
    mods_folder = executable_folder.join(custom_mods_folder);
  }

  // For Codename Engine, we have to check mods and addons folders
  // so only error if we're not dealing with Codename or the folder doesn't exist
  let is_codename = engine_type.to_lowercase() == "codename";

  if (!mods_folder.exists() || !mods_folder.is_dir()) && !is_codename {
    return Err(format!("Mods folder not found: {}", mods_folder.display()));
  } else if (!mods_folder.exists() || !mods_folder.is_dir()) && is_codename {
    // For Codename, try addons folder if mods folder doesn't exist
    let addons_folder = executable_folder.join("addons");
    if !addons_folder.exists() || !addons_folder.is_dir() {
      return Err(
        format!(
          "Neither mods nor addons folders found: {} / {}",
          mods_folder.display(),
          addons_folder.display()
        )
      );
    }
  }

  info!(
    "Searching for mods in {} with engine type: {}",
    mods_folder.display(),
    engine_type
  );
  let mut metadata_files = Vec::new();

  // Handle each engine type according to its specific file patterns
  match engine_type.to_lowercase().as_str() {
    "psych" => {
      // For Psych Engine: Look for pack.json and pack.png
      let result = find_psych_engine_mods(&mods_folder)?;
      metadata_files.extend(result);
    }
    "vanilla" => {
      // For Vanilla: Look for _polymod_meta.json and _polymod_icon.png
      let result = find_polymod_mods(&mods_folder)?;
      metadata_files.extend(result);
    }
    "fps-plus" => {
      // For FPS Plus: Look for meta.json and icon.png
      let result = find_fpsplus_polymod_mods(&mods_folder)?;
      metadata_files.extend(result);
    }
    "codename" => {
      // For Codename: Look for data/config/credits.xml in both mods and addons folders

      // First check the mods folder if it exists
      if mods_folder.exists() && mods_folder.is_dir() {
        let result = find_codename_mods(&mods_folder)?;
        metadata_files.extend(result);
      } else {
        debug!("Mods folder not found at {}, skipping", mods_folder.display());
      }

      // Then also check the addons folder
      let addons_folder = executable_folder.join("addons");
      if addons_folder.exists() && addons_folder.is_dir() {
        info!(
          "Also searching for Codename Engine mods in {}",
          addons_folder.display()
        );
        let addons_result = find_codename_mods(&addons_folder)?;
        metadata_files.extend(addons_result);
      } else {
        debug!(
          "Addons folder not found at {}, skipping",
          addons_folder.display()
        );
      }
    }
    _ => {
      return Err(format!("Unsupported engine type: {}", engine_type));
    }
  }

  info!("Found {} mod metadata files", metadata_files.len());
  Ok(metadata_files)
}

/// Find Psych Engine mods (looks for pack.json and pack.png)
fn find_psych_engine_mods(
  mods_folder: &Path
) -> Result<Vec<ModMetadataFile>, String> {
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
                  }
                  Err(e) => {
                    warn!(
                      "Failed to parse pack.json in {}: {}",
                      path.display(),
                      e
                    );
                  }
                }
              }
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
            let name = json_content
              .as_ref()
              .and_then(|json| json.get("name").and_then(|v| v.as_str()))
              .map(|s| s.to_string())
              .unwrap_or_else(|| {
                path
                  .file_name()
                  .map(|n| n.to_string_lossy().to_string())
                  .unwrap_or_else(|| "Unknown Mod".to_string())
              });

            // Extract description from pack.json if available
            let description = json_content.as_ref().and_then(|json| {
              json
                .get("description")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
            });

            // Extract restart info if available
            let restart_required = json_content
              .as_ref()
              .and_then(|json| json.get("restart").and_then(|v| v.as_bool()));

            metadata_files.push(ModMetadataFile {
              name,
              description,
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
                Some(
                  get_mod_icon_data(
                    &icon_path.to_string_lossy().to_string()
                  ).unwrap_or_default()
                )
              } else {
                None
              },
              enabled: None, // Will be set by the command handler
              version: None,
              homepage: None,
              contributors: None,
              license: None,
              restart_required,
              dependencies: None,
              parent_mod_id: None, // Not needed right now
            });
          }
        } else if is_archive(&path) {
          debug!("Checking archive: {}", path.display());

          // Try to find pack.json in the archive
          let json_content = read_json_from_archive(&path, "pack.json");
          let icon_exists = file_exists_in_archive(&path, "pack.png");

          if json_content.is_some() || icon_exists {
            // Get the archive name without extension to use as mod folder name
            let archive_name = path
              .file_stem()
              .map(|n| n.to_string_lossy().to_string())
              .unwrap_or_else(|| "Unknown Archive".to_string());

            // Get mod name from pack.json if available, otherwise use archive name
            let name = json_content
              .as_ref()
              .and_then(|json| json.get("name").and_then(|v| v.as_str()))
              .map(|s| s.to_string())
              .unwrap_or_else(|| archive_name.clone());

            // Extract description from pack.json if available
            let description = json_content.as_ref().and_then(|json| {
              json
                .get("description")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
            });

            // Extract restart info if available
            let restart_required = json_content
              .as_ref()
              .and_then(|json| json.get("restart").and_then(|v| v.as_bool())); // Icon from archive needs special handling - we don't extract it
            let icon_data = if icon_exists {
              read_binary_from_archive(&path, "pack.png").and_then(|_| {
                Some("data:image/png;base64,".to_string())
              })
            } else {
              None
            };

            metadata_files.push(ModMetadataFile {
              name,
              description,
              folder_path: path.to_string_lossy().to_string(), // Use archive path as folder path
              config_file_path: Some(path.to_string_lossy().to_string()), // Use archive path as config file path too
              icon_file_path: if icon_exists {
                Some(format!("{}:pack.png", path.to_string_lossy().to_string()))
              } else {
                None
              },
              icon_data,
              enabled: None, // Will be set by the command handler
              version: None,
              homepage: None,
              contributors: None,
              license: None,
              restart_required,
              dependencies: None,
              parent_mod_id: None, // Not needed right now
            });

            debug!("Found Psych Engine mod in archive: {}", path.display());
          }
        }
      }
    }
    Err(e) => {
      return Err(
        format!("Failed to read directory {}: {}", mods_folder.display(), e)
      );
    }
  }

  Ok(metadata_files)
}

/// Find Polymod mods (Vanilla) (looks for _polymod_meta.json/_polymod_meta_disabled.json and _polymod_icon.png)
fn find_polymod_mods(
  mods_folder: &Path
) -> Result<Vec<ModMetadataFile>, String> {
  debug!("Searching for Polymod mods in {}", mods_folder.display());
  let mut metadata_files = Vec::new();

  match fs::read_dir(mods_folder) {
    Ok(entries) => {
      for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_dir() {
          debug!("Checking mod folder: {}", path.display());

          // Look for _polymod_meta.json or _polymod_meta_disabled.json
          let json_path = path.join("_polymod_meta.json");
          let disabled_json_path = path.join("_polymod_meta_disabled.json");
          let mut json_content = None;
          let mut is_disabled = false;

          // First check for _polymod_meta.json (enabled mods)
          if json_path.exists() && json_path.is_file() {
            match fs::read_to_string(&json_path) {
              Ok(content) => {
                match serde_json::from_str::<serde_json::Value>(&content) {
                  Ok(json) => {
                    json_content = Some(json);
                    debug!("Found _polymod_meta.json in {}", path.display());
                  }
                  Err(e) => {
                    warn!(
                      "Failed to parse _polymod_meta.json in {}: {}",
                      path.display(),
                      e
                    );
                  }
                }
              }
              Err(e) => {
                warn!(
                  "Failed to read _polymod_meta.json in {}: {}",
                  path.display(),
                  e
                );
              }
            }
          } else if
            // Then check for _polymod_meta_disabled.json (disabled mods)
            disabled_json_path.exists() &&
            disabled_json_path.is_file()
          {
            match fs::read_to_string(&disabled_json_path) {
              Ok(content) => {
                match serde_json::from_str::<serde_json::Value>(&content) {
                  Ok(json) => {
                    json_content = Some(json);
                    is_disabled = true;
                    debug!(
                      "Found _polymod_meta_disabled.json in {}",
                      path.display()
                    );
                  }
                  Err(e) => {
                    warn!(
                      "Failed to parse _polymod_meta_disabled.json in {}: {}",
                      path.display(),
                      e
                    );
                  }
                }
              }
              Err(e) => {
                warn!(
                  "Failed to read _polymod_meta_disabled.json in {}: {}",
                  path.display(),
                  e
                );
              }
            }
          }

          // Look for _polymod_icon.png
          let icon_path = path.join("_polymod_icon.png");
          let icon_exists = icon_path.exists() && icon_path.is_file();

          if json_content.is_some() || icon_exists {
            let title = json_content
              .as_ref()
              .and_then(|json|
                json
                  .get("title")
                  .and_then(|v| v.as_str())
                  .map(|s| s.to_string())
              ) // Convert Option<&str> to Option<String>
              .unwrap_or_else(|| {
                path.file_name().map_or_else(
                  || "Unknown Mod".to_string(),
                  |n| n.to_string_lossy().to_string()
                )
              }); // title is now String

            // Get version information
            let version = json_content
              .as_ref()
              .and_then(|json| json.get("mod_version").and_then(|v| v.as_str()))
              .map(|v| v.to_string());

            // Extract description
            let base_description = json_content
              .as_ref()
              .and_then(|json| json.get("description").and_then(|v| v.as_str()))
              .map(|s| s.to_string());

            // Get homepage if available
            let homepage = json_content
              .as_ref()
              .and_then(|json| json.get("homepage").and_then(|v| v.as_str()))
              .map(|s| s.to_string());

            // Get license if available
            let license = json_content
              .as_ref()
              .and_then(|json| json.get("license").and_then(|v| v.as_str()))
              .map(|s| s.to_string());

            // Extract dependencies information
            let dependencies = json_content
              .as_ref()
              .and_then(|json|
                json.get("dependencies").and_then(|v| v.as_object())
              )
              .map(|deps_obj| {
                let mut deps_map = HashMap::new();
                for (key, value) in deps_obj {
                  if let Some(version_req) = value.as_str() {
                    deps_map.insert(key.clone(), version_req.to_string());
                  }
                }
                deps_map
              })
              .filter(|m| !m.is_empty());

            // Extract contributor information
            let mut contributors_info = Vec::new();

            if
              let Some(contributors) = json_content
                .as_ref()
                .and_then(|json|
                  json.get("contributors").and_then(|v| v.as_array())
                )
            {
              for contributor in contributors {
                if
                  let Some(name) = contributor
                    .get("name")
                    .and_then(|v| v.as_str())
                {
                  let role = contributor
                    .get("role")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Contributor");
                  contributors_info.push(crate::models::ContributorMetadata {
                    name: name.to_string(),
                    role: role.to_string(),
                    email: contributor
                      .get("email")
                      .and_then(|v| v.as_str())
                      .map(|s| s.to_string()),
                    url: contributor
                      .get("url")
                      .and_then(|v| v.as_str())
                      .map(|s| s.to_string()),
                  });
                }
              }
            }

            let collected_contributors = if !contributors_info.is_empty() {
              Some(contributors_info)
            } else {
              None
            }; // Determine the config file path based on whether it's the normal or disabled version
            let config_path = if is_disabled {
              disabled_json_path.to_string_lossy().to_string()
            } else if json_content.is_some() {
              json_path.to_string_lossy().to_string()
            } else {
              String::new()
            };

            metadata_files.push(ModMetadataFile {
              name: title,
              description: base_description,
              folder_path: path.to_string_lossy().to_string(),
              config_file_path: if !config_path.is_empty() {
                Some(config_path)
              } else {
                None
              },
              icon_file_path: if icon_exists {
                Some(icon_path.to_string_lossy().to_string())
              } else {
                None
              },
              icon_data: if icon_exists {
                Some(
                  get_mod_icon_data(&icon_path.to_string_lossy().to_string())?
                )
              } else {
                None
              },
              enabled: Some(!is_disabled), // Set the enabled state based on which file we found
              version,
              homepage,
              contributors: collected_contributors,
              license,
              dependencies,
              restart_required: None, // Polymod doesn't have restart info
              parent_mod_id: None, // Not needed right now
            });
          }
        } else if is_archive(&path) {
          debug!("Checking archive for Polymod mod: {}", path.display());

          // Try to find _polymod_meta.json in the archive
          let json_content = read_json_from_archive(
            &path,
            "_polymod_meta.json"
          );
          // Check for icons
          let icon_exists = file_exists_in_archive(&path, "_polymod_icon.png");

          if json_content.is_some() || icon_exists {
            // Get the archive name without extension to use as mod folder name
            let archive_name = path
              .file_stem()
              .map(|n| n.to_string_lossy().to_string())
              .unwrap_or_else(|| "Unknown Archive".to_string());

            // Get mod name from metadata if available, otherwise use archive name
            let title = json_content
              .as_ref()
              .and_then(|json| json.get("title").and_then(|v| v.as_str()))
              .map(|s| s.to_string())
              .unwrap_or_else(|| archive_name.clone());

            // Extract other metadata if available
            let version = json_content
              .as_ref()
              .and_then(|json| json.get("mod_version").and_then(|v| v.as_str()))
              .map(|v| v.to_string());

            let base_description = json_content
              .as_ref()
              .and_then(|json| json.get("description").and_then(|v| v.as_str()))
              .map(|s| s.to_string());

            let homepage = json_content
              .as_ref()
              .and_then(|json| json.get("homepage").and_then(|v| v.as_str()))
              .map(|s| s.to_string());

            let license = json_content
              .as_ref()
              .and_then(|json| json.get("license").and_then(|v| v.as_str()))
              .map(|s| s.to_string());

            // Dependencies
            let dependencies = json_content
              .as_ref()
              .and_then(|json|
                json.get("dependencies").and_then(|v| v.as_object())
              )
              .map(|deps_obj| {
                let mut deps_map = HashMap::new();
                for (key, value) in deps_obj {
                  if let Some(version_req) = value.as_str() {
                    deps_map.insert(key.clone(), version_req.to_string());
                  }
                }
                deps_map
              })
              .filter(|m| !m.is_empty());

            // Extract contributor information
            let mut contributors_info = Vec::new();
            if
              let Some(contributors) = json_content
                .as_ref()
                .and_then(|json|
                  json.get("contributors").and_then(|v| v.as_array())
                )
            {
              for contributor in contributors {
                if
                  let Some(name) = contributor
                    .get("name")
                    .and_then(|v| v.as_str())
                {
                  let role = contributor
                    .get("role")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Contributor");
                  contributors_info.push(crate::models::ContributorMetadata {
                    name: name.to_string(),
                    role: role.to_string(),
                    email: contributor
                      .get("email")
                      .and_then(|v| v.as_str())
                      .map(|s| s.to_string()),
                    url: contributor
                      .get("url")
                      .and_then(|v| v.as_str())
                      .map(|s| s.to_string()),
                  });
                }
              }
            }

            let collected_contributors = if !contributors_info.is_empty() {
              Some(contributors_info)
            } else {
              None
            };

            // Icon from archive
            let icon_data = if icon_exists {
              read_binary_from_archive(&path, "_polymod_icon.png").and_then(
                |_| {
                  // Just placeholder handling for icon data in archives
                  Some("data:image/png;base64,".to_string())
                }
              )
            } else {
              None
            };

            metadata_files.push(ModMetadataFile {
              name: title,
              description: base_description,
              folder_path: path.to_string_lossy().to_string(), // Use archive path as folder path
              config_file_path: Some(path.to_string_lossy().to_string()), // Use archive path as config file path
              icon_file_path: if icon_exists {
                Some(
                  format!(
                    "{}:_polymod_icon.png",
                    path.to_string_lossy().to_string()
                  )
                )
              } else {
                None
              },
              icon_data,
              enabled: Some(true), // Default to enabled for archives
              version,
              homepage,
              contributors: collected_contributors,
              license,
              dependencies,
              restart_required: None,
              parent_mod_id: None, // Not needed right now
            });

            debug!("Found Polymod mod in archive: {}", path.display());
          }
        }
      }
    }
    Err(e) => {
      return Err(
        format!("Failed to read directory {}: {}", mods_folder.display(), e)
      );
    }
  }

  Ok(metadata_files)
}

/// Find FPS Plus mods (looks for meta.json/meta_disabled.json and icon.png)
/// This is similar to the Polymod function but with different file names

fn find_fpsplus_polymod_mods(
  mods_folder: &Path
) -> Result<Vec<ModMetadataFile>, String> {
  debug!("Searching for FPS Plus mods in {}", mods_folder.display());
  let mut metadata_files = Vec::new();

  match fs::read_dir(mods_folder) {
    Ok(entries) => {
      for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_dir() {
          debug!("Checking mod folder: {}", path.display());

          // Look for meta.json or meta_disabled.json
          let json_path = path.join("meta.json");
          let disabled_json_path = path.join("meta_disabled.json");
          let mut json_content = None;
          let mut is_disabled = false;

          // First check for meta.json (enabled mods)
          if json_path.exists() && json_path.is_file() {
            match fs::read_to_string(&json_path) {
              Ok(content) => {
                match serde_json::from_str::<serde_json::Value>(&content) {
                  Ok(json) => {
                    json_content = Some(json);
                    debug!("Found meta.json in {}", path.display());
                  }
                  Err(e) => {
                    warn!(
                      "Failed to parse meta.json in {}: {}",
                      path.display(),
                      e
                    );
                  }
                }
              }
              Err(e) => {
                warn!("Failed to read meta.json in {}: {}", path.display(), e);
              }
            }
          } else if
            // Then check for meta_disabled.json (disabled mods)
            disabled_json_path.exists() &&
            disabled_json_path.is_file()
          {
            match fs::read_to_string(&disabled_json_path) {
              Ok(content) => {
                match serde_json::from_str::<serde_json::Value>(&content) {
                  Ok(json) => {
                    json_content = Some(json);
                    is_disabled = true;
                    debug!("Found meta_disabled.json in {}", path.display());
                  }
                  Err(e) => {
                    warn!(
                      "Failed to parse meta_disabled.json in {}: {}",
                      path.display(),
                      e
                    );
                  }
                }
              }
              Err(e) => {
                warn!(
                  "Failed to read meta_disabled.json in {}: {}",
                  path.display(),
                  e
                );
              }
            }
          }

          // Look for icon.png
          let icon_path = path.join("icon.png");
          let icon_exists = icon_path.exists() && icon_path.is_file();

          if json_content.is_some() || icon_exists {
            let title = json_content
              .as_ref()
              .and_then(|json|
                json
                  .get("title")
                  .and_then(|v| v.as_str())
                  .map(|s| s.to_string())
              )
              .unwrap_or_else(|| {
                path.file_name().map_or_else(
                  || "Unknown Mod".to_string(),
                  |n| n.to_string_lossy().to_string()
                )
              });

            // Get version information
            let version = json_content
              .as_ref()
              .and_then(|json| json.get("mod_version").and_then(|v| v.as_str()))
              .map(|v| v.to_string());

            // Extract description
            let base_description = json_content
              .as_ref()
              .and_then(|json| json.get("description").and_then(|v| v.as_str()))
              .map(|s| s.to_string());

            // Get homepage if available
            let homepage = json_content
              .as_ref()
              .and_then(|json| json.get("homepage").and_then(|v| v.as_str()))
              .map(|s| s.to_string());
            // Get license if available
            let license = json_content
              .as_ref()
              .and_then(|json| json.get("license").and_then(|v| v.as_str()))
              .map(|s| s.to_string());

            // Extract dependencies information
            let dependencies = json_content
              .as_ref()
              .and_then(|json|
                json.get("dependencies").and_then(|v| v.as_object())
              )
              .map(|deps_obj| {
                let mut deps_map = HashMap::new();
                for (key, value) in deps_obj {
                  if let Some(version_req) = value.as_str() {
                    deps_map.insert(key.clone(), version_req.to_string());
                  }
                }
                deps_map
              })
              .filter(|m| !m.is_empty());

            // Extract contributor information
            let mut contributors_info = Vec::new();

            if
              let Some(contributors) = json_content
                .as_ref()
                .and_then(|json|
                  json.get("contributors").and_then(|v| v.as_array())
                )
            {
              for contributor in contributors {
                if
                  let Some(name) = contributor
                    .get("name")
                    .and_then(|v| v.as_str())
                {
                  let role = contributor
                    .get("role")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Contributor");
                  // Construct ContributorMetadata struct
                  contributors_info.push(crate::models::ContributorMetadata {
                    name: name.to_string(),
                    role: role.to_string(),
                    email: contributor
                      .get("email")
                      .and_then(|v| v.as_str())
                      .map(|s| s.to_string()),
                    url: contributor
                      .get("url")
                      .and_then(|v| v.as_str())
                      .map(|s| s.to_string()),
                  });
                }
              }
            }

            let collected_contributors = if !contributors_info.is_empty() {
              Some(contributors_info)
            } else {
              None
            };

            // Determine the config file path based on whether it's the normal or disabled version
            let config_path = if is_disabled {
              disabled_json_path.to_string_lossy().to_string()
            } else if json_content.is_some() {
              json_path.to_string_lossy().to_string()
            } else {
              String::new()
            };

            metadata_files.push(ModMetadataFile {
              name: title,
              description: base_description,
              folder_path: path.to_string_lossy().to_string(),
              config_file_path: if !config_path.is_empty() {
                Some(config_path)
              } else {
                None
              },
              icon_file_path: if icon_exists {
                Some(icon_path.to_string_lossy().to_string())
              } else {
                None
              },
              icon_data: if icon_exists {
                Some(
                  get_mod_icon_data(&icon_path.to_string_lossy().to_string())?
                )
              } else {
                None
              },
              enabled: Some(!is_disabled), // Set the enabled state based on which file we found
              version,
              homepage,
              contributors: collected_contributors,
              license,
              dependencies,
              restart_required: None, // Polymod doesn't have restart info
              parent_mod_id: None, // Not needed right now
            });
          }
        } else if is_archive(&path) {
          debug!("Checking archive for FPS Plus mod: {}", path.display());

          // Try to find meta.json in the archive
          let json_content = read_json_from_archive(&path, "meta.json");
          // Check for icons
          let icon_exists = file_exists_in_archive(&path, "icon.png");

          if json_content.is_some() || icon_exists {
            // Get the archive name without extension to use as mod folder name
            let archive_name = path
              .file_stem()
              .map(|n| n.to_string_lossy().to_string())
              .unwrap_or_else(|| "Unknown Archive".to_string());

            // Get mod name from metadata if available, otherwise use archive name
            let title = json_content
              .as_ref()
              .and_then(|json| json.get("title").and_then(|v| v.as_str()))
              .map(|s| s.to_string())
              .unwrap_or_else(|| archive_name.clone());

            // Extract other metadata if available
            let version = json_content
              .as_ref()
              .and_then(|json| json.get("mod_version").and_then(|v| v.as_str()))
              .map(|v| v.to_string());

            let base_description = json_content
              .as_ref()
              .and_then(|json| json.get("description").and_then(|v| v.as_str()))
              .map(|s| s.to_string());

            let homepage = json_content
              .as_ref()
              .and_then(|json| json.get("homepage").and_then(|v| v.as_str()))
              .map(|s| s.to_string());

            let license = json_content
              .as_ref()
              .and_then(|json| json.get("license").and_then(|v| v.as_str()))
              .map(|s| s.to_string());

            // Dependencies
            let dependencies = json_content
              .as_ref()
              .and_then(|json|
                json.get("dependencies").and_then(|v| v.as_object())
              )
              .map(|deps_obj| {
                let mut deps_map = HashMap::new();
                for (key, value) in deps_obj {
                  if let Some(version_req) = value.as_str() {
                    deps_map.insert(key.clone(), version_req.to_string());
                  }
                }
                deps_map
              })
              .filter(|m| !m.is_empty());

            // Extract contributor information
            let mut contributors_info = Vec::new();
            if
              let Some(contributors) = json_content
                .as_ref()
                .and_then(|json|
                  json.get("contributors").and_then(|v| v.as_array())
                )
            {
              for contributor in contributors {
                if
                  let Some(name) = contributor
                    .get("name")
                    .and_then(|v| v.as_str())
                {
                  let role = contributor
                    .get("role")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Contributor");
                  contributors_info.push(crate::models::ContributorMetadata {
                    name: name.to_string(),
                    role: role.to_string(),
                    email: contributor
                      .get("email")
                      .and_then(|v| v.as_str())
                      .map(|s| s.to_string()),
                    url: contributor
                      .get("url")
                      .and_then(|v| v.as_str())
                      .map(|s| s.to_string()),
                  });
                }
              }
            }

            let collected_contributors = if !contributors_info.is_empty() {
              Some(contributors_info)
            } else {
              None
            };

            // Icon from archive
            let icon_data = if icon_exists {
              read_binary_from_archive(&path, "icon.png").and_then(|_| {
                // Just placeholder handling for icon data in archives
                Some("data:image/png;base64,".to_string())
              })
            } else {
              None
            };

            metadata_files.push(ModMetadataFile {
              name: title,
              description: base_description,
              folder_path: path.to_string_lossy().to_string(), // Use archive path as folder path
              config_file_path: Some(path.to_string_lossy().to_string()), // Use archive path as config file path
              icon_file_path: if icon_exists {
                Some(format!("{}:icon.png", path.to_string_lossy().to_string()))
              } else {
                None
              },
              icon_data,
              enabled: Some(true), // Default to enabled for archives
              version,
              homepage,
              contributors: collected_contributors,
              license,
              dependencies,
              restart_required: None,
              parent_mod_id: None, // Not needed right now
            });

            debug!("Found FPS Plus mod in archive: {}", path.display());
          }
        }
      }
    }
    Err(e) => {
      return Err(
        format!("Failed to read directory {}: {}", mods_folder.display(), e)
      );
    }
  }

  Ok(metadata_files)
}

/// Find Codename Engine mods (looks for data/config/credits.xml)
fn find_codename_mods(
  mods_folder: &Path
) -> Result<Vec<ModMetadataFile>, String> {
  debug!("Searching for codename engine mods in {}", mods_folder.display());
  let mut metadata_files = Vec::new();

  match fs::read_dir(mods_folder) {
    Ok(entries) => {
      for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_dir() {
          // Codename engine mods don't have names or descriptions

          // Name is just the folder name
          let name = path.file_name().map_or_else(
            || "Unknown Mod".to_string(),
            |n| n.to_string_lossy().to_string()
          );

          // Check for data/config/credits.xml
          let credits_path = path
            .join("data")
            .join("config")
            .join("credits.xml");
          if credits_path.exists() && credits_path.is_file() {
            debug!("Found credits.xml in {}", path.display());
            // Parse the XML file using roxmltree
            let xml_content = match fs::read_to_string(&credits_path) {
              Ok(content) => content,
              Err(e) => {
                warn!(
                  "Failed to read credits.xml in {}: {}",
                  path.display(),
                  e
                );
                continue;
              }
            };
            let _xml_doc = match roxmltree::Document::parse(&xml_content) {
              Ok(doc) => doc,
              Err(e) => {
                warn!(
                  "Failed to parse credits.xml in {}: {}",
                  path.display(),
                  e
                );
                continue;
              }
            };
            // TODO: Actually parse the XML content to extract contributors
            // Codename has a different credits structure, very unlike Polymod mods

            metadata_files.push(ModMetadataFile {
              name,
              description: None,
              folder_path: path.to_string_lossy().to_string(),
              config_file_path: None,
              icon_file_path: None,
              icon_data: None,
              enabled: Some(true), // Default to enabled for Codename mods
              version: None,
              homepage: None,
              contributors: None,
              license: None,
              dependencies: None,
              restart_required: None,
              parent_mod_id: None, // Not needed right now
            });
          } else {
            debug!("No credits.xml found in {}", path.display());
            metadata_files.push(ModMetadataFile {
              name,
              description: None,
              folder_path: path.to_string_lossy().to_string(),
              config_file_path: None,
              icon_file_path: None,
              icon_data: None,
              enabled: Some(true), // Default to enabled for Codename mods
              version: None,
              homepage: None,
              contributors: None,
              license: None,
              dependencies: None,
              restart_required: None,
              parent_mod_id: None, // Not needed right now
            });
          }
        } else if is_archive(&path) {
          debug!("Checking archive for Codename mod: {}", path.display());

          // Try to find data/config/credits.xml in the archive
          let xml_content = read_text_from_archive(
            &path,
            "data/config/credits.xml"
          );
          let has_credits = xml_content.is_some();

          // Get the archive name without extension to use as mod name
          let archive_name = path
            .file_stem()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown Archive".to_string());

          // If we have credits.xml, treat as Codename mod
          if has_credits {
            debug!(
              "Detected Codename Engine mod in archive: {}",
              path.display()
            );

            let contributors_info = Vec::new();

            // If we have credits.xml, try to parse contributors
            if let Some(content) = xml_content {
              if let Ok(_doc) = roxmltree::Document::parse(&content) {
                // TODO: Parse contributors from XML
              }
            }

            let collected_contributors = if !contributors_info.is_empty() {
              Some(contributors_info)
            } else {
              None
            };

            metadata_files.push(ModMetadataFile {
              name: archive_name,
              description: None,
              folder_path: path.to_string_lossy().to_string(), // Use archive path as folder path
              config_file_path: Some(path.to_string_lossy().to_string()), // Use archive path as config file path
              icon_file_path: None,
              icon_data: None,
              enabled: Some(true), // Default to enabled for archives
              version: None,
              homepage: None,
              contributors: collected_contributors,
              license: None,
              dependencies: None,
              restart_required: None,
              parent_mod_id: None, // Not needed right now
            });
          }
        }
      }
    }
    Err(e) => {
      return Err(
        format!("Failed to read directory {}: {}", mods_folder.display(), e)
      );
    }
  }
  // Also look in the addons folder
  // Remove the last part of the path
  let addons_folder = mods_folder.parent().unwrap().join("addons");
  debug!("Searching for codename engine mods in {}", addons_folder.display());
  if addons_folder.exists() {
    match fs::read_dir(&addons_folder) {
      Ok(entries) => {
        for entry in entries.filter_map(|e| e.ok()) {
          let path = entry.path();

          if path.is_dir() {
            // Codename engine mods don't have names or descriptions

            // Name is just the folder name
            let name = path.file_name().map_or_else(
              || "Unknown Mod".to_string(),
              |n| n.to_string_lossy().to_string()
            );

            // Check for data/config/credits.xml
            let credits_path = path
              .join("data")
              .join("config")
              .join("credits.xml");
            if credits_path.exists() && credits_path.is_file() {
              debug!("Found credits.xml in {}", path.display());
              // Parse the XML file using roxmltree
              let xml_content = match fs::read_to_string(&credits_path) {
                Ok(content) => content,
                Err(e) => {
                  warn!(
                    "Failed to read credits.xml in {}: {}",
                    path.display(),
                    e
                  );
                  continue;
                }
              };
              let _xml_doc = match roxmltree::Document::parse(&xml_content) {
                Ok(doc) => doc,
                Err(e) => {
                  warn!(
                    "Failed to parse credits.xml in {}: {}",
                    path.display(),
                    e
                  );
                  continue;
                }
              };
              // TODO: Actually parse the XML content to extract contributors
              // Codename has a different credits structure, very unlike Polymod mods

              metadata_files.push(ModMetadataFile {
                name,
                description: None,
                folder_path: path.to_string_lossy().to_string(),
                config_file_path: None,
                icon_file_path: None,
                icon_data: None,
                enabled: Some(true), // Default to enabled for Codename mods
                version: None,
                homepage: None,
                contributors: None,
                license: None,
                dependencies: None,
                restart_required: None,
                parent_mod_id: None, // Not needed right now
              });
            } else {
              debug!("No credits.xml found in {}", path.display());
              metadata_files.push(ModMetadataFile {
                name,
                description: None,
                folder_path: path.to_string_lossy().to_string(),
                config_file_path: None,
                icon_file_path: None,
                icon_data: None,
                enabled: Some(true), // Default to enabled for Codename mods
                version: None,
                homepage: None,
                contributors: None,
                license: None,
                dependencies: None,
                restart_required: None,
                parent_mod_id: None, // Not needed right now
              });
            }
          } else if is_archive(&path) {
            debug!("Checking archive for Codename mod: {}", path.display());

            // Try to find data/config/credits.xml in the archive
            let xml_content = read_text_from_archive(
              &path,
              "data/config/credits.xml"
            );
            let has_credits = xml_content.is_some();

            // Get the archive name without extension to use as mod name
            let archive_name = path
              .file_stem()
              .map(|n| n.to_string_lossy().to_string())
              .unwrap_or_else(|| "Unknown Archive".to_string());

            // If we have credits.xml, treat as Codename mod
            if has_credits {
              debug!(
                "Detected Codename Engine mod in archive: {}",
                path.display()
              );

              let contributors_info = Vec::new();

              // If we have credits.xml, try to parse contributors
              if let Some(content) = xml_content {
                if let Ok(_doc) = roxmltree::Document::parse(&content) {
                  // TODO: Parse contributors from XML
                }
              }

              let collected_contributors = if !contributors_info.is_empty() {
                Some(contributors_info)
              } else {
                None
              };

              metadata_files.push(ModMetadataFile {
                name: archive_name,
                description: None,
                folder_path: path.to_string_lossy().to_string(), // Use archive path as folder path
                config_file_path: Some(path.to_string_lossy().to_string()), // Use archive path as config file path
                icon_file_path: None,
                icon_data: None,
                enabled: Some(true), // Default to enabled for archives
                version: None,
                homepage: None,
                contributors: collected_contributors,
                license: None,
                dependencies: None,
                restart_required: None,
                parent_mod_id: None, // Not needed right now
              });
            }
          }
        }
      }
      Err(e) => {
        return Err(
          format!("Failed to read directory {}: {}", addons_folder.display(), e)
        );
      }
    }
  }
  Ok(metadata_files)
}

pub fn find_mod_dependency(
  mods_folder_path: &str,
  dependency_name: &str,
  required_version: &str
) -> Result<String, String> {
  let path = Path::new(mods_folder_path);

  // Check for if the dependency is in the mods folder (case-insensitive)
  if path.exists() && path.is_dir() {
    for entry in fs
      ::read_dir(path)
      .map_err(|e|
        format!("Failed to read directory {}: {}", path.display(), e)
      )? {
      let entry = entry.map_err(|e|
        format!("Failed to read entry in directory {}: {}", path.display(), e)
      )?;
      let entry_name = entry.file_name().to_string_lossy().to_lowercase();

      if entry_name == dependency_name.to_lowercase() {
        let entry_path = entry.path();
        let entry_path_str = entry_path.to_string_lossy().to_string();

        // If no specific version is required, return immediately
        if required_version.is_empty() || required_version == "*" {
          return Ok(entry_path_str);
        }

        // Get the dependency's version from its metadata
        let dependency_version = get_mod_version(&entry_path_str)?;

        // Validate that the version meets the requirements
        if is_version_compatible(&dependency_version, required_version) {
          return Ok(entry_path_str);
        } else {
          return Err(
            format!(
              "Dependency {} found but version {} does not satisfy requirement {}",
              dependency_name,
              dependency_version,
              required_version
            )
          );
        }
      }
    }
  }

  Err(
    format!("Dependency {} not found in {}", dependency_name, mods_folder_path)
  )
}

/// Extract the executable's directory path
pub fn get_executable_directory(
  executable_path: &str
) -> Result<PathBuf, String> {
  let path = Path::new(executable_path);

  match path.parent() {
    Some(dir) => Ok(dir.to_path_buf()),
    None =>
      Err(format!("Cannot determine parent directory for {}", executable_path)),
  }
}

/// Extract a mod's version from its metadata files
fn get_mod_version(mod_path: &str) -> Result<String, String> {
  let path = Path::new(mod_path);

  // First try _polymod_meta.json (Vanilla)
  let polymod_path = path.join("_polymod_meta.json");
  let disabled_polymod_path = path.join("_polymod_meta_disabled.json");
  if disabled_polymod_path.exists() && disabled_polymod_path.is_file() {
    return Err(format!("Mod is disabled: {}", disabled_polymod_path.display()));
  } else if polymod_path.exists() && polymod_path.is_file() {
    match fs::read_to_string(&polymod_path) {
      Ok(content) => {
        match serde_json::from_str::<serde_json::Value>(&content) {
          Ok(json) => {
            return json
              .get("mod_version")
              .and_then(|v| v.as_str())
              .map(|s| s.to_string())
              .ok_or_else(||
                "No version found in _polymod_meta.json".to_string()
              );
          }
          Err(e) => {
            return Err(format!("Failed to parse _polymod_meta.json: {}", e));
          }
        }
      }
      Err(e) => {
        return Err(format!("Failed to read _polymod_meta.json: {}", e));
      }
    }
  }

  // Then try meta.json (FPS Plus)
  let fpsplus_path = path.join("meta.json");
  if fpsplus_path.exists() && fpsplus_path.is_file() {
    match fs::read_to_string(&fpsplus_path) {
      Ok(content) => {
        match serde_json::from_str::<serde_json::Value>(&content) {
          Ok(json) => {
            return json
              .get("mod_version")
              .and_then(|v| v.as_str())
              .map(|s| s.to_string())
              .ok_or_else(|| "No version found in meta.json".to_string());
          }
          Err(e) => {
            return Err(format!("Failed to parse meta.json: {}", e));
          }
        }
      }
      Err(e) => {
        return Err(format!("Failed to read meta.json: {}", e));
      }
    }
  }

  // If no version found, return an error
  Err("No version found in mod metadata".to_string())
}

/// Check if the dependency version is compatible with the required version (Polymod)
pub fn is_version_compatible(
  actual_version: &str,
  required_version: &str
) -> bool {
  // * or empty string - any version is allowed
  if required_version.is_empty() || required_version == "*" {
    return true;
  }

  // Parse the actual version
  let actual_parts: Vec<&str> = actual_version.split('.').collect();
  if actual_parts.is_empty() {
    // Invalid version format
    return false;
  }

  // Handle the 1.0.* format (any patch version)
  if required_version.ends_with(".*") {
    let req_base = required_version.trim_end_matches(".*");
    let req_parts: Vec<&str> = req_base.split('.').collect();

    if req_parts.len() == 2 {
      // This is the 1.0.* format (any patch version)
      return (
        actual_parts.len() >= 2 &&
        actual_parts[0] == req_parts[0] &&
        actual_parts[1] == req_parts[1]
      );
    } else if req_parts.len() == 1 {
      // This is the 1.* format (any minor version)
      return actual_parts.len() >= 1 && actual_parts[0] == req_parts[0];
    }

    return false;
  }

  // Handle comparison operators
  if required_version.starts_with(">") {
    let version = if required_version.starts_with(">=") {
      &required_version[2..]
    } else {
      &required_version[1..]
    };

    let req_parts: Vec<&str> = version.split('.').collect();
    if req_parts.is_empty() {
      return false;
    }

    // Compare versions numerically
    let cmp = compare_versions(actual_parts.as_slice(), req_parts.as_slice());

    return if required_version.starts_with(">=") { cmp >= 0 } else { cmp > 0 };
  }

  if required_version.starts_with("<") {
    let version = if required_version.starts_with("<=") {
      &required_version[2..]
    } else {
      &required_version[1..]
    };

    let req_parts: Vec<&str> = version.split('.').collect();
    if req_parts.is_empty() {
      return false;
    }

    // Compare versions numerically
    let cmp = compare_versions(actual_parts.as_slice(), req_parts.as_slice());

    return if required_version.starts_with("<=") { cmp <= 0 } else { cmp < 0 };
  }

  // Exact match
  actual_version == required_version
}

/// Compare two version arrays numerically
/// Returns -1 if a < b, 0 if a == b, 1 if a > b
fn compare_versions(a: &[&str], b: &[&str]) -> i32 {
  let max_len = a.len().max(b.len());

  for i in 0..max_len {
    let a_val = if i < a.len() { a[i].parse::<i32>().unwrap_or(0) } else { 0 };

    let b_val = if i < b.len() { b[i].parse::<i32>().unwrap_or(0) } else { 0 };

    if a_val < b_val {
      return -1;
    } else if a_val > b_val {
      return 1;
    }
  }

  0 // Versions are equal
}

/// Semantic version structure for proper semver comparison
#[derive(Debug, Clone, PartialEq, Eq)]
struct SemVer {
  major: u32,
  minor: u32,
  patch: u32,
  prerelease: Option<String>,
  build: Option<String>,
}

impl SemVer {
  /// Parse a semantic version string using the proper semver regex
  fn parse(version: &str) -> Result<Self, String> {
    // Semver regex pattern
    let semver_regex = Regex::new(
      r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"
    ).map_err(|e| format!("Failed to compile semver regex: {}", e))?;

    let captures = semver_regex
      .captures(version)
      .ok_or_else(|| format!("Invalid semantic version format: {}", version))?;

    let major = captures
      .get(1)
      .unwrap()
      .as_str()
      .parse::<u32>()
      .map_err(|e| format!("Invalid major version: {}", e))?;

    let minor = captures
      .get(2)
      .unwrap()
      .as_str()
      .parse::<u32>()
      .map_err(|e| format!("Invalid minor version: {}", e))?;

    let patch = captures
      .get(3)
      .unwrap()
      .as_str()
      .parse::<u32>()
      .map_err(|e| format!("Invalid patch version: {}", e))?;

    let prerelease = captures.get(4).map(|m| m.as_str().to_string());
    let build = captures.get(5).map(|m| m.as_str().to_string());

    Ok(SemVer {
      major,
      minor,
      patch,
      prerelease,
      build,
    })
  }

  /// Compare two semantic versions according to semver specification
  /// Returns -1 if self < other, 0 if self == other, 1 if self > other
  fn compare(&self, other: &SemVer) -> i32 {
    // Compare major.minor.patch first
    if self.major != other.major {
      return if self.major < other.major { -1 } else { 1 };
    }
    if self.minor != other.minor {
      return if self.minor < other.minor { -1 } else { 1 };
    }
    if self.patch != other.patch {
      return if self.patch < other.patch { -1 } else { 1 };
    }

    // Handle prerelease comparison
    match (&self.prerelease, &other.prerelease) {
      (None, None) => 0, // Both are release versions
      (None, Some(_)) => 1, // Release version > prerelease
      (Some(_), None) => -1, // Prerelease < release version
      (Some(pre1), Some(pre2)) => {
        // Compare prerelease identifiers
        let parts1: Vec<&str> = pre1.split('.').collect();
        let parts2: Vec<&str> = pre2.split('.').collect();

        for (part1, part2) in parts1.iter().zip(parts2.iter()) {
          // Try to parse as numbers first
          let cmp = match (part1.parse::<u32>(), part2.parse::<u32>()) {
            (Ok(n1), Ok(n2)) => n1.cmp(&n2),
            (Ok(_), Err(_)) => std::cmp::Ordering::Less, // Numeric < alphanumeric
            (Err(_), Ok(_)) => std::cmp::Ordering::Greater, // Alphanumeric > numeric
            (Err(_), Err(_)) => part1.cmp(part2), // Lexical comparison
          };

          match cmp {
            std::cmp::Ordering::Less => {
              return -1;
            }
            std::cmp::Ordering::Greater => {
              return 1;
            }
            std::cmp::Ordering::Equal => {
              continue;
            }
          }
        }

        // If all compared parts are equal, longer prerelease wins
        parts1.len().cmp(&parts2.len()) as i32
      }
    }
  }
}

/// Compare two semantic version strings according to semver specification
/// Returns -1 if version_a < version_b, 0 if version_a == version_b, 1 if version_a > version_b
/// Returns an error if either version string is not a valid semantic version
pub fn compare_semver(version_a: &str, version_b: &str) -> Result<i32, String> {
  let semver_a = SemVer::parse(version_a)?;
  let semver_b = SemVer::parse(version_b)?;
  Ok(semver_a.compare(&semver_b))
}

/// Check if a semantic version is compatible with a required version constraint
/// Supports operators: =, >, >=, <, <=, ^, ~, and ranges like "1.0.0 - 2.0.0"
pub fn is_semver_compatible(
  actual_version: &str,
  required_version: &str
) -> Result<bool, String> {
  // Handle empty or wildcard requirements
  if required_version.is_empty() || required_version == "*" {
    return Ok(true);
  }

  let actual = SemVer::parse(actual_version)?;

  // Handle range syntax (e.g., "1.0.0 - 2.0.0")
  if required_version.contains(" - ") {
    let parts: Vec<&str> = required_version.split(" - ").collect();
    if parts.len() == 2 {
      let min_version = SemVer::parse(parts[0])?;
      let max_version = SemVer::parse(parts[1])?;
      return Ok(
        actual.compare(&min_version) >= 0 && actual.compare(&max_version) <= 0
      );
    }
  }

  // Handle caret range (^1.2.3 means >=1.2.3 <2.0.0)
  if required_version.starts_with('^') {
    let version_str = &required_version[1..];
    let required = SemVer::parse(version_str)?;

    // Same major version, but >= required version
    return Ok(actual.major == required.major && actual.compare(&required) >= 0);
  }

  // Handle tilde range (~1.2.3 means >=1.2.3 <1.3.0)
  if required_version.starts_with('~') {
    let version_str = &required_version[1..];
    let required = SemVer::parse(version_str)?;

    // Same major.minor version, but >= required version
    return Ok(
      actual.major == required.major &&
        actual.minor == required.minor &&
        actual.compare(&required) >= 0
    );
  }

  // Handle comparison operators
  if required_version.starts_with(">=") {
    let version_str = &required_version[2..];
    let required = SemVer::parse(version_str)?;
    return Ok(actual.compare(&required) >= 0);
  }

  if required_version.starts_with("<=") {
    let version_str = &required_version[2..];
    let required = SemVer::parse(version_str)?;
    return Ok(actual.compare(&required) <= 0);
  }

  if required_version.starts_with('>') {
    let version_str = &required_version[1..];
    let required = SemVer::parse(version_str)?;
    return Ok(actual.compare(&required) > 0);
  }

  if required_version.starts_with('<') {
    let version_str = &required_version[1..];
    let required = SemVer::parse(version_str)?;
    return Ok(actual.compare(&required) < 0);
  }

  if required_version.starts_with('=') {
    let version_str = &required_version[1..];
    let required = SemVer::parse(version_str)?;
    return Ok(actual.compare(&required) == 0);
  }

  // Default to exact match
  let required = SemVer::parse(required_version)?;
  Ok(actual.compare(&required) == 0)
}

/// Helper function to read JSON files from ZIP archives
fn read_json_from_archive(
  archive_path: &Path,
  file_path: &str
) -> Option<serde_json::Value> {
  let file = match File::open(archive_path) {
    Ok(file) => file,
    Err(e) => {
      warn!("Failed to open archive {}: {}", archive_path.display(), e);
      return None;
    }
  };

  let mut archive = match ZipArchive::new(file) {
    Ok(archive) => archive,
    Err(e) => {
      warn!("Failed to read archive {}: {}", archive_path.display(), e);
      return None;
    }
  };

  // Try to find the file inside the archive
  let result = match archive.by_name(file_path) {
    Ok(mut file) => {
      let mut contents = String::new();
      if let Err(e) = file.read_to_string(&mut contents) {
        warn!(
          "Failed to read {} from archive {}: {}",
          file_path,
          archive_path.display(),
          e
        );
        return None;
      }

      match serde_json::from_str::<serde_json::Value>(&contents) {
        Ok(json) => Some(json),
        Err(e) => {
          warn!(
            "Failed to parse {} from archive {}: {}",
            file_path,
            archive_path.display(),
            e
          );
          None
        }
      }
    }
    Err(e) => {
      debug!(
        "File {} not found in archive {}: {}",
        file_path,
        archive_path.display(),
        e
      );
      None
    }
  };

  result
}

/// Helper function to read text files from ZIP archives
fn read_text_from_archive(
  archive_path: &Path,
  file_path: &str
) -> Option<String> {
  let file = match File::open(archive_path) {
    Ok(file) => file,
    Err(e) => {
      warn!("Failed to open archive {}: {}", archive_path.display(), e);
      return None;
    }
  };

  let mut archive = match ZipArchive::new(file) {
    Ok(archive) => archive,
    Err(e) => {
      warn!("Failed to read archive {}: {}", archive_path.display(), e);
      return None;
    }
  };

  // Try to find the file inside the archive
  let result = match archive.by_name(file_path) {
    Ok(mut file) => {
      let mut contents = String::new();
      if let Err(e) = file.read_to_string(&mut contents) {
        warn!(
          "Failed to read {} from archive {}: {}",
          file_path,
          archive_path.display(),
          e
        );
        return None;
      }
      Some(contents)
    }
    Err(e) => {
      debug!(
        "File {} not found in archive {}: {}",
        file_path,
        archive_path.display(),
        e
      );
      None
    }
  };

  result
}

/// Helper function to check if a file exists in a ZIP archive
fn file_exists_in_archive(archive_path: &Path, file_path: &str) -> bool {
  let file = match File::open(archive_path) {
    Ok(file) => file,
    Err(e) => {
      debug!("Failed to open archive {}: {}", archive_path.display(), e);
      return false;
    }
  };

  let mut archive = match ZipArchive::new(file) {
    Ok(archive) => archive,
    Err(e) => {
      debug!("Failed to read archive {}: {}", archive_path.display(), e);
      return false;
    }
  };

  let result = archive.by_name(file_path).is_ok();
  result
}

/// Helper function to read binary data from file in ZIP archive
fn read_binary_from_archive(
  archive_path: &Path,
  file_path: &str
) -> Option<Vec<u8>> {
  let file = match File::open(archive_path) {
    Ok(file) => file,
    Err(e) => {
      warn!("Failed to open archive {}: {}", archive_path.display(), e);
      return None;
    }
  };

  let mut archive = match ZipArchive::new(file) {
    Ok(archive) => archive,
    Err(e) => {
      warn!("Failed to read archive {}: {}", archive_path.display(), e);
      return None;
    }
  };

  // Try to find the file inside the archive
  let result = match archive.by_name(file_path) {
    Ok(mut file) => {
      let mut contents = Vec::new();
      if let Err(e) = file.read_to_end(&mut contents) {
        warn!(
          "Failed to read binary {} from archive {}: {}",
          file_path,
          archive_path.display(),
          e
        );
        return None;
      }
      Some(contents)
    }
    Err(e) => {
      debug!(
        "File {} not found in archive {}: {}",
        file_path,
        archive_path.display(),
        e
      );
      None
    }
  };

  result
}

/// Check if path is a supported archive format
fn is_archive(path: &Path) -> bool {
  if let Some(ext) = path.extension() {
    let ext_str = ext.to_string_lossy().to_lowercase();
    return ext_str == "zip";
  }
  false
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
