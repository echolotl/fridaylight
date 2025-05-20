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
 * Represents a contributor to a mod
 */
export interface Contributor {
  name: string;
  icon?: string;   // Path to contributor icon relative to .flight folder
  role?: string;   // Specific role description to be displayed under the name
}

/**
 * Represents a contributor group with members
 */
export interface ContributorGroup {
  group: string;
  members: Contributor[];
}

/**
 * Represents a mod folder for organization
 */
export interface Folder {
  id: string; // Unique ID for the folder
  name: string; // Name of the folder
  color: string; // Color for the folder icon
  mods: string[]; // Array of mod IDs contained in this folder
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
  logo_position?: string; // Position of logo in banner: left_bottom, left_middle, middle
  version?: string;
  description?: string;
  engine: Engine; // Extended engine information
  display_order: number; // Position in the overall list or within a folder
  folder_id?: string | null; // ID of the folder this mod belongs to, if any
  display_order_in_folder?: number; // Position within the folder
  contributors?: ContributorGroup[]; // List of contributors to the mod, organized by groups
  last_played?: number; // Unix timestamp when mod was last played
  date_added?: number; // Unix timestamp when mod was added
}

/**
 * Represents a display item in lists (can be either a mod or folder)
 */
export interface DisplayItem {
  id: string;
  type: "mod" | "folder";
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

/**
 * GameBanana API response for mod details
 * This interface is used to represent the data structure of a mod fetched from GameBanana API.
 */

export interface GameBananaMod {
  id: number;
  name: string;
  owner: string;
  description: string;
  thumbnailUrl: string;
  downloadUrl: string;
  views: number;
  downloads: number;
  likes: number;
  modelName: string;
  profileUrl: string;
  imageUrl: string;
  initialVisibility: string;
  period: string;

  // Submitter details
  submitterId: number;
  submitterName: string;
  submitterIsOnline: boolean;
  submitterHasRipe: boolean;
  submitterProfileUrl: string;
  submitterAvatarUrl: string;
  submitterMoreByUrl: string;
  submitterUPic?: string;

  // Post count
  postCount: number;

  // Category details
  categoryName: string;
  categoryProfileUrl: string;
  categoryIconUrl: string;

  // Additional fields from normal mod data
  singularTitle: string;
  iconClasses: string;
  dateAdded: number;
  dateModified: number;
  dateUpdated: number;
  hasFiles: boolean;
  tags: string[];
  previewImages: GameBananaModImage[]; 
  version: string;
  isObsolete: boolean;
  hasContentRatings: boolean;
  viewCount: number;
  isOwnedByAccessor: boolean;
  wasFeatured: boolean;
}

/**
 * Represents an image associated with a GameBanana mod
 * This interface is used to represent the data structure of images fetched from GameBanana API.
 */

export interface GameBananaModImage {
  imageType: string;
  baseUrl: string;
  fileName: string;
  file100: string;
  file220?: string;
  file530?: string;
  file800?: string;
  height100?: number;
  width100?: number;
  height220?: number;
  width220?: number;
  height530?: number;
  width530?: number;
  height800?: number;
  width800?: number;
}

/**
 * Represents the settings for the application
 */

export interface AppSettings {
  accentColor: string | ColorOption;
  installLocation: string;
  theme: string;  // Changed from enableLightTheme to theme
  useSystemTheme: boolean;
  customCSS: string;
  validateFnfMods: boolean; // Whether to validate FNF mods structure before adding
  showTerminalOutput: boolean; // Whether to show terminal output from running mods
  compactMode: boolean; // Whether to display the sidebar in compact mode with only icons
}

interface ColorOption {
  label: string;
  value: string;
}
