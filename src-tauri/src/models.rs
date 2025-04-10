use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

// GameBanana API Structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameBananaMod {
    // Existing fields
    pub id: i64,
    pub name: String,
    pub owner: String,
    pub description: String,
    pub thumbnailUrl: String,
    pub downloadUrl: String,
    pub views: i64,
    pub downloads: i64,
    pub likes: i64,
    
    // New fields from GameBanana API
    pub modelName: String,
    pub profileUrl: String,
    pub imageUrl: String,
    pub initialVisibility: String,
    pub period: String,
    
    // Submitter details
    pub submitterId: i64,
    pub submitterName: String,
    pub submitterIsOnline: bool,
    pub submitterHasRipe: bool,
    pub submitterProfileUrl: String,
    pub submitterAvatarUrl: String,
    pub submitterMoreByUrl: String,
    
    // Post count
    pub postCount: i64,
    
    // Category details
    pub categoryName: String,
    pub categoryProfileUrl: String,
    pub categoryIconUrl: String,
    
    // Additional fields from normal mod data
    pub singularTitle: String,
    pub iconClasses: String,
    pub dateAdded: i64,
    pub dateModified: i64,
    pub dateUpdated: i64,
    pub hasFiles: bool,
    pub tags: Vec<String>,
    pub previewImages: Vec<GameBananaModImage>,
    pub version: String,
    pub isObsolete: bool,
    pub hasContentRatings: bool,
    pub viewCount: i64,
    pub isOwnedByAccessor: bool,
    pub wasFeatured: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameBananaModImage {
    pub imageType: String,
    pub baseUrl: String,
    pub fileName: String,
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

// Define a structure to hold mod information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub executable_path: Option<String>,
    pub icon_data: Option<String>, // Base64 encoded icon data
    pub banner_data: Option<String>, // Base64 encoded banner image data
    pub logo_data: Option<String>, // Base64 encoded logo image data
    pub version: Option<String>,
    pub engine_type: Option<String>,
    pub display_order: Option<i64>,
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
    pub enabled: Option<bool>,     // Whether the mod is enabled
}

// Response structure for mod disable/enable operations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModDisableResult {
    pub success: bool,
    pub enabled: bool,
    pub message: String,
}

// Define a structure for the engine mods response
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