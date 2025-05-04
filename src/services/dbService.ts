import Database from "@tauri-apps/plugin-sql";
import { invoke } from "@tauri-apps/api/core";
import { v4 as uuidv4 } from "uuid";
import { Mod, Folder, DisplayItem, AppSettings } from "../types";
import { StoreService } from "./storeService";
import { formatEngineName } from "../utils"; 

// Constants for database lock protection
const MAX_RETRY_ATTEMPTS = 10; // Increased maximum retry attempts
const INITIAL_RETRY_DELAY_MS = 50; // Start with a shorter delay
const MAX_RETRY_DELAY_MS = 1000; // Cap the maximum delay

// Mutex for database operations
class Mutex {
  private mutex = Promise.resolve();
  private owner: string | null = null;

  async lock(operation: string = 'unknown'): Promise<() => void> {
    console.log(`🔒 Mutex: Requesting lock for operation: ${operation}`);
    
    let resolve: () => void = () => {};
    const unlock = new Promise<void>((res) => {
      resolve = () => {
        console.log(`🔓 Mutex: Released lock for operation: ${operation}`);
        this.owner = null;
        res();
      };
    });

    const previous = this.mutex;
    this.mutex = previous.then(() => unlock);

    try {
      console.log(`⏳ Mutex: Waiting for previous operations to complete for: ${operation}`);
      await previous;
      this.owner = operation;
      console.log(`✅ Mutex: Lock acquired for operation: ${operation}`);
      return resolve;
    } catch (error) {
      console.error(`❌ Mutex: Error waiting for lock for operation: ${operation}`, error);
      // Make sure we still return a valid unlock function even if there was an error
      return resolve;
    }
  }

  getCurrentOwner(): string | null {
    return this.owner;
  }
}

// Global mutex for database operations
const dbMutex = new Mutex();

// Transaction state tracking to prevent nested transactions
let transactionActive = false;

/**
 * Utility function to handle database operations with retry logic for lock protection
 * @param operation Function that performs the database operation
 * @param useTransaction Whether to wrap the operation in a transaction
 * @param operationName Name of the operation for tracking purposes
 * @returns Promise resolving to the operation result
 */
