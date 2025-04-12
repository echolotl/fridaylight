use crate::filesystem::{check_for_custom_images, find_executables, extract_executable_icon};
use crate::gamebanana::{get_mod_info, get_download_url, extract_banner_url};
use crate::models::{DownloadStarted, DownloadProgress, DownloadFinished, DownloadError, ModInfo};
use crate::utils::{fetch_image_as_base64, extract_rar_archive};
use futures_util::StreamExt;
use log::{debug, error, info, warn};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tauri::{Manager, Emitter};

// Command to download a mod from GameBanana
pub async fn download_gamebanana_mod(
    url: String, 
    name: String,
    mod_id: i64,
    install_location: Option<String>,
    app: tauri::AppHandle
) -> Result<String, String> {
    info!("Starting download process for mod: {} (ID: {})", name, mod_id);
    
    // First, fetch the download page information to get the actual download URL
    info!("Fetching download page information from GameBanana API");
    
    // Emit download started event
    app.emit("download-started", DownloadStarted {
        mod_id,
        name: name.clone(),
        content_length: 0, // We don't know the size yet
        thumbnail_url: format!("https://gamebanana.com/mods/embeddables/{}", mod_id),
    }).unwrap_or_else(|e| error!("Failed to emit download-started event: {}", e));
    
    // Emit progress event for the download page fetch step
    app.emit("download-progress", DownloadProgress {
        mod_id,
        name: name.clone(),
        bytes_downloaded: 0,
        total_bytes: 100,
        percentage: 5,
        step: "Fetching download information".to_string(),
    }).unwrap_or_else(|e| error!("Failed to emit download-progress event: {}", e));
    
    // Fetch mod information for preview images
    let mod_info_response = match get_mod_info(mod_id).await {
        Ok(info) => Some(info),
        Err(e) => {
            warn!("Failed to fetch mod info: {}", e);
            None
        }
    };
    
    // Extract banner URL and fetch banner image
    let banner_url = mod_info_response.as_ref()
        .and_then(|info| extract_banner_url(info, mod_id));
    
    debug!("Banner URL: {:?}", banner_url);
    
    // Fetch the banner image if available
    let banner_data = if let Some(url) = banner_url {
        match fetch_image_as_base64(&url).await {
            Some(data) => {
                debug!("Successfully fetched banner image");
                Some(data)
            },
            None => {
                warn!("Failed to fetch banner image");
                None
            }
        }
    } else {
        None
    };
      // Use the provided URL if it's a direct download URL, otherwise get the default one
    let actual_download_url = if url.contains("gamebanana.com/dl/") {
        info!("Using provided direct download URL: {}", url);
        url
    } else {
        info!("Getting default download URL for mod ID: {}", mod_id);
        match get_download_url(mod_id).await {
            Ok(url) => url,
            Err(e) => {
                error!("Failed to get download URL: {}", e);
                
                // Emit error event
                app.emit("download-error", DownloadError {
                    mod_id,
                    name: name.clone(),
                    error: e.clone(),
                }).unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));
                
                return Err(e);
            }
        }
    };
    
    info!("Found actual download URL: {}", actual_download_url);
    
    // Emit progress event for preparing download
    app.emit("download-progress", DownloadProgress {
        mod_id,
        name: name.clone(),
        bytes_downloaded: 20,
        total_bytes: 100,
        percentage: 20,
        step: "Preparing to download mod file".to_string(),
    }).unwrap_or_else(|e| error!("Failed to emit download-progress event: {}", e));
    
    // Get the download folder
    let downloads_dir = match app.path().download_dir() {
        Ok(path) => {
            debug!("Download directory: {}", path.display());
            path
        },
        Err(e) => {
            let error_msg = format!("Failed to find downloads directory: {}", e);
            error!("{}", error_msg);
            
            // Emit error event
            app.emit("download-error", DownloadError {
                mod_id,
                name: name.clone(),
                error: error_msg.clone(),
            }).unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));
            
            return Err(error_msg);
        },
    };
    
    // Download the file with progress tracking
    debug!("Sending HTTP request to download mod");
    
    // Use reqwest to perform download with progress tracking
    let client = reqwest::Client::new();
    let response = match client.get(&actual_download_url).send().await {
        Ok(resp) => {
            debug!("Received response with status: {}", resp.status());
            if !resp.status().is_success() {
                let err_msg = format!("Server returned error status: {}", resp.status());
                error!("{}", err_msg);
                
                // Emit error event
                app.emit("download-error", DownloadError {
                    mod_id,
                    name: name.clone(),
                    error: err_msg.clone(),
                }).unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));
                
                return Err(err_msg);
            }
            resp
        },
        Err(e) => {
            let error_msg = format!("Failed to download mod: {}", e);
            error!("{}", error_msg);
            
            // Emit error event
            app.emit("download-error", DownloadError {
                mod_id,
                name: name.clone(),
                error: error_msg.clone(),
            }).unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));
            
            return Err(error_msg);
        }
    };
    
    // Create a unique filename with appropriate extension based on Content-Type header
    let extension = response.headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
        .and_then(|ct| {
            if ct.contains("application/zip") || ct.contains("application/x-zip") {
                Some("zip")
            } else if ct.contains("application/x-7z-compressed") {
                Some("7z")
            } else if ct.contains("application/x-rar-compressed") || ct.contains("application/vnd.rar") {
                Some("rar")
            } else {
                // Default to zip if unknown
                Some("zip")
            }
        })
        .unwrap_or("zip");
    
    let filename = format!("FNF-{}-{}.{}", name.replace(' ', "-"), chrono::Utc::now().timestamp(), extension);
    let download_path = downloads_dir.join(&filename);
    
    debug!("Download path: {}", download_path.display());
    
    // Get the content length for progress tracking
    let total_size = response.content_length().unwrap_or(0) as usize;
    
    // Update the download started event with actual content length
    app.emit("download-started", DownloadStarted {
        mod_id,
        name: name.clone(),
        content_length: total_size,
        thumbnail_url: format!("https://gamebanana.com/mods/embeddables/{}", mod_id),
    }).unwrap_or_else(|e| error!("Failed to emit updated download-started event: {}", e));
    
    // Create a file to write to
    let mut file = match std::fs::File::create(&download_path) {
        Ok(file) => file,
        Err(e) => {
            let error_msg = format!("Failed to create file: {}", e);
            error!("{}", error_msg);
            
            // Emit error event
            app.emit("download-error", DownloadError {
                mod_id,
                name: name.clone(),
                error: error_msg.clone(),
            }).unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));
            
            return Err(error_msg);
        }
    };
    
    // Stream the response body with progress updates
    let mut stream = response.bytes_stream();
    let mut downloaded: usize = 0;
    let mut last_percentage = 0;
    
    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => { // chunk is of type reqwest::Bytes here
                // Write the chunk to the file
                // &chunk dereferences Bytes to &[u8] for write_all
                if let Err(e) = std::io::Write::write_all(&mut file, &chunk) {
                    let error_msg = format!("Failed to write to file: {}", e);
                    error!("{}", error_msg);
                    
                    // Emit error event
                    app.emit("download-error", DownloadError {
                        mod_id,
                        name: name.clone(),
                        error: error_msg.clone(),
                    }).unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));
                    
                    return Err(error_msg);
                }
                
                // Update progress
                downloaded += chunk.len();
                let percentage = if total_size > 0 {
                    ((downloaded as f64 / total_size as f64) * 60.0) as u8 + 20 // 20-80% range for download
                } else {
                    30 // Default to middle of range if size unknown
                };
                
                // Only emit progress events if percentage has changed
                if percentage != last_percentage {
                    app.emit("download-progress", DownloadProgress {
                        mod_id,
                        name: name.clone(),
                        bytes_downloaded: downloaded,
                        total_bytes: total_size,
                        percentage,
                        step: "Downloading mod file".to_string(),
                    }).unwrap_or_else(|e| error!("Failed to emit download-progress event: {}", e));
                    
                    last_percentage = percentage;
                }
            },
            Err(e) => {
                let error_msg = format!("Failed to download chunk: {}", e);
                error!("{}", error_msg);
                
                // Emit error event
                app.emit("download-error", DownloadError {
                    mod_id,
                    name: name.clone(),
                    error: error_msg.clone(),
                }).unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));
                
                return Err(error_msg);
            }
        }
    }
    
    // Emit progress event for extraction
    app.emit("download-progress", DownloadProgress {
        mod_id,
        name: name.clone(),
        bytes_downloaded: total_size,
        total_bytes: total_size,
        percentage: 80,
        step: "Preparing to extract mod".to_string(),
    }).unwrap_or_else(|e| error!("Failed to emit download-progress event: {}", e));
    
    // Get the install location - use provided location or fall back to default
    let install_dir = if let Some(location) = install_location {
        let path = PathBuf::from(&location);
        info!("Using provided install location: {}", path.display());
        path
    } else {
        let default_path = get_default_install_location(&app);
        info!("Using default install location: {}", default_path.display());
        default_path
    };
    
    debug!("Using install location: {}", install_dir.display());
    
    // Create the install directory if it doesn't exist
    if !install_dir.exists() {
        debug!("Creating install directory: {}", install_dir.display());
        if let Err(e) = fs::create_dir_all(&install_dir) {
            let error_msg = format!("Failed to create install directory: {}", e);
            error!("{}", error_msg);
            
            // Emit error event
            app.emit("download-error", DownloadError {
                mod_id,
                name: name.clone(),
                error: error_msg.clone(),
            }).unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));
            
            return Err(error_msg);
        }
    }
    
    // Sanitize mod name for folder name
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
    
    // Create unique folder for this mod
    let mod_folder = install_dir.join(&sanitized_name);
    if mod_folder.exists() {
        debug!("Mod folder already exists, removing it: {}", mod_folder.display());
        if let Err(e) = fs::remove_dir_all(&mod_folder) {
            let error_msg = format!("Failed to remove existing mod folder: {}", e);
            error!("{}", error_msg);
            
            // Emit error event
            app.emit("download-error", DownloadError {
                mod_id,
                name: name.clone(),
                error: error_msg.clone(),
            }).unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));
            
            return Err(error_msg);
        }
    }
    
    debug!("Creating mod folder: {}", mod_folder.display());
    if let Err(e) = fs::create_dir_all(&mod_folder) {
        let error_msg = format!("Failed to create mod folder: {}", e);
        error!("{}", error_msg);
        
        // Emit error event
        app.emit("download-error", DownloadError {
            mod_id,
            name: name.clone(),
            error: error_msg.clone(),
        }).unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));
        
        return Err(error_msg);
    }
    
    // Extract the archive based on its type
    let extraction_result = extract_archive(&download_path, &mod_folder, &name, mod_id, &app);
    if let Err(e) = extraction_result {
        return Err(e);
    }
    
    // Emit progress event for finalizing
    app.emit("download-progress", DownloadProgress {
        mod_id,
        name: name.clone(),
        bytes_downloaded: 95,
        total_bytes: 100,
        percentage: 95,
        step: "Finalizing mod installation".to_string(),
    }).unwrap_or_else(|e| error!("Failed to emit download-progress event: {}", e));
    
    // Find executable in the extracted files
    debug!("Searching for executables in mod folder");
    let executables = find_executables(&mod_folder);
    let executable_path = executables.first().map(|p| p.to_string_lossy().to_string());
    
    // Extract icon if we have an executable
    let icon_data = match &executable_path {
        Some(exe_path) => {
            debug!("Extracting icon from: {}", exe_path);
            extract_executable_icon(Path::new(exe_path))
        },
        None => None,
    };
    
    // Check for custom images in fnfml folder
    let (custom_banner_data, custom_logo_data) = check_for_custom_images(&mod_folder);
    
    // Use custom banner/logo if available, otherwise use downloaded ones
    let final_banner_data = custom_banner_data.or(banner_data);
    let final_logo_data = custom_logo_data;
    
    // Create the mod info with banner
    let id = uuid::Uuid::new_v4().to_string();    let mod_info = ModInfo {
        id: id.clone(),
        name: name.clone(),
        path: mod_folder.to_string_lossy().to_string(),
        executable_path,
        display_order: Some(0),
        icon_data,
        description: None,
        banner_data: final_banner_data,
        logo_data: final_logo_data,
        version: mod_info_response.as_ref()
            .and_then(|info| info.get("_sVersion"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        engine_type: mod_info_response.as_ref()
            .and_then(|info| info.get("_aMetadata"))
            .and_then(|meta| meta.get("engine"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        engine: None, // Initialize with None for now
    };
    
    // Add the mod to our state
    let mods_state = app.state::<crate::models::ModsState>();
    let mut mods = mods_state.0.lock().unwrap();
    mods.insert(id.clone(), mod_info.clone());
    
    info!("Successfully downloaded, extracted, and added mod '{}' to mods list", name);
    
    // Emit download finished event
    app.emit("download-finished", DownloadFinished {
        mod_id,
        name: name.clone(),
        mod_info: mod_info.clone(),
    }).unwrap_or_else(|e| error!("Failed to emit download-finished event: {}", e));
    
    // Emit progress event for completion
    app.emit("download-progress", DownloadProgress {
        mod_id,
        name: name.clone(),
        bytes_downloaded: 100,
        total_bytes: 100,
        percentage: 100,
        step: "Mod installation complete".to_string(),
    }).unwrap_or_else(|e| error!("Failed to emit download-progress event: {}", e));
    
    Ok(mod_folder.to_string_lossy().to_string())
}

// Helper function to extract archives of different types
fn extract_archive(
    download_path: &PathBuf,
    mod_folder: &PathBuf,
    name: &str,
    mod_id: i64,
    app: &tauri::AppHandle
) -> Result<(), String> {
    // Depending on file extension, use appropriate extraction method
    if download_path.extension().and_then(|e| e.to_str()) == Some("zip") {
        extract_zip_archive(download_path, mod_folder, name, mod_id, app)
    } else if download_path.extension().and_then(|e| e.to_str()) == Some("7z") {
        extract_7z_archive(download_path, mod_folder, name, mod_id, app)
    } else if download_path.extension().and_then(|e| e.to_str()) == Some("rar") {
        extract_rar_file(download_path, mod_folder, name, mod_id, app)
    } else {
        // Try to extract as zip by default
        match extract_zip_archive(download_path, mod_folder, name, mod_id, app) {
            Ok(_) => Ok(()),
            Err(e) => {
                let error_msg = format!("Could not extract archive with unknown format: {}", e);
                error!("{}", error_msg);
                
                // Emit error event
                app.emit("download-error", DownloadError {
                    mod_id,
                    name: name.to_string(),
                    error: error_msg.clone(),
                }).unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));
                
                Err(error_msg)
            }
        }
    }
}

// Helper function to extract ZIP archives
fn extract_zip_archive(
    download_path: &PathBuf,
    mod_folder: &PathBuf,
    name: &str,
    mod_id: i64,
    app: &tauri::AppHandle
) -> Result<(), String> {
    let file = match std::fs::File::open(download_path) {
        Ok(file) => file,
        Err(e) => {
            let error_msg = format!("Failed to open downloaded zip file: {}", e);
            error!("{}", error_msg);
            
            // Emit error event
            app.emit("download-error", DownloadError {
                mod_id,
                name: name.to_string(),
                error: error_msg.clone(),
            }).unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));
            
            return Err(error_msg);
        }
    };
    
    let mut archive = match zip::ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(e) => {
            let error_msg = format!("Failed to read zip archive: {}", e);
            error!("{}", error_msg);
            
            // Emit error event
            app.emit("download-error", DownloadError {
                mod_id,
                name: name.to_string(),
                error: error_msg.clone(),
            }).unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));
            
            return Err(error_msg);
        }
    };
    
    let total_files = archive.len();
    let mut last_percentage = 80; // Starting percentage for extraction
    
    for i in 0..total_files {
        let file_result = archive.by_index(i);

        let mut file = match file_result {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to extract file from zip: {}", e);
                continue; // Skip this file and try the next one
            }
        };
        
        let outpath = match file.enclosed_name() {
            Some(path) => mod_folder.join(path),
            None => continue,
        };
        
        // Update extraction progress
        let extract_percentage = ((i as f64 / total_files as f64) * 10.0) as u8 + 85; // 85-95% range for extraction
        if extract_percentage != last_percentage {
            app.emit("download-progress", DownloadProgress {
                mod_id,
                name: name.to_string(),
                bytes_downloaded: i,
                total_bytes: total_files,
                percentage: extract_percentage,
                step: format!("Extracting file {}/{}", i+1, total_files),
            }).unwrap_or_else(|e| error!("Failed to emit download-progress event: {}", e));
            
            last_percentage = extract_percentage;
        }
        
        if file.name().ends_with('/') {
            // Create directory
            debug!("Creating directory: {}", outpath.display());
            if let Err(e) = fs::create_dir_all(&outpath) {
                error!("Failed to create directory: {}", e);
                continue;
            }
        } else {
            // Create parent directory if it doesn't exist
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    debug!("Creating parent directory: {}", parent.display());
                    if let Err(e) = fs::create_dir_all(parent) {
                        error!("Failed to create parent directory: {}", e);
                        continue;
                    }
                }
            }
            
            // Extract file
            debug!("Extracting file: {}", outpath.display());
            let mut outfile = match std::fs::File::create(&outpath) {
                Ok(file) => file,
                Err(e) => {
                    error!("Failed to create output file: {}", e);
                    continue;
                }
            };
            
            if let Err(e) = std::io::copy(&mut file, &mut outfile) {
                error!("Failed to copy file contents: {}", e);
            }
        }
    }
    
    Ok(())
}

