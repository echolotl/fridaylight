use log::{ info, error };
use tauri::Emitter;
use crate::models::GLOBAL_APP_HANDLE;

// Set the global app handle
pub fn set_global_app_handle(handle: tauri::AppHandle) {
    if let Ok(mut app_handle) = GLOBAL_APP_HANDLE.lock() {
        *app_handle = Some(handle);
        info!("Global app handle has been set successfully");
    } else {
        error!("Failed to set global app handle");
    }
}

// Get the global app handle

pub fn get_global_app_handle() -> Option<tauri::AppHandle> {
    if let Ok(app_handle) = GLOBAL_APP_HANDLE.lock() {
        app_handle.clone()
    } else {
        error!("Failed to lock global app handle");
        None
    }
}

// Emit an event using the global app handle
pub fn emit_event<T: serde::Serialize + Clone>(event: &str, payload: T) -> Result<(), String> {
    if let Ok(app_handle) = GLOBAL_APP_HANDLE.lock() {
        if let Some(app) = app_handle.as_ref() {
            match app.emit(event, payload.clone()) {
                Ok(_) => {
                    info!("Event '{}' emitted successfully", event);
                    Ok(())
                }
                Err(e) => {
                    let err_msg = format!("Failed to emit event '{}': {}", event, e);
                    error!("{}", err_msg);
                    Err(err_msg)
                }
            }
        } else {
            let err_msg = "App handle not available, couldn't emit event";
            error!("{}", err_msg);
            Err(err_msg.to_string())
        }
    } else {
        let err_msg = "Failed to lock global app handle";
        error!("{}", err_msg);
        Err(err_msg.to_string())
    }
}
