import { invoke } from '@tauri-apps/api/core'
import {
  notificationService,
  OngoingNotificationResult,
} from '@services/notificationService'
import { StoreService } from './storeService'
import { downloadState, downloadingMods } from '../stores/downloadState'
import { formatEngineName } from '@utils/index'

// Map to track mod IDs to download IDs
const modIdToDownloadIdMap = new Map<number, string>()

// Define an interface for the folder existence check result
export interface FolderExistsResult {
  showFolderExistsDialog: boolean
  modName: string
  continueDownload: () => Promise<OperationResult | DownloadModResult>
  updateMod?: () => Promise<OperationResult | DownloadModResult>
}

export interface ModpackInfo {
  mod: GameBananaMod
  type: string | null
  compatibleEngines: any[]
}

// Define return type interfaces for clarity
export interface FileSelectionResult {
  showFileSelector: boolean
  files: any[]
  alternateFileSources: any[]
  mod: GameBananaMod
}

export interface EngineSelectionResult {
  showEngineSelectDialog: boolean
  modpackInfo: ModpackInfo
}

export interface ModTypeSelectionResult {
  showModTypeModal: boolean
  mod: GameBananaMod
}

export interface OperationResult {
  success: boolean
  error?: string
  modInfo?: any
}

export interface DeepLinkEngineSelectionResult {
  showEngineSelectDialog: boolean
  compatibleEngines: any[]
  modpackType: string
  modName: string
  downloadUrl: string
  modId: number
  modelType: string
}

export type DownloadModResult =
  | FileSelectionResult
  | EngineSelectionResult
  | ModTypeSelectionResult
  | OperationResult
  | FolderExistsResult

class GbConsole {
  private static style = 'color: #ff7; font-weight: bold;'
  private static errorStyle = 'color: #f00; font-weight: bold;'
  private static warnStyle = 'color: #ff0; font-weight: bold;'
  private static infoStyle = 'color: #0ff;'
  private static debugStyle = 'color: #ffd; font-weight: bold;'

  static log(message: string, ...args: any[]) {
    console.log(`%c[GameBanana] ${message}`, this.style, ...args)
  }

  static error(message: string, ...args: any[]) {
    console.error(`%c[GameBanana] ${message}`, this.errorStyle, ...args)
  }

  static warn(message: string, ...args: any[]) {
    console.warn(`%c[GameBanana] ${message}`, this.warnStyle, ...args)
  }

  static info(message: string, ...args: any[]) {
    console.info(`%c[GameBanana] ${message}`, this.infoStyle, ...args)
  }

  static debug(message: string, ...args: any[]) {
    console.debug(`%c[GameBanana] ${message}`, this.debugStyle, ...args)
  }
}
export class GameBananaService {
  private static instance: GameBananaService
  private pendingDownloadNotification: OngoingNotificationResult | null = null

  private constructor() {}

  /**
   * Get the singleton instance of GameBananaService
   */
  public static getInstance(): GameBananaService {
    if (!GameBananaService.instance) {
      GameBananaService.instance = new GameBananaService()
    }
    return GameBananaService.instance
  }

  /**
   * Get installation location from settings
   */
  public async getInstallLocation(): Promise<string | null> {
    try {
      const storeService = StoreService.getInstance()
      return await storeService.getSetting('installLocation')
    } catch (error) {
      GbConsole.warn('Could not get install location from settings:', error)
      return null
    }
  }

  /**
   * Check if a mod folder already exists before downloading
   */
  public async checkModFolderExists(modName: string): Promise<boolean> {
    try {
      // Get the install location from settings
      let installLocation: string | null = null
      try {
        installLocation = await this.getInstallLocation()
      } catch (error) {
        GbConsole.warn('Could not get install location from settings:', error)
      }

      // Call the backend command to check if the folder exists
      return await invoke<boolean>('check_mod_folder_exists', {
        name: modName,
        installLocation,
      })
    } catch (error) {
      GbConsole.error('Error checking if mod folder exists:', error)
      return false // Assume it doesn't exist in case of error
    }
  }

  /**
   * Determine if a mod is a modpack based on its category
   */
  public determineIfModpack(
    mod: GameBananaMod,
    selectedModType?: string
  ): boolean {
    // Check if we're in a modpack tab
    if (
      selectedModType &&
      ['psychModpacks', 'vsliceModpacks', 'codenameModpacks'].includes(
        selectedModType
      )
    ) {
      return true
    }

    // Check mod category if available
    if (mod.category_name) {
      const lowerCaseCategoryName = mod.category_name.toLowerCase()
      if (lowerCaseCategoryName.includes('executables')) return false
      if (lowerCaseCategoryName.includes('psych')) return true
      if (lowerCaseCategoryName.includes('v-slice')) return true
      if (lowerCaseCategoryName.includes('codename')) return true
    }

    return false
  }

  /**
   * Determine modpack type (psych, vanilla/v-slice, codename, or null if not a modpack)
   */
  public determineModpackType(
    mod: GameBananaMod,
    selectedModType?: string
  ): string | null {
    // First check the current tab
    if (selectedModType === 'psychModpacks') return 'psych'
    if (selectedModType === 'vsliceModpacks') return 'vanilla'
    if (selectedModType === 'codenameModpacks') return 'codename'

    // Check mod category if available
    if (mod.category_name) {
      const lowerCaseCategoryName = mod.category_name.toLowerCase()
      if (lowerCaseCategoryName.includes('psych')) return 'psych'
      if (lowerCaseCategoryName.includes('v-slice')) return 'vanilla'
      if (lowerCaseCategoryName.includes('codename')) return 'codename'
    }

    return null
  }
  /**
   * Get mods folder path for an engine mod
   */
  public getModsFolderPath(engineMod: any, isAddon: boolean = false): string {
    // Get base directory first in all cases
    const basePath = engineMod.path
    const executablePath = engineMod.executable_path || ''

    if (!basePath) return 'Unknown path'

    // Get parent directory of executable if it exists
    let baseDir = basePath
    if (executablePath) {
      // Extract the directory from the executable path
      const lastSlashIndex = executablePath.lastIndexOf('/')
      if (lastSlashIndex > 0) {
        baseDir = executablePath.substring(0, lastSlashIndex)
      } else {
        const lastBackslashIndex = executablePath.lastIndexOf('\\')
        if (lastBackslashIndex > 0) {
          baseDir = executablePath.substring(0, lastBackslashIndex)
        }
      }
    }

    // For Codename Engine addons, use addons folder instead of mods
    if (isAddon) {
      return `${baseDir}/addons`
    }

    // Then check if the engine has a specified custom mods folder path
    if (
      engineMod.engine &&
      engineMod.engine.mods_folder &&
      engineMod.engine.mods_folder_path
    ) {
      // Combine the base directory with the custom mods folder path
      return `${baseDir}/${engineMod.engine.mods_folder_path}`
    }

    // If no custom path specified, use default mods folder
    return `${baseDir}/mods`
  }

