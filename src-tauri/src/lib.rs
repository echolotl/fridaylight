// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use image::{ImageBuffer, Rgba};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File}; // Import File
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::ptr;
use std::sync::Mutex;
use tauri::{Manager, State};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_store::StoreBuilder;

// For Windows icon extraction
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::HINSTANCE;
#[cfg(target_os = "windows")]
use winapi::shared::windef::HICON;
#[cfg(target_os = "windows")]
use winapi::um::shellapi::ExtractIconW;

// Define a structure to hold mod information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModInfo {
    id: String,
    name: String,
    path: String,
    executable_path: Option<String>,
    icon_data: Option<String>, // Base64 encoded icon data
}

// Create a state to manage our mods
pub struct ModsState(pub Mutex<HashMap<String, ModInfo>>);

// Find the executable file(s) within a directory
fn find_executables(dir_path: &Path) -> Vec<PathBuf> {
    let mut executables = Vec::new();

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.is_file() {
                // On Windows, look for .exe files
                if let Some(extension) = path.extension() {
                    if extension.to_string_lossy().to_lowercase() == "exe" {
                        executables.push(path);
                    }
                }
            } else if path.is_dir() {
                // Recursively search subdirectories, but avoid going too deep
                executables.extend(find_executables(&path));
            }
        }
    }

    executables
}

// Extract icon from executable and convert to base64 data URL
#[cfg(target_os = "windows")]
fn extract_executable_icon(exe_path: &Path) -> Option<String> {
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
fn extract_executable_icon(_exe_path: &Path) -> Option<String> {
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

// Command to open a folder dialog and get the selected folder path
#[tauri::command]
async fn select_mod_folder(app: tauri::AppHandle) -> Result<String, String> {
    let folder = app
        .dialog()
        .file()
        .set_title("Select Mod Folder")
        .blocking_pick_folder();

    match folder {
        Some(path) => Ok(path.to_string()),
        None => Err("No folder selected".to_string()),
    }
}

// Command to open a file dialog and select an executable
#[tauri::command]
async fn select_executable(app: tauri::AppHandle) -> Result<String, String> {
    let file = app
        .dialog()
        .file()
        .set_title("Select Executable")
        .add_filter("Executable", &["exe"])
        .blocking_pick_file();

    match file {
        Some(path) => Ok(path.to_string()),
        None => Err("No file selected".to_string()),
    }
}

// Command to add a mod to our list
#[tauri::command]
fn add_mod(path: String, mods_state: State<'_, ModsState>) -> Result<ModInfo, String> {
    let path_obj = Path::new(&path);

    // Get the folder name as the mod name
    let name = match path_obj.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return Err("Invalid folder path".to_string()),
    };

    // Find executable file(s)
    let executables = find_executables(path_obj);
    let executable_path = executables.first().map(|p| p.to_string_lossy().to_string());

    // Extract icon if we have an executable
    let icon_data = match &executable_path {
        Some(exe_path) => extract_executable_icon(Path::new(exe_path)),
        None => None,
    };

    // Create a unique ID
    let id = uuid::Uuid::new_v4().to_string();

    let mod_info = ModInfo {
        id: id.clone(),
        name,
        path,
        executable_path,
        icon_data,
    };

    // Add to our state
    let mut mods = mods_state.0.lock().unwrap();
    mods.insert(id, mod_info.clone());

    Ok(mod_info)
}

// Command to get all mods
#[tauri::command]
fn get_mods(mods_state: State<'_, ModsState>) -> Vec<ModInfo> {
    let mods = mods_state.0.lock().unwrap();
    mods.values().cloned().collect()
}

// Command to launch a mod
#[tauri::command]
fn launch_mod(id: String, mods_state: State<'_, ModsState>) -> Result<(), String> {
    let mods = mods_state.0.lock().unwrap();

    if let Some(mod_info) = mods.get(&id) {
        println!("Launching mod: {}", mod_info.name);

        if let Some(exe_path) = &mod_info.executable_path {
            // Launch the executable
            match Command::new(exe_path).spawn() {
                Ok(_) => {
                    println!("Successfully launched: {}", exe_path);
                    Ok(())
                }
                Err(e) => {
                    let error_msg = format!("Failed to launch executable: {}", e);
                    println!("{}", error_msg);
                    Err(error_msg)
                }
            }
        } else {
            Err(format!("No executable found for mod: {}", mod_info.name))
        }
    } else {
        Err("Mod not found".to_string())
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(ModsState(Mutex::new(HashMap::new())))
        .invoke_handler(tauri::generate_handler![
            greet,
            select_mod_folder,
            select_executable,
            add_mod,
            get_mods,
            launch_mod
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
