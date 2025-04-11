use crate::models::ModInfo;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use image::{ImageBuffer, Rgba};
use log::{debug, error, warn};
use std::fs;
use std::path::{Path, PathBuf};
use std::ptr;

#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::HINSTANCE;
#[cfg(target_os = "windows")]
use winapi::shared::windef::HICON;
#[cfg(target_os = "windows")]
use winapi::um::shellapi::ExtractIconW;

// Find the executable file(s) within a directory with priority for FNF patterns
pub fn find_executables(dir_path: &Path) -> Vec<PathBuf> {
    let mut executables = Vec::new();
    let mut prioritized_executables = Vec::new();
    
    // Common FNF executable names to prioritize
    let priority_names = [
        "Funkin", "funkin", 
        "FNF", "fnf",
        "Friday Night Funkin", 
        "game", "Game",
        "play", "Play",
        "start", "Start",
        "launcher", "Launcher"
    ];
    
    // Priority folders where executables are commonly found
    let priority_folders = [
        "bin", "game", "Game", "FNF", "fnf", 
        "executable", "Executable", "exe",
        "Friday Night Funkin", "Funkin"
    ];
    
    // Track recursion depth to prevent going too deep
    fn search_directory(
        dir_path: &Path, 
        executables: &mut Vec<PathBuf>, 
        prioritized_executables: &mut Vec<PathBuf>,
        priority_names: &[&str],
        priority_folders: &[&str],
        depth: usize
    ) {
        // Limit recursion depth to prevent excessive searching
        if depth > 5 {  // Increased from 3 to 5 for deeper searching
            return;
        }
        
        // Check if current directory is a priority folder
        let is_priority_dir = if let Some(dir_name) = dir_path.file_name() {
            let dir_name_str = dir_name.to_string_lossy();
            priority_folders.iter().any(|&name| dir_name_str.contains(name))
        } else {
            false
        };
        
        if let Ok(entries) = fs::read_dir(dir_path) {
            debug!("Searching for executables in directory: {}", dir_path.display());
            
            // First pass: collect all executables
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                
                if path.is_file() {
                    // On Windows, look for .exe files
                    if let Some(extension) = path.extension() {
                        if extension.to_string_lossy().to_lowercase() == "exe" {
                            let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                            debug!("Found executable: {}", file_name);
                            
                            // Check if this is a priority executable (by name)
                            let is_priority = priority_names.iter()
                                .any(|&name| file_name.to_lowercase().contains(&name.to_lowercase()));
                            
                            // Add to appropriate list
                            if is_priority || is_priority_dir {
                                debug!("Prioritizing executable: {}", file_name);
                                prioritized_executables.push(path.clone());
                            } else {
                                executables.push(path.clone());
                            }
                        }
                    }
                } else if path.is_dir() {
                    // Recursively search subdirectories
                    search_directory(&path, executables, prioritized_executables, 
                                    priority_names, priority_folders, depth + 1);
                }
            }
        }
    }
    
    // Start the recursive search at depth 0
    search_directory(dir_path, &mut executables, &mut prioritized_executables, 
                    &priority_names, &priority_folders, 0);
    
    // Combine the lists, with prioritized executables first
    debug!("Found {} prioritized and {} regular executables", 
           prioritized_executables.len(), executables.len());
    
    prioritized_executables.append(&mut executables);
    
    if prioritized_executables.is_empty() {
        debug!("No executables found in: {}", dir_path.display());
    } else {
        debug!("Final executable list for {}: {:#?}", 
               dir_path.display(), 
               prioritized_executables.iter()
                  .map(|p| p.file_name().unwrap_or_default().to_string_lossy())
                  .collect::<Vec<_>>());
    }
    
    prioritized_executables
}

