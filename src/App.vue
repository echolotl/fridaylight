<template>
  <q-layout view="lHh Lpr lFf" class="transparent-bg">
    <!-- Loading overlay shown during initialization -->
    <div v-if="isInitializing" class="loading-overlay">
      <div class="loading-container">
        <img class="logo" />
        <q-linear-progress
          :value="initProgress"
          size="md"
          rounded
          class="q-mt-md"
          color="primary"
        />
        <div class="text-caption q-mt-sm phantom-font">
          {{ initStatusText }}
        </div>
      </div>
    </div>
    <div v-if="!isInitializing">
      <Sidebar v-model="sidebarOpen" @resize="handleSidebarResize" />
      <!-- Engine selection for modpacks from deep links -->
      <EngineSelectionDialog
        v-model="showEngineSelectDialog"
        :compatible-engines="compatibleEngines"
        :engine-type="currentModpackType"
        :mod-name="currentModName"
        @select="onEngineSelected"
        @cancel="cancelDownload"
      />

      <!-- Missing mod path dialog -->
      <MessageDialog
        v-model="showModPathMissingDialog"
        title="Missing Mod"
        message="The following mod folder no longer exists:"
        icon="warning"
        icon-color="negative"
        confirm-label="Remove From List"
        confirm-color="negative"
        @confirm="removeMissingMod"
      >
        <div class="text-caption">{{ missingModPath }}</div>
        <p class="text-body2 q-mt-sm">
          Would you like to remove this mod from your library?
        </p>
      </MessageDialog>

      <!-- Global Context Menu -->
      <ContextMenu
        :visible="showContextMenu"
        :position="contextMenuPosition"
        :options="contextMenuOptions"
        @close="closeContextMenu"
      />
    </div>
  </q-layout>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, provide, reactive } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { onOpenUrl } from '@tauri-apps/plugin-deep-link'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { listen } from '@tauri-apps/api/event'
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'
import Sidebar from './components/layout/Sidebar.vue'
import EngineSelectionDialog from './components/modals/EngineSelectionDialog.vue'
import MessageDialog from './components/modals/MessageDialog.vue'
import ContextMenu from './components/common/ContextMenu.vue'
import { DatabaseService } from '@services/dbService'
import { StoreService, DEFAULT_SETTINGS } from '@services/storeService'
import {
  gamebananaService,
  setupGameBananaEventListeners,
} from '@services/gamebananaService'
import { themeService } from '@services/themeService'
import { AppSettings } from './types'
import { notificationService } from '@services/notificationService'

// Define window.db
declare global {
  interface Window {
    db: any
  }
}

// Initialization state variables
const isInitializing = ref(true)
const initProgress = ref(0)
const initStatusText = ref('Starting up...')

const sidebarOpen = ref(true)
const sidebarWidth = ref(250)

const showEngineSelectDialog = ref(false)
const compatibleEngines = ref<any[]>([])
const currentModpackType = ref<string | null>(null)
const currentModName = ref<string>('Unknown Mod')
const currentDownloadUrl = ref<string>('')
const currentModId = ref<number | null>(null)
const currentModelType = ref<string>('')
const handleSidebarResize = (width: number) => {
  sidebarWidth.value = width
}

const cancelDownload = () => {
  showEngineSelectDialog.value = false

  // Show cancellation notification
  notificationService.downloadCancelledGeneric()
}

const onEngineSelected = async (engine: any) => {
  try {
    if (!currentDownloadUrl.value || !currentModId.value) {
      throw new Error('Missing download information')
    }

    // Use the gamebananaService to handle modpack download with selected engine
    const result = await gamebananaService.downloadDeepLinkModpackWithEngine(
      currentDownloadUrl.value,
      currentModName.value,
      currentModId.value,
      currentModelType.value,
      engine
    )

    // Close the dialog
    showEngineSelectDialog.value = false

    // Show an error if there was one
    if (!result.success) {
      notificationService.downloadError(
        `Failed to download ${currentModName.value}`,
        result.error || 'Unknown error'
      )
    }
  } catch (error) {
    console.error('Failed to download modpack:', error)

    notificationService.downloadError(
      'Failed to download modpack',
      error instanceof Error ? error.message : 'Unknown error'
    )
  } finally {
    // Close the dialog
    showEngineSelectDialog.value = false
  }
}