// Helper function to extract 7z archives
fn extract_7z_archive(
    download_path: &PathBuf,
    mod_folder: &PathBuf,
    name: &str,
    mod_id: i64,
    app: &tauri::AppHandle
) -> Result<(), String> {
    debug!("Extracting 7z archive: {}", download_path.display());
    
    match sevenz_rust::decompress_file(download_path, mod_folder) {
        Ok(_) => {
            debug!("Successfully extracted 7z archive to {}", mod_folder.display());
            
            // Since we don't have file count for progress, use a simple progress indicator
            app.emit("download-progress", DownloadProgress {
                mod_id,
                name: name.to_string(),
                bytes_downloaded: 90,
                total_bytes: 100,
                percentage: 90,
                step: "Extracting 7z archive completed".to_string(),
            }).unwrap_or_else(|e| error!("Failed to emit download-progress event: {}", e));
            
            Ok(())
        },
        Err(e) => {
            let error_msg = format!("Failed to extract 7z archive: {}", e);
            error!("{}", error_msg);
            
            // Emit error event
            app.emit("download-error", DownloadError {
                mod_id,
                name: name.to_string(),
                error: error_msg.clone(),
            }).unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));
            
            Err(error_msg)
        }
    }
}

// Helper function to extract RAR archives
fn extract_rar_file(
    download_path: &PathBuf,
    mod_folder: &PathBuf,
    name: &str,
    mod_id: i64,
    app: &tauri::AppHandle
) -> Result<(), String> {
    debug!("Extracting RAR archive: {}", download_path.display());
    
    // Convert path to string for the unrar crate
    let archive_path = download_path.to_string_lossy().to_string();
    let extraction_path = mod_folder.to_string_lossy().to_string();
    
    // Create an Archive instance
    let archive = unrar::Archive::new(&archive_path);
    
    // Open the archive for processing and extract all files
    match archive.open_for_processing() {
        Ok(open_archive) => {
            let result = extract_rar_archive(open_archive, &extraction_path);
            
            match result {
                Ok(_) => {
                    debug!("Successfully extracted RAR archive to {}", mod_folder.display());
                    
                    // Since we don't have file count for progress, use a simple progress indicator
                    app.emit("download-progress", DownloadProgress {
                        mod_id,
                        name: name.to_string(),
                        bytes_downloaded: 90,
                        total_bytes: 100,
                        percentage: 90,
                        step: "Extracting RAR archive completed".to_string(),
                    }).unwrap_or_else(|e| error!("Failed to emit download-progress event: {}", e));
                    
                    Ok(())
                },
                Err(e) => {
                    let error_msg = format!("Failed to extract RAR archive: {}", e);
                    error!("{}", error_msg);
                    
                    // Emit error event
                    app.emit("download-error", DownloadError {
                        mod_id,
                        name: name.to_string(),
                        error: error_msg.clone(),
                    }).unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));
                    
                    Err(error_msg)
                }
            }
        },
        Err(e) => {
            let error_msg = format!("Failed to open RAR archive: {}", e);
            error!("{}", error_msg);
            
            // Emit error event
            app.emit("download-error", DownloadError {
                mod_id,
                name: name.to_string(),
                error: error_msg.clone(),
            }).unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));
            
            Err(error_msg)
        }
    }
}

// Function to get the install location from settings (defaults to app data dir)
pub fn get_default_install_location(app: &tauri::AppHandle) -> PathBuf {
    // Default install location if settings can't be loaded
    app.path().app_data_dir().unwrap_or_else(|_| {
        // Fall back to OS downloads directory if app data dir is unavailable
        app.path().download_dir().unwrap_or_else(|_| {
            // Last resort: use current directory
            PathBuf::from(".")
        })
    }).join("mods")
}