  /**
   * Get compatible engine mods for a modpack
   */
  public async getCompatibleEngineMods(
    engineType: string | null
  ): Promise<any[]> {
    if (!engineType) return []

    try {
      // Fetch all mods
      let allMods: any[] = []
      if (window.db && window.db.service) {
        allMods = await window.db.service.getAllMods()
      } else {
        allMods = await invoke<any[]>('get_mods')
      }

      // Filter mods by engine type
      return allMods.filter((mod: { engine: { engine_type: string } }) => {
        // Check engine.engine_type
        if (mod.engine && mod.engine.engine_type) {
          return (
            mod.engine.engine_type.toLowerCase() === engineType.toLowerCase()
          )
        }
        return false
      })
    } catch (error) {
      GbConsole.error('Failed to get compatible engine mods:', error)
      return []
    }
  }

  /**
   * Save a mod to the database
   */
  public async saveModToDatabase(mod: any): Promise<boolean> {
    try {
      // Check if DatabaseService is initialized
      if (!window.db || !window.db.service) {
        GbConsole.warn('Database service not initialized yet, cannot save mod')
        return false
      }

      GbConsole.log('Saving mod to database using DatabaseService:', mod)

      // Make sure the mod has an engine field required by the type
      if (!mod.engine) {
        mod.engine = {
          engine_type: 'unknown',
          engine_name: '',
          engine_icon: '',
          mods_folder: false,
          mods_folder_path: '',
        }
      }

      // Use the DatabaseService to save the mod
      await window.db.service.saveMod(mod)

      GbConsole.log('Mod saved successfully to database:', mod.name)
      return true
    } catch (error) {
      GbConsole.error('Failed to save mod to database:', error)
      return false
    }
  }

  /**
   * Download a mod from GameBanana
   */
  public async downloadMod(
    mod: GameBananaMod,
    selectedModType?: string
  ): Promise<DownloadModResult> {
    try {
      // Show loading notification
      this.pendingDownloadNotification = notificationService.downloadPreparing(
        mod.name
      )

      // First check if this mod has multiple download options
      const downloadInfo = await invoke<any>('get_mod_download_files_command', {
        modId: mod.id,
        modelType: mod.model_name || 'Mod',
      })
      // Check if there are multiple files or alternative downloads
      if (
        (downloadInfo._aFiles && downloadInfo._aFiles.length > 1) ||
        (downloadInfo._aAlternateFileSources &&
          downloadInfo._aAlternateFileSources.length > 0)
      ) {
        // Return information for file selector dialog
        this.dismissNotification()
        return {
          showFileSelector: true,
          files: downloadInfo._aFiles,
          alternateFileSources: downloadInfo._aAlternateFileSources || [],
          mod,
        }
      }

      // If there's only one file or no file selection needed, continue
      // Determine if this is a modpack
      const isModpack = selectedModType ? true : this.determineIfModpack(mod)
      const modpackType = selectedModType || this.determineModpackType(mod)
      GbConsole.log(
        `Mod "${mod.name}" is ${
          isModpack ? 'a modpack' : 'not a modpack'
        }, type: ${modpackType || 'unknown'}`
      )
      const modDownloadFile = downloadInfo._aFiles[0]
      const hasExecutable = modDownloadFile._bContainsExe

      if (isModpack && modpackType && !hasExecutable) {
        // Handle modpack download logic
        GbConsole.log('Modpack detected:', modpackType)
        const engineMods = await this.getCompatibleEngineMods(modpackType)

        if (engineMods.length === 0) {
          // No compatible engine found, show error
          this.dismissNotification()

          notificationService.modpackNoEngineError(
            mod.name,
            await formatEngineName(
              modpackType || 'whatever engine this mod needs'
            )
          )

          return {
            success: false,
            error: `No compatible ${await formatEngineName(modpackType)} installation found`,
          }
        } else {
          // Compatible engine found, proceed with showing engine selection dialog
          this.dismissNotification()

          return {
            showEngineSelectDialog: true,
            modpackInfo: {
              mod,
              type: modpackType,
              compatibleEngines: engineMods,
            },
          }
        }
      } else if (!isModpack && !modpackType && !hasExecutable) {
        // If we can't determine the mod type automatically, let the user choose
        this.dismissNotification()

        return {
          showModTypeModal: true,
          mod,
        }
      } else {
        // If it's only a modpack, proceed with normal download
        return await this.startDownload(mod)
      }
    } catch (error) {
      // Show error notification
      notificationService.downloadError(mod.name, String(error))

      this.dismissNotification()
      GbConsole.error('Failed to prepare mod download:', error)

      return {
        success: false,
        error: String(error),
      }
    }
  }

  /**
   * Start the download of a mod with GameBanana
   */
  public async startDownload(
    mod: GameBananaMod
  ): Promise<OperationResult | FolderExistsResult> {
    try {
      // Check if the mod folder already exists before downloading
      const folderExists = await this.checkModFolderExists(mod.name)

      GbConsole.log(
        `Checking if folder exists for mod "${mod.name}": ${folderExists}`
      )
      if (folderExists) {
        // Return a folder exists result with a function to continue the download
        return {
          showFolderExistsDialog: true,
          modName: mod.name,
          continueDownload: async () => {
            // Continue with the actual download
            return this.proceedWithDownload(mod, true)
          },
          updateMod: async () => {
            // Update the existing mod
            return this.proceedWithUpdate(mod)
          },
        }
      }

      // No folder exists, proceed with the download directly
      return await this.proceedWithDownload(mod)
    } catch (error) {
      // Show error notification
      notificationService.downloadError(mod.name, String(error))

      GbConsole.error('Failed to download mod:', error)
      return { success: false, error: String(error) }
    }
  }

