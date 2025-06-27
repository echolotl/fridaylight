use crate::filesystem::{
  check_for_custom_images,
  find_executables,
  extract_executable_icon,
};
use crate::gamebanana::{
  get_mod_info,
  get_download_url,
  extract_banner_url,
  extract_contributors,
};
use crate::models::{
  DownloadError,
  DownloadFinished,
  DownloadProgress,
  DownloadStarted,
  GBFile,
  GBProfilePage,
  ModInfo,
  CURRENT_METADATA_VERSION,
};
use crate::utils::{ fetch_image_as_base64, extract_rar_archive };
use futures_util::StreamExt;
use log::{ debug, error, info, warn };
use serde::{ Deserialize, Serialize };
use tauri::path::BaseDirectory;
use std::fs;
use std::path::{ Path, PathBuf };
use tauri::{ Manager, Emitter };

#[derive(Debug, Deserialize, Serialize)]
struct EngineConfig {
  engine_type: String,
  engine_name: String,
  engine_url: String,
  engine_banner: Option<String>,
  engine_logo: String,
  engine_icon: String,
  engine_description: String,
  engine_version: String,
}

// Command to download a mod from GameBanana
pub async fn download_gamebanana_mod(
  file: GBFile,
  info: GBProfilePage,
  install_location: Option<String>,
  folder_name: Option<String>,
  update_existing: Option<bool>,
  app: tauri::AppHandle
) -> Result<ModInfo, String> {
  info!(
    "Starting download process for mod: {} (ID: {})",
    info.name,
    file.id_row
  );

  let model_type = info.category.model_name.clone().replace("Category", "");
  let file_id = file.id_row;
  let mod_id = info.id_row;

  // First, fetch the download page information to get the actual download URL
  info!("Fetching download page information from GameBanana API");

  // Emit download started event
  app
    .emit("download-started", DownloadStarted {
      mod_id: file_id,
      name: info.name.clone(),
      content_length: file.filesize as usize,
      thumbnail_url: None,
    })
    .unwrap_or_else(|e| error!("Failed to emit download-started event: {}", e));

  // Extract banner URL and fetch banner image
  let banner_url = extract_banner_url(info.clone());

  debug!("Banner URL: {:?}", banner_url);

  // Fetch the banner image if available
  let banner_data = if let Some(url) = banner_url {
    match fetch_image_as_base64(&url).await {
      Some(data) => {
        debug!("Successfully fetched banner image");
        Some(data)
      }
      None => {
        warn!("Failed to fetch banner image");
        None
      }
    }
  } else {
    None
  };

  // Get the download folder
  let downloads_dir = match app.path().download_dir() {
    Ok(path) => {
      debug!("Download directory: {}", path.display());
      path
    }
    Err(e) => {
      let error_msg = format!("Failed to find downloads directory: {}", e);
      error!("{}", error_msg);

      // Emit error event
      app
        .emit("download-error", DownloadError {
          mod_id,
          name: info.name.clone(),
          error: error_msg.clone(),
        })
        .unwrap_or_else(|e|
          error!("Failed to emit download-error event: {}", e)
        );

      return Err(error_msg);
    }
  };

  // Download the file with progress tracking
  debug!("Sending HTTP request to download mod");

  // Use reqwest to perform download with progress tracking
  let client = reqwest::Client::new();
  let response = match client.get(&file.download_url).send().await {
    Ok(resp) => {
      debug!("Received response with status: {}", resp.status());
      if !resp.status().is_success() {
        let err_msg = format!(
          "Server returned error status: {}",
          resp.status()
        );
        error!("{}", err_msg);

        // Emit error event
        app
          .emit("download-error", DownloadError {
            mod_id,
            name: info.name.clone(),
            error: err_msg.clone(),
          })
          .unwrap_or_else(|e|
            error!("Failed to emit download-error event: {}", e)
          );

        return Err(err_msg);
      }
      resp
    }
    Err(e) => {
      let error_msg = format!("Failed to download mod: {}", e);
      error!("{}", error_msg);

      // Emit error event
      app
        .emit("download-error", DownloadError {
          mod_id,
          name: info.name.clone(),
          error: error_msg.clone(),
        })
        .unwrap_or_else(|e|
          error!("Failed to emit download-error event: {}", e)
        );

      return Err(error_msg);
    }
  };

  // Create a unique filename with appropriate extension based on Content-Type header
  let extension = response
    .headers()
    .get(reqwest::header::CONTENT_TYPE)
    .and_then(|ct| ct.to_str().ok())
    .and_then(|ct| {
      if ct.contains("application/zip") || ct.contains("application/x-zip") {
        Some("zip")
      } else if ct.contains("application/x-7z-compressed") {
        Some("7z")
      } else if
        ct.contains("application/x-rar-compressed") ||
        ct.contains("application/vnd.rar")
      {
        Some("rar")
      } else {
        // Default to zip if unknown
        Some("zip")
      }
    })
    .unwrap_or("zip");

  let filename = format!(
    "FNF-{}-{}.{}",
    info.name.replace(' ', "-"),
    chrono::Utc::now().timestamp(),
    extension
  );
  let download_path = downloads_dir.join(&filename);

  debug!("Download path: {}", download_path.display());

  // Get the content length for progress tracking
  let total_size = response.content_length().unwrap_or(0) as usize;
  // Update the download started event with actual content length
  app
    .emit("download-started", DownloadStarted {
      mod_id,
      name: info.name.clone(),
      content_length: total_size,
      thumbnail_url: None,
    })
    .unwrap_or_else(|e|
      error!("Failed to emit updated download-started event: {}", e)
    );

  // Create a file to write to
  let mut file = match std::fs::File::create(&download_path) {
    Ok(file) => file,
    Err(e) => {
      let error_msg = format!("Failed to create file: {}", e);
      error!("{}", error_msg);

      // Emit error event
      app
        .emit("download-error", DownloadError {
          mod_id,
          name: info.name.clone(),
          error: error_msg.clone(),
        })
        .unwrap_or_else(|e|
          error!("Failed to emit download-error event: {}", e)
        );

      return Err(error_msg);
    }
  };

  // Stream the response body with progress updates
  let mut stream = response.bytes_stream();
  let mut downloaded: usize = 0;
  let mut last_percentage = 0;

  while let Some(chunk_result) = stream.next().await {
    match chunk_result {
      Ok(chunk) => {
        // chunk is of type reqwest::Bytes here
        // Write the chunk to the file
        // &chunk dereferences Bytes to &[u8] for write_all
        if let Err(e) = std::io::Write::write_all(&mut file, &chunk) {
          let error_msg = format!("Failed to write to file: {}", e);
          error!("{}", error_msg);

          // Emit error event
          app
            .emit("download-error", DownloadError {
              mod_id,
              name: info.name.clone(),
              error: error_msg.clone(),
            })
            .unwrap_or_else(|e|
              error!("Failed to emit download-error event: {}", e)
            );

          return Err(error_msg);
        }

        // Update progress
        downloaded += chunk.len();
        let percentage = if total_size > 0 {
          ((((downloaded as f64) / (total_size as f64)) * 60.0) as u8) + 20 // 20-80% range for download
        } else {
          30 // Default to middle of range if size unknown
        };

        // Only emit progress events if percentage has changed
        if percentage != last_percentage {
          app
            .emit("download-progress", DownloadProgress {
              mod_id: file_id,
              name: info.name.clone(),
              bytes_downloaded: downloaded,
              total_bytes: total_size,
              percentage,
              step: "Downloading mod...".to_string(),
            })
            .unwrap_or_else(|e|
              error!("Failed to emit download-progress event: {}", e)
            );

          last_percentage = percentage;
        }
      }
      Err(e) => {
        let error_msg = format!("Failed to download chunk: {}", e);
        error!("{}", error_msg);

        // Emit error event
        app
          .emit("download-error", DownloadError {
            mod_id,
            name: info.name.clone(),
            error: error_msg.clone(),
          })
          .unwrap_or_else(|e|
            error!("Failed to emit download-error event: {}", e)
          );

        return Err(error_msg);
      }
    }
  }

  // Emit progress event for extraction
  app
    .emit("download-progress", DownloadProgress {
      mod_id: file_id,
      name: info.name.clone(),
      bytes_downloaded: total_size,
      total_bytes: total_size,
      percentage: 80,
      step: "Preparing to extract mod".to_string(),
    })
    .unwrap_or_else(|e|
      error!("Failed to emit download-progress event: {}", e)
    );

  let install_path = PathBuf::from(
    install_location.unwrap_or_else(|| {
      get_default_install_location(&app).to_string_lossy().to_string()
    })
  );

  debug!("Using install location: {}", install_path.display());

  // Create the install directory if it doesn't exist
  if !install_path.exists() {
    debug!("Creating install directory: {}", install_path.display());
    if let Err(e) = fs::create_dir_all(&install_path) {
      let error_msg = format!("Failed to create install directory: {}", e);
      error!("{}", error_msg);

      // Emit error event
      app
        .emit("download-error", DownloadError {
          mod_id,
          name: info.name.clone(),
          error: error_msg.clone(),
        })
        .unwrap_or_else(|e|
          error!("Failed to emit download-error event: {}", e)
        );

      return Err(error_msg);
    }
  }

  // Sanitize mod name for folder name
  let folder = if folder_name.is_some() {
    folder_name.clone()
  } else {
    Some(
      info.name
        .replace(' ', "-")
        .replace('/', "_")
        .replace('\\', "_")
        .replace(':', "")
        .replace('*', "")
        .replace('?', "")
        .replace('"', "")
        .replace('<', "")
        .replace('>', "")
        .replace('|', "")
    )
  };

  // Create unique folder for this mod
  let mod_folder = install_path.join(
    folder.unwrap_or_else(|| info.name.to_string())
  );
  if mod_folder.exists() {
    if update_existing.unwrap_or(false) {
      debug!(
        "Mod folder already exists, updating in place: {}",
        mod_folder.display()
      );
      // When updating, we keep the existing folder and just extract over it
    } else {
      app
        .emit("download-error", DownloadError {
          mod_id,
          name: info.name.clone(),
          error: format!(
            "Tried to install to an existing mod folder: {}",
            mod_folder.display()
          ),
        })
        .unwrap_or_else(|e|
          error!("Failed to emit download-error event: {}", e)
        );
      return Err(
        format!(
          "Tried to install to an existing mod folder: {}",
          mod_folder.display()
        )
      );
    }
  }

  debug!("Creating mod folder: {}", mod_folder.display());
  if let Err(e) = fs::create_dir_all(&mod_folder) {
    let error_msg = format!("Failed to create mod folder: {}", e);
    error!("{}", error_msg);

    // Emit error event
    app
      .emit("download-error", DownloadError {
        mod_id,
        name: info.name.clone(),
        error: error_msg.clone(),
      })
      .unwrap_or_else(|e| error!("Failed to emit download-error event: {}", e));

    return Err(error_msg);
  }

  // Extract the archive based on its type
  let extraction_result = extract_archive(
    &download_path,
    &mod_folder,
    &info.name,
    mod_id,
    &app
  );
  if let Err(e) = extraction_result {
    return Err(e);
  }

  // Reorganize modpack structure if needed (for better user experience)
  if let Err(e) = reorganize_modpack(&mod_folder) {
    warn!("Failed to reorganize modpack structure: {}", e);
    // Continue anyway as this is not critical
  }

  // Emit progress event for finalizing
  app
    .emit("download-progress", DownloadProgress {
      mod_id: file_id,
      name: info.name.clone(),
      bytes_downloaded: 95,
      total_bytes: 100,
      percentage: 95,
      step: "Finalizing mod installation".to_string(),
    })
    .unwrap_or_else(|e|
      error!("Failed to emit download-progress event: {}", e)
    );

  // Find executable in the extracted files
  debug!("Searching for executables in mod folder");
  let executables = find_executables(&mod_folder);
  let executable_path = executables
    .first()
    .map(|p| p.to_string_lossy().to_string());

  // Extract icon if we have an executable
  let icon_data = match &executable_path {
    Some(exe_path) => {
      debug!("Extracting icon from: {}", exe_path);
      extract_executable_icon(Path::new(exe_path))
    }
    None => None,
  };

  // Check for custom images in .flight folder
  let (custom_banner_data, custom_logo_data) = check_for_custom_images(
    &mod_folder
  );

  // Use custom banner/logo if available, otherwise use downloaded ones
  let final_banner_data = custom_banner_data.or(banner_data);
  let final_logo_data = custom_logo_data;

  // Create the mod info with banner
  let id = uuid::Uuid::new_v4().to_string();
  let mod_info = ModInfo {
    id: id.clone(),
    name: info.name.clone(),
    path: mod_folder.to_string_lossy().to_string(),
    executable_path,
    display_order: Some(0),
    icon_data,
    description: info.description.clone(),
    banner_data: final_banner_data,
    logo_data: final_logo_data,
    logo_position: Some("left_bottom".to_string()),
    version: info.version.clone(),
    engine: None, // Initialize with None for now
    process_id: None, // Initialize with None since mod is not running yet
    contributors: extract_contributors(&info),
    metadata_version: Some(CURRENT_METADATA_VERSION),
    date_added: Some(chrono::Utc::now().timestamp()), // Set current timestamp as date added
    last_played: None, // Initialize with None since mod is not played yet
    gamebanana: Some(crate::models::ModInfoGBData {
      url: info.profile_url,
      id: mod_id,
      model_type: model_type.clone(),
    }),
    save_terminal_output: false, // Default to false
  };

  // Create metadata.json file in the .flight folder
  if let Some(contributors) = &mod_info.contributors {
    // Create metadata object
    let mut metadata =
      serde_json::json!({
            "name": info.name.clone(),
            "logo_position": "left_bottom",
            "metadata_version": CURRENT_METADATA_VERSION,
        });

    // Add optional fields if they exist
    if let Some(version) = &mod_info.version {
      metadata["version"] = serde_json::Value::String(version.clone());
    }

    if let Some(description) = &mod_info.description {
      metadata["description"] = serde_json::Value::String(description.clone());
    }

    // Note: We don't add engine info as we won't get it from GameBanana

    // Add contributors array
    let contributors_array = serde_json
      ::to_value(contributors)
      .unwrap_or(serde_json::Value::Array(vec![]));
    metadata["contributors"] = contributors_array;

    // Add gamebanana data if available
    if let Some(gamebanana) = &mod_info.gamebanana {
      metadata["gamebanana"] =
        serde_json::json!({
        "url": gamebanana.url,
        "id": gamebanana.id,
        "model_type": gamebanana.model_type,
      });
    }
    // Save metadata.json file
    match crate::filesystem::save_metadata_json(&mod_folder, metadata) {
      Ok(_) => debug!("Successfully saved metadata.json with contributors"),
      Err(e) => warn!("Failed to save metadata.json: {}", e),
    }
  }

  // Add the mod to our state
  let mods_state = app.state::<crate::models::ModsState>();
  let mut mods = mods_state.0.lock().unwrap();
  mods.insert(id.clone(), mod_info.clone());

  info!(
    "Successfully downloaded, extracted, and added mod '{}' to mods list",
    info.name
  );

  // Emit download finished event
  app
    .emit("download-finished", DownloadFinished {
      mod_id: file_id,
      name: info.name.clone(),
      mod_info: mod_info.clone(),
    })
    .unwrap_or_else(|e|
      error!("Failed to emit download-finished event: {}", e)
    );

  // Emit progress event for completion
  app
    .emit("download-progress", DownloadProgress {
      mod_id: file_id,
      name: info.name.clone(),
      bytes_downloaded: 100,
      total_bytes: 100,
      percentage: 100,
      step: "Mod installation complete".to_string(),
    })
    .unwrap_or_else(|e|
      error!("Failed to emit download-progress event: {}", e)
    );

  Ok(mod_info)
}

