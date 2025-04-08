use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use log::{debug, error};
use reqwest;

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