  /**
   * Proceed with the actual download after checks
   */
  private async proceedWithDownload(
    mod: GameBananaMod,
    isRetry: boolean = false
  ): Promise<OperationResult> {
    try {
      // First notification for downloading
      this.pendingDownloadNotification?.dismiss()
      this.pendingDownloadNotification = notificationService.downloadProgress(
        mod.name
      )

      // Get the install location from settings
      let installLocation: string | null = null
      try {
        installLocation = await this.getInstallLocation()
      } catch (error) {
        GbConsole.warn('Could not get install location from settings:', error)
      }

      // If this is a retry (folder exists case), append a timestamp to make the folder unique
      let modName = mod.name
      if (isRetry) {
        modName = `${modName} (${Math.random().toString(16).slice(2, 6)})`
        GbConsole.log(`Folder already exists, using unique name: ${modName}`)
      }

      // Create a unique download entry and initialize tracking
      this.ensureUniqueDownload(mod.id, modName, mod.thumbnail_url)

      // Pass mod ID along with URL, name, and install location
      const result = await invoke<string>('download_gamebanana_mod_command', {
        url: mod.download_url,
        name: modName,
        modId: mod.id,
        modelType: mod.model_name || 'Mod',
        installLocation,
      })

      // Process the result
      return await this.processDownloadResult(result, { ...mod, name: modName })
    } catch (error) {
      notificationService.downloadError(mod.name, String(error))
      this.dismissNotification()

      GbConsole.error('Failed to download mod:', error)
      return { success: false, error: String(error) }
    }
  }

  /**
   * Process the download result and save to database
   */
  private async processDownloadResult(
    result: string,
    mod: GameBananaMod
  ): Promise<OperationResult> {
    try {
      let modInfo: any
      let modPath: string

      try {
        // Try to parse as JSON first
        const parsed = JSON.parse(result)
        modPath = parsed.path
        modInfo = parsed.mod_info
      } catch {
        // If parsing fails, assume it's just the path string
        modPath = result
        // Get mod info directly from the backend
        const allMods = await invoke<any[]>('get_mods')
        modInfo = allMods.find(m => m.path === modPath)

        // If we still don't have mod info, create a basic one
        if (!modInfo) {
          modInfo = {
            id: crypto.randomUUID(),
            name: mod.name,
            path: modPath,
            executable_path: null,
            icon_data: null,
            banner_data: mod.thumbnail_url,
            version: mod.version || null,
            engine_type: null,
          }
        }
      }

      // Save the mod to the database
      if (modInfo) {
        await this.saveModToDatabase(modInfo)
      }

      // Show success notification
      notificationService.downloadSuccess(mod.name)
      this.dismissNotification()

      // Trigger the refresh event to update the mod list
      const refreshEvent = new CustomEvent('refresh-mods')
      window.dispatchEvent(refreshEvent)

      return { success: true, modInfo }
    } catch (error) {
      // Show error notification
      notificationService.downloadError(mod.name, String(error))
      this.dismissNotification()
      GbConsole.error('Failed to process download result:', error)
      return { success: false, error: String(error) }
    }
  }

  /**
   * Process the update result and update database entry with new version
   */
  private async processUpdateResult(
    result: string,
    mod: GameBananaMod
  ): Promise<OperationResult> {
    try {
      let modInfo: any
      let modPath: string

      try {
        // Try to parse as JSON first
        const parsed = JSON.parse(result)
        modPath = parsed.path
        modInfo = parsed.mod_info
      } catch {
        // If parsing fails, assume it's just the path string
        modPath = result
        // Get mod info directly from the backend
        const allMods = await invoke<any[]>('get_mods')
        modInfo = allMods.find(m => m.path === modPath)

        // If we still don't have mod info, create a basic one
        if (!modInfo) {
          modInfo = {
            id: crypto.randomUUID(),
            name: mod.name,
            path: modPath,
            executable_path: null,
            icon_data: null,
            banner_data: mod.thumbnail_url,
            version: mod.version || null,
            engine_type: null,
          }
        }
      }

      // For updates, fetch the latest mod information from GameBanana to get new version
      try {
        GbConsole.log(
          `Fetching updated mod info for ${mod.name} (ID: ${mod.id})`
        )
        const updatedModInfo = await invoke<any>('get_mod_info_command', {
          modId: mod.id,
          modelType: mod.model_name || 'Mod',
        })

        if (updatedModInfo && modInfo) {
          // Update the mod info with new version and other updated details
          modInfo.version =
            updatedModInfo._sVersion ||
            updatedModInfo.version ||
            modInfo.version

          // Update last modified timestamp
          modInfo.date_modified = Date.now()

          GbConsole.log(`Updated mod info - New version: ${modInfo.version}`)
        }
      } catch (error) {
        GbConsole.warn(
          'Failed to fetch updated mod info from GameBanana, using existing info:',
          error
        )
      }

      // Save the updated mod to the database
      if (modInfo) {
        await this.saveModToDatabase(modInfo)
      }

      // Show success notification
      notificationService.updateSuccess(
        mod.name,
        'Updated to version ' + modInfo.version
      )
      this.dismissNotification()

      // Trigger the refresh event to update the mod list
      const refreshEvent = new CustomEvent('refresh-mods')
      window.dispatchEvent(refreshEvent)

      return { success: true, modInfo }
    } catch (error) {
      // Show error notification
      notificationService.updateError(mod.name, String(error))
      this.dismissNotification()
      GbConsole.error('Failed to process update result:', error)
      return { success: false, error: String(error) }
    }
  }

  /**
   * Download a specific file from a mod
   */
  public async downloadModFile(
    mod: GameBananaMod,
    selectedFile: any
  ): Promise<DownloadModResult> {
    try {
      // Create a new notification for the download process
      this.pendingDownloadNotification?.dismiss()
      this.pendingDownloadNotification = notificationService.downloadProgress(
        mod.name
      )

      // Check if the mod folder already exists before downloading
      const folderExists = await this.checkModFolderExists(mod.name)

      GbConsole.log(
        `Checking if folder exists for mod "${mod.name}": ${folderExists}`
      )

      if (folderExists) {
        // Dismiss the current notification
        this.dismissNotification() // Return a folder exists result with a function to continue the download
        return {
          showFolderExistsDialog: true,
          modName: mod.name,
          continueDownload: async () => {
            // Continue with the actual download using the selected file
            return this.proceedWithSpecificFileDownload(mod, selectedFile, true)
          },
          updateMod: async () => {
            // Update the existing mod using the selected file
            return this.proceedWithSpecificFileUpdate(mod, selectedFile)
          },
        }
      }

      // No folder exists, proceed with the download directly
      return await this.proceedWithSpecificFileDownload(
        mod,
        selectedFile,
        false
      )
    } catch (error) {
      // Show error notification
      notificationService.downloadError(mod.name, String(error))

      this.dismissNotification()
      GbConsole.error('Failed to download mod:', error)

      return { success: false, error: String(error) }
    }
  }