// Helper function to find out if a mod is going to be installed to a folder which already exists
pub fn simulate_mod_folder_creation(
  info: GBProfilePage,
  install_location: Option<String>,
  folder_name: Option<String>,
  app: &tauri::AppHandle
) -> bool {
  let folder_path = if folder_name.is_some() {
    folder_name.clone()
  } else {
    Some(
      info.name
        .replace(' ', "-")
        .replace('/', "_")
        .replace('\\', "_")
        .replace(':', "")
        .replace('*', "")
        .replace('?', "")
        .replace('"', "")
        .replace('<', "")
        .replace('>', "")
        .replace('|', "")
    )
  };

  // Log install location provided
  if let Some(location) = &install_location {
    debug!("Install location provided: {}", location);
  } else {
    debug!("No install location provided, using default");
  }
  // Get full install path
  let install_path = PathBuf::from(
    install_location.unwrap_or_else(|| {
      get_default_install_location(app).to_string_lossy().to_string()
    })
  );
  let folder_path = PathBuf::from(
    folder_path.unwrap_or_else(|| info.name.to_string())
  );
  let full_install_path = install_path.join(folder_path);

  // Check if the full install path exists
  if full_install_path.exists() {
    warn!(
      "The mod folder '{}' already exists at the specified install location",
      full_install_path.display()
    );
    true // Folder exists, so we cannot safely create it
  } else {
    info!(
      "The mod folder '{}' does not exist, it can be created safely",
      full_install_path.display()
    );
    false // Folder does not exist, safe to create
  }
}