// Extract icon from executable and convert to base64 data URL
#[cfg(target_os = "windows")]
pub fn extract_executable_icon(exe_path: &Path) -> Option<String> {
    // Create a default icon as a fallback
    let default_icon = || -> Option<String> {
        let mut buffer = Vec::new();
        // Create a small solid-colored square as default icon
        let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(32, 32);

        // Fill with a color (blue in this case)
        for pixel in img.pixels_mut() {
            *pixel = Rgba([30, 144, 255, 255]); // DodgerBlue
        }

        if img
            .write_to(
                &mut std::io::Cursor::new(&mut buffer),
                image::ImageFormat::Png,
            )
            .is_ok()
        {
            let b64 = BASE64.encode(&buffer);
            Some(format!("data:image/png;base64,{}", b64))
        } else {
            None
        }
    };

    // Try using the ico crate to read icons directly from the exe file
    if let Ok(file) = std::fs::File::open(exe_path) {
        if let Ok(icon_dir) = ico::IconDir::read(file) {
            if !icon_dir.entries().is_empty() {
                // Filter for smaller icons, preferably 32x32 or close
                let entry = icon_dir
                    .entries()
                    .iter()
                    .filter(|e| e.width() <= 32)
                    .max_by_key(|e| e.width())
                    .unwrap_or(&icon_dir.entries()[0]);

                if let Ok(image) = entry.decode() {
                    // Convert to buffer
                    let mut buffer = Vec::new();

                    // Convert IconImage to RgbaImage
                    let width = image.width();
                    let height = image.height();
                    let rgba_data = image.rgba_data();
                    // Create an ImageBuffer from the raw RGBA data.
                    // Use unwrap_or_else for safety in case the data is invalid.
                    let rgba_image = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, rgba_data.to_vec())
                        .unwrap_or_else(|| {
                            eprintln!("Warning: Failed to create ImageBuffer from icon data. Using empty buffer.");
                            ImageBuffer::new(width, height) // Provide a fallback empty buffer
                        });

                    // Flip the image vertically to fix upside down issue
                    let flipped_image = image::imageops::flip_vertical(&rgba_image);

                    // Save as PNG to memory buffer
                    if flipped_image
                        .write_to(
                            &mut std::io::Cursor::new(&mut buffer),
                            image::ImageFormat::Png,
                        )
                        .is_ok()
                    {
                        let b64 = BASE64.encode(&buffer);
                        return Some(format!("data:image/png;base64,{}", b64));
                    }
                }
            }
        }
    }

    // If the ico crate approach didn't work, try using the Windows API directly
    unsafe {
        use std::mem;
        use winapi::um::wingdi::{
            CreateCompatibleDC, DeleteDC, GetDIBits, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
            DIB_RGB_COLORS,
        };
        use winapi::um::winuser::{GetIconInfo, ICONINFO};

        let wide_path: Vec<u16> = exe_path
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let h_instance = std::ptr::null_mut::<std::ffi::c_void>() as HINSTANCE;
        let icon_handle = ExtractIconW(h_instance, wide_path.as_ptr(), 0);

        if !icon_handle.is_null() && icon_handle != ptr::null_mut() {
            // Create a new RGB image with transparency
            let mut icon_info: ICONINFO = mem::zeroed();
            let icon_info_result = GetIconInfo(icon_handle, &mut icon_info);

            if icon_info_result != 0 {
                // We've got the icon info, now extract the bitmap data
                let h_dc = CreateCompatibleDC(ptr::null_mut());

                if !h_dc.is_null() {
                    // Get the icon dimensions
                    let mut bitmap_info: BITMAPINFO = mem::zeroed();
                    bitmap_info.bmiHeader.biSize = mem::size_of::<BITMAPINFOHEADER>() as u32;
                    bitmap_info.bmiHeader.biWidth = 32;
                    bitmap_info.bmiHeader.biHeight = -32; // Negative height for top-down DIB
                    bitmap_info.bmiHeader.biPlanes = 1;
                    bitmap_info.bmiHeader.biBitCount = 32; // 32-bit RGBA
                    bitmap_info.bmiHeader.biCompression = BI_RGB;

                    // Create a buffer for the pixel data
                    let mut pixels: Vec<u8> = vec![0; (32 * 32 * 4) as usize];

                    // Extract the bitmap data
                    let result = GetDIBits(
                        h_dc,
                        icon_info.hbmColor,
                        0,
                        32,
                        pixels.as_mut_ptr() as *mut _,
                        &mut bitmap_info,
                        DIB_RGB_COLORS,
                    );

                    // Clean up resources
                    DeleteDC(h_dc);
                    winapi::um::wingdi::DeleteObject(icon_info.hbmColor as _);
                    winapi::um::wingdi::DeleteObject(icon_info.hbmMask as _);

                    if result != 0 {
                        // Fix pixel format (Windows uses BGRA, we need RGBA)
                        for chunk in pixels.chunks_mut(4) {
                            // Swap B and R channels
                            let b = chunk[0];
                            chunk[0] = chunk[2];
                            chunk[2] = b;
                        }

                        // Create an image from the pixel data
                        let img = ImageBuffer::<Rgba<u8>, _>::from_raw(32, 32, pixels)
                            .unwrap_or(ImageBuffer::new(32, 32));

                        // Convert to PNG and encode as base64
                        let mut buffer = Vec::new();
                        if img
                            .write_to(
                                &mut std::io::Cursor::new(&mut buffer),
                                image::ImageFormat::Png,
                            )
                            .is_ok()
                        {
                            let b64 = BASE64.encode(&buffer);
                            winapi::um::winuser::DestroyIcon(icon_handle);
                            return Some(format!("data:image/png;base64,{}", b64));
                        }
                    }
                }
            }

            // Clean up if we couldn't extract the bitmap
            winapi::um::winuser::DestroyIcon(icon_handle);
        }
    }

    // Return default icon if all extraction methods fail
    default_icon()
}