  /**
   * Proceed with downloading a specific file after folder existence checks
   */
  private async proceedWithSpecificFileDownload(
    mod: GameBananaMod,
    selectedFile: any,
    isRetry: boolean = false
  ): Promise<DownloadModResult> {
    try {
      // Create a new notification for the download process
      this.pendingDownloadNotification = notificationService.downloadProgress(
        mod.name
      )

      // Check if the selected file contains an executable
      if (selectedFile._bContainsExe) {
        GbConsole.log(
          'Selected file contains an executable, treating as standard mod'
        )

        // Update notification to downloading
        this.updateNotification(`Downloading "${mod.name}"...`)

        // Get the install location from settings
        let installLocation: string | null = null
        try {
          installLocation = await this.getInstallLocation()
        } catch (error) {
          GbConsole.warn('Could not get install location from settings:', error)
        }

        GbConsole.log('Using selected file URL:', selectedFile._sDownloadUrl)
        GbConsole.log('Using installation location:', installLocation)

        // If this is a retry (folder exists case), append a timestamp to make the folder unique
        let modName = mod.name
        if (isRetry) {
          modName = `${modName} (${Math.random().toString(16).slice(2, 6)})`
          GbConsole.log(`Folder already exists, using unique name: ${modName}`)
        }

        // Ensure no duplicate download entries exist
        this.ensureUniqueDownload(mod.id, modName, mod.thumbnail_url)

        // Call backend to download using the specific file URL
        const result = await invoke<string>('download_gamebanana_mod_command', {
          url: selectedFile._sDownloadUrl,
          name: modName,
          modId: mod.id,
          modelType: mod.model_name || 'Mod',
          installLocation,
        })

        // Process the result
        return await this.processDownloadResult(result, {
          ...mod,
          name: modName,
        })
      } // If file doesn't contain an executable, check if this is a modpack
      const isModpack = this.determineIfModpack(mod)
      const modpackType = this.determineModpackType(mod)

      if (isModpack) {
        // Handle modpack download logic for selected file
        const engineMods = await this.getCompatibleEngineMods(modpackType)

        if (engineMods.length === 0) {
          // No compatible engine found, show error
          this.dismissNotification()

          // Show error notification
          notificationService.modpackNoEngineError(
            mod.name,
            await formatEngineName(
              modpackType || 'whatever engine this mod needs'
            )
          )

          return { success: false, error: 'No compatible engine found' }
        } else {
          // Compatible engine found, store the selected file URL for later use
          this.dismissNotification()

          return {
            showEngineSelectDialog: true,
            modpackInfo: {
              mod: { ...mod, download_url: selectedFile._sDownloadUrl }, // Override with selected URL
              type: modpackType,
              compatibleEngines: engineMods,
            },
          }
        }
      }

      // If not a modpack and doesn't contain an executable, proceed with standard download
      this.updateNotification(`Downloading "${mod.name}"...`)

      // Get the install location from settings
      let installLocation: string | null = null
      try {
        installLocation = await this.getInstallLocation()
      } catch (error) {
        GbConsole.warn('Could not get install location from settings:', error)
      }

      GbConsole.log('Using selected file URL:', selectedFile._sDownloadUrl)
      GbConsole.log('Using installation location:', installLocation)

      // If this is a retry (folder exists case), append a timestamp to make the folder unique
      let modName = mod.name
      if (isRetry) {
        modName = `${modName} (${mod.id.toString(16).slice(0, 4)})`
        GbConsole.log(`Folder already exists, using unique name: ${modName}`)
      }

      // Ensure no duplicate download entries exist
      this.ensureUniqueDownload(mod.id, modName, mod.thumbnail_url)

      // Call backend to download using the specific file URL
      const result = await invoke<string>('download_gamebanana_mod_command', {
        url: selectedFile._sDownloadUrl,
        name: modName,
        modId: mod.id,
        modelType: mod.model_name || 'Mod',
        installLocation,
      })

      // Process the result
      return await this.processDownloadResult(result, { ...mod, name: modName })
    } catch (error) {
      // Show error notification
      notificationService.downloadError(mod.name, String(error))

      this.dismissNotification()
      GbConsole.error('Failed to download mod:', error)

      return { success: false, error: String(error) }
    }
  }
  /**
   * Proceed with updating an existing mod
   */
  private async proceedWithUpdate(
    mod: GameBananaMod
  ): Promise<OperationResult> {
    try {
      // Check if this is a modpack - if so, disable updating
      const isModpack = this.determineIfModpack(mod)
      if (isModpack) {
        GbConsole.log(`Mod "${mod.name}" is a modpack - updates not supported`)
        return {
          success: false,
          error: 'Updates not available for modpacks... yet',
        }
      }

      // Show notification for updating
      this.pendingDownloadNotification?.dismiss()
      this.pendingDownloadNotification = notificationService.updateProgress(
        mod.name
      )

      // Get the install location from settings
      let installLocation: string | null = null
      try {
        installLocation = await this.getInstallLocation()
      } catch (error) {
        GbConsole.warn('Could not get install location from settings:', error)
      }

      GbConsole.log(`Updating mod "${mod.name}" in place`)

      // Ensure no duplicate download entries exist
      this.ensureUniqueDownload(mod.id, mod.name, mod.thumbnail_url) // Call backend to update the mod using the update command
      const result = await invoke<string>('update_gamebanana_mod_command', {
        url: mod.download_url,
        name: mod.name,
        modId: mod.id,
        modelType: mod.model_name || 'Mod',
        installLocation,
      })

      // Process the update result with database entry update
      return await this.processUpdateResult(result, mod)
    } catch (error) {
      // Show error notification
      this.dismissNotification()
      notificationService.updateError(mod.name, String(error))

      GbConsole.error('Failed to update mod:', error)
      return { success: false, error: String(error) }
    }
  }
  /**
   * Proceed with updating an existing mod using a specific file
   */
  private async proceedWithSpecificFileUpdate(
    mod: GameBananaMod,
    selectedFile: any
  ): Promise<OperationResult> {
    try {
      // Check if this is a modpack - if so, disable updating
      const isModpack = this.determineIfModpack(mod)
      if (isModpack) {
        GbConsole.log(`Mod "${mod.name}" is a modpack - updates not supported`)
        notificationService.updateNotAvailable(mod.name)
        return { success: false, error: 'Updates not available for modpacks' }
      }

      // Show notification for updating
      this.pendingDownloadNotification?.dismiss()
      this.pendingDownloadNotification = notificationService.updatePreparing(
        mod.name
      )

      // Get the install location from settings
      let installLocation: string | null = null
      try {
        installLocation = await this.getInstallLocation()
      } catch (error) {
        GbConsole.warn('Could not get install location from settings:', error)
      }

      GbConsole.log(`Updating mod "${mod.name}" in place using selected file`)
      GbConsole.log('Using selected file URL:', selectedFile._sDownloadUrl)

      // Ensure no duplicate download entries exist
      this.ensureUniqueDownload(mod.id, mod.name, mod.thumbnail_url) // Call backend to update the mod using the update command with specific file
      const result = await invoke<string>('update_gamebanana_mod_command', {
        url: selectedFile._sDownloadUrl,
        name: mod.name,
        modId: mod.id,
        modelType: mod.model_name || 'Mod',
        installLocation,
      })

      // Process the update result with database entry update
      return await this.processUpdateResult(result, mod)
    } catch (error) {
      // Show error notification
      notificationService.updateError(mod.name, String(error))
      this.dismissNotification()

      GbConsole.error('Failed to update mod:', error)
      return { success: false, error: String(error) }
    }
  }
  /**
   * Download a modpack for a selected engine
   */
  public async downloadModpackForEngine(
    modpackInfo: ModpackInfo,
    engineMod: any
  ): Promise<OperationResult> {
    if (!modpackInfo || !engineMod) {
      return { success: false, error: 'Missing modpack info or engine' }
    }

    try {
      // Create a new notification for the download process
      this.pendingDownloadNotification = notificationService.downloadProgress(
        modpackInfo.mod.name
      )

      const mod = modpackInfo.mod
      // Store mod ID to ensure we can cleanup tracking afterward
      const modId = mod.id

      // Check if this is a Codename Engine addon installation
      const isAddon = engineMod.isAddon || false

      // Get the installation path for the selected engine's mods folder
      const modsFolderPath = this.getModsFolderPath(engineMod, isAddon)
      if (!modsFolderPath) {
        throw new Error('Could not determine mods folder path')
      }

      // Check if we already have a specific download URL from the file selector
      // This happens when the user first selected a file, then selected an engine
      let downloadUrl = mod.download_url

      GbConsole.log(
        `Starting download for "${mod.name}" modpack to ${modsFolderPath}${isAddon ? ' (as addon)' : ''}`
      )

      // Fix URL if needed
      if (downloadUrl.startsWith('//')) {
        downloadUrl = 'https:' + downloadUrl
      } else if (!downloadUrl.includes('://')) {
        downloadUrl = 'https://' + downloadUrl
      }

      // Ensure no duplicate download entries exist
      const downloadId = this.ensureUniqueDownload(
        modId,
        mod.name,
        mod.thumbnail_url
      )

      // Use the default download URL for direct download
      const result = await invoke<string>('download_gamebanana_mod_command', {
        url: downloadUrl,
        name: mod.name,
        modId: mod.id,
        modelType: mod.model_name || 'Mod',
        installLocation: modsFolderPath,
      })

      GbConsole.log('Download result:', result)

      // Dismiss the loading notification
      this.dismissNotification()

      // Show success notification
      notificationService.downloadSuccess(mod.name)
      this.dismissNotification()

      // Manual cleanup of download tracking entry
      downloadState.completeDownload(downloadId)
      modIdToDownloadIdMap.delete(modId)

      // No need to refresh the main mods list as this mod is only in the engine's mods folder
      // and will be displayed in the engine's mod list

      return { success: true }
    } catch (error) {
      // Show error notification
      notificationService.downloadError(modpackInfo.mod.name, String(error))

      this.dismissNotification()
      GbConsole.error('Failed to install modpack:', error)

      return { success: false, error: String(error) }
    }
  }