// Simulate engine folder creation to check if it can be created safely
pub fn simulate_engine_folder_creation(
  engine_type: String,
  install_location: Option<String>,
  folder_name: Option<String>,
  app: &tauri::AppHandle
) -> bool {
  let config = match load_engine_config(&engine_type, &app) {
    Ok(config) => config,
    Err(e) => {
      error!("Failed to load engine configuration: {}", e);
      return false; // Cannot simulate if config loading fails
    }
  };

  let engine_name = config.engine_name;
  let install_path = if let Some(location) = install_location {
    PathBuf::from(location)
  } else {
    get_default_install_location(&app)
  };
  let folder_path = if folder_name.is_some() {
    folder_name.clone()
  } else {
    Some(
      engine_name
        .replace(' ', "-")
        .replace('/', "_")
        .replace('\\', "_")
        .replace(':', "")
        .replace('*', "")
        .replace('?', "")
        .replace('"', "")
        .replace('<', "")
        .replace('>', "")
        .replace('|', "")
    )
  };
  let folder_path = PathBuf::from(
    folder_path.unwrap_or_else(|| engine_name.to_string())
  );
  let full_install_path = install_path.join(folder_path);

  // Check if the full install path exists
  if full_install_path.exists() {
    warn!(
      "The engine folder '{}' already exists at the specified install location",
      full_install_path.display()
    );
    return true; // Folder exists, so it can be updated
  } else {
    info!(
      "The engine folder '{}' does not exist, it can be created safely",
      full_install_path.display()
    );
    return false; // Folder does not exist, safe to create
  }
}

