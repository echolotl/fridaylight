import { Mod } from '@main-types'
import { StoreService } from './storeService'
import { DatabaseService } from './dbService'
import { GBFile, GBProfilePage } from '@custom-types/gamebanana'
import { invoke } from '@tauri-apps/api/core'
import { downloadState } from '@stores/downloadState'
import { sep } from '@tauri-apps/api/path'
import { listen } from '@tauri-apps/api/event'
import { notificationService } from './notificationService'

const fileIdToDownloadId = new Map<number, string>()

export class GameBananaService {
  private static instance: GameBananaService

  private constructor() {}

  public static getInstance(): GameBananaService {
    if (!GameBananaService.instance) {
      GameBananaService.instance = new GameBananaService()
    }
    return GameBananaService.instance
  }

  public async getInstallLocation(): Promise<string> {
    try {
      const storeService = StoreService.getInstance()
      return await storeService.getSetting('installLocation')
    } catch (error) {
      console.error('Error getting install location:', error)
      throw new Error('Failed to retrieve install location')
    }
  }

  public async saveModToDatabase(mod: Mod) {
    try {
      const dbService = DatabaseService.getInstance()
      await dbService.saveMod(mod)
    } catch (error) {
      console.error('Error saving mod to database:', error)
      throw new Error('Failed to save mod to database')
    }
  }

  public async getModDownloadFiles(id: number, modelType: string) {
    return await invoke<GBFile[]>('get_mod_download_files', {
      id,
      modelType,
    })
  }

  public async checkModFolderExists(
    mod: GBProfilePage,
    installLocation?: string,
    folderName?: string
  ): Promise<boolean> {
    if (!installLocation) {
      installLocation = await this.getInstallLocation()
      if (!installLocation) {
        throw new Error('Install location is not set')
      }
    }
    return await invoke<boolean>('check_mod_folder_exists', {
      info: mod,
      installLocation: installLocation,
      folderName: folderName,
    })
  }

  /**
   * Checks if the engine folder exists.
   * @param engineType The type of the engine.
   * @param installLocation The installation location of the engine.
   * @param folderName The name of the folder to check.
   * @returns A promise that resolves to a boolean indicating if the folder exists.
   */
  public async checkEngineFolderExists(
    engineType: string,
    installLocation?: string,
    folderName?: string
  ): Promise<boolean> {
    if (!installLocation) {
      installLocation = await this.getInstallLocation()
      if (!installLocation) {
        throw new Error('Install location is not set')
      }
    }
    return await invoke<boolean>('check_engine_folder_exists', {
      engineType,
      install_location: installLocation,
      folder_name: folderName,
    })
  }

  /**
   * Downloads a mod from GameBanana.
   * @param fileToDownload The file to download.
   * @param modInfo The mod information.
   * @param folderName The folder name to install the mod.
   * @param update Whether to update the mod if it already exists.
   */
  public async downloadMod(
    fileToDownload: GBFile,
    modInfo: GBProfilePage,
    folderName?: string,
    update?: boolean
  ) {
    try {
      const InstallPath = await this.getInstallLocation()
      if (!InstallPath) {
        throw new Error('Install location is not set')
      }

      const downloadId = downloadState.createDownload(
        fileToDownload._idRow,
        modInfo._sName
      )
      fileIdToDownloadId.set(fileToDownload._idRow, downloadId)
      downloadState.updateDownloadProgress({
        id: downloadId,
        step: `Preparing download...`,
        isComplete: false,
        isError: false,
      })

      const result = await invoke<Mod>('download_gamebanana_mod_command', {
        info: modInfo,
        file: fileToDownload,
        installLocation: InstallPath,
        folderName: folderName,
        updateExisting: update || false,
      })

      if (result && !update) {
        await this.saveModToDatabase(result)
      } else if (result && update) {
        // If updating, we update the existing mod
        const dbService = DatabaseService.getInstance()
        const existingMod = await dbService.getModByPath(result.path)
        if (existingMod) {
          result.id = existingMod.id
          await dbService.saveMod(result)
        } else {
          console.warn(
            `No existing mod found to update for path: ${result.path}`
          )
          await this.saveModToDatabase(result)
        }
      }

      const refreshEvent = new CustomEvent('refresh-mods')
      window.dispatchEvent(refreshEvent)
    } catch (error) {
      console.error('Error downloading mod:', error)
      throw new Error('Failed to download mod')
    }
  }

  /**
   * Downloads a modpack from GameBanana.
   * This doesn't save the modpack to the database, as it is not a mod.
   * @param fileToDownload The file to download.
   * @param modInfo The mod information.
   * @param engineInstallation The engine we are installing to's information.
   * @param folderName The folder name to install the modpack.
   * @param update Whether to update the modpack if it already exists.
   */
  public async downloadModpack(
    fileToDownload: GBFile,
    modInfo: GBProfilePage,
    engineInstallation: Mod,
    folderName?: string,
    update?: boolean
  ) {
    try {
      const installPath =
        engineInstallation.path +
        sep() +
        (folderName || engineInstallation.engine.mods_folder_path)

      const downloadId = downloadState.createDownload(
        fileToDownload._idRow,
        modInfo._sName
      )
      fileIdToDownloadId.set(fileToDownload._idRow, downloadId)
      downloadState.updateDownloadProgress({
        id: downloadId,
        step: `Preparing download...`,
        isComplete: false,
        isError: false,
      })

      await invoke<Mod>('download_gamebanana_mod_command', {
        info: modInfo,
        file: fileToDownload,
        installLocation: installPath,
        folderName: folderName,
        updateExisting: update || false,
      })
      notificationService.downloadSuccess(modInfo._sName)
    } catch (error) {
      console.error('Error downloading modpack:', error)
      throw new Error('Failed to download modpack')
    }
  }

