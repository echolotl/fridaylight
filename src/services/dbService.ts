import Database from '@tauri-apps/plugin-sql'
import { invoke } from '@tauri-apps/api/core'
import { v4 as uuidv4 } from 'uuid'
import {
  Mod,
  Folder,
  DisplayItem,
  AppSettings,
  EngineModProfile,
  Engine,
} from '../types'
import { StoreService } from './storeService'

// Constants for database lock protection
const MAX_RETRY_ATTEMPTS = 10 // Increased maximum retry attempts
const INITIAL_RETRY_DELAY_MS = 50 // Start with a shorter delay
const MAX_RETRY_DELAY_MS = 1000 // Cap the maximum delay

export const DEFAULT_ENGINE: Engine = {
  engine_type: 'unknown',
  mods_folder: false,
}

// Mutex for database operations
class Mutex {
  private mutex = Promise.resolve()
  private owner: string | null = null

  async lock(operation: string = 'unknown'): Promise<() => void> {
    dbConsole.debug(`🔒 Mutex: Requesting lock for operation: ${operation}`)

    let resolve: () => void = () => {}
    const unlock = new Promise<void>(res => {
      resolve = () => {
        dbConsole.debug(`🔓 Mutex: Released lock for operation: ${operation}`)
        this.owner = null
        res()
      }
    })

    const previous = this.mutex
    this.mutex = previous.then(() => unlock)

    try {
      dbConsole.debug(
        `⏳ Mutex: Waiting for previous operations to complete for: ${operation}`
      )
      await previous
      this.owner = operation
      dbConsole.debug(`✅ Mutex: Lock acquired for operation: ${operation}`)
      return resolve
    } catch (error) {
      dbConsole.error(
        `❌ Mutex: Error waiting for lock for operation: ${operation}`,
        error
      )
      // Make sure we still return a valid unlock function even if there was an error
      return resolve
    }
  }

  getCurrentOwner(): string | null {
    return this.owner
  }
}

class dbConsole {
  private static style = 'color: #77f; font-weight: bold;'
  private static errorStyle = 'color: #f00; font-weight: bold;'
  private static warnStyle = 'color: #ff0; font-weight: bold;'
  private static infoStyle = 'color: #0ff;'
  private static debugStyle = 'color: #dff; font-weight: bold;'

  static log(message: string, ...args: any[]) {
    console.log(`%c[DbService] ${message}`, this.style, ...args)
  }
  static debug(message: string, ...args: any[]) {
    console.debug(`%c[DbService] ${message}`, this.debugStyle, ...args)
  }
  static error(message: string, ...args: any[]) {
    console.error(`%c[DbService] ${message}`, this.errorStyle, ...args)
  }
  static warn(message: string, ...args: any[]) {
    console.warn(`%c[DbService] ${message}`, this.warnStyle, ...args)
  }
  static info(message: string, ...args: any[]) {
    console.info(`%c[DbService] ${message}`, this.infoStyle, ...args)
  }
}

// Global mutex for database operations
const dbMutex = new Mutex()

// Transaction state tracking to prevent nested transactions
let transactionActive = false

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
  operationName: string = 'unknown'
): Promise<T> {
  let attempts = 0
  dbConsole.log(
    `withDatabaseLock - Starting '${operationName}' with useTransaction=${useTransaction}`
  )

  // Acquire global mutex lock to prevent concurrent operations
  const unlock = await dbMutex.lock(operationName)

  try {
    while (attempts < MAX_RETRY_ATTEMPTS) {
      try {
        // Get database instance from DatabaseService
        const db = DatabaseService.getInstance().getDb()

        if (useTransaction && !transactionActive) {
          // Set flag before starting transaction
          dbConsole.log(
            `| withDatabaseLock - Starting new transaction for '${operationName}'`
          )
          transactionActive = true
          let transactionStarted = false

          try {
            // Check if we're already in a transaction
            const inTransactionCheck = await db.select(
              'PRAGMA transaction_status'
            )
            const inTransaction =
              inTransactionCheck &&
              inTransactionCheck.length > 0 &&
              inTransactionCheck[0].transaction_status !== 'idle' &&
              inTransactionCheck[0].transaction_status !== undefined

            if (!inTransaction) {
              dbConsole.log(
                `| withDatabaseLock - No existing transaction, starting new one for '${operationName}'`
              )
              await db.execute('BEGIN EXCLUSIVE TRANSACTION') // Use EXCLUSIVE for better locking
              transactionStarted = true
              dbConsole.log(`| Started new transaction for '${operationName}'`)
            } else {
              dbConsole.log(
                `| Transaction already active, using existing transaction for '${operationName}'`
              )
            }

            const result = await operation(db)

            // Only commit if we started the transaction
            if (transactionStarted) {
              await db.execute('COMMIT')
              dbConsole.log(
                `| Transaction committed successfully for '${operationName}'`
              )
            }

            transactionActive = false
            return result
          } catch (error) {
            dbConsole.error(
              `| withDatabaseLock - Error in transaction for '${operationName}':`,
              error
            )
            // Only try to rollback if we started a transaction
            if (transactionStarted) {
              try {
                dbConsole.log(
                  `| Error in transaction, attempting rollback for '${operationName}'`
                )
                await db.execute('ROLLBACK')
              } catch (rollbackError) {
                dbConsole.error(
                  `| Rollback failed for '${operationName}':`,
                  rollbackError
                )
                // Continue with the original error
              }
            }

            transactionActive = false
            throw error
          }
        } else if (useTransaction && transactionActive) {
          // If a transaction is already active, just run the operation without starting a new one
          dbConsole.log(
            `| Using existing transaction for operation '${operationName}'`
          )
          const result = await operation(db)
          return result
        } else {
          // No transaction needed
          const result = await operation(db)
          dbConsole.log(
            `| withDatabaseLock - Operation completed successfully (no transaction) for '${operationName}'`
          )
          return result
        }
      } catch (error) {
        attempts++
        dbConsole.error(
          `| withDatabaseLock - Attempt ${attempts} failed for '${operationName}':`,
          error
        )

        // Check if error is SQLITE_BUSY or a database lock error
        const isLockError =
          error instanceof Error &&
          (error.message.includes('database is locked') ||
            error.message.includes('SQLITE_BUSY'))

        // If it's not a lock error or we've reached max attempts, throw the error
        if (!isLockError || attempts >= MAX_RETRY_ATTEMPTS) {
          dbConsole.error(
            `| Database operation '${operationName}' failed after ${attempts} attempts:`,
            error
          )
          throw error
        }

        // Wait before retrying with exponential backoff capped at max delay
        const delay = Math.min(
          INITIAL_RETRY_DELAY_MS * Math.pow(2, attempts - 1),
          MAX_RETRY_DELAY_MS
        )
        dbConsole.warn(
          `| Database locked, retrying operation '${operationName}' (attempt ${attempts}/${MAX_RETRY_ATTEMPTS}) after ${delay}ms...`
        )
        await new Promise(resolve => setTimeout(resolve, delay))
      }
    }

    // This should never be reached due to the throw in the loop, but TypeScript requires a return
    dbConsole.error(
      `| withDatabaseLock - Reached maximum retry attempts without success or error for '${operationName}'`
    )
    throw new Error(
      `| Failed to acquire database lock after maximum retry attempts for operation '${operationName}'`
    )
  } finally {
    // Always release the mutex lock
    dbConsole.log(
      `| withDatabaseLock - Releasing mutex lock for '${operationName}'`
    )
    unlock()
  }
}