  /**
   * Download an engine directly
   */
  public async downloadEngine(
    engineType: string
  ): Promise<OperationResult | FolderExistsResult> {
    try {
      // Check if the engine folder already exists before downloading
      // Determine the folder name based on engine type
      const engineName = await formatEngineName(engineType)
      const folderExists = await this.checkModFolderExists(engineName)

      if (folderExists) {
        // Return a folder exists result with a function to continue the download
        return {
          showFolderExistsDialog: true,
          modName: engineName,
          continueDownload: async () => {
            // Continue with the actual download
            return this.proceedWithEngineDownload(engineType, true)
          },
        }
      }

      // No folder exists, proceed with the download directly
      return await this.proceedWithEngineDownload(engineType)
    } catch (error) {
      // Show error notification
      notificationService.downloadError(
        await formatEngineName(engineType),
        String(error)
      )

      GbConsole.error(`Failed to download ${engineType} engine:`, error)
      return { success: false, error: String(error) }
    }
  }

  /**
   * Proceed with the actual engine download after checks
   */
  private async proceedWithEngineDownload(
    engineType: string,
    isRetry: boolean = false
  ): Promise<OperationResult> {
    try {
      let engineName = await formatEngineName(engineType)
      // Show loading notification
      this.pendingDownloadNotification?.dismiss()
      this.pendingDownloadNotification =
        notificationService.downloadPreparing(engineName)

      // Get the install location from settings
      let installLocation: string | null = null
      try {
        if (window.db && window.db.service) {
          installLocation =
            await window.db.service.getSetting('installLocation')
        }
      } catch (error) {
        GbConsole.warn('Could not get install location from settings:', error)
      }

      // If this is a retry (folder exists case), append a timestamp to make the folder unique
      if (isRetry) {
        const timestamp = new Date().toISOString().replace(/[:.]/g, '-')
        engineName = `${engineName}_${timestamp}`
        GbConsole.log(`Folder already exists, using unique name: ${engineName}`)
      }

      GbConsole.log(
        `Downloading ${engineType} engine to ${
          installLocation || 'default location'
        } with name ${engineName}`
      )

      // Call backend to download the engine directly
      const result = await invoke<string>('download_engine_command', {
        engineType,
        installLocation,
        customName: isRetry ? engineName : undefined, // Only send custom name if it's a retry
      })

      // Process the result
      let modInfo: any

      try {
        // Try to parse as JSON
        const parsed = JSON.parse(result)
        modInfo = parsed.mod_info
      } catch {
        // If parsing fails, assume it's just the path string
        const modPath = result
        // Get mod info directly from the backend
        const allMods = await invoke<any[]>('get_mods')
        modInfo = allMods.find(m => m.path === modPath)
      }

      // Save the mod to the database
      if (modInfo) {
        await this.saveModToDatabase(modInfo)
      }

      // Dismiss loading notification
      this.dismissNotification()

      // Show success notification
      notificationService.downloadSuccess(engineName)
      this.dismissNotification()

      // Trigger the refresh event to update the mod list
      const refreshEvent = new CustomEvent('refresh-mods')
      window.dispatchEvent(refreshEvent)

      return { success: true, modInfo }
    } catch (error) {
      // Show error notification
      notificationService.downloadError(
        await formatEngineName(engineType),
        String(error)
      )

      // Dismiss any pending notification
      this.dismissNotification()
      GbConsole.error(`Failed to download ${engineType} engine:`, error)

      return { success: false, error: String(error) }
    }
  }