// State for missing mod path dialog
const showModPathMissingDialog = ref(false)
const missingModPath = ref('')

// Function to remove a mod when it's missing
const removeMissingMod = async () => {
  if (!missingModPath.value) return

  console.log('Removing missing mod with path:', missingModPath.value)

  try {
    // Find mod ID based on path
    const dbService = DatabaseService.getInstance()
    const mod = await dbService.getModByPath(missingModPath.value)

    if (mod) {
      // Delete the mod from the database
      await dbService.deleteMod(mod.id)

      // Show success notification
      notificationService.success({
        message: 'Mod removed successfully!',
      })

      // Refresh the mods list by dispatching an event
      const refreshEvent = new CustomEvent('refresh-mods')
      window.dispatchEvent(refreshEvent)
    } else {
      console.warn('Could not find mod with path:', missingModPath.value)
    }
  } catch (error) {
    console.error('Error removing missing mod:', error)
    notificationService.error({
      message: 'Failed to remove mod',
      caption: String(error),
    })
  } finally {
    // Reset state
    showModPathMissingDialog.value = false
    missingModPath.value = ''
  }
}

// State for context menu
const showContextMenu = ref(false)
const contextMenuPosition = ref({ x: 0, y: 0 })
const contextMenuOptions = ref<any[]>([])

// Function to close the context menu
const closeContextMenu = () => {
  showContextMenu.value = false
  contextMenuOptions.value = []
}

// Global event listener for custom context menu
onMounted(() => {
  // Prevent default context menu
  document.addEventListener('contextmenu', event => {
    event.preventDefault()
  })

  // Listen for custom context menu events
  document.addEventListener('show-context-menu', ((event: CustomEvent) => {
    // Get position and options from the event
    const { position, options } = event.detail

    // Update context menu state
    contextMenuPosition.value = position
    contextMenuOptions.value = options
    showContextMenu.value = true
  }) as EventListener)
})

// Cleanup event listeners on unmount
onUnmounted(() => {
  document.removeEventListener('contextmenu', event => {
    event.preventDefault()
  })

  document.removeEventListener('show-context-menu', (() => {}) as EventListener)
})

// Initialize application settings
const appSettings = reactive<AppSettings>({ ...DEFAULT_SETTINGS })

// Provide app settings to all components
provide('appSettings', appSettings)

// Load app settings from database and update the reactive object
const loadAppSettings = async () => {
  try {
    const storeService = StoreService.getInstance()

    // Load each setting individually
    const accentColor = await storeService.getSetting('accentColor')
    if (accentColor) appSettings.accentColor = accentColor

    const installLocation = await storeService.getSetting('installLocation')
    if (installLocation) appSettings.installLocation = installLocation

    const theme = await storeService.getSetting('theme')
    if (theme) appSettings.theme = theme
    const useSystemTheme = await storeService.getSetting('useSystemTheme')

    if (useSystemTheme !== undefined)
      appSettings.useSystemTheme = useSystemTheme === true

    const validateFnfMods = await storeService.getSetting('validateFnfMods')

    if (validateFnfMods !== undefined)
      appSettings.validateFnfMods = validateFnfMods === true

    const showTerminalOutput =
      await storeService.getSetting('showTerminalOutput')

    await storeService.getSetting('showTerminalOutput')
    if (showTerminalOutput !== undefined)
      appSettings.showTerminalOutput = showTerminalOutput === true

    console.log('Loaded app settings:', appSettings)
  } catch (error) {
    console.error('Error loading app settings:', error)
  }
}

// Apply theme based on system or user preference
const applyTheme = async (themeValue: string | boolean) => {
  // Convert legacy boolean value to theme string
  let activeThemeValue: string
  if (typeof themeValue === 'boolean') {
    activeThemeValue = themeValue ? 'light' : 'dark'
  } else {
    activeThemeValue = themeValue
  }

  console.log('Applying theme:', activeThemeValue)

  try {
    // Apply theme using the theme service
    await themeService.applyTheme(activeThemeValue)

    // Check if we're running on Windows 11 for Mica effect
    const isWindows11 = await invoke<boolean>('is_windows_11')
    console.log('Is Windows 11:', isWindows11, 'Theme:', activeThemeValue)

    // Handle platform-specific styling
    if (!isWindows11) {
      // Apply solid theme styling for non-Windows 11 platforms
      handleNonWindows11Styling(activeThemeValue)
    } else {
      // Handle Windows 11 Mica effect
      await handleWindows11Styling(activeThemeValue)
    }
  } catch (error) {
    console.error('Failed to apply theme:', error)
    // Fallback to basic theme application
    fallbackThemeApplication(activeThemeValue)
  }
}

