use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use log::{debug, error};
use reqwest;
use std::path::Path;
use image::{ImageBuffer, Rgba};

// Function to fetch and convert an image to base64 data URL
pub async fn fetch_image_as_base64(url: &str) -> Option<String> {
    debug!("Fetching image from URL: {}", url);
    
    // Create a client with a timeout
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .ok()?;
    
    // Fetch the image data
    let response = match client.get(url).send().await {
        Ok(resp) => {
            if !resp.status().is_success() {
                error!("Failed to fetch image, status: {}", resp.status());
                return None;
            }
            resp
        },
        Err(e) => {
            error!("Failed to fetch image: {}", e);
            return None;
        }
    };
    
    // Get the content type *before* consuming the response body
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "image/jpeg".to_string()); // Default to JPEG if no content type or invalid UTF-8
        
    // Get the bytes
    let bytes = match response.bytes().await {
        Ok(b) => b,
        Err(e) => {
            error!("Failed to get image bytes: {}", e);
            return None;
        }
    };
    
    // Convert to base64
    let b64 = BASE64.encode(&bytes);
    
    // Return as a data URL
    Some(format!("data:{};base64,{}", content_type, b64))
}

/// Encode raw bytes into a data URL with given mime type
pub fn encode_data_to_data_url(data: &[u8], mime: &str) -> String {
    let b64 = BASE64.encode(data);
    format!("data:{};base64,{}", mime, b64)
}

/// Read a file and encode its contents as a data URL
pub fn encode_file_to_data_url(path: &Path) -> Option<String> {
    match std::fs::read(path) {
        Ok(data) => {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("bin").to_lowercase();
            let mime = match ext.as_str() {
                "png" => "image/png",
                "jpg" | "jpeg" => "image/jpeg",
                "webp" => "image/webp",
                "gif" => "image/gif",
                _ => "application/octet-stream",
            };
            Some(encode_data_to_data_url(&data, mime))
        }
        Err(_) => None,
    }
}

/// Generate a default colored square icon (32x32 PNG) as a data URL
pub fn default_icon_data() -> String {
    let mut buffer = Vec::new();
    let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(32, 32);
    for pixel in img.pixels_mut() {
        *pixel = Rgba([30, 144, 255, 255]);
    }
    let _ = img.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png);
    encode_data_to_data_url(&buffer, "image/png")
}

// Function to extract RAR archives
pub fn extract_rar_archive(
    mut archive: unrar::OpenArchive<unrar::Process, unrar::CursorBeforeHeader>,
    extraction_path: &str
) -> Result<(), String> {
    // Create a loop to read headers and process files
    loop {
        // Try to read the next header
        let archive_with_file = match archive.read_header() {
            Ok(Some(a)) => a,
            Ok(None) => break, // No more headers, extraction complete
            Err(e) => return Err(format!("Error reading RAR header: {}", e)),
        };
        
        let file_path = archive_with_file.entry().filename.to_string_lossy().to_string();
        debug!("Extracting from RAR: {}", file_path);
        
        // Extract the file
        let (data, next_archive) = match archive_with_file.read() {
            Ok((data, next)) => (data, next),
            Err(e) => return Err(format!("Error extracting file '{}': {}", file_path, e)),
        };
        
        // Create the output path
        let outpath = std::path::Path::new(extraction_path).join(&file_path);
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = outpath.parent() {
            if !parent.exists() {
                debug!("Creating parent directory: {}", parent.display());
                if let Err(e) = std::fs::create_dir_all(parent) {
                    return Err(format!("Failed to create directory '{}': {}", parent.display(), e));
                }
            }
        }
        
        // Write the file
        if !data.is_empty() {
            if let Err(e) = std::fs::write(&outpath, &data) {
                return Err(format!("Failed to write file '{}': {}", outpath.display(), e));
            }
        } else {
            // Empty data often means it's a directory
            if !outpath.exists() {
                debug!("Creating directory: {}", outpath.display());
                if let Err(e) = std::fs::create_dir_all(&outpath) {
                    return Err(format!("Failed to create directory '{}': {}", outpath.display(), e));
                }
            }
        }
        
        // Move to the next file
        archive = next_archive;
    }
    
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn is_windows_11_or_greater() -> bool {
    use windows_version::OsVersion;
    
    // Windows 11 starts at build 22000
    if OsVersion::current() <= OsVersion::new(10, 0, 0, 22000) {
        debug!("Windows version is less than 11: {:?}", OsVersion::current());
        false // Not Windows 11 or greater
    } else {
        debug!("Windows version is 11 or greater: {:?}", OsVersion::current());
        true // Is Windows 11 or greater
    }
}

#[cfg(not(target_os = "windows"))]
pub fn is_windows_11_or_greater() -> bool {
    // Not Windows, so definitely not Windows 11
    false
}