async function withDatabaseLock<T>(
  operation: (db?: any) => Promise<T>,
  useTransaction: boolean = false,
  operationName: string = "unknown"
): Promise<T> {
  let attempts = 0;
  console.log(`withDatabaseLock - Starting '${operationName}' with useTransaction=${useTransaction}`);

  // Acquire global mutex lock to prevent concurrent operations
  const unlock = await dbMutex.lock(operationName);

  try {
    while (attempts < MAX_RETRY_ATTEMPTS) {
      try {
        // Get database instance from DatabaseService
        const db = DatabaseService.getInstance().getDb();

        if (useTransaction && !transactionActive) {
          // Set flag before starting transaction
          console.log(`| withDatabaseLock - Starting new transaction for '${operationName}'`);
          transactionActive = true;
          let transactionStarted = false;

          try {
            // Check if we're already in a transaction
            const inTransactionCheck = await db.select(
              "PRAGMA transaction_status"
            );
            const inTransaction =
              inTransactionCheck &&
              inTransactionCheck.length > 0 &&
              inTransactionCheck[0].transaction_status !== "idle" &&
              inTransactionCheck[0].transaction_status !== undefined;

            if (!inTransaction) {
              console.log(`| withDatabaseLock - No existing transaction, starting new one for '${operationName}'`);
              await db.execute("BEGIN EXCLUSIVE TRANSACTION"); // Use EXCLUSIVE for better locking
              transactionStarted = true;
              console.log(`| Started new transaction for '${operationName}'`);
            } else {
              console.log(
                `| Transaction already active, using existing transaction for '${operationName}'`
              );
            }

            const result = await operation(db);

            // Only commit if we started the transaction
            if (transactionStarted) {
              await db.execute("COMMIT");
              console.log(`| Transaction committed successfully for '${operationName}'`);
            }

            transactionActive = false;
            return result;
          } catch (error) {
            console.error(`| withDatabaseLock - Error in transaction for '${operationName}':`, error);
            // Only try to rollback if we started a transaction
            if (transactionStarted) {
              try {
                console.log(`| Error in transaction, attempting rollback for '${operationName}'`);
                await db.execute("ROLLBACK");
              } catch (rollbackError) {
                console.error(`| Rollback failed for '${operationName}':`, rollbackError);
                // Continue with the original error
              }
            }

            transactionActive = false;
            throw error;
          }
        } else if (useTransaction && transactionActive) {
          // If a transaction is already active, just run the operation without starting a new one
          console.log(`| Using existing transaction for operation '${operationName}'`);
          const result = await operation(db);
          return result;
        } else {
          // No transaction needed
          const result = await operation(db);
          console.log(`| withDatabaseLock - Operation completed successfully (no transaction) for '${operationName}'`);
          return result;
        }
      } catch (error) {
        attempts++;
        console.error(`| withDatabaseLock - Attempt ${attempts} failed for '${operationName}':`, error);

        // Check if error is SQLITE_BUSY or a database lock error
        const isLockError =
          error instanceof Error &&
          (error.message.includes("database is locked") ||
            error.message.includes("SQLITE_BUSY"));

        // If it's not a lock error or we've reached max attempts, throw the error
        if (!isLockError || attempts >= MAX_RETRY_ATTEMPTS) {
          console.error(
            `| Database operation '${operationName}' failed after ${attempts} attempts:`,
            error
          );
          throw error;
        }

        // Wait before retrying with exponential backoff capped at max delay
        const delay = Math.min(
          INITIAL_RETRY_DELAY_MS * Math.pow(2, attempts - 1),
          MAX_RETRY_DELAY_MS
        );
        console.warn(
          `| Database locked, retrying operation '${operationName}' (attempt ${attempts}/${MAX_RETRY_ATTEMPTS}) after ${delay}ms...`
        );
        await new Promise((resolve) => setTimeout(resolve, delay));
      }
    }

    // This should never be reached due to the throw in the loop, but TypeScript requires a return
    console.error(`| withDatabaseLock - Reached maximum retry attempts without success or error for '${operationName}'`);
    throw new Error(
      `| Failed to acquire database lock after maximum retry attempts for operation '${operationName}'`
    );
  } finally {
    // Always release the mutex lock
    console.log(`| withDatabaseLock - Releasing mutex lock for '${operationName}'`);
    unlock();
  }
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
   * Get the database instance
   */
  public getDb(): any {
    if (!this.db) {
      throw new Error("Database not initialized");
    }
    return this.db;
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
    }

    try {
      // Initialize the store service
      await this.storeService.initialize();

      this.db = await Database.load("sqlite:mods.db");

      // Create or update tables with lock protection
      await withDatabaseLock(() => this.createModsTable(), false, "createModsTable");
      await withDatabaseLock(() => this.createFoldersTable(), false, "createFoldersTable");
      await withDatabaseLock(() => this.createModFoldersTable(), false, "createModFoldersTable");

      // Set initialized flag
      this.initialized = true;
      console.log("Database initialized successfully");
    } catch (error) {
      console.error("Failed to initialize database:", error);
      throw error;
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
        if (!columns.includes("banner_data")) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN banner_data TEXT`);
          console.log("Added banner_data column to mods table");
        }

        if (!columns.includes("logo_data")) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN logo_data TEXT`);
          console.log("Added logo_data column to mods table");
        }

        if (!columns.includes("logo_position")) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN logo_position TEXT`);
          console.log("Added logo_position column to mods table");
        }

        if (!columns.includes("version")) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN version TEXT`);
          console.log("Added version column to mods table");
        }

        if (!columns.includes("engine_type")) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN engine_type TEXT`);
          console.log("Added engine_type column to mods table");
        }

        if (!columns.includes("display_order")) {
          await this.db.execute(
            `ALTER TABLE mods ADD COLUMN display_order INTEGER DEFAULT 9999`
          );
          console.log("Added display_order column to mods table");
        }

        if (!columns.includes("description")) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN description TEXT`);
          console.log("Added description column to mods table");
        }
        if (!columns.includes("engine_data")) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN engine_data TEXT`);
          console.log("Added engine_data column to mods table");
        }

        if (!columns.includes("display_order_in_folder")) {
          await this.db.execute(
            `ALTER TABLE mods ADD COLUMN display_order_in_folder INTEGER DEFAULT 0`
          );
          console.log("Added display_order_in_folder column to mods table");
        }

        if (!columns.includes("folder_id")) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN folder_id TEXT`);
          console.log("Added folder_id column to mods table");
        }

        if (!columns.includes("contributors_data")) {
          await this.db.execute(`ALTER TABLE mods ADD COLUMN contributors_data TEXT`);
          console.log("Added contributors_data column to mods table");
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
            logo_position TEXT,
            version TEXT,
            description TEXT,
            engine_type TEXT,
            engine_data TEXT,
            display_order INTEGER DEFAULT 0,
            folder_id TEXT,
            display_order_in_folder INTEGER DEFAULT 0,
            contributors_data TEXT
          )
        `);
        console.log("Created mods table with all columns");
      }
    } catch (error) {
      console.error("Failed to create/update mods table:", error);
      throw error;
    }
  }

  /**
   * Create or update the folders table
   */
  private async createFoldersTable(): Promise<void> {
    try {
      const foldersTableInfo: any[] = await this.db.select(
        `PRAGMA table_info(folders)`
      );

      if (foldersTableInfo.length === 0) {
        // Table doesn't exist, create it
        await this.db.execute(`
          CREATE TABLE IF NOT EXISTS folders (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            color TEXT NOT NULL,
            display_order INTEGER DEFAULT 0
          )
        `);
        console.log("Created folders table");
      } else {
        // Check if display_order column exists
        const columns = foldersTableInfo.map((col: any) => col.name);
        if (!columns.includes("display_order")) {
          await this.db.execute(
            `ALTER TABLE folders ADD COLUMN display_order INTEGER DEFAULT 9999`
          );
          console.log("Added display_order column to folders table");
        }
      }
    } catch (error) {
      console.error("Failed to initialize folders table:", error);
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
      const modFoldersTableInfo: any[] = await this.db.select(
        `PRAGMA table_info(mod_folders)`
      );

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
        console.log("Created mod_folders mapping table");
      }
    } catch (error) {
      console.error("Failed to initialize mod_folders table:", error);
      throw error;
    }
  }

  /**
   * Get all mods from the database
   */
  public async getAllMods(): Promise<Mod[]> {
    if (!this.db) {
      throw new Error("Database not initialized");
    }

    return withDatabaseLock(async (db) => {
      const mods = await db.select(
        "SELECT * FROM mods ORDER BY display_order ASC"
      );
      return mods.map((mod: any) => {
        // Parse engine data if it exists
        let engine;
        try {
          engine = mod.engine_data
            ? JSON.parse(mod.engine_data)
            : {
                engine_type: "unknown",
                engine_name: "Unknown Engine",
                engine_icon: "",
                mods_folder: false,
                mods_folder_path: "",
              };
        } catch (e) {
          console.error("Failed to parse engine data for mod:", mod.id, e);
          engine = {
            engine_type: "unknown",
            engine_name: "Unknown Engine",
            engine_icon: "",
            mods_folder: false,
            mods_folder_path: "",
          };
        }

        // Contributors data is no longer stored in database, component will load directly from metadata.json
        return {
          ...mod,
          engine,
        };
      });
    }, false, "getAllMods").catch((error) => {
      console.error("Failed to get mods:", error);
      throw error;
    });
  }

  /**
   * Get all folders from the database
   */
  public async getAllFolders(): Promise<Folder[]> {
    if (!this.db) {
      throw new Error("Database not initialized");
    }

    return withDatabaseLock(async (db) => {
      const folders = await db.select(`
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
        mods: folder.mod_ids ? folder.mod_ids.split(",") : [],
      }));
    }, false, "getAllFolders").catch((error) => {
      console.error("Failed to get folders:", error);
      throw error;
    });
  }

  // Helper to format engine name from engine type
  private formatEngineName(engineType: string): string {
    return formatEngineName(engineType); 
  }

  // Helper to upsert mods in a single transaction with rollback
  private async _upsertMods(mods: Mod[], db?: any): Promise<void> {
    console.log("_upsertMods - Starting to save mods to database...");
    
    // If db is provided, we're already in a transaction with a lock, so use it directly
    if (db) {
      console.log("_upsertMods - Using provided database connection (already in transaction)");
      
      for (const mod of mods) {
        console.log(`_upsertMods - Processing mod: ${mod.name} (${mod.id})`);
        
        try {
          // Create engine object if it doesn't exist
          if (!mod.engine) {
            console.log("_upsertMods - Creating missing engine object");
            mod.engine = {
              engine_type: "unknown",
              engine_name: this.formatEngineName("unknown"),
              engine_icon: "",
              mods_folder: false,
              mods_folder_path: "",
            };
          }

          // Generate fresh engine_data JSON
          const engineData = JSON.stringify(mod.engine);
          
          // Generate contributors_data JSON if contributors exist
          const contributorsData = mod.contributors ? JSON.stringify(mod.contributors) : null;
          
          console.log(`_upsertMods - Final mod engine data:
            engine.engine_type: ${mod.engine?.engine_type}
            engine_data: ${engineData}
            contributors: ${contributorsData ? 'present' : 'none'}
          `);
          
          console.log("_upsertMods - Executing SQL to save mod...");
          await db.execute(
            `
            INSERT OR REPLACE INTO mods (
              id, name, path, executable_path, icon_data, banner_data, logo_data, logo_position,
              version, description, engine_data, display_order, folder_id,
              display_order_in_folder, contributors_data
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
          `,
            [
              mod.id,
              mod.name,
              mod.path,
              mod.executable_path || null,
              mod.icon_data || null,
              mod.banner_data || null,
              mod.logo_data || null,
              mod.logo_position || null,
              mod.version || null,
              mod.description || null,
              engineData,
              mod.display_order || 9999,
              mod.folder_id || null,
              mod.display_order_in_folder !== undefined
                ? mod.display_order_in_folder
                : 0,
              contributorsData,
            ]
          );
          console.log(`_upsertMods - Successfully saved mod: ${mod.name}`);
        } catch (error) {
          console.error(`_upsertMods - Error processing mod ${mod.name}:`, error);
          throw error;
        }
      }
      console.log("_upsertMods - All mods saved successfully");
      return;
    }
    
    // No db provided, we need to acquire our own lock
    try {
      return withDatabaseLock(async (db) => {
        console.log("_upsertMods - Got database lock, processing mods...");
        
        for (const mod of mods) {
          // Reuse the same code as above, but with our own db connection
          // Create engine object if it doesn't exist
          if (!mod.engine) {
            mod.engine = {
              engine_type: "unknown",
              engine_name: this.formatEngineName("unknown"),
              engine_icon: "",
              mods_folder: false,
              mods_folder_path: "",
            };
          }

          // Generate fresh engine_data JSON
          const engineData = JSON.stringify(mod.engine);
          
          // Generate contributors_data JSON if contributors exist
          const contributorsData = mod.contributors ? JSON.stringify(mod.contributors) : null;
          
          await db.execute(
            `
            INSERT OR REPLACE INTO mods (
              id, name, path, executable_path, icon_data, banner_data, logo_data, 
              version, description, engine_data, display_order, folder_id,
              display_order_in_folder, contributors_data
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
          `,
            [
              mod.id,
              mod.name,
              mod.path,
              mod.executable_path || null,
              mod.icon_data || null,
              mod.banner_data || null,
              mod.logo_data || null,
              mod.version || null,
              mod.description || null,
              engineData,
              mod.display_order || 9999,
              mod.folder_id || null,
              mod.display_order_in_folder !== undefined
                ? mod.display_order_in_folder
                : 0,
              contributorsData,
            ]
          );
        }
      }, true, "_upsertMods");
    } catch (error) {
      console.error("_upsertMods - Error in transaction:", error);
      throw error;
    }
  }

  // Helper to upsert mods and then sync with backend
  private async _saveAndSyncMods(mods: Mod[], existingDb?: any): Promise<void> {
    console.log("_saveAndSyncMods - About to save mods with engine data:", mods.map(mod => ({
      id: mod.id,
      name: mod.name, 
      engine: mod.engine ? JSON.stringify(mod.engine) : undefined
    })));
    
    // If we were passed a database connection, we're already in a transaction with a lock
    if (existingDb) {
      console.log("_saveAndSyncMods - Using provided database connection (already in transaction)");
      
      // Pass the existing db connection to _upsertMods to avoid nested locks
      await this._upsertMods(mods, existingDb);
      
      console.log("_saveAndSyncMods - Mods saved, about to sync with backend");
  
      // Then sync with backend (in the same transaction)
      await this.syncModsWithBackend(existingDb);
      return;
    }
    
    // No existing db connection, we need to acquire our own lock
    // Use withDatabaseLock to handle the entire operation within a single lock
    await withDatabaseLock(async (db) => {
      // Pass the db connection to _upsertMods to avoid nested locks
      await this._upsertMods(mods, db);
      
      console.log("_saveAndSyncMods - Mods saved, about to sync with backend");
  
      // Then sync with backend (in the same transaction)
      await this.syncModsWithBackend(db);
    }, true, "_saveAndSyncMods");
  }

  // Helper to upsert folders and then sync with backend
  private async _saveAndSyncFolders(folders: Folder[], existingDb?: any): Promise<void> {
    console.log("_saveAndSyncFolders - About to save folders");
    
    // If we were passed a database connection, we're already in a transaction with a lock
    if (existingDb) {
      console.log("_saveAndSyncFolders - Using provided database connection (already in transaction)");
      
      // First save the folders with the existing connection
      for (const folder of folders) {
        await existingDb.execute(
          `INSERT OR REPLACE INTO folders (id, name, color, display_order) 
           VALUES (?, ?, ?, ?)`,
          [folder.id, folder.name, folder.color, folder.display_order || 9999]
        );
        
        // Update the folder_id for all mods in this folder
        if (folder.mods && Array.isArray(folder.mods)) {
          for (const modId of folder.mods) {
            await existingDb.execute(
              `UPDATE mods SET folder_id = ? WHERE id = ?`,
              [folder.id, modId]
            );
          }
        }
      }
      
      console.log("_saveAndSyncFolders - Folders saved, about to sync with backend");
      
      // Sync with backend using the same connection
      await this.syncModsWithBackend(existingDb);
      return;
    }
    
    // No existing connection, acquire our own lock for the entire operation
    await withDatabaseLock(async (db) => {
      // Save the folders
      for (const folder of folders) {
        await db.execute(
          `INSERT OR REPLACE INTO folders (id, name, color, display_order) 
           VALUES (?, ?, ?, ?)`,
          [folder.id, folder.name, folder.color, folder.display_order || 9999]
        );
        
        // Update the folder_id for all mods in this folder
        if (folder.mods && Array.isArray(folder.mods)) {
          for (const modId of folder.mods) {
            await db.execute(
              `UPDATE mods SET folder_id = ? WHERE id = ?`,
              [folder.id, modId]
            );
          }
        }
      }
      
      console.log("_saveAndSyncFolders - Folders saved, about to sync with backend");
      
      // Sync with backend in the same transaction
      await this.syncModsWithBackend(db);
    }, true, "_saveAndSyncFolders");
  }

  /**
   * Save a mod to the database
   */
  public async saveMod(mod: Mod): Promise<void> {
    if (!this.db) {
      throw new Error("Database not initialized");
    }

    // Log the mod's engine data before saving
    console.log("DATABASE SERVICE - Received mod with engine data:", {
      id: mod.id,
      name: mod.name,
      engine: mod.engine ? JSON.stringify(mod.engine) : undefined,
    });

    return withDatabaseLock(async (db) => {
      // Pass the db connection to _saveAndSyncMods to avoid nested locks
      await this._saveAndSyncMods([mod], db);
    }, true, "saveMod").catch(
      (error) => {
        console.error("Failed to save mod:", error);
        throw error;
      }
    );
  }

  /**
   * Save multiple mods to the database
   */
  public async saveMods(mods: Mod[]): Promise<void> {
    if (!this.db) {
      throw new Error("Database not initialized");
    }

    return withDatabaseLock(async (db) => {
      // Pass the db connection to _saveAndSyncMods to avoid nested locks
      await this._saveAndSyncMods(mods, db);
    }, true, "saveMods").catch(
      (error) => {
        console.error("Failed to save mods:", error);
        throw error;
      }
    );
  }

  /**
   * Delete a mod from the database
   */
  public async deleteMod(modId: string): Promise<void> {
    if (!this.db) {
      throw new Error("Database not initialized");
    }

    return withDatabaseLock(async (db) => {
      await db.execute("DELETE FROM mods WHERE id = ?", [modId]);
      // Sync with backend, passing the db connection to avoid nested locks
      await this.syncModsWithBackend(db);
    }, true, "deleteMod").catch((error) => {
      console.error("Failed to delete mod:", error);
      throw error;
    });
  }

  /**
   * Save a folder to the database
   */
  public async saveFolder(folder: Folder): Promise<void> {
    if (!this.db) {
      throw new Error("Database not initialized");
    }
    return withDatabaseLock(async (db) => {
      // Pass the database connection to _saveAndSyncFolders to avoid nested locks
      await this._saveAndSyncFolders([folder], db);
    }, true, "saveFolder").catch(
      (error) => {
        console.error("Failed to save folder:", error);
        throw error;
      }
    );
  }

  /**
   * Save multiple folders to the database
   */
  public async saveFolders(folders: Folder[]): Promise<void> {
    if (!this.db) {
      throw new Error("Database not initialized");
    }
    return withDatabaseLock(async (db) => {
      // Pass the database connection to _saveAndSyncFolders to avoid nested locks
      await this._saveAndSyncFolders(folders, db);
    }, true, "saveFolders").catch(
      (error) => {
        console.error("Failed to save folders:", error);
        throw error;
      }
    );
  }

  /**
   * Delete a folder from the database
   */
  public async deleteFolder(folderId: string): Promise<void> {
    if (!this.db) {
      throw new Error("Database not initialized");
    }

    return withDatabaseLock(async (db) => {
      try {
        // Get mods in this folder
        await db.select("SELECT id FROM mods WHERE folder_id = ?", [folderId]);

        // Remove folder_id from all mods in this folder
        await db.execute(
          `UPDATE mods SET folder_id = NULL WHERE folder_id = ?`,
          [folderId]
        );

        // Delete the folder
        await db.execute("DELETE FROM folders WHERE id = ?", [folderId]);
        
        // Sync with backend after the operation, passing the db connection to avoid nested locks
        await this.syncModsWithBackend(db);
      } catch (error) {
        throw error;
      }
    }, true, "deleteFolder").catch((error) => {
      console.error("Failed to delete folder:", error);
      throw error;
    });
  }

  /**
   * Move a mod to a folder
   */
  public async moveModToFolder(
    modId: string,
    folderId: string | null
  ): Promise<void> {
    if (!this.db) {
      throw new Error("Database not initialized");
    }

    return withDatabaseLock(async (db) => {
      // Update the mod's folder_id
      await db.execute(
        `UPDATE mods SET folder_id = ? WHERE id = ?`,
        [folderId, modId]
      );

      // If we're adding to a folder (not removing), update the folder's mods list
      if (folderId) {
        // Get the current folder data to preserve its display_order
        const folderResult = await db.select(
          `SELECT * FROM folders WHERE id = ?`,
          [folderId]
        );

        if (folderResult.length > 0) {
          const folder = folderResult[0];
          const folderMods = folder.mods ? JSON.parse(folder.mods) : [];

          // Add the mod to the folder's mods list if not already there
          if (!folderMods.includes(modId)) {
            folderMods.push(modId);

            // Update the folder with the new mods list but preserve display_order
            await db.execute(
              `UPDATE folders SET mods = ? WHERE id = ?`,
              [JSON.stringify(folderMods), folderId]
            );
          }
        }
      }

      // Sync with backend, passing db connection to avoid nested locks
      await this.syncModsWithBackend(db);
    }, true, "moveModToFolder").catch((error) => {
      console.error("Failed to move mod to folder:", error);
      throw error;
    });
  }

  /**
   * Update display order for mods and folders
   */
  public async updateDisplayOrder(items: DisplayItem[]): Promise<void> {
    if (!this.db) {
      throw new Error("Database not initialized");
    }

    return withDatabaseLock(async (db) => {
      for (let i = 0; i < items.length; i++) {
        const item = items[i];
        const display_order = i;

        if (item.type === "folder") {
          await db.execute(
            `UPDATE folders SET display_order = ? WHERE id = ?`,
            [display_order, item.id]
          );
        } else if (item.type === "mod") {
          await db.execute(
            `UPDATE mods SET display_order = ? WHERE id = ?`,
            [display_order, item.id]
          );
        }
      }

      // Sync with backend, passing the db connection to avoid nested locks
      await this.syncModsWithBackend(db);
    }, true, "updateDisplayOrder").catch((error) => {
      console.error("Failed to update display order:", error);
      throw error;
    });
  }

  /**
   * Update display order for mods within a folder
   * This performs a batch update of all mods' display_order_in_folder property
   * in a single transaction for better performance
   */
  public async updateDisplayOrderInFolder(folderId: string, mods: Mod[]): Promise<void> {
    if (!this.db) {
      throw new Error("Database not initialized");
    }

    return withDatabaseLock(async (db) => {
      // Begin a transaction for all updates
      for (let i = 0; i < mods.length; i++) {
        const mod = mods[i];
        const displayOrderInFolder = i;

        await db.execute(
          `UPDATE mods SET display_order_in_folder = ? WHERE id = ? AND folder_id = ?`,
          [displayOrderInFolder, mod.id, folderId]
        );
      }

      // Sync with backend, passing the db connection to avoid nested locks
      await this.syncModsWithBackend(db);
    }, true, "updateDisplayOrderInFolder").catch((error) => {
      console.error("Failed to update display order in folder:", error);
      throw error;
    });
  }

  /**
   * Get a setting from the store
   * @deprecated Use StoreService.getSetting() instead
   */
  public async getSetting(key: string): Promise<string | null> {
    console.warn(
      "DatabaseService.getSetting is deprecated, use StoreService directly instead"
    );
    const value = await this.storeService.getSetting(key as keyof AppSettings);
    return value !== null ? String(value) : null;
  }

  /**
   * Save a setting to the store
   * @deprecated Use StoreService.saveSetting() instead
   */
  public async saveSetting(key: string, value: string): Promise<void> {
    console.warn(
      "DatabaseService.saveSetting is deprecated, use StoreService directly instead"
    );
    await this.storeService.saveSetting(key as keyof AppSettings, value as any);
  }

  /**
   * Sync mods with the Rust backend
   * @param existingDb Optional database connection from parent operation
   */
  private async syncModsWithBackend(existingDb?: any): Promise<void> {
    try {
      let mods: Mod[];
      
      // If we have an existing DB connection, use it directly
      if (existingDb) {
        console.log("syncModsWithBackend - Using provided database connection");
        
        // Query mods using the provided connection
        const result = await existingDb.select(
          "SELECT * FROM mods ORDER BY display_order ASC"
        );
        mods = result.map((mod: any) => {
          // Parse engine data if it exists
          let engine;
          try {
            engine = mod.engine_data
              ? JSON.parse(mod.engine_data)
              : {
                  engine_type: "unknown",
                  engine_name: "Unknown Engine",
                  engine_icon: "",
                  mods_folder: false,
                  mods_folder_path: "",
                };
          } catch (e) {
            console.error("Failed to parse engine data for mod:", mod.id, e);
            engine = {
              engine_type: "unknown",
              engine_name: "Unknown Engine",
              engine_icon: "",
              mods_folder: false,
              mods_folder_path: "",
            };
          }

          // Parse contributors data if it exists
          let contributors = undefined;
          if (mod.contributors_data) {
            try {
              contributors = JSON.parse(mod.contributors_data);
            } catch (e) {
              console.error("Failed to parse contributors data for mod:", mod.id, e);
            }
          }

          return {
            ...mod,
            engine,
            contributors,
          };
        });
      } else {
        // No existing connection, acquire our own lock
        console.log("syncModsWithBackend - Getting mods with own database lock");
        
        // Use a separate query to get mods without starting a new transaction
        mods = await withDatabaseLock(async (db) => {
          const result = await db.select(
            "SELECT * FROM mods ORDER BY display_order ASC"
          );
          return result.map((mod: any) => {
            // Parse engine data if it exists
            let engine;
            try {
              engine = mod.engine_data
                ? JSON.parse(mod.engine_data)
                : {
                    engine_type: "unknown",
                    engine_name: "Unknown Engine",
                    engine_icon: "",
                    mods_folder: false,
                    mods_folder_path: "",
                  };
            } catch (e) {
              console.error("Failed to parse engine data for mod:", mod.id, e);
              engine = {
                engine_type: "unknown",
                engine_name: "Unknown Engine",
                engine_icon: "",
                mods_folder: false,
                mods_folder_path: "",
              };
            }

            // Parse contributors data if it exists
            let contributors = undefined;
            if (mod.contributors_data) {
              try {
                contributors = JSON.parse(mod.contributors_data);
              } catch (e) {
                console.error("Failed to parse contributors data for mod:", mod.id, e);
              }
            }

            return {
              ...mod,
              engine,
              contributors,
            };
          });
        }, false, "syncModsWithBackend");
      }

      // Add a small delay to ensure any active transaction has settled
      await new Promise((resolve) => setTimeout(resolve, 50));

      await invoke("sync_mods_from_database", { modsData: mods });
      console.log("Successfully synced mods with backend");
    } catch (error) {
      console.error("Failed to sync mods with backend:", error);
      // Don't throw to avoid disrupting the transaction flow
      // but still log the error
    }
  }

  /**
   * Clear all data from the database
   * This wipes all tables but preserves the schema
   */
  public async clearDatabase(): Promise<void> {
    if (!this.db) {
      throw new Error("Database not initialized");
    }

    return withDatabaseLock(async (db) => {
      try {
        // Delete all mods
        await db.execute("DELETE FROM mods");
        console.log("Deleted all mods from database");
        
        // Delete all folders
        await db.execute("DELETE FROM folders");
        console.log("Deleted all folders from database");
        
        // Delete all mod folder mappings (for backward compatibility)
        await db.execute("DELETE FROM mod_folders");
        console.log("Deleted all mod folder mappings from database");
        
        // Sync with backend to reflect the cleared state
        await this.syncModsWithBackend(db);
        
        console.log("Database cleared successfully");
      } catch (error) {
        console.error("Failed to clear database:", error);
        throw error;
      }
    }, true, "clearDatabase");
  }

  /**
   * Helper method to create an empty mod
   */
  public createEmptyMod(): Mod {
    return {
      id: uuidv4(),
      name: "",
      path: "",
      display_order: 9999,
      engine: {
        engine_type: "unknown",
        engine_name: "Unknown Engine",
        engine_icon: "",
        mods_folder: false,
        mods_folder_path: "",
      },
    };
  }

  /**
   * Helper method to create an empty folder
   */
  public createEmptyFolder(): Folder {
    return {
      id: uuidv4(),
      name: "New Folder",
      color: "#FF0088",
      mods: [],
      display_order: 9999,
    };
  }

  /**
   * Get a mod by its path
   * @param path The filesystem path of the mod
   * @returns The mod if found, null otherwise
   */
  public async getModByPath(path: string): Promise<Mod | null> {
    if (!this.db) {
      throw new Error("Database not initialized");
    }

    return withDatabaseLock(async (db) => {
      const results = await db.select("SELECT * FROM mods WHERE path = ?", [path]);
      
      if (results.length === 0) {
        return null;
      }
      
      const mod = results[0];
      
      // Parse engine data if it exists
      let engine;
      try {
        engine = mod.engine_data
          ? JSON.parse(mod.engine_data)
          : {
              engine_type: "unknown",
              engine_name: "Unknown Engine",
              engine_icon: "",
              mods_folder: false,
              mods_folder_path: "",
            };
      } catch (e) {
        console.error("Failed to parse engine data for mod:", mod.id, e);
        engine = {
          engine_type: "unknown",
          engine_name: "Unknown Engine",
          engine_icon: "",
          mods_folder: false,
          mods_folder_path: "",
        };
      }

      return {
        ...mod,
        engine,
      };
    }, false, "getModByPath").catch((error) => {
      console.error("Failed to get mod by path:", error);
      throw error;
    });
  }
}