// Handle styling for non-Windows 11 platforms
const handleNonWindows11Styling = (themeValue: string) => {
  // Only apply solid-theme to light and dark themes
  if (themeValue === 'light' || themeValue === 'dark') {
    document.body.classList.add('solid-theme')
  } else {
    document.body.classList.remove('solid-theme')
  }

  // Remove transparent background styles
  document.documentElement.style.setProperty(
    '--transparent-bg-override',
    'none'
  )

  // Set background to solid color instead of transparent
  const bgColor = `var(--theme-bg)`

  document.body.style.removeProperty('background')
  document.body.style.backgroundColor = bgColor
  document
    .querySelector('.q-layout')
    ?.setAttribute('style', 'background: ' + bgColor + ' !important')
}

// Handle Windows 11 Mica effect styling
const handleWindows11Styling = async (themeValue: string) => {
  // Only light and dark themes should be transparent for Mica
  if (themeService.supportsWindowsMica(themeValue)) {
    document.body.classList.remove('solid-theme')
    document.documentElement.style.setProperty(
      '--transparent-bg-override',
      'transparent'
    )
    document.body.style.removeProperty('background')
    document.body.setAttribute('style', 'background: transparent !important')

    // Make sure q-layout is also transparent for Mica to work properly
    const qLayout = document.querySelector('.q-layout')
    if (qLayout) {
      qLayout.removeAttribute('style')
      qLayout.setAttribute('style', 'background: transparent !important')
    }

    // Apply Mica effect based on theme
    try {
      await invoke('change_mica_theme', {
        window: 'main',
        dark: themeValue !== 'light',
      })
      console.log(
        'Applied Mica theme effect:',
        themeValue !== 'light' ? 'dark' : 'light'
      )
    } catch (error) {
      console.error('Failed to apply Mica effect:', error)
    }
  } else {
    // For custom themes on Windows 11, use solid styling
    document.body.classList.remove('solid-theme')
    document.documentElement.style.setProperty(
      '--transparent-bg-override',
      'none'
    )

    // Set background to solid color based on the theme
    const bgColor = `var(--theme-bg)`

    document.body.style.removeProperty('background')
    document.body.style.backgroundColor = bgColor
    document
      .querySelector('.q-layout')
      ?.setAttribute('style', 'background: ' + bgColor + ' !important')

    // Remove Mica effect for non-standard themes
    try {
      await invoke('remove_mica_theme', {
        window: 'main',
      })
      console.log('Removed Mica effect for theme:', themeValue)
    } catch (error) {
      console.error('Failed to remove Mica effect:', error)
    }
  }
}

// Fallback theme application without theme service
const fallbackThemeApplication = (themeValue: string) => {
  console.warn('Using fallback theme application for:', themeValue)

  // Remove all theme classes
  document.body.classList.forEach(className => {
    if (className.endsWith('-theme')) {
      document.body.classList.remove(className)
    }
  })

  // Add the active theme class
  document.body.classList.add(`${themeValue}-theme`)
}

// Get the current system theme (light or dark)
const getSystemTheme = (): boolean => {
  return (
    window.matchMedia &&
    window.matchMedia('(prefers-color-scheme: light)').matches
  )
}

// Handle system theme changes
const handleSystemThemeChange = async (event: MediaQueryListEvent) => {
  // Only react to system theme changes if that setting is enabled
  const useSystemTheme = await getUseSystemThemeSetting()
  if (useSystemTheme) {
    const isLightTheme = event.matches
    applyTheme(isLightTheme)
  }
}

// Get the current "use system theme" setting from the database
const getUseSystemThemeSetting = async (): Promise<boolean> => {
  try {
    const storeService = StoreService.getInstance()
    return await storeService.getSetting('useSystemTheme')
  } catch (error) {
    console.error('Error fetching useSystemTheme setting:', error)
    return true // Default to true on error
  }
}

