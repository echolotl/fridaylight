use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use std::sync::Mutex;
use std::fmt;

// Constants for metadata versioning
pub const MIN_METADATA_VERSION: u32 = 1;
pub const CURRENT_METADATA_VERSION: u32 = 1;

// Error type for metadata validation
#[derive(Debug, Clone)]
pub enum MetadataError {
  MissingVersion,
  VersionTooLow {
    provided: u32,
    minimum: u32,
  },
}

impl fmt::Display for MetadataError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MetadataError::MissingVersion =>
        write!(f, "Missing required metadata_version field"),
      MetadataError::VersionTooLow { provided, minimum } =>
        write!(
          f,
          "Metadata version {} is too old. Minimum supported version is {}",
          provided,
          minimum
        ),
    }
  }
}

// Define a structure for GameBanana mods (used for ModCard)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameBananaMod {
  pub id: i64,
  pub name: String,
  pub owner: String,
  pub description: String,
  pub thumbnail_url: String,
  pub download_url: String,
  pub views: i64,
  pub downloads: i64,
  pub likes: i64,
  pub model_name: String,
  pub profile_url: String,
  pub image_url: String,
  pub initial_visibility: String,
  pub period: String,
  pub submitter_id: i64,
  pub submitter_name: String,
  pub submitter_profile_url: String,
  pub submitter_avatar_url: String,
  pub submitter_upic: Option<String>,
  pub post_count: i64,
  pub category_name: String,
  pub category_profile_url: String,
  pub category_icon_url: String,
  pub singular_title: String,
  pub icon_classes: String,
  pub date_added: i64,
  pub date_modified: i64,
  pub date_updated: i64,
  pub has_files: bool,
  pub tags: Vec<String>,
  pub preview_images: Vec<GameBananaModImage>,
  pub version: String,
  pub is_obsolete: bool,
  pub has_content_ratings: bool,
  pub view_count: i64,
  pub is_owned_by_accessor: bool,
  pub was_featured: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameBananaModImage {
  pub image_type: String,
  pub base_url: String,
  pub file_name: String,
  pub file100: String,
  pub file220: Option<String>,
  pub file530: Option<String>,
  pub file800: Option<String>,
  pub height100: Option<i64>,
  pub width100: Option<i64>,
  pub height220: Option<i64>,
  pub width220: Option<i64>,
  pub height530: Option<i64>,
  pub width530: Option<i64>,
  pub height800: Option<i64>,
  pub width800: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Engine {
  pub engine_type: Option<String>,
  pub engine_name: Option<String>,
  pub engine_icon: Option<String>, // Base64 encoded engine icon
  pub mods_folder: Option<bool>,
  pub mods_folder_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contributor {
  pub name: String,
  pub icon: Option<String>, // Path to contributor icon or base64 data
  pub role: Option<String>, // Role description of the contributor
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContributorGroup {
  pub group: String,
  pub members: Vec<Contributor>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModInfo {
  pub id: String,
  pub name: String,
  pub path: String,
  pub metadata_version: Option<u32>,
  pub description: Option<String>,
  pub executable_path: Option<String>,
  pub icon_data: Option<String>, // Base64 encoded icon data
  pub banner_data: Option<String>, // Base64 encoded banner image data
  pub logo_data: Option<String>, // Base64 encoded logo image data
  pub logo_position: Option<String>, // Position of logo in banner: left_bottom, left_middle, middle
  pub version: Option<String>,
  pub engine: Option<Engine>, // Extended engine information
  pub display_order: Option<i64>,
  pub process_id: Option<u32>, // Track the running process ID
  pub contributors: Option<Vec<ContributorGroup>>, // List of contributor groups
  pub last_played: Option<i64>, // Unix timestamp when mod was last played
  pub date_added: Option<i64>, // Unix timestamp when mod was added
  pub gamebanana: Option<ModInfoGBData>, // GameBanana mod data
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModInfoGBData {
  pub url: String,
  pub id: i64,
  pub model_type: String,
}

impl ModInfo {
  pub fn validate_metadata_version(&self) -> Result<(), MetadataError> {
    match self.metadata_version {
      None => Err(MetadataError::MissingVersion),
      Some(version) if version < MIN_METADATA_VERSION => {
        Err(MetadataError::VersionTooLow {
          provided: version,
          minimum: MIN_METADATA_VERSION,
        })
      }
      Some(_) => Ok(()),
    }
  }
}

// Define a structure for mod metadata files
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModMetadataFile {
  pub name: String,
  pub description: Option<String>,
  pub folder_path: String,
  pub config_file_path: Option<String>,
  pub icon_file_path: Option<String>,
  pub icon_data: Option<String>, // Base64 encoded icon data
  pub enabled: Option<bool>, // Whether the mod is enabled
  pub version: Option<String>, // Version of the mod
  pub homepage: Option<String>, // Homepage/website URL
  pub contributors: Option<Vec<ContributorMetadata>>,
  pub license: Option<String>, // License information
  pub restart_required: Option<bool>, // Whether a restart is required
  pub dependencies: Option<HashMap<String, String>>, // Dependencies of the mod with version requirements
  pub parent_mod_id: Option<String>, // ID of the parent mod this metadata belongs to
}

// Contributor structure for mod metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContributorMetadata {
  pub name: String,
  pub role: String,
  pub email: Option<String>,
  pub url: Option<String>,
}

// Response structure for mod disable/enable operations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModDisableResult {
  pub success: bool,
  pub enabled: bool,
  pub message: String,
}

// Structure for the engine mods response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngineModsResponse {
  pub engine_type: String,
  pub executable_path: String,
  pub mods: Vec<ModMetadataFile>,
}

// Create a state to manage our mods
pub struct ModsState(pub Mutex<HashMap<String, ModInfo>>);

// Define the response structure for GameBanana mods
#[derive(Debug, Serialize, Deserialize)]
pub struct GameBananaResponse {
  pub mods: Vec<GameBananaMod>,
  pub total: i64,
}

// Event structures for download progress tracking
#[derive(Clone, Serialize)]
pub struct DownloadStarted {
  pub mod_id: i64,
  pub name: String,
  pub content_length: usize,
  pub thumbnail_url: String,
}

#[derive(Clone, Serialize)]
pub struct DownloadProgress {
  pub mod_id: i64,
  pub name: String,
  pub bytes_downloaded: usize,
  pub total_bytes: usize,
  pub percentage: u8,
  pub step: String,
}

#[derive(Clone, Serialize)]
pub struct DownloadFinished {
  pub mod_id: i64,
  pub name: String,
  pub mod_info: ModInfo,
}

#[derive(Clone, Serialize)]
pub struct DownloadError {
  pub mod_id: i64,
  pub name: String,
  pub error: String,
}

// Create a state to manage terminal output for each running mod
pub struct TerminalOutputState(pub Mutex<HashMap<String, String>>);

// Global thread-safe instance of ModsState for use across threads
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    pub static ref GLOBAL_MODS_STATE: Arc<Mutex<HashMap<String, ModInfo>>> = Arc::new(Mutex::new(HashMap::new()));
}

// Global app handle that can be accessed from anywhere, thread-safe
lazy_static! {
  pub static ref GLOBAL_APP_HANDLE: Mutex<Option<tauri::AppHandle>> =
    Mutex::new(None);
}