// Helper function to load engine configuration from JSON
fn load_engine_config(
  engine_id: &str,
  app: &tauri::AppHandle
) -> Result<EngineConfig, String> {
  let config_path = format!("resources/{}.json", engine_id);

  let config_file_path = app
    .path()
    .resolve(&config_path, BaseDirectory::Resource)
    .map_err(|e|
      format!("Failed to resolve config path for {}: {}", engine_id, e)
    )?;

  if !config_file_path.exists() {
    return Err(format!("Engine configuration file not found: {}", config_path));
  }

  let config_content = fs
    ::read_to_string(&config_file_path)
    .map_err(|e|
      format!("Failed to read config file for {}: {}", engine_id, e)
    )?;

  let config: EngineConfig = serde_json
    ::from_str(&config_content)
    .map_err(|e|
      format!("Failed to parse config JSON for {}: {}", engine_id, e)
    )?;

  info!("Successfully loaded configuration for {} engine", engine_id);
  Ok(config)
}

// Function to download a specific FNF engine directly
pub async fn download_engine(
  engine_id: String,
  install_location: Option<String>,
  custom_name: Option<String>,
  update_existing: Option<bool>,
  download_id: i64,
  app: tauri::AppHandle
) -> Result<ModInfo, String> {
  info!("Starting direct engine download for: {}", engine_id);

  // Load engine configuration from JSON
  let config = load_engine_config(&engine_id, &app)?;

  let engine_type = config.engine_type;
  let engine_name = config.engine_name;
  let engine_url = config.engine_url;
  let engine_banner = config.engine_banner;
  let engine_logo = config.engine_logo;
  let engine_icon = config.engine_icon;
  let engine_description = config.engine_description;
  let engine_version = config.engine_version;

  // Emit download started event
  app
    .emit("download-started", DownloadStarted {
      mod_id: download_id,
      name: engine_name.to_string(),
      content_length: 0, // We don't know the size yet
      thumbnail_url: None,
    })
    .unwrap_or_else(|e| error!("Failed to emit download-started event: {}", e));

  // Emit progress event for the engine download fetch step
  app
    .emit("download-progress", DownloadProgress {
      mod_id: download_id,
      name: engine_name.to_string(),
      percentage: 5,
      bytes_downloaded: 0,
      total_bytes: 100, // Placeholder
      step: "Preparing to download engine...".to_string(),
    })
    .unwrap_or_else(|e|
      error!("Failed to emit download-progress event: {}", e)
    );

  // Get the download folder
  let downloads_dir = match app.path().download_dir() {
    Ok(path) => {
      debug!("Download directory: {}", path.display());
      path
    }
    Err(e) => {
      let error_msg = format!("Failed to find downloads directory: {}", e);
      error!("{}", error_msg);

      // Emit error event
      app
        .emit("download-error", DownloadError {
          mod_id: download_id,
          name: engine_name.to_string(),
          error: error_msg.clone(),
        })
        .unwrap_or_else(|e|
          error!("Failed to emit download-error event: {}", e)
        );

      return Err(error_msg);
    }
  };

  // Download the file with progress tracking
  debug!("Sending HTTP request to download engine");

  // Use reqwest to perform download with progress tracking
  let client = reqwest::Client::new();
  let response = match client.get(&engine_url).send().await {
    Ok(resp) => {
      debug!("Received response with status: {}", resp.status());
      if !resp.status().is_success() {
        let err_msg = format!(
          "Server returned error status: {}",
          resp.status()
        );
        error!("{}", err_msg);

        // Emit error event
        app
          .emit("download-error", DownloadError {
            mod_id: download_id,
            name: engine_name.to_string(),
            error: err_msg.clone(),
          })
          .unwrap_or_else(|e|
            error!("Failed to emit download-error event: {}", e)
          );

        return Err(err_msg);
      }
      resp
    }
    Err(e) => {
      let error_msg = format!("Failed to download engine: {}", e);
      error!("{}", error_msg);

      // Emit error event
      app
        .emit("download-error", DownloadError {
          mod_id: download_id,
          name: engine_name.to_string(),
          error: error_msg.clone(),
        })
        .unwrap_or_else(|e|
          error!("Failed to emit download-error event: {}", e)
        );

      return Err(error_msg);
    }
  };

  // Detect archive type from Content-Type header or URL
  let extension = response
    .headers()
    .get(reqwest::header::CONTENT_TYPE)
    .and_then(|ct| ct.to_str().ok())
    .and_then(|ct| {
      if ct.contains("application/zip") || ct.contains("application/x-zip") {
        Some("zip")
      } else if ct.contains("application/x-7z-compressed") {
        Some("7z")
      } else if
        ct.contains("application/x-rar-compressed") ||
        ct.contains("application/vnd.rar")
      {
        Some("rar")
      } else {
        None // Fall back to URL detection
      }
    })
    .or_else(|| {
      // Fall back to detecting from URL extension
      if engine_url.ends_with(".7z") {
        Some("7z")
      } else if engine_url.ends_with(".rar") {
        Some("rar")
      } else if engine_url.ends_with(".zip") {
        Some("zip")
      } else {
        Some("zip") // Default to zip if unknown
      }
    })
    .unwrap_or("zip");

  // Create a unique filename for the download
  let filename = format!(
    "FNF-{}-{}.{}",
    engine_name.replace(' ', "-"),
    chrono::Utc::now().timestamp(),
    extension
  );
  let download_path = downloads_dir.join(&filename);

  debug!("Download path: {}", download_path.display());

  // Get the content length for progress tracking
  let total_size = response.content_length().unwrap_or(0) as usize;

  // Update the download started event with actual content length
  app
    .emit("download-started", DownloadStarted {
      mod_id: download_id,
      name: engine_name.to_string(),
      content_length: total_size,
      thumbnail_url: None,
    })
    .unwrap_or_else(|e|
      error!("Failed to emit updated download-started event: {}", e)
    );

  // Create a file to write to
  let mut file = match std::fs::File::create(&download_path) {
    Ok(file) => file,
    Err(e) => {
      let error_msg = format!("Failed to create file: {}", e);
      error!("{}", error_msg);

      // Emit error event
      app
        .emit("download-error", DownloadError {
          mod_id: download_id,
          name: engine_name.to_string(),
          error: error_msg.clone(),
        })
        .unwrap_or_else(|e|
          error!("Failed to emit download-error event: {}", e)
        );

      return Err(error_msg);
    }
  };

  // Stream the response body with progress updates
  let mut stream = response.bytes_stream();
  let mut downloaded: usize = 0;
  let mut last_percentage = 0;

  while let Some(chunk_result) = stream.next().await {
    match chunk_result {
      Ok(chunk) => {
        // Write the chunk to the file
        if let Err(e) = std::io::Write::write_all(&mut file, &chunk) {
          let error_msg = format!("Failed to write to file: {}", e);
          error!("{}", error_msg);

          // Emit error event
          app
            .emit("download-error", DownloadError {
              mod_id: download_id,
              name: engine_name.to_string(),
              error: error_msg.clone(),
            })
            .unwrap_or_else(|e|
              error!("Failed to emit download-error event: {}", e)
            );

          return Err(error_msg);
        }

        // Update progress
        downloaded += chunk.len();
        let percentage = if total_size > 0 {
          ((((downloaded as f64) / (total_size as f64)) * 60.0) as u8) + 20 // 20-80% range for download
        } else {
          30 // Default to middle of range if size unknown
        };

        // Only emit progress events if percentage has changed
        if percentage != last_percentage {
          app
            .emit("download-progress", DownloadProgress {
              mod_id: download_id,
              name: engine_name.to_string(),
              bytes_downloaded: downloaded,
              total_bytes: total_size,
              percentage,
              step: "Downloading engine...".to_string(),
            })
            .unwrap_or_else(|e|
              error!("Failed to emit download-progress event: {}", e)
            );

          last_percentage = percentage;
        }
      }
      Err(e) => {
        let error_msg = format!("Failed to download chunk: {}", e);
        error!("{}", error_msg);

        // Emit error event
        app
          .emit("download-error", DownloadError {
            mod_id: download_id,
            name: engine_name.to_string(),
            error: error_msg.clone(),
          })
          .unwrap_or_else(|e|
            error!("Failed to emit download-error event: {}", e)
          );

        return Err(error_msg);
      }
    }
  }

  // Emit progress event for extraction
  app
    .emit("download-progress", DownloadProgress {
      mod_id: download_id,
      name: engine_name.to_string(),
      bytes_downloaded: total_size,
      total_bytes: total_size,
      percentage: 80,
      step: "Preparing to extract engine...".to_string(),
    })
    .unwrap_or_else(|e|
      error!("Failed to emit download-progress event: {}", e)
    );

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
      app
        .emit("download-error", DownloadError {
          mod_id: download_id,
          name: engine_name.to_string(),
          error: error_msg.clone(),
        })
        .unwrap_or_else(|e|
          error!("Failed to emit download-error event: {}", e)
        );

      return Err(error_msg);
    }
  }

  // Use the custom name if provided, otherwise use the default engine name
  let folder_name = match custom_name {
    Some(name) => {
      info!("Using custom name for engine folder: {}", name);
      name
    }
    None => {
      // Create sanitized name from engine name as before
      let sanitized_name = engine_name.replace(' ', "-");
      info!("Using default name for engine folder: {}", sanitized_name);
      sanitized_name
    }
  };

  // Create engine folder with the determined name
  let engine_folder = install_dir.join(&folder_name);

  if engine_folder.exists() {
    if update_existing.unwrap_or(false) {
      debug!(
        "Engine folder already exists, updating in place: {}",
        engine_folder.display()
      );
      // When updating, we keep the existing folder and just extract over it
      // Note: We don't remove the folder when updating to preserve any user data/mods
    } else {
      debug!(
        "Engine folder already exists, removing it: {}",
        engine_folder.display()
      );
      if let Err(e) = fs::remove_dir_all(&engine_folder) {
        let error_msg =
          format!("Failed to remove existing engine folder: {}", e);
        error!("{}", error_msg);

        // Emit error event
        app
          .emit("download-error", DownloadError {
            mod_id: download_id,
            name: engine_name.to_string(),
            error: error_msg.clone(),
          })
          .unwrap_or_else(|e|
            error!("Failed to emit download-error event: {}", e)
          );

        return Err(error_msg);
      }
    }
  }

  // Only create the directory if it doesn't exist (for updates, it should already exist)
  if !engine_folder.exists() {
    debug!("Creating engine folder: {}", engine_folder.display());
    if let Err(e) = fs::create_dir_all(&engine_folder) {
      let error_msg = format!("Failed to create engine folder: {}", e);
      error!("{}", error_msg);

      // Emit error event
      app
        .emit("download-error", DownloadError {
          mod_id: download_id,
          name: engine_name.to_string(),
          error: error_msg.clone(),
        })
        .unwrap_or_else(|e|
          error!("Failed to emit download-error event: {}", e)
        );

      return Err(error_msg);
    }
  }

  // Extract the archive
  let extraction_result = extract_archive(
    &download_path,
    &engine_folder,
    &engine_name,
    download_id,
    &app
  );
  if let Err(e) = extraction_result {
    return Err(e);
  }

  // Reorganize structure if needed
  if let Err(e) = reorganize_modpack(&engine_folder) {
    warn!("Failed to reorganize engine structure: {}", e);
    // Continue anyway as this is not critical
  }

  // Emit progress event for finalizing
  app
    .emit("download-progress", DownloadProgress {
      mod_id: download_id,
      name: engine_name.to_string(),
      bytes_downloaded: 95,
      total_bytes: 100,
      percentage: 95,
      step: "Finalizing engine installation...".to_string(),
    })
    .unwrap_or_else(|e|
      error!("Failed to emit download-progress event: {}", e)
    );

  // Find executable in the extracted files
  debug!("Searching for executables in engine folder");
  let executables = find_executables(&engine_folder);
  let executable_path = executables
    .first()
    .map(|p| p.to_string_lossy().to_string());

  // Extract icon if we have an executable
  let icon_data = match &executable_path {
    Some(exe_path) => {
      debug!("Extracting icon from: {}", exe_path);
      extract_executable_icon(Path::new(exe_path))
    }
    None => None,
  };

  // Create custom metadata folder if it doesn't exist
  let metadata_folder = engine_folder.join(".flight");
  if !metadata_folder.exists() {
    debug!("Creating metadata folder: {}", metadata_folder.display());
    if let Err(e) = fs::create_dir_all(&metadata_folder) {
      warn!("Failed to create metadata folder: {}", e);
      // Continue anyway
    }
  }

  // Copy standard banner and logo from resources
  let banner_path =
    "resources/".to_string() + engine_banner.as_deref().unwrap_or("");
  let logo_path = "resources/".to_string() + &engine_logo;
  let engine_icon_path = "resources/".to_string() + &engine_icon;

  let banner_src = app
    .path()
    .resolve(&banner_path, BaseDirectory::Resource)
    .unwrap_or_else(|e| {
      error!("Failed to resolve banner path: {}", e);
      PathBuf::new()
    });
  info!("Resolved banner path: {}", banner_src.display());
  let logo_src = app
    .path()
    .resolve(&logo_path, BaseDirectory::Resource)
    .unwrap_or_else(|e| {
      error!("Failed to resolve logo path: {}", e);
      PathBuf::new()
    });
  info!("Resolved logo path: {}", logo_src.display());
  let icon_src = app
    .path()
    .resolve(&engine_icon_path, BaseDirectory::Resource)
    .unwrap_or_else(|e| {
      error!("Failed to resolve engine icon path: {}", e);
      PathBuf::new()
    });
  info!("Resolved engine icon path: {}", icon_src.display());

  // Read the banner, logo, and icon files if they exist in resources
  let banner_data = if banner_src.exists() {
    match
      crate::modutils::get_mod_icon_data(
        &banner_src.to_string_lossy().to_string()
      )
    {
      Ok(data) => Some(data),
      Err(e) => {
        warn!("Failed to read banner image: {}", e);
        None
      }
    }
  } else {
    debug!("{} banner image not found", engine_name);
    None
  };

  let logo_data = if logo_src.exists() {
    match
      crate::modutils::get_mod_icon_data(
        &logo_src.to_string_lossy().to_string()
      )
    {
      Ok(data) => Some(data),
      Err(e) => {
        warn!("Failed to read logo image: {}", e);
        None
      }
    }
  } else {
    None
  };
  let engine_icon_data = if icon_src.exists() {
    debug!("Loading engine icon from: {}", icon_src.display());
    match
      crate::modutils::get_mod_icon_data(
        &icon_src.to_string_lossy().to_string()
      )
    {
      Ok(data) => Some(data),
      Err(e) => {
        warn!("Failed to read engine icon image: {}", e);
        None
      }
    }
  } else {
    debug!("{} engine icon image not found", engine_name);
    None
  };

  // Create the mod info
  let id = uuid::Uuid::new_v4().to_string();
  let mod_info = ModInfo {
    id: id.clone(),
    name: engine_name.to_string(),
    path: engine_folder.to_string_lossy().to_string(),
    executable_path,
    display_order: Some(0),
    icon_data: icon_data.clone(),
    description: Some(engine_description.to_string()),
    banner_data: banner_data,
    logo_data: logo_data,
    logo_position: Some("left_bottom".to_string()),
    version: Some(engine_version.to_string()),
    engine: Some(crate::models::Engine {
      engine_type: Some(engine_type.to_string()),
      engine_name: Some(engine_name.to_string()),
      engine_icon: engine_icon_data,
      mods_folder: Some(true),
      mods_folder_path: Some("mods".to_string()),
    }),
    process_id: None, // Initialize with None since mod is not running yet
    contributors: None,
    metadata_version: Some(CURRENT_METADATA_VERSION),
    date_added: Some(chrono::Utc::now().timestamp()), // Set current timestamp as date added
    last_played: None, // Initialize with None since mod is not played yet
    gamebanana: None, // No GameBanana data for engines
    save_terminal_output: false, // Default to false
  };

  // Add the mod to our state
  let mods_state = app.state::<crate::models::ModsState>();
  let mut mods = mods_state.0.lock().unwrap();
  mods.insert(id.clone(), mod_info.clone());

  info!("Successfully downloaded and installed {} engine", engine_name);

  // Emit download finished event
  app
    .emit("download-finished", DownloadFinished {
      mod_id: download_id,
      name: engine_name.to_string(),
      mod_info: mod_info.clone(),
    })
    .unwrap_or_else(|e|
      error!("Failed to emit download-finished event: {}", e)
    );

  // Emit progress event for completion
  app
    .emit("download-progress", DownloadProgress {
      mod_id: download_id,
      name: engine_name.to_string(),
      bytes_downloaded: 100,
      total_bytes: 100,
      percentage: 100,
      step: "Engine installation complete".to_string(),
    })
    .unwrap_or_else(|e|
      error!("Failed to emit download-progress event: {}", e)
    );

  Ok(mod_info)
}