  /**
   * Download a mod from a deep link
   */
  public async downloadModFromDeepLink(
    downloadUrl: string,
    modId: number,
    modelType: string
  ): Promise<OperationResult | DeepLinkEngineSelectionResult> {
    try {
      // Show notification that download is being prepared
      this.pendingDownloadNotification?.dismiss()
      this.pendingDownloadNotification =
        notificationService.downloadPreparing('mod')

      // Get the mod info from GameBanana API to get the name and other details
      const modInfo = await invoke<any>('get_mod_info_command', {
        modId: modId,
        modelType: modelType || 'Mod',
      })

      if (!modInfo || !modInfo._sName) {
        throw new Error('Failed to fetch mod information from GameBanana')
      }

      const modName = modInfo._sName

      // Check if this is a modpack
      const isModpack = this.determineDeepLinkModpackStatus(modInfo)
      const modpackType = this.determineDeepLinkModpackType(modInfo)

      GbConsole.log('Mod analysis:', { isModpack, modpackType })

      // Update notification
      this.updateNotification(`Preparing to download "${modName}"...`)

      // If it's a modpack, check for compatible engines
      if (isModpack && modpackType) {
        // Get compatible engines for this modpack
        const engineMods = await this.getCompatibleEngineMods(modpackType)

        if (engineMods.length === 0) {
          // No compatible engine found
          this.dismissNotification()
          notificationService.modpackNoEngineError(
            modName,
            await formatEngineName(
              modpackType || 'whatever engine this mod needs'
            )
          )

          return { success: false, error: 'No compatible engine found' }
        } else {
          // Return engine selection info
          this.dismissNotification()

          return {
            showEngineSelectDialog: true,
            compatibleEngines: engineMods,
            modpackType,
            modName,
            downloadUrl,
            modId,
            modelType,
          }
        }
      }

      // If not a modpack, proceed with standard mod download
      // Get the install location from settings
      let installLocation: string | null = null
      try {
        const storeService = StoreService.getInstance()
        installLocation = await storeService.getSetting('installLocation')
      } catch (error) {
        GbConsole.warn('Could not get install location from settings:', error)
      }

      // Fix the URL
      let fixedUrl = downloadUrl
      if (fixedUrl.startsWith('//')) {
        fixedUrl = 'https:' + fixedUrl
      } else if (!fixedUrl.includes('://')) {
        fixedUrl = 'https://' + fixedUrl
      }

      GbConsole.log(`Starting download for "${modName}" from URL: ${fixedUrl}`)

      // IMPORTANT: Remove any existing download tracking for this mod ID to avoid duplicates
      const existingDownloadId = modIdToDownloadIdMap.get(modId)
      if (existingDownloadId) {
        GbConsole.log(
          `Found existing download tracking ID ${existingDownloadId} for mod ID ${modId}, cleaning up`
        )
        // Delete the existing download entry
        delete downloadingMods[existingDownloadId]
        modIdToDownloadIdMap.delete(modId)
      }

      // Create a new download entry to track progress
      const downloadId = downloadState.createDownload(
        modId,
        modName,
        `https://gamebanana.com/mods/embeddables/${modId}`
      )
      modIdToDownloadIdMap.set(modId, downloadId)

      GbConsole.log(
        `Created download tracking with ID ${downloadId} for modId ${modId}`
      )

      // Update the download status manually to ensure it shows up
      downloadState.updateDownloadProgress({
        id: downloadId,
        bytesDownloaded: 0,
        totalBytes: 100,
        percentage: 5,
        step: 'Preparing download...',
        isComplete: false,
        isError: false,
      })

      // Download the mod
      const result = await invoke<string>('download_gamebanana_mod_command', {
        url: fixedUrl,
        name: modName,
        modId,
        modelType: modelType || 'Mod',
        installLocation,
      })

      // Process the result
      let modInfoResult: any

      try {
        // Try to parse as JSON
        const parsed = JSON.parse(result)
        modInfoResult = parsed.mod_info
      } catch {
        // If parsing fails, assume it's just the path string
        const modPath = result
        // Get mod info directly from the backend
        const allMods = await invoke<any[]>('get_mods')
        modInfoResult = allMods.find(m => m.path === modPath)

        // If we still don't have mod info, create a basic one
        if (!modInfoResult) {
          modInfoResult = {
            id: crypto.randomUUID(),
            name: modName,
            path: result,
            executable_path: null,
            icon_data: null,
            banner_data: `https://gamebanana.com/mods/embeddables/${modId}`,
            version: modInfo._sVersion || null,
            engine_type: null,
          }
        }
      }

      // Save the mod to the database
      if (modInfoResult) {
        try {
          if (window.db && window.db.service) {
            await window.db.service.saveMod(modInfoResult)
          }
        } catch (error) {
          GbConsole.error('Failed to save mod to database:', error)
        }
      }

      // Update download status to complete
      if (downloadId) {
        GbConsole.log(`Manually completing download tracking for mod ${modId}`)
        downloadState.updateDownloadProgress({
          id: downloadId,
          bytesDownloaded: 100,
          totalBytes: 100,
          percentage: 100,
          step: 'Installation complete',
          isComplete: true,
          isError: false,
        })

        setTimeout(() => {
          downloadState.completeDownload(downloadId)
          modIdToDownloadIdMap.delete(modId)
        }, 2000)
      }

      // Dismiss notification
      this.dismissNotification()

      // Show success notification
      notificationService.downloadSuccess(modName)
      this.dismissNotification()

      // Trigger refresh event to update mod list
      const refreshEvent = new CustomEvent('refresh-mods')
      window.dispatchEvent(refreshEvent)

      return { success: true, modInfo: modInfoResult }
    } catch (error) {
      // Dismiss notification
      this.dismissNotification()

      // Show error notification
      notificationService.downloadError('mod', String(error))

      GbConsole.error('Failed to download mod from deep link:', error)
      return { success: false, error: String(error) }
    }
  }

