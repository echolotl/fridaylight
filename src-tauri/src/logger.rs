use log::{debug, error, info, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::fs::{self, File};
use std::sync::Once;
use tauri::Manager;

static INIT: Once = Once::new();

pub fn init(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let mut result = Ok(());
    
    INIT.call_once(|| {
        // Get app data directory for log files
        let app_dir = match app_handle.path().app_data_dir() {
            Ok(dir) => dir,
            Err(e) => {
                result = Err(format!("Failed to get app data directory: {}", e));
                return;
            }
        };
        
        // Create logs directory if it doesn't exist
        let logs_dir = app_dir.join("logs");
        if let Err(e) = fs::create_dir_all(&logs_dir) {
            result = Err(format!("Failed to create logs directory: {}", e));
            return;
        }
        
        // Create log file with timestamp
        let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
        let log_file_path = logs_dir.join(format!("fnfml_{}.log", timestamp));
        
        let log_file = match File::create(&log_file_path) {
            Ok(file) => file,
            Err(e) => {
                result = Err(format!("Failed to create log file: {}", e));
                return;
            }
        };
        
        // Initialize logger with both terminal and file outputs
        if let Err(e) = CombinedLogger::init(vec![
            // Terminal logger for development
            TermLogger::new(
                LevelFilter::Debug,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            // File logger for persistent logs
            WriteLogger::new(
                LevelFilter::Info,
                Config::default(),
                log_file,
            ),
        ]) {
            result = Err(format!("Failed to initialize logger: {}", e));
            return;
        }
        
        info!("Logger initialized. Log file: {}", log_file_path.display());
        debug!("Application started");
    });
    
    result
}