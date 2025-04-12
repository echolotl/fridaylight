/**
 * Core types for Friday Night Funkin' Mod Launcher
 */

/**
 * Represents an FNF game engine
 */
export interface Engine {
  engine_type: string;
  engine_name: string;
  engine_icon: string;
  mods_folder: boolean;
  mods_folder_path: string;
}

/**
 * Represents a mod folder for organization
 */
export interface Folder {
  id: string;           // Unique ID for the folder
  name: string;         // Name of the folder
  color: string;        // Color for the folder icon
  mods: string[];       // Array of mod IDs contained in this folder
  display_order: number; // Position in the overall display order
}

/**
 * Represents a Friday Night Funkin' mod
 */
export interface Mod {
  id: string;
  name: string;
  path: string;
  executable_path?: string;
  icon_data?: string;
  banner_data?: string;
  logo_data?: string | null;
  version?: string;
  description?: string;
  engine_type?: string;  // Kept for backward compatibility 
  engine: Engine;        // Extended engine information
  display_order: number; // Position in the overall list or within a folder
  folder_id?: string | null;    // ID of the folder this mod belongs to, if any
  display_order_in_folder?: number; // Position within the folder
}

/**
 * Represents a display item in lists (can be either a mod or folder)
 */
export interface DisplayItem {
  id: string;
  type: 'mod' | 'folder';
  data: Mod | Folder;
  display_order: number;
}

/**
 * Represents the state of a mod being downloaded
 */
export interface DownloadingMod {
  modId: string;
  name: string;
  step: string;
  percentage: number;
  isComplete: boolean;
  isError: boolean;
  error?: string;
}

/**
 * Database settings
 */
export interface DbSettings {
  key: string;
  value: string;
}