  /**
   * Download a modpack from a deep link with selected engine
   */
  public async downloadDeepLinkModpackWithEngine(
    downloadUrl: string,
    modName: string,
    modId: number,
    modelType: string,
    engine: any
  ): Promise<OperationResult> {
    try {
      // Show loading notification
      this.pendingDownloadNotification =
        notificationService.downloadPreparing(modName)
      // Get the installation path for the selected engine's mods folder
      const modsFolderPath = this.getModsFolderPath(engine)
      if (!modsFolderPath) {
        throw new Error('Could not determine mods folder path')
      }

      GbConsole.log(
        `Starting download for "${modName}" modpack to ${modsFolderPath}`
      )

      // Fix the URL since it doesn't have the colon (or shouldn't have it)
      let fixedUrl = downloadUrl
      if (fixedUrl.startsWith('//')) {
        fixedUrl = 'https:' + fixedUrl
      } else if (!fixedUrl.includes('://')) {
        fixedUrl = 'https://' + fixedUrl
      }

      // Create a download entry to track progress
      const downloadId = downloadState.createDownload(
        modId,
        modName,
        `https://gamebanana.com/mods/embeddables/${modId}`
      )
      modIdToDownloadIdMap.set(modId, downloadId)

      // Call the backend to download the modpack to the engine's mods folder
      const result = await invoke<string>('download_gamebanana_mod_command', {
        url: fixedUrl,
        name: modName,
        modId: modId,
        modelType: modelType || 'Mod',
        installLocation: modsFolderPath,
      })

      GbConsole.log('Download result:', result)

      // Dismiss notification
      this.dismissNotification()

      // Show success notification
      notificationService.downloadSuccess(modName)
      this.dismissNotification()

      // Manual cleanup of download tracking entry
      if (downloadId) {
        GbConsole.log(
          `Manually completing download tracking for modpack ${modId}`
        )
        downloadState.completeDownload(downloadId)
        modIdToDownloadIdMap.delete(modId)
      }

      // No need to refresh the main mods list as this mod is only in the engine's mods folder
      // and will be displayed in the engine's mod list

      return { success: true }
    } catch (error) {
      // Dismiss notification
      this.dismissNotification()

      // Show error notification
      notificationService.downloadError(modName, String(error))
      this.dismissNotification()

      GbConsole.error('Failed to download modpack:', error)
      return { success: false, error: String(error) }
    }
  }

  // Utility functions for deep link modpack detection
  private determineDeepLinkModpackStatus(modInfo: any): boolean {
    // Check category ID
    if (modInfo._aCategory && modInfo._aCategory._idRow) {
      const categoryId = parseInt(modInfo._aCategory._idRow)
      const modpackCategoryIds = [28367, 29202, 34764] // Psych, V-Slice, Codename

      if (modpackCategoryIds.includes(categoryId)) {
        return true
      }
    }

    // Check tags
    if (modInfo._aTags && Array.isArray(modInfo._aTags)) {
      const modpackTags = ['psychmodpack', 'v-slicemodpack', 'codenamemodpack']

      for (const tag of modInfo._aTags) {
        if (
          typeof tag === 'string' &&
          modpackTags.some(mpt => tag.toLowerCase().includes(mpt))
        ) {
          return true
        }
      }
    }

    // Check description
    if (modInfo._sDescription) {
      const desc = modInfo._sDescription.toLowerCase()

      if (
        desc.includes('psych engine modpack') ||
        desc.includes('v-slice modpack') ||
        desc.includes('codename engine modpack')
      ) {
        return true
      }
    }

    return false
  }

  private determineDeepLinkModpackType(modInfo: any): string | null {
    // Check category ID first (most reliable)
    if (modInfo._aCategory && modInfo._aCategory._idRow) {
      const categoryId = parseInt(modInfo._aCategory._idRow)

      if (categoryId === 28367) return 'psych'
      if (categoryId === 29202) return 'vanilla'
      if (categoryId === 34764) return 'codename'
    }

    // Check tags next
    if (modInfo._aTags && Array.isArray(modInfo._aTags)) {
      for (const tag of modInfo._aTags) {
        if (typeof tag === 'string') {
          const tagLower = tag.toLowerCase()
          if (tagLower.includes('psych')) return 'psych'
          if (tagLower.includes('v-slice') || tagLower.includes('vslice'))
            return 'vanilla'
          if (tagLower.includes('codename')) return 'codename'
        }
      }
    }

    // Check description last (least reliable)
    if (modInfo._sDescription) {
      const desc = modInfo._sDescription.toLowerCase()

      if (desc.includes('psych engine')) return 'psych'
      if (desc.includes('v-slice')) return 'vanilla'
      if (desc.includes('codename engine')) return 'codename'
    }

    return null
  } // Helper methods for notification management
  private dismissNotification(): void {
    if (this.pendingDownloadNotification) {
      this.pendingDownloadNotification.dismiss()
      this.pendingDownloadNotification = null
    }
  }
  private updateNotification(message: string): void {
    if (this.pendingDownloadNotification) {
      this.pendingDownloadNotification.update({
        message: message,
      })
    }
  }

  /**
   * Helper method to ensure no duplicate download entries exist
   * Returns the download ID to use for tracking
   */
  private ensureUniqueDownload(
    modId: number,
    modName: string,
    thumbnailUrl?: string
  ): string {
    GbConsole.log(`Ensuring unique download for mod ID ${modId}`)

    // Clean up any existing download tracking for this mod ID
    const existingDownloadId = modIdToDownloadIdMap.get(modId)
    if (existingDownloadId) {
      GbConsole.log(
        `Found existing download tracking ID ${existingDownloadId} for mod ID ${modId}, cleaning up`
      )
      // Delete the existing download entry
      delete downloadingMods[existingDownloadId]
      modIdToDownloadIdMap.delete(modId)
    }

    // Create a new download entry
    const downloadId = downloadState.createDownload(
      modId,
      modName,
      thumbnailUrl || `https://gamebanana.com/mods/embeddables/${modId}`
    )
    modIdToDownloadIdMap.set(modId, downloadId)

    GbConsole.log(
      `Created download tracking with ID ${downloadId} for modId ${modId}`
    )

    // Initialize download state
    downloadState.updateDownloadProgress({
      id: downloadId,
      bytesDownloaded: 0,
      totalBytes: 100,
      percentage: 5,
      step: 'Preparing download...',
      isComplete: false,
      isError: false,
    })

    return downloadId
  }
}