// Get the manually set theme preference
const getThemePreference = async (): Promise<string> => {
  try {
    const storeService = StoreService.getInstance()

    // Get theme from the store
    const theme = await storeService.getSetting('theme')
    if (theme) {
      return theme
    }

    return 'dark' // Default to dark theme
  } catch (error) {
    console.error('Error fetching theme preference:', error)
    return 'dark' // Default to dark theme
  }
}

// Process deep link mod download
const processDeepLinkModDownload = async (
  downloadUrl: string,
  modId: number,
  archiveExt: string,
  modelType: string
) => {
  try {
    console.log('Processing mod download from deep link:', {
      downloadUrl,
      modId,
      archiveExt,
      modelType,
    })

    // Use the gamebananaService to handle the deep link download
    const result = await gamebananaService.downloadModFromDeepLink(
      downloadUrl,
      modId,
      modelType
    )

    // Type guard to check if we need to show the engine selection dialog
    if ('showEngineSelectDialog' in result && result.showEngineSelectDialog) {
      // Show the engine selection dialog with the data from the result
      compatibleEngines.value = result.compatibleEngines
      currentModpackType.value = result.modpackType
      currentModName.value = result.modName
      currentDownloadUrl.value = result.downloadUrl
      currentModId.value = result.modId
      currentModelType.value = result.modelType
      showEngineSelectDialog.value = true
    } else if ('success' in result) {
      // Handle regular operation result
      if (!result.success && result.error) {
        notificationService.downloadError(
          'Failed to download mod',
          result.error
        )
      }
    }
  } catch (error) {
    console.error('Failed to download mod from deep link:', error)

    // Show error notification
    notificationService.downloadError(
      'Failed to download mod',
      error instanceof Error ? error.message : 'Unknown error'
    )
  }
}

// Initialize the app
let cleanupEventListeners: (() => void) | undefined

