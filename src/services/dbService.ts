import Database from '@tauri-apps/plugin-sql';
import { invoke } from '@tauri-apps/api/core';
import { v4 as uuidv4 } from 'uuid';
import { Mod, Folder, DisplayItem, AppSettings } from '../types';
import { StoreService } from './storeService';

// Constants for database lock protection
const MAX_RETRY_ATTEMPTS = 3;     // Maximum number of retry attempts
const RETRY_DELAY_MS = 500;       // Delay between retry attempts in milliseconds

/**
 * Utility function to handle database operations with retry logic for lock protection
 * @param operation Function that performs the database operation
 * @returns Promise resolving to the operation result
 */
async function withDatabaseLock<T>(operation: () => Promise<T>): Promise<T> {
  let attempts = 0;
  
  while (attempts < MAX_RETRY_ATTEMPTS) {
    try {
      return await operation();
    } catch (error) {
      attempts++;
      
      // Check if error is SQLITE_BUSY or a database lock error
      const isLockError = error instanceof Error && 
        (error.message.includes('database is locked') || 
         error.message.includes('SQLITE_BUSY'));
      
      // If it's not a lock error or we've reached max attempts, throw the error
      if (!isLockError || attempts >= MAX_RETRY_ATTEMPTS) {
        throw error;
      }
      
      // Wait before retrying
      console.warn(`Database locked, retrying operation (attempt ${attempts}/${MAX_RETRY_ATTEMPTS})...`);
      await new Promise(resolve => setTimeout(resolve, RETRY_DELAY_MS));
    }
  }
  
  // This should never be reached due to the throw in the loop, but TypeScript requires a return
  throw new Error('Failed to acquire database lock after maximum retry attempts');
}

/**
 * DatabaseService class handles all database operations
 */
export class DatabaseService {
  private static instance: DatabaseService;
  private db: any = null;
  private initialized = false;
  private storeService: StoreService;

  private constructor() {
    this.storeService = StoreService.getInstance();
  }

  /**
   * Get the singleton instance of DatabaseService
   */
  public static getInstance(): DatabaseService {
    if (!DatabaseService.instance) {
      DatabaseService.instance = new DatabaseService();
    }
    return DatabaseService.instance;
  }
  /**
   * Initialize the database
   */
  public async initialize(): Promise<void> {
    if (this.initialized) {
        // Reinitialize the database
        await this.db.close();
        this.db = null;
        this.initialized = false;
    };

    try {
      // Initialize the store service
      await this.storeService.initialize();
      
      this.db = await Database.load('sqlite:mods.db');
      
      // Create or update tables with lock protection
      await withDatabaseLock(() => this.createModsTable());
      // Check and migrate settings if needed
      await withDatabaseLock(() => this.migrateSettingsToStore());
      await withDatabaseLock(() => this.createFoldersTable());
      await withDatabaseLock(() => this.createModFoldersTable());

      // Set initialized flag
      this.initialized = true;
      console.log('Database initialized successfully');
    } catch (error) {
      console.error('Failed to initialize database:', error);
      throw error;
    }
  }

  /**
   * Migrate settings from database to store
   */
  private async migrateSettingsToStore(): Promise<void> {
    try {
      // Check if the settings table exists
      const tableInfo = await this.db.select(`SELECT name FROM sqlite_master WHERE type='table' AND name='settings'`);
      
      if (tableInfo.length > 0) {
        console.log('Found settings table, migrating settings to store...');
        
        // Get all settings from the database
        const dbSettings = await this.db.select('SELECT key, value FROM settings');
        
        if (dbSettings.length > 0) {
          const settings: Partial<AppSettings> = {};
          
          // Map the old settings to the new AppSettings format
          for (const setting of dbSettings) {
            let settingKey = setting.key;
            let settingValue = setting.value;

            // Convert string values to appropriate types
            if (settingValue === 'true') settingValue = true;
            else if (settingValue === 'false') settingValue = false;
            
            // Handle the theme migration from enableLightTheme to theme
            if (settingKey === 'enableLightTheme') {
              settingKey = 'theme';
              settingValue = settingValue === true || settingValue === 'true' ? 'light' : 'dark';
            }
            
            // Only add if the key exists in AppSettings
            if (settingKey in this.storeService.DEFAULT_SETTINGS) {
              settings[settingKey as keyof AppSettings] = settingValue;
            }
          }
          
          // Save migrated settings to the store
          await this.storeService.saveSettings(settings);
          console.log('Settings successfully migrated to store');
          
          // Optionally, remove the settings table since we don't need it anymore
          // Commented out for safety - uncomment after thorough testing
          // await this.db.execute('DROP TABLE settings');
          // console.log('Settings table removed');
        }
      } else {
        console.log('No settings table found, no migration needed');
      }
    } catch (error) {
      console.error('Failed to migrate settings to store:', error);
      // Don't throw error to continue initialization
    }
  }