/**
 * DatabaseService class handles all database operations
 */
export class DatabaseService {
  private static instance: DatabaseService
  private db: any = null
  private initialized = false
  private storeService: StoreService
  private migrationDowns:
    | Array<
        (db: {
          execute: (arg0: string) => unknown
          select: (query: string) => Promise<unknown[]>
        }) => Promise<void>
      >
    | undefined

  private constructor() {
    this.storeService = StoreService.getInstance()
  }

  /**
   * Get the singleton instance of DatabaseService
   */
  public static getInstance(): DatabaseService {
    if (!DatabaseService.instance) {
      DatabaseService.instance = new DatabaseService()
    }
    return DatabaseService.instance
  }

  /**
   * Get the database instance
   */
  public getDb(): any {
    if (!this.db) {
      throw new Error('Database not initialized')
    }
    return this.db
  }
  /**
   * Initialize the database
   */
  public async initialize(): Promise<void> {
    if (this.initialized) {
      // Reinitialize the database
      await this.db.close()
      this.db = null
      this.initialized = false
    }

    try {
      // Initialize the store service
      await this.storeService.initialize()

      this.db = await Database.load('sqlite:mods.db')

      // Create or update tables with lock protection
      await withDatabaseLock(
        () => this.createModsTable(),
        false,
        'createModsTable'
      )
      await withDatabaseLock(
        () => this.createFoldersTable(),
        false,
        'createFoldersTable'
      )
      await withDatabaseLock(
        () => this.createProfilesTable(),
        false,
        'createProfilesTable'
      )

      await withDatabaseLock(
        () => this._runMigrations(),
        false,
        'runMigrations'
      )

      // Set initialized flag
      this.initialized = true

      // On initialization, we still need to sync with backend using the regular method
      // since we need to read from the database first
      await this.syncModsWithBackend()
      dbConsole.log('Database initialized successfully')
    } catch (error) {
      dbConsole.error('Failed to initialize database:', error)
      throw error
    }
  }

