import { Mod } from '@main-types'
import { StoreService } from './storeService'
import { DatabaseService } from './dbService'
import { GBFile, GBProfilePage } from '@custom-types/gamebanana'
import { invoke } from '@tauri-apps/api/core'
import { downloadState } from '@stores/downloadState'
import { sep } from '@tauri-apps/api/path'

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
    installLocation: string,
    folderName?: string
  ): Promise<boolean> {
    return await invoke<boolean>('check_mod_folder_exists', {
      info: mod,
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

      downloadState.createDownload(fileToDownload._idRow, modInfo._sName)
      downloadState.updateDownloadProgress({
        id: fileToDownload._idRow.toString(),
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

      if (result) {
        await this.saveModToDatabase(result)
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

      downloadState.createDownload(fileToDownload._idRow, modInfo._sName)
      downloadState.updateDownloadProgress({
        id: fileToDownload._idRow.toString(),
        step: `Preparing download...`,
        isComplete: false,
        isError: false,
      })

      await invoke<Mod>('download_gamebanana_modpack_command', {
        info: modInfo,
        file: fileToDownload,
        installLocation: installPath,
        folderName: folderName,
        updateExisting: update || false,
      })
    } catch (error) {
      console.error('Error downloading modpack:', error)
      throw new Error('Failed to download modpack')
    }
  }
}