// Non-Windows implementation (just returns a default icon)
#[cfg(not(target_os = "windows"))]
pub fn extract_executable_icon(_exe_path: &Path) -> Option<String> {
    let mut buffer = Vec::new();
    let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(32, 32);

    // Fill with a color (blue in this case)
    for pixel in img.pixels_mut() {
        *pixel = Rgba([30, 144, 255, 255]); // DodgerBlue
    }

    if img
        .write_to(
            &mut std::io::Cursor::new(&mut buffer),
            image::ImageFormat::Png,
        )
        .is_ok()
    {
        let b64 = BASE64.encode(&buffer);
        Some(format!("data:image/png;base64,{}", b64))
    } else {
        None
    }
}

// Function to check for custom banner/logo images in the .flight folder
pub fn check_for_custom_images(mod_folder: &Path) -> (Option<String>, Option<String>) {
    let fnfml_folder = mod_folder.join(".flight");
    let mut banner_data = None;
    let mut logo_data = None;
    
    if fnfml_folder.exists() && fnfml_folder.is_dir() {
        debug!("Found metadata folder in mod: {}!", mod_folder.display());
        
        // Check for banner image (banner.png or banner.webp)
        let banner_png = fnfml_folder.join("banner.png");
        let banner_webp = fnfml_folder.join("banner.webp");
        
        if banner_png.exists() && banner_png.is_file() {
            debug!("Found custom banner.png");
            banner_data = match std::fs::read(&banner_png) {
                Ok(data) => {
                    let b64 = BASE64.encode(&data);
                    Some(format!("data:image/png;base64,{}", b64))
                },
                Err(e) => {
                    error!("Failed to read banner.png: {}", e);
                    None
                }
            };
        } else if banner_webp.exists() && banner_webp.is_file() {
            debug!("Found custom banner.webp");
            banner_data = match std::fs::read(&banner_webp) {
                Ok(data) => {
                    let b64 = BASE64.encode(&data);
                    Some(format!("data:image/webp;base64,{}", b64))
                },
                Err(e) => {
                    error!("Failed to read banner.webp: {}", e);
                    None
                }
            };
        }
        
        // Check for logo image (logo.png or logo.webp)
        let logo_png = fnfml_folder.join("logo.png");
        let logo_webp = fnfml_folder.join("logo.webp");
        
        if logo_png.exists() && logo_png.is_file() {
            debug!("Found custom logo.png");
            logo_data = match std::fs::read(&logo_png) {
                Ok(data) => {
                    let b64 = BASE64.encode(&data);
                    Some(format!("data:image/png;base64,{}", b64))
                },
                Err(e) => {
                    error!("Failed to read logo.png: {}", e);
                    None
                }
            };
        } else if logo_webp.exists() && logo_webp.is_file() {
            debug!("Found custom logo.webp");
            logo_data = match std::fs::read(&logo_webp) {
                Ok(data) => {
                    let b64 = BASE64.encode(&data);
                    Some(format!("data:image/webp;base64,{}", b64))
                },
                Err(e) => {
                    error!("Failed to read logo.webp: {}", e);
                    None
                }
            };
        }
    }
    
    (banner_data, logo_data)
}

// Function to read metadata.json file from the .flight folder
fn read_metadata_json(mod_folder: &Path) -> Option<serde_json::Value> {
    let fnfml_folder = mod_folder.join(".flight");
    let metadata_path = fnfml_folder.join("metadata.json");
    
    if metadata_path.exists() && metadata_path.is_file() {
        debug!("Found metadata.json in: {}", metadata_path.display());
        
        match fs::read_to_string(&metadata_path) {
            Ok(content) => {
                match serde_json::from_str(&content) {
                    Ok(json) => {
                        debug!("Successfully parsed metadata.json");
                        return Some(json);
                    },
                    Err(e) => {
                        error!("Failed to parse metadata.json: {}", e);
                    }
                }
            },
            Err(e) => {
                error!("Failed to read metadata.json: {}", e);
            }
        }
    }
    
    None
}