onMounted(async () => {
  try {
    const currentWindow = getCurrentWindow()
    console.log('Current window:', currentWindow)
    currentWindow.show()
    // Update progress bar - Step 1: Initialize theme service
    initStatusText.value = 'Initializing theme service...'
    initProgress.value = 0.1

    // Initialize the theme service first
    await themeService.initialize() // Update progress bar - Step 2: Apply theme
    initStatusText.value = 'Applying theme...'
    initProgress.value = 0.15

    // Apply initial theme based on system or user preference
    const useSystemThemeInitial = await getUseSystemThemeSetting()
    if (useSystemThemeInitial) {
      // If using system theme, apply light or dark based on system preference
      const isSystemLight = getSystemTheme()
      await applyTheme(isSystemLight ? 'light' : 'dark')
    } else {
      // If using custom theme, apply the saved theme directly
      const themeValue = await getThemePreference()
      await applyTheme(themeValue)
    }

    // Set up download event listeners
    cleanupEventListeners = setupGameBananaEventListeners()

    // Update progress bar - Step 3: Initialize deep link handler
    initStatusText.value = 'Setting up deep link handler...'
    initProgress.value = 0.2

    // Set up deep link handler
    onOpenUrl(async url => {
      console.log('Deep link received:', url[0])
      await getCurrentWindow().setFocus()

      // Process the deep link URL
      if (url[0].startsWith('flmod://')) {
        try {
          // Remove the protocol prefix
          const data = url[0].substring(8) // Remove "flmod://"

          // Split the URL by $ delimiter
          const parts = data.split('$')
          if (parts.length >= 4) {
            let downloadUrl = parts[0]
            const modType = parts[1]
            const modId = parseInt(parts[2])
            const archiveExt = parts[3]

            downloadUrl = downloadUrl.replace('https//', 'https://') // Fix URL format

            console.log('Parsed deep link:', {
              downloadUrl,
              modType,
              modId,
              archiveExt,
            })

            processDeepLinkModDownload(downloadUrl, modId, archiveExt, modType)
          } else {
            console.error('Invalid deep link format, missing required parts')
          }
        } catch (error) {
          console.error('Failed to process deep link:', error)
        }
      }
    })

    // Update progress bar - Step 4: Initialize database
    initStatusText.value = 'Initializing database...'
    initProgress.value = 0.3

    // Initialize the database service
    const dbService = DatabaseService.getInstance()
    await dbService.initialize()

    // Make the database service available globally
    window.db = {
      // Direct access to the database service
      select: async (query: string, params?: any[]) => {
        const db = await dbService['db']
        return db.select(query, params)
      },
      execute: async (query: string, params?: any[]) => {
        const db = await dbService['db']
        return db.execute(query, params)
      },
      service: dbService,
    }

    // Update progress bar - Step 3: Load mods
    initStatusText.value = 'Loading mods...'
    initProgress.value = 0.4 // Load mods from the database
    const mods = await dbService.getAllMods()
    console.log('Loaded mods from database:', mods)

    if (mods && mods.length > 0) {
      console.log(`Loading ${mods.length} mods from database to backend`)
      // No need to sync with backend as the dbService handles this internally
    } else {
      console.log('No mods found in database')
    }

    // Update progress bar - Step 5: Load settings
    initStatusText.value = 'Loading settings...'
    initProgress.value = 0.6

    // Load app settings
    await loadAppSettings()

    // Set up listener for missing mod paths
    await listen<string>('mod-path-missing', event => {
      console.log('Received mod-path-missing event:', event)
      missingModPath.value = event.payload
      showModPathMissingDialog.value = true
    })

    // Listen for system theme changes
    const mediaQuery = window.matchMedia('(prefers-color-scheme: light)')
    mediaQuery.addEventListener('change', handleSystemThemeChange)

    // Update progress bar - Step 6: Check for updates
    initStatusText.value = 'Checking for updates...'
    initProgress.value = 0.8

    // Check for updates
    try {
      const updateResult = await check()
      if (updateResult !== null) {
        // Update progress bar - Updates available
        initStatusText.value = 'Installing update...'

        // Install the update if there is one
        let contentLength = 0
        await updateResult.downloadAndInstall(event => {
          switch (event.event) {
            case 'Started': {
              contentLength = event.data.contentLength ?? 0
              break
            }
            case 'Progress': {
              let downloaded = 0
              downloaded += event.data.chunkLength
              const percent = contentLength
                ? Math.round((downloaded / contentLength) * 100)
                : 0
              initProgress.value = 0.8 + percent / 1000
              break
            }
            case 'Finished':
              initProgress.value = 1.0
              break
          }
        })

        await relaunch()
      } else {
        console.log('No updates available')
      }
    } catch (error) {
      console.error('Failed to check for updates:', error)
    }

    // Update progress bar - Step 7: Complete
    initStatusText.value = 'Ready!'
    initProgress.value = 1.0

    // Hide the loading overlay after a small delay to ensure transitions are smooth
    setTimeout(() => {
      isInitializing.value = false
    }, 500)

    console.log('App initialized and database tables updated')
  } catch (error) {
    console.error('Failed to initialize database:', error)

    // Update progress bar with error state
    initStatusText.value = 'Error during initialization: ' + String(error)
    initProgress.value = 1.0

    // Hide the loading overlay after a delay
    setTimeout(() => {
      isInitializing.value = false
    }, 2000)
    onUnmounted(() => {
      // Clean up the event listener for system theme changes
      const mediaQuery = window.matchMedia('(prefers-color-scheme: light)')
      mediaQuery.removeEventListener('change', handleSystemThemeChange)

      // Clean up download event listeners
      if (typeof cleanupEventListeners === 'function') {
        cleanupEventListeners()
      }
    })
  }
})
</script>

<style>
:root {
  --red: #c03221;
  --orange: #fe621d;
  --yellow: #f7c548;
  --green: #35ce8d;
  --blue: #39dbce;
  --purple: #c490d1;
  --pink: #db2955;
}

html,
body {
  margin: 0;
  padding: 0;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  color: var(--theme-text);
}
</style>

<style scoped>
.transparent-bg {
  background: transparent !important;
}

/* Loading overlay styles */
.loading-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 9999;
}

.loading-container {
  width: 400px;
  text-align: center;
  padding: 24px;
  border-radius: 12px;
  background-color: var(--theme-card);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  border: 2px solid var(--theme-border);
}
.logo {
  width: 100%;
  height: 125px;
  mask: url('/images/fridaylight.svg') no-repeat center;
  -webkit-mask: url('/images/fridaylight.svg') no-repeat center;
  background-color: var(--theme-text);
  image-rendering: optimizeQuality;
}

.loading-container h2 {
  margin-top: 0;
  margin-bottom: 24px;
}

.loading-container .text-caption {
  color: var(--theme-text-secondary, #aaaaaa);
  margin-top: 12px;
}
</style>