// Initialize Tauri event listeners for tracking downloads
export function setupGameBananaEventListeners(): () => void {
  let removeDownloadStartedListener: (() => void) | undefined
  let removeDownloadProgressListener: (() => void) | undefined
  let removeDownloadFinishedListener: (() => void) | undefined
  let removeDownloadErrorListener: (() => void) | undefined

  // Import Tauri event API
  import('@tauri-apps/api/event').then(({ listen }) => {
    // Set up event listeners for downloads
    listen('download-started', (event: any) => {
      GbConsole.log('Download started:', event.payload)

      // Extract mod ID, supporting both modId and mod_id formats
      const modId =
        event.payload.modId ??
        event.payload.mod_id ??
        Math.floor(Math.random() * 1000000)
      const name = event.payload.name || 'Unknown Mod'

      // Check if we already have a download entry for this mod ID
      let downloadId = modIdToDownloadIdMap.get(modId)

      if (!downloadId) {
        // Create a new download entry and get its unique ID
        downloadId = downloadState.createDownload(
          modId,
          name,
          event.payload.thumbnailUrl || ''
        )
        // Store this mapping for future reference
        modIdToDownloadIdMap.set(modId, downloadId)
      }

      // Update with initial data
      downloadState.updateDownloadProgress({
        id: downloadId,
        bytesDownloaded: 0,
        totalBytes: event.payload.contentLength || 0,
        percentage: 0,
        step: 'Starting download...',
        isComplete: false,
        isError: false,
      })
    }).then((unsubscribe: () => void) => {
      removeDownloadStartedListener = unsubscribe
    })

    listen('download-progress', (event: any) => {
      GbConsole.log('Download progress:', event.payload)

      // Extract mod ID, supporting both modId and mod_id formats
      const modId = event.payload.modId ?? event.payload.mod_id

      // Skip events without mod IDs or necessary data
      if (modId === undefined) {
        GbConsole.warn(
          'Received download progress event with undefined modId and mod_id:',
          event.payload
        )
        return
      }

      // Get the download ID from our mapping
      const downloadId = modIdToDownloadIdMap.get(modId)
      if (!downloadId) {
        GbConsole.warn(
          `No download ID found for mod ID ${modId}, creating new entry on the fly`
        )
        // Create a new download entry on the fly if one doesn't exist
        const newDownloadId = downloadState.createDownload(
          modId,
          event.payload.name || 'Unknown Mod',
          ''
        )
        modIdToDownloadIdMap.set(modId, newDownloadId)

        // Update the newly created download with progress
        downloadState.updateDownloadProgress({
          id: newDownloadId,
          bytesDownloaded: event.payload.bytesDownloaded || 0,
          totalBytes: event.payload.totalBytes || 100,
          percentage: event.payload.percentage || 0,
          step: event.payload.step || 'Downloading...',
          isComplete: false,
          isError: false,
        })
        return
      }

      downloadState.updateDownloadProgress({
        id: downloadId,
        bytesDownloaded: event.payload.bytesDownloaded || 0,
        totalBytes: event.payload.totalBytes || 100,
        percentage: event.payload.percentage || 0,
        step: event.payload.step || 'Downloading...',
        isComplete: false,
        isError: false,
      })
    }).then((unsubscribe: () => void) => {
      removeDownloadProgressListener = unsubscribe
    })

    listen('download-finished', (event: any) => {
      GbConsole.log('Download finished:', event.payload)

      // Extract mod ID, supporting both modId and mod_id formats
      const modId = event.payload.modId ?? event.payload.mod_id

      // Handle undefined mod ID
      if (modId === undefined) {
        GbConsole.warn(
          'Received download finished event with undefined modId and mod_id:',
          event.payload
        )
        return
      }

      // Get the download ID from our mapping
      const downloadId = modIdToDownloadIdMap.get(modId)
      if (!downloadId) {
        GbConsole.warn(
          `No download ID found for mod ID ${modId}, creating new entry for completion`
        )
        // Create a new download entry on the fly if one doesn't exist
        const newDownloadId = downloadState.createDownload(
          modId,
          event.payload.name || 'Unknown Mod',
          ''
        )

        // Update it to completed state
        downloadState.updateDownloadProgress({
          id: newDownloadId,
          bytesDownloaded: 100,
          totalBytes: 100,
          percentage: 100,
          step: 'Download complete',
          isComplete: true,
          isError: false,
        })

        // Remove after delay
        setTimeout(() => downloadState.completeDownload(newDownloadId), 2000)
        return
      }

      downloadState.updateDownloadProgress({
        id: downloadId,
        bytesDownloaded: 100,
        totalBytes: 100,
        percentage: 100,
        step: 'Download complete',
        isComplete: true,
        isError: false,
      })

      // Remove from downloads list after a delay
      setTimeout(() => {
        downloadState.completeDownload(downloadId)
        // Clean up our mapping
        modIdToDownloadIdMap.delete(modId)
      }, 2000)

      // Refresh the mods list by forcing navigation to the home page
      // This will ensure that the newly added mod appears in the list
      setTimeout(() => {
        // Emit a custom event to refresh mod list
        const event = new CustomEvent('refresh-mods')
        window.dispatchEvent(event)
      }, 1000)
    }).then((unsubscribe: () => void) => {
      removeDownloadFinishedListener = unsubscribe
    })

    listen('download-error', (event: any) => {
      GbConsole.log('Download error:', event.payload)

      const modId = event.payload.modId ?? event.payload.mod_id

      // Handle undefined mod ID
      if (modId === undefined) {
        GbConsole.warn(
          'Received download error event with undefined modId and mod_id:',
          event.payload
        )
        return
      }

      // Get the download ID from our mapping
      const downloadId = modIdToDownloadIdMap.get(modId)
      if (!downloadId) {
        GbConsole.warn(
          `No download ID found for mod ID ${modId}, creating new entry for error state`
        )
        // Create a new download entry on the fly if one doesn't exist
        const newDownloadId = downloadState.createDownload(
          modId,
          event.payload.name || 'Unknown Mod',
          ''
        )

        // Update it to error state
        downloadState.updateDownloadProgress({
          id: newDownloadId,
          bytesDownloaded: 0,
          totalBytes: 100,
          percentage: 0,
          step: 'Error',
          isComplete: false,
          isError: true,
          error: event.payload.error || 'Unknown error',
        })

        // Remove after delay
        setTimeout(
          () =>
            downloadState.errorDownload(
              newDownloadId,
              event.payload.error || 'Unknown error'
            ),
          5000
        )
        return
      }

      downloadState.updateDownloadProgress({
        id: downloadId,
        bytesDownloaded: 0,
        totalBytes: 100,
        percentage: 0,
        step: 'Error',
        isComplete: false,
        isError: true,
        error: event.payload.error || 'Unknown error',
      })

      // Remove from downloads list after a delay
      setTimeout(() => {
        downloadState.errorDownload(
          downloadId,
          event.payload.error || 'Unknown error'
        )
        // Clean up our mapping
        modIdToDownloadIdMap.delete(modId)
      }, 5000)
    }).then((unsubscribe: () => void) => {
      removeDownloadErrorListener = unsubscribe
    })
  })

  // Return a cleanup function to use in onBeforeUnmount
  return () => {
    removeDownloadStartedListener?.()
    removeDownloadProgressListener?.()
    removeDownloadFinishedListener?.()
    removeDownloadErrorListener?.()
  }
}

// Export an instance of the service
export const gamebananaService = GameBananaService.getInstance()