  /**
   * Create or update the mods table
   */
  private async createModsTable(): Promise<void> {
    try {
      // Check if the mods table exists
      const tableInfo: any[] = await this.db.select(`PRAGMA table_info(mods)`);
      
      if (tableInfo.length > 0) {
        // Table exists, check for missing columns
        const columns = tableInfo.map((col: any) => col.name);
        
        // Add missing columns if they don't exist
        if (!columns.includes('banner_data')) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN banner_data TEXT`);
          console.log('Added banner_data column to mods table');
        }
        
        if (!columns.includes('logo_data')) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN logo_data TEXT`);
          console.log('Added logo_data column to mods table');
        }
        
        if (!columns.includes('version')) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN version TEXT`);
          console.log('Added version column to mods table');
        }
        
        if (!columns.includes('engine_type')) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN engine_type TEXT`);
          console.log('Added engine_type column to mods table');
        }
        
        if (!columns.includes('display_order')) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN display_order INTEGER DEFAULT 9999`);
          console.log('Added display_order column to mods table');
        }
        
        if (!columns.includes('description')) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN description TEXT`);
          console.log('Added description column to mods table');
        }
          if (!columns.includes('engine_data')) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN engine_data TEXT`);
          console.log('Added engine_data column to mods table');
        }
        
        if (!columns.includes('display_order_in_folder')) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN display_order_in_folder INTEGER DEFAULT 0`);
          console.log('Added display_order_in_folder column to mods table');
        }

        if (!columns.includes('folder_id')) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN folder_id TEXT`);
          console.log('Added folder_id column to mods table');
        }
      } else {
        // Table doesn't exist, create it with all columns
        await this.db.execute(`
          CREATE TABLE IF NOT EXISTS mods (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            executable_path TEXT,
            icon_data TEXT,
            banner_data TEXT,
            logo_data TEXT,
            version TEXT,
            description TEXT,
            engine_type TEXT,
            engine_data TEXT,
            display_order INTEGER DEFAULT 9999,
            folder_id TEXT
          )
        `);
        console.log('Created mods table with all columns');
      }
    } catch (error) {
      console.error('Failed to create/update mods table:', error);
      throw error;
    }
  }

  /**
   * Create or update the folders table
   */
  private async createFoldersTable(): Promise<void> {
    try {
      const foldersTableInfo: any[] = await this.db.select(`PRAGMA table_info(folders)`);
      
      if (foldersTableInfo.length === 0) {
        // Table doesn't exist, create it
        await this.db.execute(`
          CREATE TABLE IF NOT EXISTS folders (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            color TEXT NOT NULL,
            display_order INTEGER DEFAULT 9999
          )
        `);
        console.log('Created folders table');
      } else {
        // Check if display_order column exists
        const columns = foldersTableInfo.map((col: any) => col.name);
        if (!columns.includes('display_order')) {
          await this.db.execute(`ALTER TABLE folders ADD COLUMN display_order INTEGER DEFAULT 9999`);
          console.log('Added display_order column to folders table');
        }
      }
    } catch (error) {
      console.error('Failed to initialize folders table:', error);
      throw error;
    }
  }

  /**
   * Create or update the mod_folders table
   * Note: We're moving away from this table in favor of using folder_id in the mods table
   * but keeping it for backward compatibility
   */
  private async createModFoldersTable(): Promise<void> {
    try {
      const modFoldersTableInfo: any[] = await this.db.select(`PRAGMA table_info(mod_folders)`);
      
      if (modFoldersTableInfo.length === 0) {
        // Table doesn't exist, create it
        await this.db.execute(`
          CREATE TABLE IF NOT EXISTS mod_folders (
            mod_id TEXT NOT NULL,
            folder_id TEXT NOT NULL,
            PRIMARY KEY (mod_id, folder_id),
            FOREIGN KEY (mod_id) REFERENCES mods (id) ON DELETE CASCADE,
            FOREIGN KEY (folder_id) REFERENCES folders (id) ON DELETE CASCADE
          )
        `);
        console.log('Created mod_folders mapping table');
      }
    } catch (error) {
      console.error('Failed to initialize mod_folders table:', error);
      throw error;
    }
  }

  /**
   * Get all mods from the database
   */
  public async getAllMods(): Promise<Mod[]> {
    if (!this.db) {
      throw new Error('Database not initialized');
    }

    return withDatabaseLock(async () => {
      const mods = await this.db.select('SELECT * FROM mods ORDER BY display_order ASC');
      return mods.map((mod: any) => {
        // Parse engine data if it exists
        let engine;
        try {
          engine = mod.engine_data ? JSON.parse(mod.engine_data) : {
            engine_type: mod.engine_type || 'unknown',
            engine_name: mod.engine_type || 'Unknown Engine',
            engine_icon: '',
            mods_folder: false,
            mods_folder_path: ''
          };
        } catch (e) {
          console.error('Failed to parse engine data for mod:', mod.id, e);
          engine = {
            engine_type: mod.engine_type || 'unknown',
            engine_name: mod.engine_type || 'Unknown Engine',
            engine_icon: '',
            mods_folder: false,
            mods_folder_path: ''
          };
        }

        return {
          ...mod,
          engine
        };
      });
    }).catch(error => {
      console.error('Failed to get mods:', error);
      throw error;
    });
  }

  /**
   * Get all folders from the database
   */
  public async getAllFolders(): Promise<Folder[]> {
    if (!this.db) {
      throw new Error('Database not initialized');
    }

    return withDatabaseLock(async () => {
      const folders = await this.db.select(`
        SELECT f.id, f.name, f.color, f.display_order, GROUP_CONCAT(m.id) as mod_ids
        FROM folders f
        LEFT JOIN mods m ON m.folder_id = f.id
        GROUP BY f.id
        ORDER BY f.display_order ASC
      `);

      return folders.map((folder: any) => ({
        id: folder.id,
        name: folder.name,
        color: folder.color,
        display_order: folder.display_order || 9999,
        mods: folder.mod_ids ? folder.mod_ids.split(',') : []
      }));
    }).catch(error => {
      console.error('Failed to get folders:', error);
      throw error;
    });
  }

  // Helper to upsert mods in a single transaction with rollback
  private async _upsertMods(mods: Mod[]): Promise<void> {
    try {
      await this.db.execute('BEGIN TRANSACTION');
      for (const mod of mods) {
        const engineData = JSON.stringify(mod.engine);
        await this.db.execute(`
          INSERT OR REPLACE INTO mods (
            id, name, path, executable_path, icon_data, banner_data, logo_data, 
            version, description, engine_type, engine_data, display_order, folder_id,
            display_order_in_folder
          ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        `, [
          mod.id,
          mod.name,
          mod.path,
          mod.executable_path || null,
          mod.icon_data || null,
          mod.banner_data || null,
          mod.logo_data || null,
          mod.version || null,
          mod.description || null,
          mod.engine_type || mod.engine.engine_type,
          engineData,
          mod.display_order || 9999,
          mod.folder_id || null,
          mod.display_order_in_folder !== undefined ? mod.display_order_in_folder : 0
        ]);
      }
      await this.db.execute('COMMIT');
    } catch (error) {
      await this.db.execute('ROLLBACK');
      throw error;
    }
  }

  // Helper to upsert mods and then sync with backend
  private async _saveAndSyncMods(mods: Mod[]): Promise<void> {
    await this._upsertMods(mods);
    await this.syncModsWithBackend();
  }

  // Helper to upsert folders in a single transaction with rollback
  private async _upsertFolders(folders: Folder[]): Promise<void> {
    try {
      await this.db.execute('BEGIN TRANSACTION');
      for (const folder of folders) {
        await this.db.execute(`
          INSERT OR REPLACE INTO folders (id, name, color, display_order) 
          VALUES (?, ?, ?, ?)
        `, [folder.id, folder.name, folder.color, folder.display_order || 9999]);
        // Update the folder_id for all mods in this folder
        if (folder.mods && Array.isArray(folder.mods)) {
          for (const modId of folder.mods) {
            await this.db.execute(`
              UPDATE mods SET folder_id = ? WHERE id = ?
            `, [folder.id, modId]);
          }
        }
      }
      await this.db.execute('COMMIT');
    } catch (error) {
      await this.db.execute('ROLLBACK');
      throw error;
    }
  }

  // Helper to upsert folders and then sync with backend
  private async _saveAndSyncFolders(folders: Folder[]): Promise<void> {
    await this._upsertFolders(folders);
    await this.syncModsWithBackend();
  }

  /**
   * Save a mod to the database
   */
  public async saveMod(mod: Mod): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized');
    }

    return withDatabaseLock(() => this._saveAndSyncMods([mod]))
      .catch(error => {
        console.error('Failed to save mod:', error);
        throw error;
      });
  }

  /**
   * Save multiple mods to the database
   */
  public async saveMods(mods: Mod[]): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized');
    }

    return withDatabaseLock(() => this._saveAndSyncMods(mods))
      .catch(error => {
        console.error('Failed to save mods:', error);
        throw error;
      });
  }

  /**
   * Delete a mod from the database
   */
  public async deleteMod(modId: string): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized');
    }

    return withDatabaseLock(async () => {
      await this.db.execute('DELETE FROM mods WHERE id = ?', [modId]);
      // Sync with backend
      await this.syncModsWithBackend();
    }).catch(error => {
      console.error('Failed to delete mod:', error);
      throw error;
    });
  }

  /**
   * Save a folder to the database
   */
  public async saveFolder(folder: Folder): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized');
    }
    return withDatabaseLock(() => this._saveAndSyncFolders([folder]))
      .catch(error => {
        console.error('Failed to save folder:', error);
        throw error;
      });
  }

  /**
   * Save multiple folders to the database
   */
  public async saveFolders(folders: Folder[]): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized');
    }
    return withDatabaseLock(() => this._saveAndSyncFolders(folders))
      .catch(error => {
        console.error('Failed to save folders:', error);
        throw error;
      });
  }

  /**
   * Delete a folder from the database
   */
  public async deleteFolder(folderId: string): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized');
    }

    return withDatabaseLock(async () => {
      try {
        // Get mods in this folder
        await this.db.select('SELECT id FROM mods WHERE folder_id = ?', [folderId]);

        // Remove folder_id from all mods in this folder
        await this.db.execute(`
          UPDATE mods SET folder_id = NULL WHERE folder_id = ?
        `, [folderId]);
        
        // Delete the folder
        await this.db.execute('DELETE FROM folders WHERE id = ?', [folderId]);
        // Sync with backend after the operation
        await this.syncModsWithBackend();
      } catch (error) {
        throw error;
      }
    }).catch(error => {
      console.error('Failed to delete folder:', error);
      throw error;
    });
  }

  /**
   * Move a mod to a folder
   */
  public async moveModToFolder(modId: string, folderId: string | null): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized');
    }

    return withDatabaseLock(async () => {
      try {
        // Start a transaction
        await this.db.execute('BEGIN TRANSACTION');
        
        // Update the mod's folder_id
        await this.db.execute(`
          UPDATE mods SET folder_id = ? WHERE id = ?
        `, [folderId, modId]);
        
        // If we're adding to a folder (not removing), update the folder's mods list
        if (folderId) {
          // Get the current folder data to preserve its display_order
          const folderResult = await this.db.select(`
            SELECT * FROM folders WHERE id = ?
          `, [folderId]);
          
          if (folderResult.length > 0) {
            const folder = folderResult[0];
            const folderMods = folder.mods ? JSON.parse(folder.mods) : [];
            
            // Add the mod to the folder's mods list if not already there
            if (!folderMods.includes(modId)) {
              folderMods.push(modId);
              
              // Update the folder with the new mods list but preserve display_order
              await this.db.execute(`
                UPDATE folders 
                SET mods = ? 
                WHERE id = ?
              `, [JSON.stringify(folderMods), folderId]);
            }
          }
        }
        
        // Commit the transaction
        await this.db.execute('COMMIT');

        // Sync with backend
        await this.syncModsWithBackend();
      } catch (error) {
        // Rollback on error
        if (this.db) {
          await this.db.execute('ROLLBACK');
        }
        throw error;
      }
    }).catch(error => {
      console.error('Failed to move mod to folder:', error);
      throw error;
    });
  }

  /**
   * Update display order for mods and folders
   */
  public async updateDisplayOrder(items: DisplayItem[]): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized');
    }

    return withDatabaseLock(async () => {
      try {
        // Start a transaction
        await this.db.execute('BEGIN TRANSACTION');

        for (let i = 0; i < items.length; i++) {
          const item = items[i];
          const display_order = i;

          if (item.type === 'folder') {
            await this.db.execute(`
              UPDATE folders SET display_order = ? WHERE id = ?
            `, [display_order, item.id]);
          } else if (item.type === 'mod') {
            await this.db.execute(`
              UPDATE mods SET display_order = ? WHERE id = ?
            `, [display_order, item.id]);
          }
        }

        // Commit the transaction
        await this.db.execute('COMMIT');

        // Sync with backend
        await this.syncModsWithBackend();
      } catch (error) {
        // Rollback on error
        await this.db.execute('ROLLBACK');
        throw error;
      }
    }).catch(error => {
      console.error('Failed to update display order:', error);
      throw error;
    });
  }

  /**
   * Get a setting from the store
   * @deprecated Use StoreService.getSetting() instead
   */
  public async getSetting(key: string): Promise<string | null> {
    console.warn('DatabaseService.getSetting is deprecated, use StoreService directly instead');
    const value = await this.storeService.getSetting(key as keyof AppSettings);
    return value !== null ? String(value) : null;
  }

  /**
   * Save a setting to the store
   * @deprecated Use StoreService.saveSetting() instead
   */
  public async saveSetting(key: string, value: string): Promise<void> {
    console.warn('DatabaseService.saveSetting is deprecated, use StoreService directly instead');
    await this.storeService.saveSetting(key as keyof AppSettings, value as any);
  }

  /**
   * Sync mods with the Rust backend
   */
  private async syncModsWithBackend(): Promise<void> {
    try {
      const mods = await this.getAllMods();
      await invoke('sync_mods_from_database', { modsData: mods });
      console.log('Successfully synced mods with backend');
    } catch (error) {
      console.error('Failed to sync mods with backend:', error);
      throw error;
    }
  }

  /**
   * Helper method to create an empty mod
   */
  public createEmptyMod(): Mod {
    return {
      id: uuidv4(),
      name: '',
      path: '',
      display_order: 9999,
      engine: {
        engine_type: 'unknown',
        engine_name: 'Unknown Engine',
        engine_icon: '',
        mods_folder: false,
        mods_folder_path: ''
      }
    };
  }

  /**
   * Helper method to create an empty folder
   */
  public createEmptyFolder(): Folder {
    return {
      id: uuidv4(),
      name: 'New Folder',
      color: '#FF0088',
      mods: [],
      display_order: 9999
    };
  }
}