// Function to create a ModInfo object from a path
pub fn create_mod_info(path: &str) -> Result<ModInfo, String> {
    let path_obj = Path::new(path);

    // Get the folder name as the default mod name
    let folder_name = match path_obj.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => {
            error!("Invalid folder path: {}", path);
            return Err("Invalid folder path".to_string());
        }
    };

    // Read metadata.json if it exists
    let metadata = read_metadata_json(path_obj);
    
    // Extract metadata values or use defaults
    let name = match &metadata {
        Some(json) => {
            if let Some(name_value) = json.get("name").and_then(|v| v.as_str()) {
                debug!("Using name from metadata.json: {}", name_value);
                name_value.to_string()
            } else {
                folder_name
            }
        },
        None => folder_name,
    };
    
    let description = match &metadata {
        Some(json) => {
            if let Some(desc_value) = json.get("description").and_then(|v| v.as_str()) {
                debug!("Using description from metadata.json");
                Some(desc_value.to_string())
            } else {
                None
            }
        },
        None => None,
    };
    
    let version = match &metadata {
        Some(json) => {
            if let Some(version_value) = json.get("version").and_then(|v| v.as_str()) {
                debug!("Using version from metadata.json: {}", version_value);
                Some(version_value.to_string())
            } else {
                None
            }
        },
        None => None,
    };
      let engine_type = match &metadata {
        Some(json) => {
            if let Some(engine_value) = json.get("engine_type").and_then(|v| v.as_str()) {
                debug!("Using engine_type from metadata.json: {}", engine_value);
                Some(engine_value.to_string())
            } else {
                None
            }
        },
        None => None,
    };
    
    // Parse the new engine field if it exists
    let engine = match &metadata {
        Some(json) => {
            if let Some(engine_json) = json.get("engine") {
                debug!("Found engine metadata in metadata.json");
                
                // Extract engine_type from engine object
                let engine_type = engine_json.get("engine_type")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                    
                // Extract engine_name from engine object
                let engine_name = engine_json.get("engine_name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                      // Extract engine_icon from engine object
                let engine_icon = engine_json.get("engine_icon")
                    .and_then(|v| v.as_str())
                    .and_then(|icon_path| {
                        // Convert relative path to absolute path based on mod folder
                        let absolute_path = path_obj.join(icon_path);
                        debug!("Loading engine icon from: {}", absolute_path.display());
                        
                        // Try to read the file
                        match fs::read(&absolute_path) {
                            Ok(data) => {
                                // Determine file extension/mime type
                                let mime_type = if let Some(ext) = absolute_path.extension() {
                                    match ext.to_string_lossy().to_lowercase().as_str() {
                                        "png" => "image/png",
                                        "webp" => "image/webp", 
                                        "jpg" | "jpeg" => "image/jpeg",
                                        "gif" => "image/gif",
                                        _ => "application/octet-stream" // Default mime type
                                    }
                                } else {
                                    "application/octet-stream" // Default mime type
                                };
                                
                                // Convert to base64
                                let b64 = BASE64.encode(&data);
                                Some(format!("data:{};base64,{}", mime_type, b64))
                            },
                            Err(e) => {
                                error!("Failed to read engine icon at {}: {}", absolute_path.display(), e);
                                Some(icon_path.to_string()) // Fallback to original path if file can't be read
                            }
                        }
                    });
                    
                // Extract mods_folder boolean
                let mods_folder = engine_json.get("mods_folder")
                    .and_then(|v| v.as_bool());
                    
                // Extract mods_folder_path
                let mods_folder_path = engine_json.get("mods_folder_path")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                
                // Create the Engine object
                Some(crate::models::Engine {
                    engine_type,
                    engine_name,
                    engine_icon,
                    mods_folder,
                    mods_folder_path,
                })
            } else {
                None
            }
        },
        None => None,
    };

    // Find executable file(s)
    debug!("Searching for executables in: {}", path);
    let executables = find_executables(path_obj);
    let executable_path = executables.first().map(|p| p.to_string_lossy().to_string());
    
    if let Some(exe) = &executable_path {
        debug!("Found executable: {}", exe);
    } else {
        warn!("No executable found for mod: {}", name);
    }

    // Extract icon if we have an executable
    let icon_data = match &executable_path {
        Some(exe_path) => {
            debug!("Extracting icon from: {}", exe_path);
            extract_executable_icon(Path::new(exe_path))
        },
        None => None,
    };

    // Check for custom images
    let (banner_data, logo_data) = check_for_custom_images(path_obj);

    // Create a unique ID
    let id = uuid::Uuid::new_v4().to_string();
    debug!("Generated mod ID: {}", id);    let mod_info = ModInfo {
        id,
        name,
        path: path.to_string(),
        description,
        display_order: Some(0), // default display order (top of the list)
        executable_path,
        icon_data,
        banner_data,
        logo_data,
        version,
        engine_type, // Keep for backward compatibility
        engine,      // Add the new engine field
    };

    Ok(mod_info)
}