use crate::models::{ModInfo, ModMetadataFile};
use log::{debug, error, info, warn};
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Read;
use std::collections::HashMap;
use serde_json;
use std::io;
use roxmltree;
use base64::Engine; // Import the Engine trait

/// Find and process mod metadata files based on engine type
pub fn find_mod_metadata_files(executable_folder: &Path, engine_type: &str) -> Result<Vec<ModMetadataFile>, String> {
    let mods_folder = executable_folder.join("mods");
    
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
                            .map(|s| s.to_string()) // Convert Option<&str> to Option<String>
                            .unwrap_or_else(|| { // Closure now returns String
                                path.file_name()
                                    .map(|n| n.to_string_lossy().to_string()) // Option<String>
                                    .unwrap_or_else(|| "Unknown Mod".to_string()) // String
                            }); // name is now String
                        
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
                        // Get the title from _polymod_meta.json
                        let title = json_content.as_ref()
                            .and_then(|json| json.get("title").and_then(|v| v.as_str()).map(|s| s.to_string())) // Convert Option<&str> to Option<String>
                            .unwrap_or_else(|| { // Closure now returns String
                                path.file_name().map_or_else(
                                    || "Unknown Mod".to_string(), // Returns String
                                    |n| n.to_string_lossy().to_string() // Returns String
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
    
    match fs::read(file_path) {
        Ok(data) => {
            let extension = Path::new(file_path)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("png");
            
            let mime_type = match extension.to_lowercase().as_str() {
                "png" => "image/png",
                "jpg" | "jpeg" => "image/jpeg",
                "webp" => "image/webp",
                "gif" => "image/gif",
                _ => "application/octet-stream",
            };
            
            let b64 = base64::engine::general_purpose::STANDARD.encode(&data);
            Ok(format!("data:{};base64,{}", mime_type, b64))
        },
        Err(e) => {
            warn!("Failed to read image file {}: {}", file_path, e);
            Err(format!("Failed to read image file: {}", e))
        }
    }
}