// Helper function to extract archives of different types
fn extract_archive(
  download_path: &PathBuf,
  mod_folder: &PathBuf,
  name: &str,
  mod_id: i64,
  app: &tauri::AppHandle
) -> Result<(), String> {
  // Store the extraction result in a variable
  let result = if
    download_path.extension().and_then(|e| e.to_str()) == Some("zip")
  {
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
        let error_msg =
          format!("Could not extract archive with unknown format: {}", e);
        error!("{}", error_msg);

        // Emit error event
        app
          .emit("download-error", DownloadError {
            mod_id,
            name: name.to_string(),
            error: error_msg.clone(),
          })
          .unwrap_or_else(|e|
            error!("Failed to emit download-error event: {}", e)
          );

        Err(error_msg)
      }
    }
  };

  // If extraction was successful, delete the archive file
  if result.is_ok() {
    debug!(
      "Extraction successful, deleting archive file: {}",
      download_path.display()
    );
    if let Err(e) = fs::remove_file(download_path) {
      warn!("Failed to delete archive file after extraction: {}", e);
      // Continue anyway as this is not critical
    } else {
      debug!("Archive file deleted successfully");
    }
  }

  // Return the original extraction result
  result
}

// Function to reorganize modpack content for easier use
// This checks if there's only one directory in the root and if so,
// moves all its contents up a level so we can see them
fn reorganize_modpack(mod_folder: &Path) -> Result<bool, String> {
  debug!("Checking modpack structure at: {}", mod_folder.display());

  // Get all entries in the directory
  let entries = match fs::read_dir(mod_folder) {
    Ok(entries) => entries.filter_map(|e| e.ok()).collect::<Vec<_>>(),
    Err(e) => {
      let error_msg = format!("Failed to read modpack directory: {}", e);
      error!("{}", error_msg);
      return Err(error_msg);
    }
  };

  // Check if there's only one entry and it's a directory
  if entries.len() == 1 && entries[0].path().is_dir() {
    let single_dir = entries[0].path();
    debug!(
      "Modpack contains a single directory, reorganizing: {}",
      single_dir.file_name().unwrap_or_default().to_string_lossy()
    );

    // Get all contents of this single directory
    let sub_entries = match fs::read_dir(&single_dir) {
      Ok(entries) => entries.filter_map(|e| e.ok()).collect::<Vec<_>>(),
      Err(e) => {
        let error_msg = format!("Failed to read subdirectory: {}", e);
        error!("{}", error_msg);
        return Err(error_msg);
      }
    };

    // Move each item up one level
    for entry in sub_entries {
      let source_path = entry.path();
      let filename = match source_path.file_name() {
        Some(name) => name,
        None => {
          continue;
        } // Skip entries without a valid filename
      };

      let dest_path = mod_folder.join(filename);
      debug!("Moving {} to {}", source_path.display(), dest_path.display());

      // Use different methods for files and directories
      if source_path.is_dir() {
        if let Err(e) = fs::rename(&source_path, &dest_path) {
          warn!("Failed to move directory {}: {}", source_path.display(), e);
          // Fall back to copy+delete for cross-device moves
          if let Err(e) = copy_dir_all(&source_path, &dest_path) {
            let error_msg = format!("Failed to copy directory: {}", e);
            error!("{}", error_msg);
            return Err(error_msg);
          }
          if let Err(e) = fs::remove_dir_all(&source_path) {
            warn!("Failed to remove source directory after copy: {}", e);
            // Continue anyway as the files were copied successfully
          }
        }
      } else {
        if let Err(e) = fs::rename(&source_path, &dest_path) {
          warn!("Failed to move file {}: {}", source_path.display(), e);
          // Fall back to copy+delete for cross-device moves
          if let Err(e) = fs::copy(&source_path, &dest_path) {
            let error_msg = format!("Failed to copy file: {}", e);
            error!("{}", error_msg);
            return Err(error_msg);
          }
          if let Err(e) = fs::remove_file(&source_path) {
            warn!("Failed to remove source file after copy: {}", e);
            // Continue anyway as the file was copied successfully
          }
        }
      }
    }

    // Delete the now-empty directory
    debug!("Removing empty directory: {}", single_dir.display());
    if let Err(e) = fs::remove_dir(&single_dir) {
      let error_msg = format!("Failed to remove empty directory: {}", e);
      error!("{}", error_msg);
      return Err(error_msg);
    }

    debug!("Successfully reorganized modpack structure");
    Ok(true)
  } else {
    debug!("Modpack contains multiple entries, no reorganization needed.");
    Ok(false)
  }
}

