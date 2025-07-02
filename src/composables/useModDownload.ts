import { notificationService } from '@services/notificationService'
import { GBAltFile, GBFile, GBProfilePage } from '@custom-types/gamebanana'
import { Mod } from '@main-types'
import { gamebananaService } from '@services/gamebananaService'
import { formatEngineName, sanitizeFileName } from '@utils/index'
import { ref } from 'vue'

export function useModDownload(
  openModDetails?: (
    modId: string | number | true,
    modelType: string | number | true
  ) => void
) {
  interface ModDownload {
    file: GBFile
    info: GBProfilePage
    folderName?: string
    update?: boolean
  }
  interface ModpackDownload {
    file: GBFile
    info: GBProfilePage
    engineInstallation: Mod
    folderName?: string
    update?: boolean
  }

  type DownloadItem = ModDownload | ModpackDownload

  // Modal state management
  const isFileSelectionModalOpen = ref(false)
  const isEngineSelectionModalOpen = ref(false)
  const isFolderExistsModalOpen = ref(false)
  const isModTypeSelectionModalOpen = ref(false)

  // Modal data
  const currentModalFiles = ref<GBFile[]>([])
  const currentModalAlternateFiles = ref<GBAltFile[]>([])
  const currentModalModName = ref('')
  const currentModalCompatibleEngines = ref<Mod[]>([])
  const currentModalEngineType = ref('')

  // Modal promise resolvers
  let fileSelectionResolver:
    | ((value: GBFile | PromiseLike<GBFile>) => void)
    | null = null
  let fileSelectionRejecter: ((reason?: any) => void) | null = null
  let engineSelectionResolver:
    | ((value: Mod | PromiseLike<Mod>) => void)
    | null = null
  let engineSelectionRejecter: ((reason?: any) => void) | null = null
  let folderExistsResolver:
    | ((
        value:
          | 'update'
          | 'download-anyway'
          | 'cancel'
          | PromiseLike<'update' | 'download-anyway' | 'cancel'>
      ) => void)
    | null = null
  let folderExistsRejecter: ((reason?: any) => void) | null = null
  let modTypeSelectionResolver:
    | ((
        value:
          | { modType: string; engineMod?: Mod; isAddon: boolean }
          | PromiseLike<{ modType: string; engineMod?: Mod; isAddon: boolean }>
      ) => void)
    | null = null
  let modTypeSelectionRejecter: ((reason?: any) => void) | null = null

  // Download handling
  const downloadMod = async (mod: GBProfilePage) => {
    const downloadItem: DownloadItem = {
      file: {} as GBFile,
      info: mod,
    }

    try {
      // Step 1: Check if mod has multiple files or alternate file sources
      if (
        (mod._aFiles && mod._aFiles.length > 1) ||
        (mod._aAlternateFileSources && mod._aAlternateFileSources.length > 0)
      ) {
        // Show File Selector Dialog
        const selectedFile = await showFileSelectionModal(
          mod._aFiles || [],
          mod._aAlternateFileSources || [],
          mod._sName
        )
        downloadItem.file = selectedFile
      } else {
        // Single file - use it directly
        if (mod._aFiles && mod._aFiles.length > 0) {
          downloadItem.file = mod._aFiles[0] as GBFile
        } else {
          throw new Error('No files available for download.')
        }
      }

      // Step 2: Check if file has executable
      if (downloadItem.file._aAnalysisWarnings?.contains_exe) {
        // File has executable so we treat it as a standalone mod, skip to folder existence check
        // No additional setup needed for standalone mods
      } else {
        // Step 3: Check if mod is labeled for a specific engine (modpack categories)
        let engineType = 'unknown'

        switch (mod._aCategory._idRow) {
          case 28367: // Psych Engine Modpack
            engineType = 'psych'
            break
          case 29202: // V-Slice Modpack
            engineType = 'vanilla'
            break
          case 34764: // Codename Engine Modpack
            engineType = 'codename'
            break
          default: // Not in a specific category
            engineType = 'generic'
            break
        }

        if (engineType !== 'unknown') {
          if (engineType === 'generic') {
            // Generic modpack - show mod type selection modal
            const modTypeResult = await showModTypeSelectionModal(mod._sName)
            console.info(
              `Mod "${mod._sName}"'s' user selected type:`,
              modTypeResult
            )
            if (
              (modTypeResult.modType === 'psych' ||
                modTypeResult.modType === 'codename' ||
                modTypeResult.modType === 'vanilla' ||
                modTypeResult.modType === 'fps-plus') &&
              modTypeResult.engineMod
            ) {
              ;(downloadItem as ModpackDownload).engineInstallation =
                modTypeResult.engineMod
            } else {
              // If the user picks "standalone", then we treat it as a regular mod download
              console.info(
                `Mod ${mod._sName} is a standalone modpack (picked by user), proceeding as regular mod download`
              )
            }
          } else {
            const compatibleEngines =
              await gamebananaService.getCompatibleEngines(engineType)

            if (compatibleEngines.length === 0) {
              throw new Error(
                `No compatible engines found for ${engineType} modpack`
              )
            }

            const selectedEngine = await showEngineSelectionModal(
              compatibleEngines,
              engineType,
              mod._sName
            )
            ;(downloadItem as ModpackDownload).engineInstallation =
              selectedEngine
          }
        } else {
          // If not a specific engine modpack, show mod type selection modal
          const modTypeResult = await showModTypeSelectionModal(mod._sName)

          if (modTypeResult.modType === 'modpack') {
            // User selected modpack - check if they provided an engine
            if (modTypeResult.engineMod) {
              ;(downloadItem as ModpackDownload).engineInstallation =
                modTypeResult.engineMod
            } else {
              // Need to show engine selection
              const compatibleEngines =
                await gamebananaService.getCompatibleEngines('unknown')
              const selectedEngine = await showEngineSelectionModal(
                compatibleEngines,
                'unknown',
                mod._sName
              )
              ;(downloadItem as ModpackDownload).engineInstallation =
                selectedEngine
            }
          }
          // If type is 'standalone', continue as regular mod download
        }
      }

      // Step 4: Check if folder already exists
      let folderExists = false
      if ('engineInstallation' in downloadItem) {
        // For modpacks, check in the engine's mods folder
        folderExists = await gamebananaService.checkModpackFolderExists(
          mod,
          (downloadItem as ModpackDownload).engineInstallation
        )
      } else {
        // For standalone mods, check in the main install location
        folderExists = await gamebananaService.checkModFolderExists(
          mod,
          await gamebananaService.getInstallLocation()
        )
      }
      console.info(`Folder exists for mod ${mod._sName}: ${folderExists}`)

      if (folderExists === true) {
        const folderAction = await showFolderExistsModal(mod._sName)

        switch (folderAction) {
          case 'cancel':
            return // User cancelled
          case 'update':
            downloadItem.update = true
            break
          case 'download-anyway': // This will create a separate folder for the download
          {
            const folderName = `${mod._sName}-${new Date().toISOString().slice(11, 19).replace(/:/g, '-')}`
            downloadItem.folderName = sanitizeFileName(folderName, '-')
            if ('engineInstallation' in downloadItem) {
              // For modpacks, check in the engine's mods folder
              if (
                await gamebananaService.checkModpackFolderExists(
                  mod,
                  (downloadItem as ModpackDownload).engineInstallation,
                  downloadItem.folderName
                )
              ) {
                // Handle folder already exists case
                throw new Error(
                  `Folder ${downloadItem.folderName} already exists. Trying again should generate a different one.`
                )
              }
            } else {
              // For standalone mods, check in the main install location
              if (
                await gamebananaService.checkModFolderExists(
                  mod,
                  await gamebananaService.getInstallLocation(),
                  downloadItem.folderName
                )
              ) {
                // Handle folder already exists case
                throw new Error(
                  `Folder ${downloadItem.folderName} already exists. Trying again should generate a different one.`
                )
              }
            }
            break
          }
        }
      }

      // At this point, downloadItem is fully configured for download
      console.info('Download configured:', downloadItem)

      // Step 5: Perform the download
      // If downloadItem is a ModpackDownload, handle it differently
      if ('engineInstallation' in downloadItem) {
        // Handle modpack download
        await gamebananaService.downloadModpack(
          downloadItem.file,
          downloadItem.info,
          (downloadItem as ModpackDownload).engineInstallation,
          downloadItem.folderName,
          downloadItem.update
        )
      } else {
        // Handle regular mod download
        await gamebananaService.downloadMod(
          downloadItem.file,
          downloadItem.info,
          downloadItem.folderName,
          downloadItem.update
        )
      }
    } catch (error) {
      if (error instanceof Error && error.message.includes('cancelled')) {
        // User cancelled, don't show error notification
        return
      }
      notificationService.downloadError(mod._sName, String(error))
    }
  }

  const downloadEngine = async (engineType: string) => {
    try {
      let folderName: string | undefined = undefined
      let update = false
      // Check if the folder already exists
      const folderExists = await gamebananaService.checkEngineFolderExists(
        engineType,
        await gamebananaService.getInstallLocation()
      )
      if (folderExists) {
        const action = await showFolderExistsModal(
          await formatEngineName(engineType)
        )
        if (action === 'cancel') {
          return // User cancelled
        } else if (action === 'update') {
          update = true
        } else if (action === 'download-anyway') {
          // Create a new folder for the download
          const folderNameBefore = `${formatEngineName(engineType)}-${new Date()
            .toISOString()
            .slice(11, 19)
            .replace(/:/g, '-')}`
          folderName = sanitizeFileName(folderNameBefore, '-')
        }
      }

      await gamebananaService.downloadEngine(engineType, folderName, update)
    } catch (error) {
      console.error('Error downloading engine:', error)
      if (error instanceof Error && error.message.includes('cancelled')) {
        // User cancelled, don't show error notification
        return
      }
      notificationService.downloadError(
        await formatEngineName(engineType),
        String(error)
      )
      throw new Error('Failed to download engine')
    }
  }

  // Download handling for deep links (direct URL)
  const downloadModFromUrl = async (
    mod: GBProfilePage,
    downloadUrl: string,
    archiveExt?: string
  ) => {
    const downloadItem: DownloadItem = {
      file: {
        _sDownloadUrl: downloadUrl,
        _idRow: Math.floor(Math.random() * 1000000), // Generate a unique ID for tracking
        _nFilesize: 0,
        _sFile: `${mod._sName}${archiveExt || '.zip'}`,
        // We just fakin it
        _tsDateAdded: Date.now(),
        _nDownloadCount: 0,
        _sMd5Checksum: '',
        _sAnalysisState: 'complete',
        _sAnalysisResult: 'clean',
        _sAnalysisResultVerbose: '',
        _sAvastAvState: 'complete',
        _sAvastAvResult: 'clean',
        _bHasContents: true,
      } as GBFile,
      info: mod,
    }

    try {
      // Skip Step 1: File selection - we already have the URL

      // Step 2: Check if mod is labeled for a specific engine (modpack categories)
      let engineType = 'unknown'

      switch (mod._aCategory._idRow) {
        case 28367: // Psych Engine Modpack
          engineType = 'psych'
          break
        case 29202: // V-Slice Modpack
          engineType = 'vanilla'
          break
        case 34764: // Codename Engine Modpack
          engineType = 'codename'
          break
        default: // Not in a specific category
          engineType = 'generic'
          break
      }

      if (engineType !== 'unknown') {
        if (engineType === 'generic') {
          // Generic modpack - show mod type selection modal
          const modTypeResult = await showModTypeSelectionModal(mod._sName)
          console.info(
            `Mod "${mod._sName}"'s' user selected type:`,
            modTypeResult
          )
          if (
            (modTypeResult.modType === 'psych' ||
              modTypeResult.modType === 'codename' ||
              modTypeResult.modType === 'vanilla' ||
              modTypeResult.modType === 'fps-plus') &&
            modTypeResult.engineMod
          ) {
            ;(downloadItem as ModpackDownload).engineInstallation =
              modTypeResult.engineMod
          } else {
            // If the user picks "standalone", then we treat it as a regular mod download
            console.info(
              `Mod ${mod._sName} is a standalone modpack (picked by user), proceeding as regular mod download`
            )
          }
        } else {
          const compatibleEngines =
            await gamebananaService.getCompatibleEngines(engineType)

          if (compatibleEngines.length === 0) {
            throw new Error(
              `No compatible engines found for ${engineType} modpack`
            )
          }

          const selectedEngine = await showEngineSelectionModal(
            compatibleEngines,
            engineType,
            mod._sName
          )
          ;(downloadItem as ModpackDownload).engineInstallation = selectedEngine
        }
      } else {
        // If not a specific engine modpack, show mod type selection modal
        const modTypeResult = await showModTypeSelectionModal(mod._sName)

        if (modTypeResult.modType === 'modpack') {
          // User selected modpack - check if they provided an engine
          if (modTypeResult.engineMod) {
            ;(downloadItem as ModpackDownload).engineInstallation =
              modTypeResult.engineMod
          } else {
            // Need to show engine selection
            const compatibleEngines =
              await gamebananaService.getCompatibleEngines('unknown')
            const selectedEngine = await showEngineSelectionModal(
              compatibleEngines,
              'unknown',
              mod._sName
            )
            ;(downloadItem as ModpackDownload).engineInstallation =
              selectedEngine
          }
        }
        // If type is 'standalone', continue as regular mod download
      }

      // Step 3: Check if folder already exists
      let folderExists = false
      if ('engineInstallation' in downloadItem) {
        // For modpacks, check in the engine's mods folder
        folderExists = await gamebananaService.checkModpackFolderExists(
          mod,
          (downloadItem as ModpackDownload).engineInstallation
        )
      } else {
        // For standalone mods, check in the main install location
        folderExists = await gamebananaService.checkModFolderExists(
          mod,
          await gamebananaService.getInstallLocation()
        )
      }
      console.info(`Folder exists for mod ${mod._sName}: ${folderExists}`)

      if (folderExists === true) {
        const folderAction = await showFolderExistsModal(mod._sName)

        switch (folderAction) {
          case 'cancel':
            return // User cancelled
          case 'update':
            downloadItem.update = true
            break
          case 'download-anyway': // This will create a separate folder for the download
          {
            const folderName = `${mod._sName}-${new Date().toISOString().slice(11, 19).replace(/:/g, '-')}`
            downloadItem.folderName = sanitizeFileName(folderName, '-')
            if ('engineInstallation' in downloadItem) {
              // For modpacks, check in the engine's mods folder
              if (
                await gamebananaService.checkModpackFolderExists(
                  mod,
                  (downloadItem as ModpackDownload).engineInstallation,
                  downloadItem.folderName
                )
              ) {
                // Handle folder already exists case
                throw new Error(
                  `Folder ${downloadItem.folderName} already exists. Trying again should generate a different one.`
                )
              }
            } else {
              // For standalone mods, check in the main install location
              if (
                await gamebananaService.checkModFolderExists(
                  mod,
                  await gamebananaService.getInstallLocation(),
                  downloadItem.folderName
                )
              ) {
                // Handle folder already exists case
                throw new Error(
                  `Folder ${downloadItem.folderName} already exists. Trying again should generate a different one.`
                )
              }
            }
            break
          }
        }
      }

      // At this point, downloadItem is fully configured for download
      console.info('Download configured from URL:', downloadItem)

      // Step 4: Perform the download
      // If downloadItem is a ModpackDownload, handle it differently
      if ('engineInstallation' in downloadItem) {
        // Handle modpack download
        await gamebananaService.downloadModpack(
          downloadItem.file,
          downloadItem.info,
          (downloadItem as ModpackDownload).engineInstallation,
          downloadItem.folderName,
          downloadItem.update
        )
      } else {
        // Handle regular mod download
        await gamebananaService.downloadMod(
          downloadItem.file,
          downloadItem.info,
          downloadItem.folderName,
          downloadItem.update
        )
      }
    } catch (error) {
      if (error instanceof Error && error.message.includes('cancelled')) {
        // User cancelled, don't show error notification
        return
      }
      notificationService.downloadError(mod._sName, String(error))
    }
  }

  // Modal handler functions
  const showFileSelectionModal = (
    files: GBFile[],
    alternateFiles: GBAltFile[],
    modName: string
  ): Promise<GBFile> => {
    return new Promise<GBFile>((resolve, reject) => {
      currentModalFiles.value = files
      currentModalAlternateFiles.value = alternateFiles
      currentModalModName.value = modName
      isFileSelectionModalOpen.value = true

      fileSelectionResolver = resolve
      fileSelectionRejecter = reject
    })
  }

  const showEngineSelectionModal = (
    compatibleEngines: Mod[],
    engineType: string,
    modName: string
  ): Promise<Mod> => {
    return new Promise<Mod>((resolve, reject) => {
      currentModalCompatibleEngines.value = compatibleEngines
      currentModalEngineType.value = engineType
      currentModalModName.value = modName
      isEngineSelectionModalOpen.value = true

      engineSelectionResolver = resolve
      engineSelectionRejecter = reject
    })
  }

  const showFolderExistsModal = (
    modName: string
  ): Promise<'update' | 'download-anyway' | 'cancel'> => {
    return new Promise<'update' | 'download-anyway' | 'cancel'>(
      (resolve, reject) => {
        currentModalModName.value = modName
        isFolderExistsModalOpen.value = true

        folderExistsResolver = resolve
        folderExistsRejecter = reject
      }
    )
  }

  const showModTypeSelectionModal = (
    modName: string
  ): Promise<{ modType: string; engineMod?: Mod; isAddon: boolean }> => {
    return new Promise<{ modType: string; engineMod?: Mod; isAddon: boolean }>(
      (resolve, reject) => {
        currentModalModName.value = modName
        isModTypeSelectionModalOpen.value = true

        modTypeSelectionResolver = resolve
        modTypeSelectionRejecter = reject
      }
    )
  }

  // Modal event handlers
  const handleFileSelection = (file: GBFile) => {
    isFileSelectionModalOpen.value = false
    if (fileSelectionResolver) {
      fileSelectionResolver(file)
      fileSelectionResolver = null
      fileSelectionRejecter = null
    }
  }

  const handleFileSelectionCancel = () => {
    isFileSelectionModalOpen.value = false
    if (fileSelectionRejecter) {
      fileSelectionRejecter(new Error('File selection cancelled'))
      fileSelectionResolver = null
      fileSelectionRejecter = null
    }
  }

  const handleEngineSelection = (engine: Mod) => {
    isEngineSelectionModalOpen.value = false
    if (engineSelectionResolver) {
      engineSelectionResolver(engine)
      engineSelectionResolver = null
      engineSelectionRejecter = null
    }
  }

  const handleEngineSelectionCancel = () => {
    isEngineSelectionModalOpen.value = false
    if (engineSelectionRejecter) {
      engineSelectionRejecter(new Error('Engine selection cancelled'))
      engineSelectionResolver = null
      engineSelectionRejecter = null
    }
  }

  const handleFolderExistsUpdate = () => {
    isFolderExistsModalOpen.value = false
    if (folderExistsResolver) {
      folderExistsResolver('update')
      folderExistsResolver = null
      folderExistsRejecter = null
    }
  }

  const handleFolderExistsDownloadAnyway = () => {
    isFolderExistsModalOpen.value = false
    if (folderExistsResolver) {
      folderExistsResolver('download-anyway')
      folderExistsResolver = null
      folderExistsRejecter = null
    }
  }

  const handleFolderExistsCancel = () => {
    isFolderExistsModalOpen.value = false
    if (folderExistsRejecter) {
      folderExistsRejecter(new Error('Folder exists dialog cancelled'))
      folderExistsResolver = null
      folderExistsRejecter = null
    }
  }

  const handleModTypeSelection = (result: {
    modType: string
    engineMod?: Mod
    isAddon: boolean
  }) => {
    isModTypeSelectionModalOpen.value = false
    if (modTypeSelectionResolver) {
      modTypeSelectionResolver(result)
      modTypeSelectionResolver = null
      modTypeSelectionRejecter = null
    }
  }

  const handleModTypeSelectionBack = () => {
    // Handle back button - could reopen previous modal or cancel
    handleModTypeSelectionCancel()
  }

  const handleModTypeSelectionCancel = () => {
    isModTypeSelectionModalOpen.value = false
    if (modTypeSelectionRejecter) {
      modTypeSelectionRejecter(new Error('Mod type selection cancelled'))
      modTypeSelectionResolver = null
      modTypeSelectionRejecter = null
    }
  }
  const handleModTypeSelectionOpenModDetails = (
    modId: string | number | true,
    modelType: string | number | true
  ) => {
    isModTypeSelectionModalOpen.value = false
    if (modTypeSelectionRejecter) {
      modTypeSelectionRejecter(new Error('Mod type selection cancelled'))
      modTypeSelectionResolver = null
      modTypeSelectionRejecter = null
    }
    if (openModDetails) {
      openModDetails(modId, modelType)
    }
  }

  return {
    // Functions for the component to use
    downloadMod,
    downloadEngine,
    downloadModFromUrl, // Export the deep-link download function

    // Modal state for template binding
    isFileSelectionModalOpen,
    isEngineSelectionModalOpen,
    isFolderExistsModalOpen,
    isModTypeSelectionModalOpen,

    // Modal data for template binding
    currentModalFiles,
    currentModalAlternateFiles,
    currentModalModName,
    currentModalCompatibleEngines,
    currentModalEngineType,

    // Modal handlers for template binding
    handleFileSelection,
    handleFileSelectionCancel,
    handleEngineSelection,
    handleEngineSelectionCancel,
    handleFolderExistsUpdate,
    handleFolderExistsDownloadAnyway,
    handleFolderExistsCancel,
    handleModTypeSelection,
    handleModTypeSelectionBack,
    handleModTypeSelectionCancel,
    handleModTypeSelectionOpenModDetails,
  }
}