  /**
   * Create or update the mods table
   */
  private async createModsTable(): Promise<void> {
    try {
      // Check if the mods table exists
      const tableInfo: Mod[] = await this.db.select(`PRAGMA table_info(mods)`)

      if (tableInfo.length > 0) {
        // Table exists, check for missing columns
        const columns = tableInfo.map((col: Mod) => col.name)

        // Add missing columns if they don't exist
        switch (true) {
          case !columns.includes('id'):
            await this.db.execute(
              `ALTER TABLE mods ADD COLUMN id TEXT PRIMARY KEY`
            )
            dbConsole.log('Added id column to mods table')
            break
          case !columns.includes('name'):
            await this.db.execute(
              `ALTER TABLE mods ADD COLUMN name TEXT NOT NULL`
            )
            dbConsole.log('Added name column to mods table')
            break
          case !columns.includes('path'):
            await this.db.execute(
              `ALTER TABLE mods ADD COLUMN path TEXT NOT NULL`
            )
            dbConsole.log('Added path column to mods table')
            break
          case !columns.includes('executable_path'):
            await this.db.execute(
              `ALTER TABLE mods ADD COLUMN executable_path TEXT`
            )
            dbConsole.log('Added executable_path column to mods table')
            break
          case !columns.includes('icon_data'):
            await this.db.execute(`ALTER TABLE mods ADD COLUMN icon_data TEXT`)
            dbConsole.log('Added icon_data column to mods table')
            break
          case !columns.includes('banner_data'):
            await this.db.execute(
              `ALTER TABLE mods ADD COLUMN banner_data TEXT`
            )
            dbConsole.log('Added banner_data column to mods table')
            break
          case !columns.includes('logo_data'):
            await this.db.execute(`ALTER TABLE mods ADD COLUMN logo_data TEXT`)
            dbConsole.log('Added logo_data column to mods table')
            break
          case !columns.includes('logo_position'):
            await this.db.execute(
              `ALTER TABLE mods ADD COLUMN logo_position TEXT`
            )
            dbConsole.log('Added logo_position column to mods table')
            break
          case !columns.includes('version'):
            await this.db.execute(`ALTER TABLE mods ADD COLUMN version TEXT`)
            dbConsole.log('Added version column to mods table')
            break
          case !columns.includes('description'):
            await this.db.execute(
              `ALTER TABLE mods ADD COLUMN description TEXT`
            )
            dbConsole.log('Added description column to mods table')
            break
          case !columns.includes('engine'):
            await this.db.execute(`ALTER TABLE mods ADD COLUMN engine TEXT`)
            dbConsole.log('Added engine column to mods table')
            break
          case !columns.includes('display_order'):
            await this.db.execute(
              `ALTER TABLE mods ADD COLUMN display_order INTEGER DEFAULT 0`
            )
            dbConsole.log('Added display_order column to mods table')
            break
          case !columns.includes('folder_id'):
            await this.db.execute(`ALTER TABLE mods ADD COLUMN folder_id TEXT`)
            dbConsole.log('Added folder_id column to mods table')
            break
          case !columns.includes('display_order_in_folder'):
            await this.db.execute(
              `ALTER TABLE mods ADD COLUMN display_order_in_folder INTEGER DEFAULT 0`
            )
            dbConsole.log('Added display_order_in_folder column to mods table')
            break
          case !columns.includes('last_played'):
            await this.db.execute(
              `ALTER TABLE mods ADD COLUMN last_played INTEGER`
            )
            dbConsole.log('Added last_played column to mods table')
            break
          case !columns.includes('date_added'):
            await this.db.execute(
              `ALTER TABLE mods ADD COLUMN date_added INTEGER`
            )
            dbConsole.log('Added date_added column to mods table')
            break
          case !columns.includes('save_terminal_output'):
            await this.db.execute(
              `ALTER TABLE mods ADD COLUMN save_terminal_output BOOLEAN DEFAULT 0`
            )
            dbConsole.log('Added save_terminal_output column to mods table')
            break
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
            engine TEXT,
            display_order INTEGER DEFAULT 0,
            folder_id TEXT,
            display_order_in_folder INTEGER DEFAULT 0,
            last_played INTEGER,
            date_added INTEGER,
            save_terminal_output BOOLEAN DEFAULT 0
          )
        `)
        dbConsole.log('Created mods table with all columns')
      }
    } catch (error) {
      dbConsole.error('Failed to create/update mods table:', error)
      throw error
    }
  }

  /**
   * Create or update the folders table
   */
  private async createFoldersTable(): Promise<void> {
    try {
      const foldersTableInfo: any[] = await this.db.select(
        `PRAGMA table_info(folders)`
      )

      if (foldersTableInfo.length === 0) {
        // Table doesn't exist, create it
        await this.db.execute(`
          CREATE TABLE IF NOT EXISTS folders (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            color TEXT NOT NULL,
            display_order INTEGER DEFAULT 0
          )
        `)
        dbConsole.log('Created folders table')
      } else {
        // Check if display_order column exists
        const columns = foldersTableInfo.map((col: any) => col.name)
        if (!columns.includes('display_order')) {
          await this.db.execute(
            `ALTER TABLE folders ADD COLUMN display_order INTEGER DEFAULT 9999`
          )
          dbConsole.log('Added display_order column to folders table')
        }
      }
    } catch (error) {
      dbConsole.error('Failed to initialize folders table:', error)
      throw error
    }
  }

  /**
   * Create or update the engine_mod_profiles table
   */
  private async createProfilesTable(): Promise<void> {
    try {
      const profilesTableInfo: any[] = await this.db.select(
        `PRAGMA table_info(engine_mod_profiles)`
      )

      if (profilesTableInfo.length === 0) {
        // Table doesn't exist, create it
        await this.db.execute(`
        CREATE TABLE IF NOT EXISTS engine_mod_profiles (
          id TEXT PRIMARY KEY,
          name TEXT NOT NULL,
          icon_data TEXT,
          parent_mod_id TEXT NOT NULL,
          mod_states TEXT NOT NULL,
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          FOREIGN KEY (parent_mod_id) REFERENCES mods(id) ON DELETE CASCADE
        )
      `)
        dbConsole.log('Created engine_mod_profiles table')
      }
    } catch (error) {
      dbConsole.error('Failed to initialize engine_mod_profiles table:', error)
      throw error
    }
  }

  /**
   * Get all mods from the database
   */
  public async getAllMods(): Promise<Mod[]> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }

    return withDatabaseLock(
      async db => {
        const mods = await db.select(
          'SELECT * FROM mods ORDER BY display_order ASC'
        )
        return mods.map((mod: Mod) => ({
          ...mod,
          // Parse engine field from JSON string if needed
          engine:
            typeof mod.engine === 'string'
              ? JSON.parse(mod.engine)
              : mod.engine || DEFAULT_ENGINE,
        }))
      },
      false,
      'getAllMods'
    ).catch(error => {
      dbConsole.error('Failed to get mods:', error)
      throw error
    })
  }
  /**
   * Get all mods from the database using an existing database connection
   * prevents the bane of my existence: deadlocks
   */
  private async _getAllModsWithDb(db: any): Promise<Mod[]> {
    const mods = await db.select(
      'SELECT * FROM mods ORDER BY display_order ASC'
    )
    return mods.map((mod: Mod) => ({
      ...mod,
      // Parse engine field from JSON string if needed
      engine:
        typeof mod.engine === 'string' ? JSON.parse(mod.engine) : mod.engine,
    }))
  }

  /**
   * Get all folders from the database
   */
  public async getAllFolders(): Promise<Folder[]> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }

    return withDatabaseLock(
      async db => {
        const folders = await db.select(`
        SELECT f.id, f.name, f.color, f.display_order, GROUP_CONCAT(m.id) as mod_ids
        FROM folders f
        LEFT JOIN mods m ON m.folder_id = f.id
        GROUP BY f.id
        ORDER BY f.display_order ASC
      `)

        return folders.map((folder: any) => ({
          id: folder.id,
          name: folder.name,
          color: folder.color,
          display_order: folder.display_order || 9999,
          mods: folder.mod_ids ? folder.mod_ids.split(',') : [],
        }))
      },
      false,
      'getAllFolders'
    ).catch(error => {
      dbConsole.error('Failed to get folders:', error)
      throw error
    })
  }

  // Helper to upsert mods in a single transaction with rollback
  private async _upsertMods(mods: Mod[], db?: any): Promise<void> {
    dbConsole.log('_upsertMods - Starting to save mods to database...')

    // If db is provided, we're already in a transaction with a lock, so use it directly
    if (db) {
      dbConsole.log(
        '_upsertMods - Using provided database connection (already in transaction)'
      )

      for (const mod of mods) {
        dbConsole.log(`_upsertMods - Processing mod: ${mod.name} (${mod.id})`)

        try {
          // Generate fresh engine JSON
          const engineData = JSON.stringify(mod.engine)

          // If date_added is not set, set it to the current timestamp
          if (!mod.date_added) {
            mod.date_added = Math.floor(Date.now() / 1000)
            dbConsole.log(
              `_upsertMods - Setting date_added to ${mod.date_added} for new mod`
            )
          }

          dbConsole.log('_upsertMods - Executing SQL to save mod...')
          await db.execute(
            `
            INSERT OR REPLACE INTO mods (
              id, name, path, executable_path, icon_data, banner_data, logo_data, logo_position,
              version, description, engine, display_order, folder_id,
              display_order_in_folder, last_played, date_added, save_terminal_output
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
              mod.last_played || null,
              mod.date_added || null,
              mod.save_terminal_output || false,
            ]
          )
          dbConsole.log(`_upsertMods - Successfully saved mod: ${mod.name}`)
        } catch (error) {
          dbConsole.error(
            `_upsertMods - Error processing mod ${mod.name}:`,
            error
          )
          throw error
        }
      }
      dbConsole.log('_upsertMods - All mods saved successfully')
      return
    }

    // No db provided, we need to acquire our own lock
    try {
      return withDatabaseLock(
        async db => {
          dbConsole.log('_upsertMods - Got database lock, processing mods...')

          for (const mod of mods) {
            // Generate fresh engine JSON
            const engineData = JSON.stringify(mod.engine)

            // If date_added is not set, set it to the current timestamp
            if (!mod.date_added) {
              mod.date_added = Math.floor(Date.now() / 1000)
            }

            await db.execute(
              `
            INSERT OR REPLACE INTO mods (
              id, name, path, executable_path, icon_data, banner_data, logo_data, logo_position,
              version, description, engine, display_order, folder_id,
              display_order_in_folder, last_played, date_added
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
                mod.last_played || null,
                mod.date_added || null,
              ]
            )
          }
        },
        true,
        '_upsertMods'
      )
    } catch (error) {
      dbConsole.error('_upsertMods - Error in transaction:', error)
      throw error
    }
  }

  // Helper to upsert folders in a single transaction with rollback
  private async _upsertFolders(folders: Folder[], db?: any): Promise<void> {
    dbConsole.log('_upsertFolders - Starting to save folders to database...')

    // If db is provided, we're already in a transaction with a lock, so use it directly
    if (db) {
      dbConsole.log(
        '_upsertFolders - Using provided database connection (already in transaction)'
      )

      for (const folder of folders) {
        dbConsole.log(
          `_upsertFolders - Processing folder: ${folder.name} (${folder.id})`
        )

        try {
          dbConsole.log(`_upsertFolders - Final folder data:
            id: ${folder.id}
            name: ${folder.name}
            color: ${folder.color}
            display_order: ${folder.display_order || 9999}
          `)
          dbConsole.log('_upsertFolders - Executing SQL to save folder...')

          await db.execute(
            `INSERT OR REPLACE INTO folders (id, name, color, display_order) 
             VALUES (?, ?, ?, ?)`,
            [folder.id, folder.name, folder.color, folder.display_order || 9999]
          )

          // Update the folder_id for all mods in this folder
          if (folder.mods && Array.isArray(folder.mods)) {
            for (const modId of folder.mods) {
              await db.execute(`UPDATE mods SET folder_id = ? WHERE id = ?`, [
                folder.id,
                modId,
              ])
            }
          }

          dbConsole.log(
            `_upsertFolders - Successfully saved folder: ${folder.name}`
          )
        } catch (error) {
          dbConsole.error(
            `_upsertFolders - Error processing folder ${folder.name}:`,
            error
          )
          throw error
        }
      }
      dbConsole.log('_upsertFolders - All folders saved successfully')
      return
    }

    // No db provided, we need to acquire our own lock
    try {
      return withDatabaseLock(
        async db => {
          dbConsole.log(
            '_upsertFolders - Got database lock, processing folders...'
          )

          for (const folder of folders) {
            dbConsole.log(
              `_upsertFolders - Processing folder: ${folder.name} (${folder.id})`
            )

            try {
              await db.execute(
                `INSERT OR REPLACE INTO folders (id, name, color, display_order) 
                 VALUES (?, ?, ?, ?)`,
                [
                  folder.id,
                  folder.name,
                  folder.color,
                  folder.display_order || 9999,
                ]
              )

              // Update the folder_id for all mods in this folder
              if (folder.mods && Array.isArray(folder.mods)) {
                for (const modId of folder.mods) {
                  await db.execute(
                    `UPDATE mods SET folder_id = ? WHERE id = ?`,
                    [folder.id, modId]
                  )
                }
              }

              dbConsole.log(
                `_upsertFolders - Successfully saved folder: ${folder.name}`
              )
            } catch (error) {
              dbConsole.error(
                `_upsertFolders - Error processing folder ${folder.name}:`,
                error
              )
              throw error
            }
          }
        },
        true,
        '_upsertFolders'
      )
    } catch (error) {
      dbConsole.error('_upsertFolders - Error in transaction:', error)
      throw error
    }
  }

  // Helper to sync with backend first, then upsert mods to the database
  private async _saveAndSyncMods(mods: Mod[], existingDb?: any): Promise<void> {
    dbConsole.log(
      '_saveAndSyncMods - About to save mods with engine data:',
      mods.map(mod => ({
        id: mod.id,
        name: mod.name,
        engine: mod.engine ? JSON.stringify(mod.engine) : undefined,
      }))
    )

    // First, sync directly with backend for faster UI updates
    dbConsole.log(
      '_saveAndSyncMods - Syncing with backend first for faster UI updates'
    )
    await this.syncModsDirectly(mods)

    // If we were passed a database connection, we're already in a transaction with a lock
    if (existingDb) {
      dbConsole.log(
        '_saveAndSyncMods - Using provided database connection (already in transaction)'
      )

      // Pass the existing db connection to _upsertMods to avoid nested locks
      await this._upsertMods(mods, existingDb)
      dbConsole.log('_saveAndSyncMods - Mods saved to database')
      return
    }

    // No existing db connection, we need to acquire our own lock
    // Use withDatabaseLock to handle the entire operation within a single lock
    await withDatabaseLock(
      async db => {
        // Pass the db connection to _upsertMods to avoid nested locks
        await this._upsertMods(mods, db)
        dbConsole.log('_saveAndSyncMods - Mods saved to database')
      },
      true,
      '_saveAndSyncMods'
    )
  }

  // Helper to sync with backend first, then upsert folders to the database
  private async _saveAndSyncFolders(
    folders: Folder[],
    existingDb?: any
  ): Promise<void> {
    dbConsole.log('_saveAndSyncFolders - About to save folders')

    // First, we need to get all mods to sync with backend
    let allMods: Mod[] = []
    try {
      // Get all mods from database to update folder associations
      // If we have an existing db connection, use it to avoid deadlock
      if (existingDb) {
        allMods = await this._getAllModsWithDb(existingDb)
      } else {
        allMods = await this.getAllMods()
      }

      // Update the folder assignments in memory first
      for (const folder of folders) {
        if (folder.mods && Array.isArray(folder.mods)) {
          for (const modId of folder.mods) {
            const modIndex = allMods.findIndex(m => m.id === modId)
            if (modIndex !== -1) {
              allMods[modIndex].folder_id = folder.id
            }
          }
        }
      }

      // First sync directly with backend for faster UI updates
      dbConsole.log(
        '_saveAndSyncFolders - Syncing with backend first for faster UI updates'
      )
      await this.syncModsDirectly(allMods)
    } catch (error) {
      dbConsole.error('Failed to get mods for folder sync:', error)
    } // If we were passed a database connection, we're already in a transaction with a lock
    if (existingDb) {
      dbConsole.log(
        '_saveAndSyncFolders - Using provided database connection (already in transaction)'
      )

      // Pass the existing db connection to _upsertFolders to avoid nested locks
      await this._upsertFolders(folders, existingDb)
      dbConsole.log('_saveAndSyncFolders - Folders saved to database')
      return
    }

    // No existing db connection, we need to acquire our own lock
    // Use withDatabaseLock to handle the entire operation within a single lock
    await withDatabaseLock(
      async db => {
        // Pass the db connection to _upsertFolders to avoid nested locks
        await this._upsertFolders(folders, db)
        dbConsole.log('_saveAndSyncFolders - Folders saved to database')
      },
      true,
      '_saveAndSyncFolders'
    )
  }

  /**
   * Save a mod to the database
   */
  public async saveMod(mod: Mod): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }

    if (!mod.engine) {
      mod.engine = DEFAULT_ENGINE
    }

    return withDatabaseLock(
      async db => {
        // Pass the db connection to _saveAndSyncMods to avoid nested locks
        await this._saveAndSyncMods([mod], db)
      },
      true,
      'saveMod'
    ).catch(error => {
      dbConsole.error('Failed to save mod:', error)
      throw error
    })
  }

  /**
   * Save multiple mods to the database
   */
  public async saveMods(mods: Mod[]): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }

    return withDatabaseLock(
      async db => {
        // Pass the db connection to _saveAndSyncMods to avoid nested locks
        await this._saveAndSyncMods(mods, db)
      },
      true,
      'saveMods'
    ).catch(error => {
      dbConsole.error('Failed to save mods:', error)
      throw error
    })
  }
  /**
   * Delete a mod from the database
   */
  public async deleteMod(modId: string): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }

    // First sync with backend directly (without the deleted mod) for faster UI updates
    try {
      const allMods = await this.getAllMods()
      const filteredMods = allMods.filter(mod => mod.id !== modId)
      await this.syncModsDirectly(filteredMods)
    } catch (error) {
      dbConsole.error('Failed to sync mods directly during deleteMod:', error)
    }

    return withDatabaseLock(
      async db => {
        await db.execute('DELETE FROM mods WHERE id = ?', [modId])
        dbConsole.log(`Deleted mod ${modId} from database`)
      },
      true,
      'deleteMod'
    ).catch(error => {
      dbConsole.error('Failed to delete mod:', error)
      throw error
    })
  }

  /**
   * Save a folder to the database
   */
  public async saveFolder(folder: Folder): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }
    return withDatabaseLock(
      async db => {
        // Pass the database connection to _saveAndSyncFolders to avoid nested locks
        await this._saveAndSyncFolders([folder], db)
      },
      true,
      'saveFolder'
    ).catch(error => {
      dbConsole.error('Failed to save folder:', error)
      throw error
    })
  }

  /**
   * Save multiple folders to the database
   */
  public async saveFolders(folders: Folder[]): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }
    return withDatabaseLock(
      async db => {
        // Pass the database connection to _saveAndSyncFolders to avoid nested locks
        await this._saveAndSyncFolders(folders, db)
      },
      true,
      'saveFolders'
    ).catch(error => {
      dbConsole.error('Failed to save folders:', error)
      throw error
    })
  }
  /**
   * Delete a folder from the database
   */
  public async deleteFolder(folderId: string): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }

    // First, sync with backend directly for faster UI updates
    try {
      // Get all mods
      const allMods = await this.getAllMods()

      // Remove folder_id from affected mods in memory
      const updatedMods = allMods.map(mod => {
        if (mod.folder_id === folderId) {
          return { ...mod, folder_id: null }
        }
        return mod
      })

      // Sync the updated mods with backend directly
      await this.syncModsDirectly(updatedMods)
    } catch (error) {
      dbConsole.error(
        'Failed to sync mods directly during deleteFolder:',
        error
      )
    }

    return withDatabaseLock(
      async db => {
        // Remove folder_id from all mods in this folder
        await db.execute(
          `UPDATE mods SET folder_id = NULL WHERE folder_id = ?`,
          [folderId]
        )

        // Delete the folder
        await db.execute('DELETE FROM folders WHERE id = ?', [folderId])
        dbConsole.log(`Deleted folder ${folderId} from database`)
      },
      true,
      'deleteFolder'
    ).catch(error => {
      dbConsole.error('Failed to delete folder:', error)
      throw error
    })
  }
  /**
   * Move a mod to a folder
   */
  public async moveModToFolder(
    modId: string,
    folderId: string | null
  ): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }

    // First sync with backend directly for faster UI updates
    try {
      const allMods = await this.getAllMods()

      // Update the mod's folder_id in memory
      const updatedMods = allMods.map(mod => {
        if (mod.id === modId) {
          return { ...mod, folder_id: folderId }
        }
        return mod
      })

      // Sync the updated mods with backend directly
      await this.syncModsDirectly(updatedMods)
    } catch (error) {
      dbConsole.error(
        'Failed to sync mods directly during moveModToFolder:',
        error
      )
    }

    return withDatabaseLock(
      async db => {
        // Update the mod's folder_id
        await db.execute(`UPDATE mods SET folder_id = ? WHERE id = ?`, [
          folderId,
          modId,
        ])

        // If we're adding to a folder (not removing), update the folder's mods list
        if (folderId) {
          // Get the current folder data to preserve its display_order
          const folderResult = await db.select(
            `SELECT * FROM folders WHERE id = ?`,
            [folderId]
          )

          if (folderResult.length > 0) {
            const folder = folderResult[0]
            const folderMods = folder.mods ? JSON.parse(folder.mods) : []

            // Add the mod to the folder's mods list if not already there
            if (!folderMods.includes(modId)) {
              folderMods.push(modId)

              // Update the folder with the new mods list but preserve display_order
              await db.execute(`UPDATE folders SET mods = ? WHERE id = ?`, [
                JSON.stringify(folderMods),
                folderId,
              ])
            }
          }
        }

        dbConsole.log(
          `Moved mod ${modId} to folder ${folderId || 'none'} in database`
        )
      },
      true,
      'moveModToFolder'
    ).catch(error => {
      dbConsole.error('Failed to move mod to folder:', error)
      throw error
    })
  }
  /**
   * Update display order for mods and folders
   */
  public async updateDisplayOrder(items: DisplayItem[]): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }

    // First, sync with backend directly for faster UI updates
    try {
      // Get all mods
      const allMods = await this.getAllMods()

      // Update display_order for each mod in memory
      const updatedMods = allMods.map(mod => {
        const item = items.find(
          item => item.id === mod.id && item.type === 'mod'
        )
        if (item) {
          const index = items.indexOf(item)
          return { ...mod, display_order: index }
        }
        return mod
      })

      // Sync the updated mods with backend directly
      await this.syncModsDirectly(updatedMods)
    } catch (error) {
      dbConsole.error(
        'Failed to sync mods directly during updateDisplayOrder:',
        error
      )
    }

    return withDatabaseLock(
      async db => {
        for (let i = 0; i < items.length; i++) {
          const item = items[i]
          const display_order = i

          if (item.type === 'folder') {
            await db.execute(
              `UPDATE folders SET display_order = ? WHERE id = ?`,
              [display_order, item.id]
            )
          } else if (item.type === 'mod') {
            await db.execute(`UPDATE mods SET display_order = ? WHERE id = ?`, [
              display_order,
              item.id,
            ])
          }
        }

        dbConsole.log(
          `Updated display order for ${items.length} items in database`
        )
      },
      true,
      'updateDisplayOrder'
    ).catch(error => {
      dbConsole.error('Failed to update display order:', error)
      throw error
    })
  }
  /**
   * Update display order for mods within a folder
   * This performs a batch update of all mods' display_order_in_folder property
   * in a single transaction for better performance
   */
  public async updateDisplayOrderInFolder(
    folderId: string,
    mods: Mod[]
  ): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }

    // First, sync with backend directly for faster UI updates
    try {
      // Get all mods
      const allMods = await this.getAllMods()

      // Update display_order_in_folder for each mod in memory
      const updatedMods = allMods.map(mod => {
        if (mod.folder_id === folderId) {
          const index = mods.findIndex(m => m.id === mod.id)
          if (index !== -1) {
            return { ...mod, display_order_in_folder: index }
          }
        }
        return mod
      })

      // Sync the updated mods with backend directly
      await this.syncModsDirectly(updatedMods)
    } catch (error) {
      dbConsole.error(
        'Failed to sync mods directly during updateDisplayOrderInFolder:',
        error
      )
    }

    return withDatabaseLock(
      async db => {
        // Begin a transaction for all updates
        for (let i = 0; i < mods.length; i++) {
          const mod = mods[i]
          const displayOrderInFolder = i

          await db.execute(
            `UPDATE mods SET display_order_in_folder = ? WHERE id = ? AND folder_id = ?`,
            [displayOrderInFolder, mod.id, folderId]
          )
        }

        dbConsole.log(
          `Updated display order for ${mods.length} mods in folder ${folderId} in database`
        )
      },
      true,
      'updateDisplayOrderInFolder'
    ).catch(error => {
      dbConsole.error('Failed to update display order in folder:', error)
      throw error
    })
  }

  /**
   * Get a setting from the store
   * @deprecated Use StoreService.getSetting() instead
   */
  public async getSetting(key: string): Promise<string | null> {
    dbConsole.warn(
      'DatabaseService.getSetting is deprecated, use StoreService directly instead'
    )
    const value = await this.storeService.getSetting(key as keyof AppSettings)
    return value !== null ? String(value) : null
  }

  /**
   * Save a setting to the store
   * Just a wrapper around StoreService.saveSetting
   */
  public async saveSetting(key: string, value: string): Promise<void> {
    await this.storeService.saveSetting(key as keyof AppSettings, value as any)
  }
  /**
   * Sync mods with the Rust backend
   * @param existingDb Optional database connection from parent operation
   */
  public async syncModsWithBackend(existingDb?: any): Promise<void> {
    try {
      let mods: Mod[]

      // If we have an existing DB connection, use it directly
      if (existingDb) {
        dbConsole.log(
          'syncModsWithBackend - Using provided database connection'
        )

        // Query mods using the provided connection
        const result = await existingDb.select(
          'SELECT * FROM mods ORDER BY display_order ASC'
        )
        mods = result.map((mod: Mod) => {
          return {
            ...mod,
            // Parse engine field from JSON string if needed
            engine:
              typeof mod.engine === 'string'
                ? JSON.parse(mod.engine)
                : mod.engine,
            // Add the group field required by the backend
            group: mod.folder_id || 'none',
            // Convert timestamp fields to integers to avoid Rust type errors
            last_played: mod.last_played
              ? Math.floor(Number(mod.last_played))
              : null,
            date_added: mod.date_added
              ? Math.floor(Number(mod.date_added))
              : null,
            // Convert save_terminal_output to boolean
            save_terminal_output: Boolean(mod.save_terminal_output),
          }
        })
      } else {
        // No existing connection, acquire our own lock
        dbConsole.log(
          'syncModsWithBackend - Getting mods with own database lock'
        )

        // Use a separate query to get mods without starting a new transaction
        mods = await withDatabaseLock(
          async db => {
            const result = await db.select(
              'SELECT * FROM mods ORDER BY display_order ASC'
            )
            return result.map((mod: Mod) => {
              return {
                ...mod,
                // Parse engine field from JSON string if needed
                engine:
                  typeof mod.engine === 'string'
                    ? JSON.parse(mod.engine)
                    : mod.engine,
                // Add the group field required by the backend
                group: mod.folder_id || 'none',
                // Convert timestamp fields to integers to avoid Rust type errors
                last_played: mod.last_played
                  ? Math.floor(Number(mod.last_played))
                  : null,
                date_added: mod.date_added
                  ? Math.floor(Number(mod.date_added))
                  : null,
                // Convert save_terminal_output to boolean (SQLite stores as 0/1)
                save_terminal_output: Boolean(mod.save_terminal_output),
              }
            })
          },
          false,
          'syncModsWithBackend'
        )
      } // Add a small delay to ensure any active transaction has settled
      await new Promise(resolve => setTimeout(resolve, 50))

      // Ensure all mods have properly formatted timestamp values before sending to the backend
      const sanitizedMods = mods.map(mod => ({
        ...mod,
        // Ensure engine is an object - handle both string (from DB) and object (from memory) cases
        engine:
          typeof mod.engine === 'string' ? JSON.parse(mod.engine) : mod.engine,
        last_played: mod.last_played
          ? Math.floor(Number(mod.last_played))
          : null,
        date_added: mod.date_added ? Math.floor(Number(mod.date_added)) : null,
        // Convert save_terminal_output to boolean (SQLite stores as 0/1)
        save_terminal_output: Boolean(mod.save_terminal_output),
      }))

      await invoke('sync_mods_from_database', { modsData: sanitizedMods })
      dbConsole.log('Successfully synced mods with backend')
    } catch (error) {
      dbConsole.error('Failed to sync mods with backend:', error)
      // Don't throw to avoid disrupting the transaction flow
      // but still log the error
    }
  }

  /**
   * Sync mods directly with the Rust backend without reading from database first
   * For faster UI updates, this can be called before saving to database
   * @param mods The mods to sync with the backend
   */
  public async syncModsDirectly(mods: Mod[]): Promise<void> {
    try {
      dbConsole.log('syncModsDirectly - Syncing mods directly with backend...')

      // Format the mods for the backend
      const formattedMods = mods.map(mod => ({
        ...mod,
        // Ensure engine is an object - handle both string (from DB) and object (from memory) cases
        engine:
          typeof mod.engine === 'string' ? JSON.parse(mod.engine) : mod.engine,
        // Add the group field required by the backend
        group: mod.folder_id || 'none',
        // Ensure timestamps are properly formatted
        last_played: mod.last_played
          ? Math.floor(Number(mod.last_played))
          : null,
        date_added: mod.date_added ? Math.floor(Number(mod.date_added)) : null,
        // Convert save_terminal_output to boolean (SQLite stores as 0/1)
        save_terminal_output: Boolean(mod.save_terminal_output),
      }))

      // Send mods to the backend
      await invoke('sync_mods_from_database', { modsData: formattedMods })
      dbConsole.log('Successfully synced mods directly with backend')
    } catch (error) {
      dbConsole.error('Failed to sync mods directly with backend:', error)
      // Don't throw to avoid disrupting the flow
    }
  }

  /**
   * Completely erase the database
   * This drops all tables and removes the schema
   */
  public async clearDatabase(): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }

    return withDatabaseLock(
      async db => {
        try {
          // Drop mods table
          await db.execute('DROP TABLE IF EXISTS mods')
          dbConsole.log('Dropped mods table')

          // Drop folders table
          await db.execute('DROP TABLE IF EXISTS folders')
          dbConsole.log('Dropped folders table')

          // Drop engine_mod_profiles table
          await db.execute('DROP TABLE IF EXISTS engine_mod_profiles')
          dbConsole.log('Dropped engine_mod_profiles table')

          // Drop migrations table
          await db.execute('DROP TABLE IF EXISTS migrations')
          dbConsole.log('Dropped migrations table')

          // Sync empty state to backend
          await invoke('remove_all_mods_command')
          dbConsole.log('Synced empty state to backend')

          dbConsole.log('Database completely erased successfully')
          window.location.reload()
        } catch (error) {
          dbConsole.error('Failed to completely erase database:', error)
          throw error
        }
      },
      true,
      'clearDatabase'
    )
  }

  /**
   * Helper method to create an empty mod
   */
  public createEmptyMod(): Mod {
    return {
      id: uuidv4(),
      name: '',
      path: '',
      display_order: 0,
      engine: DEFAULT_ENGINE,
      save_terminal_output: false,
    }
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
      display_order: 0,
    }
  }

  /**
   * Get a mod by its path
   * @param path The filesystem path of the mod
   * @returns The mod if found, null otherwise
   */
  public async getModByPath(path: string): Promise<Mod | null> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }

    return withDatabaseLock(
      async db => {
        const results = await db.select('SELECT * FROM mods WHERE path = ?', [
          path,
        ])

        if (results.length === 0) {
          return null
        }

        const mod = results[0]

        return {
          ...mod,
          // Parse engine field from JSON string if needed
          engine:
            typeof mod.engine === 'string'
              ? JSON.parse(mod.engine)
              : mod.engine,
        } as Mod
      },
      false,
      'getModByPath'
    ).catch(error => {
      dbConsole.error('Failed to get mod by path:', error)
      throw error
    })
  }
  /**
   * Get a mod by executable path
   * @param executablePath The filesystem path of the mod's executable
   * @return The mod if found, null otherwise
   */
  public async getModByExecutablePath(
    executablePath: string
  ): Promise<Mod | null> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }

    return withDatabaseLock(
      async db => {
        const results = await db.select(
          'SELECT * FROM mods WHERE executable_path = ?',
          [executablePath]
        )

        if (results.length === 0) {
          return null
        }

        const mod = results[0]

        return {
          ...mod,
          // Parse engine field from JSON string if needed
          engine:
            typeof mod.engine === 'string'
              ? JSON.parse(mod.engine)
              : mod.engine,
        } as Mod
      },
      false,
      'getModByExecutablePath'
    ).catch(error => {
      dbConsole.error('Failed to get mod by executable path:', error)
      throw error
    })
  }

  public async getProfilesByParentMod(
    parentModId: string
  ): Promise<EngineModProfile[]> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }

    return withDatabaseLock(
      async db => {
        const profiles = await db.select(
          'SELECT * FROM engine_mod_profiles WHERE parent_mod_id = ? ORDER BY created_at DESC',
          [parentModId]
        )
        return profiles.map((profile: EngineModProfile) => ({
          ...profile,
          mod_states: profile.mod_states,
        }))
      },
      false,
      'getProfilesByParentMod'
    ).catch(error => {
      dbConsole.error('Failed to get profiles by parent mod:', error)
      throw error
    })
  }

  public async saveProfile(profile: EngineModProfile): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }

    return withDatabaseLock(
      async db => {
        const now = Math.floor(Date.now() / 1000)
        await db.execute(
          `INSERT OR REPLACE INTO engine_mod_profiles 
         (id, name, icon_data, parent_mod_id, mod_states, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?)`,
          [
            profile.id,
            profile.name,
            profile.icon_data || null,
            profile.parent_mod_id,
            JSON.stringify(profile.mod_states),
            profile.created_at || now,
            now,
          ]
        )
        dbConsole.log(
          `Saved profile ${profile.name} for mod ${profile.parent_mod_id}`
        )
      },
      true,
      'saveProfile'
    ).catch(error => {
      dbConsole.error('Failed to save profile:', error)
      throw error
    })
  }

  public async deleteProfile(profileId: string): Promise<void> {
    if (!this.db) {
      throw new Error('Database not initialized')
    }

    return withDatabaseLock(
      async db => {
        await db.execute('DELETE FROM engine_mod_profiles WHERE id = ?', [
          profileId,
        ])
        dbConsole.log(`Deleted profile ${profileId}`)
      },
      true,
      'deleteProfile'
    ).catch(error => {
      dbConsole.error('Failed to delete profile:', error)
      throw error
    })
  }

  public async createEmptyProfile(
    parentModId: string
  ): Promise<EngineModProfile> {
    return {
      id: uuidv4(),
      name: 'New Profile',
      parent_mod_id: parentModId,
      mod_states: {},
      created_at: Math.floor(Date.now() / 1000),
      updated_at: Math.floor(Date.now() / 1000),
    }
  }

  private async _runMigrations(): Promise<void> {
    // Create migrations table if it doesn't exist
    if (!this.db) {
      throw new Error('Database not initialized')
    }
    try {
      await this.db.execute(
        `CREATE TABLE IF NOT EXISTS migrations (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          name TEXT UNIQUE NOT NULL,
          applied_at INTEGER NOT NULL
        )`
      )
    } catch (error) {
      dbConsole.error('Failed to create migrations table:', error)
      throw error
    }
    // Define migrations
    const migrations: Array<{
      name: string
      up: (db: {
        execute: (query: string) => unknown
        select: (query: string) => Promise<unknown[]>
      }) => Promise<void>
      down?: (db: {
        execute: (query: string) => unknown
        select: (query: string) => Promise<unknown[]>
      }) => Promise<void>
    }> = [
      {
        name: 'move_engine_data_to_engine',
        up: async (db: {
          execute: (query: string) => unknown
          select: (query: string) => Promise<unknown[]>
        }) => {
          // Moves engine_data to engine column
          // Also removes the engine_type column
          try {
            // First check if engine_data column exists
            const tableInfo = (await db.select(`PRAGMA table_info(mods)`)) as {
              name: string
            }[]
            const columns = tableInfo.map((col: { name: string }) => col.name)

            if (columns.includes('engine_data')) {
              dbConsole.log(
                'Found engine_data column, migrating to engine column'
              )

              // Add the engine column if it doesn't exist
              if (!columns.includes('engine')) {
                try {
                  await db.execute(`ALTER TABLE mods ADD COLUMN engine TEXT`)
                  dbConsole.log('Added engine column')
                } catch (error) {
                  dbConsole.error('Failed to add engine column:', error)
                  throw error
                }
              }

              // Copy data from engine_data to engine
              try {
                await db.execute(
                  `UPDATE mods SET engine = engine_data WHERE engine_data IS NOT NULL`
                )
                dbConsole.log('Copied engine_data to engine')
              } catch (error) {
                dbConsole.error('Failed to copy engine_data to engine:', error)
                throw error
              }

              // Now recreate the table without the old columns
              // SQLite doesn't support DROP COLUMN, so we need to recreate the table
              dbConsole.log('Recreating mods table to remove old columns...')

              // Create new table with current schema (without engine_data and engine_type)
              try {
                await db.execute(`
                  CREATE TABLE IF NOT EXISTS mods_new (
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
                    engine TEXT,
                    display_order INTEGER DEFAULT 0,
                    folder_id TEXT,
                    display_order_in_folder INTEGER DEFAULT 0,
                    last_played INTEGER,
                    date_added INTEGER,
                    save_terminal_output BOOLEAN DEFAULT 0
                  )
                `)
                dbConsole.log('Created new mods table')
              } catch (error) {
                dbConsole.error('Failed to create new mods table:', error)
                throw error
              }

              // Copy data to new table (excluding engine_data and engine_type)
              try {
                await db.execute(`
                  INSERT INTO mods_new (
                    id, name, path, executable_path, icon_data, banner_data, logo_data, logo_position,
                    version, description, engine, display_order, folder_id,
                    display_order_in_folder, last_played, date_added, save_terminal_output
                  )
                  SELECT 
                    id, name, path, executable_path, icon_data, banner_data, logo_data, logo_position,
                    version, description, engine, display_order, folder_id,
                    display_order_in_folder, last_played, date_added, save_terminal_output
                  FROM mods
                `)
                dbConsole.log('Copied data to new table')
              } catch (error) {
                dbConsole.error('Failed to copy data to new table:', error)
                // Clean up the new table if data copy failed
                try {
                  await db.execute(`DROP TABLE IF EXISTS mods_new`)
                } catch (cleanupError) {
                  dbConsole.error('Failed to clean up new table:', cleanupError)
                }
                throw error
              }

              // Drop old table and rename new one
              try {
                await db.execute(`DROP TABLE mods`)
                dbConsole.log('Dropped old mods table')
              } catch (error) {
                dbConsole.error('Failed to drop old mods table:', error)
                throw error
              }

              try {
                await db.execute(`ALTER TABLE mods_new RENAME TO mods`)
                dbConsole.log('Renamed new table to mods')
              } catch (error) {
                dbConsole.error('Failed to rename new table:', error)
                // Try to restore the old table name if possible
                try {
                  await db.execute(`ALTER TABLE mods_new RENAME TO mods_backup`)
                  dbConsole.warn(
                    'Renamed failed table to mods_backup for manual recovery'
                  )
                } catch (restoreError) {
                  dbConsole.error(
                    'Failed to create backup table:',
                    restoreError
                  )
                }
                throw error
              }

              dbConsole.log('Migration completed - old columns removed')
            } else {
              dbConsole.log(
                'engine_data column not found, migration not needed'
              )
            }
          } catch (error) {
            dbConsole.error('Error in migration:', error)
            throw error
          }
        },
      },
    ]
    // Run migrations
    for (const migration of migrations) {
      dbConsole.log(`Checking migration: ${migration.name}`)

      // Check if migration has already been applied
      const result = await this.db.select(
        'SELECT COUNT(*) as count FROM migrations WHERE name = ?',
        [migration.name]
      )

      // If the result is empty, count is 0
      const count = result && result.length > 0 ? result[0].count : 0
      dbConsole.log(`Migration ${migration.name} count: ${count}`)

      if (count === 0) {
        // Migration has not been applied, run the "up" function
        try {
          dbConsole.log(`Applying migration: ${migration.name}`)
          await migration.up(this.db)

          // Record the migration in the migrations table
          await this.db.execute(
            `INSERT INTO migrations (name, applied_at) VALUES (?, ?)`,
            [migration.name, Math.floor(Date.now() / 1000)]
          )

          dbConsole.log(`Migration ${migration.name} applied successfully`)
        } catch (error) {
          dbConsole.error(`Failed to apply migration ${migration.name}:`, error)
          throw error
        }

        // If the migration has a "down" function, just store it for now
        if (migration.down) {
          if (!this.migrationDowns) {
            this.migrationDowns = []
          }
          this.migrationDowns.push(migration.down)
        }
      } else {
        dbConsole.log(`Migration ${migration.name} already applied, skipping`)
      }
    }
  }
}