// Helper function to recursively copy directories
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
  if !dst.exists() {
    fs::create_dir_all(dst)?;
  }

  for entry in fs::read_dir(src)? {
    let entry = entry?;
    let ty = entry.file_type()?;
    let src_path = entry.path();
    let dst_path = dst.join(entry.file_name());

    if ty.is_dir() {
      copy_dir_all(&src_path, &dst_path)?;
    } else {
      fs::copy(&src_path, &dst_path)?;
    }
  }

  Ok(())
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
      app
        .emit("download-error", DownloadError {
          mod_id,
          name: name.to_string(),
          error: error_msg.clone(),
        })
        .unwrap_or_else(|e|
          error!("Failed to emit download-error event: {}", e)
        );

      return Err(error_msg);
    }
  };

  let mut archive = match zip::ZipArchive::new(file) {
    Ok(archive) => archive,
    Err(e) => {
      let error_msg = format!("Failed to read zip archive: {}", e);
      error!("{}", error_msg);

      // Emit error event
      app
        .emit("download-error", DownloadError {
          mod_id,
          name: name.to_string(),
          error: error_msg.clone(),
        })
        .unwrap_or_else(|e|
          error!("Failed to emit download-error event: {}", e)
        );

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
      None => {
        continue;
      }
    };

    // Update extraction progress
    let extract_percentage =
      ((((i as f64) / (total_files as f64)) * 10.0) as u8) + 85; // 85-95% range for extraction
    if extract_percentage != last_percentage {
      app
        .emit("download-progress", DownloadProgress {
          mod_id,
          name: name.to_string(),
          bytes_downloaded: i,
          total_bytes: total_files,
          percentage: extract_percentage,
          step: format!("Extracting file {}/{}", i + 1, total_files),
        })
        .unwrap_or_else(|e|
          error!("Failed to emit download-progress event: {}", e)
        );

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
      app
        .emit("download-progress", DownloadProgress {
          mod_id,
          name: name.to_string(),
          bytes_downloaded: 90,
          total_bytes: 100,
          percentage: 90,
          step: "Extracting 7z archive completed".to_string(),
        })
        .unwrap_or_else(|e|
          error!("Failed to emit download-progress event: {}", e)
        );

      Ok(())
    }
    Err(e) => {
      let error_msg = format!("Failed to extract 7z archive: {}", e);
      error!("{}", error_msg);

      // Emit error event
      app
        .emit("download-error", DownloadError {
          mod_id,
          name: name.to_string(),
          error: error_msg.clone(),
        })
        .unwrap_or_else(|e|
          error!("Failed to emit download-error event: {}", e)
        );

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
          debug!(
            "Successfully extracted RAR archive to {}",
            mod_folder.display()
          );

          // Since we don't have file count for progress, use a simple progress indicator
          app
            .emit("download-progress", DownloadProgress {
              mod_id,
              name: name.to_string(),
              bytes_downloaded: 90,
              total_bytes: 100,
              percentage: 90,
              step: "Extracting RAR archive completed".to_string(),
            })
            .unwrap_or_else(|e|
              error!("Failed to emit download-progress event: {}", e)
            );

          Ok(())
        }
        Err(e) => {
          let error_msg = format!("Failed to extract RAR archive: {}", e);
          error!("{}", error_msg);

          // Emit error event
          app
            .emit("download-error", DownloadError {
              mod_id,
              name: name.to_string(),
              error: error_msg.clone(),
            })
            .unwrap_or_else(|e|
              error!("Failed to emit download-error event: {}", e)
            );

          Err(error_msg)
        }
      }
    }
    Err(e) => {
      let error_msg = format!("Failed to open RAR archive: {}", e);
      error!("{}", error_msg);

      // Emit error event
      app
        .emit("download-error", DownloadError {
          mod_id,
          name: name.to_string(),
          error: error_msg.clone(),
        })
        .unwrap_or_else(|e|
          error!("Failed to emit download-error event: {}", e)
        );

      Err(error_msg)
    }
  }
}

// Function to get the install location from settings (defaults to app data dir)
pub fn get_default_install_location(app: &tauri::AppHandle) -> PathBuf {
  // Default install location if settings can't be loaded
  app
    .path()
    .app_data_dir()
    .unwrap_or_else(|_| {
      // Fall back to OS downloads directory if app data dir is unavailable
      app
        .path()
        .download_dir()
        .unwrap_or_else(|_| {
          // Last resort: use current directory
          PathBuf::from(".")
        })
    })
    .join("mods")
}