  public async downloadEngine(
    engineType: string,
    folderName?: string
  ): Promise<Mod> {
    try {
      const installLocation = await this.getInstallLocation()
      if (!installLocation) {
        throw new Error('Install location is not set')
      }

      const result = await invoke<Mod>('download_engine_command', {
        engineType,
        installLocation,
        folderName,
      })

      if (result) {
        await this.saveModToDatabase(result)
      }

      const refreshEvent = new CustomEvent('refresh-mods')
      window.dispatchEvent(refreshEvent)

      return result
    } catch (error) {
      console.error('Error downloading engine:', error)
      throw new Error('Failed to download engine')
    }
  }

  async getCompatibleEngines(engineType: string): Promise<Mod[]> {
    try {
      const dbService = DatabaseService.getInstance()
      const allMods = await dbService.getAllMods()

      // Filter mods where the engine type matches the provided engineType
      const engines = allMods.filter(
        (mod: Mod) => mod.engine.engine_type === engineType
      )

      return engines
    } catch (error) {
      console.error('Error getting compatible engines:', error)
      return Promise.reject('Failed to get compatible engines')
    }
  }
}

listen('download-progress', (event: any) => {
  console.info('Download progress:', event.payload)

  const modId = event.payload.mod_id
  if (!modId) {
    console.warn('No modId found in download progress event')
    return
  }
  let downloadId = fileIdToDownloadId.get(modId)
  if (!downloadId) {
    console.warn(`No download ID found locally for mod ID ${modId}`)
    // Try and find it in the downloadState
    const existingDownload = Object.values(downloadState.downloadingMods).find(
      download => download.modId === modId
    )
    if (existingDownload) {
      console.info(
        `Found existing download ID ${existingDownload.id} for mod ID ${modId}`
      )
      fileIdToDownloadId.set(modId, existingDownload.id)
      // Update the download ID
      downloadId = existingDownload.id
    } else {
      console.warn(`No download ID found for mod ID ${modId} in downloadState`)
      return
    }
  }
  downloadState.updateDownloadProgress({
    id: downloadId,
    step: event.payload.step,
    percentage: event.payload.percentage,
    bytesDownloaded: event.payload.bytes_downloaded || 0,
    totalBytes: event.payload.total_bytes || 0,
    isComplete: event.payload.isComplete || false,
    isError: event.payload.isError || false,
  })
})

listen('download-finished', (event: any) => {
  console.info('Download complete:', event.payload)
  const modId = event.payload.mod_id
  if (!modId) {
    console.warn('No modId found in download complete event')
    return
  }
  let downloadId = fileIdToDownloadId.get(modId)
  if (!downloadId) {
    console.warn(`No download ID found for locally mod ID ${modId}`)
    // Try and find it in the downloadState
    const existingDownload = Object.values(downloadState.downloadingMods).find(
      download => download.modId === modId
    )
    if (existingDownload) {
      console.info(
        `Found existing download ID ${existingDownload.id} for mod ID ${modId}`
      )
      fileIdToDownloadId.set(modId, existingDownload.id)
      // Update the download ID
      downloadId = existingDownload.id
    } else {
      console.warn(`No download ID found for mod ID ${modId} in downloadState`)
      return
    }
  }
  downloadState.updateDownloadProgress({
    id: downloadId,
    percentage: 100,
    step: 'Download complete!',
    isComplete: true,
    isError: false,
  })

  setTimeout(() => {
    downloadState.completeDownload(downloadId)
    const event = new CustomEvent('refresh-mods')
    window.dispatchEvent(event)
    // Clear the mapping after completion
    fileIdToDownloadId.delete(modId)
  }, 1500)
})

listen('download-error', (event: any) => {
  console.error('Download error:', event.payload)
  const modId = event.payload.mod_id
  if (!modId) {
    console.warn('No modId found in download error event')
    return
  }
  let downloadId = fileIdToDownloadId.get(modId)
  if (!downloadId) {
    console.warn(`No download ID found locally for mod ID ${modId}`)
    // Try and find it in the downloadState
    const existingDownload = Object.values(downloadState.downloadingMods).find(
      download => download.modId === modId
    )
    if (existingDownload) {
      console.info(
        `Found existing download ID ${existingDownload.id} for mod ID ${modId}`
      )
      fileIdToDownloadId.set(modId, existingDownload.id)
      // Update the download ID
      downloadId = existingDownload.id
    } else {
      console.warn(`No download ID found for mod ID ${modId} in downloadState`)
      return
    }
  }
  downloadState.updateDownloadProgress({
    id: downloadId,
    step: 'Download failed',
    isComplete: false,
    isError: true,
    error: event.payload.error || 'Unknown error',
  })
  setTimeout(() => {
    downloadState.errorDownload(
      downloadId,
      event.payload.error || 'Unknown error'
    )
    // Clear the mapping after error
    fileIdToDownloadId.delete(modId)
  }, 1500)
})

export const gamebananaService = GameBananaService.getInstance()
