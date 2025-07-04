<template>
  <q-scroll-area v-if="mod" class="mod-details phantom-font">
    <ModBanner
      :mod="mod"
      style="margin-bottom: 2rem"
      @update:title="updateTitle"
      @open-settings="$emit('open-settings')"
    />

    <div class="mod-content-wrapper">
      <!-- Left column: Main content -->
      <div class="mod-main-content">
        <div class="mod-actions">
          <div class="action-buttons">
            <q-btn
              :color="isModRunning ? 'negative' : 'primary'"
              class="action-button phantom-font"
              size="lg"
              :disabled="!mod.executable_path"
              @click="handleModAction"
            >
              {{ isModRunning ? $t('ui.actions.stop') : $t('ui.actions.play') }}
            </q-btn>

            <div v-if="!mod.executable_path" class="error-message">
              {{ $t('mods.no_executables_found') }}
            </div>

            <div v-if="error" class="error-message">
              {{ error }}
            </div>
          </div>

          <q-btn
            flat
            round
            color="var(--theme-text)"
            icon="settings"
            class="settings-button"
            @click="$emit('open-settings')"
          />
        </div>

        <!-- Terminal Output Component -->
        <TerminalOutput
          v-if="isModRunning && showTerminalOutput"
          :mod-id="mod.id"
          :is-visible="isModRunning && showTerminalOutput"
          @close="showTerminalOutput = false && clearModLogs"
          @clear="clearModLogs"
        />

        <div v-if="mod.description" class="description">
          <h6 class="phantom-font-difficulty">
            {{ $t('mods.engine_mod_details.description') }}
            <hr />
          </h6>
          <p v-html="mod.description"></p>
        </div>

        <!-- Show engine-specific mods list if mod has engine type and executable path -->
        <div
          v-if="
            mod.engine &&
            mod.engine.engine_type &&
            mod.executable_path &&
            mod.engine.mods_folder
          "
        >
          <EngineModsList
            :executable-path="mod.executable_path"
            :engine-type="
              mod.engine ? mod.engine.engine_type : DEFAULT_ENGINE.engine_type
            "
            :custom-mods-folder="
              mod.engine
                ? mod.engine.mods_folder_path
                : DEFAULT_ENGINE.mods_folder_path
            "
          />
        </div>
      </div>

      <!-- Right column: Contributor information -->
      <div class="mod-sidebar">
        <ContributorInfobox
          :folder-path="mod.path"
          :version="mod.version"
          @open-gamebanana="handleOpenGamebanana"
        />
      </div>
    </div>
  </q-scroll-area>

  <!-- Show welcome message when no mod is selected -->
  <div v-else class="welcome-message phantom-font">
    <h2 class="phantom-font-display">{{ $t('mods.mod_placeholder_text') }}</h2>
    <p>{{ $t('mods.mod_placeholder_description') }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import ModBanner from '@mods/ModBanner.vue'
import EngineModsList from '@mods/EngineModsList.vue'
import TerminalOutput from '@common/TerminalOutput.vue'
import ContributorInfobox from '@common/ContributorInfobox.vue'
import { Mod } from '@main-types'
import { StoreService } from '@services/storeService'
import { notificationService } from '@services/notificationService'
import { DEFAULT_ENGINE } from '@services/dbService'

const props = defineProps({
  mod: {
    type: Object as () => Mod | null,
    default: null,
  },
  error: {
    type: String,
    default: '',
  },
})

const emit = defineEmits([
  'update:mod',
  'launch-mod',
  'open-settings',
  'stop-mod',
  'open-gamebanana-mod',
])

const appSettings = StoreService.getInstance()

const isModRunning = ref(false)
const showTerminalOutput = ref(false)

let modTerminatedListener: (() => void) | null = null

// Handle the play/stop button click
const handleModAction = async () => {
  if (!props.mod) return

  if (isModRunning.value) {
    console.info(`Stopping mod ${props.mod.id}`)
    // Stop the mod
    try {
      await invoke('stop_mod', { id: props.mod.id })
      console.info(`Stop command successful, updating UI state`)
      isModRunning.value = false
      showTerminalOutput.value = false // Hide terminal immediately
    } catch (error) {
      console.error('Failed to stop mod:', error)
      notificationService.modError(
        props.mod.name,
        'stop',
        error instanceof Error ? error.message : String(error)
      )
    }
  } else {
    console.info(`Launching mod ${props.mod.id}`)
    // Launch the mod
    emit('launch-mod', props.mod.id)

    // Show terminal output based on user settings
    if (appSettings) {
      const terminalSetting = await appSettings.getSetting('showTerminalOutput')
      // If the setting is not found, default to false
      showTerminalOutput.value =
        terminalSetting !== undefined ? terminalSetting : false
      console.info(
        `Setting terminal visibility to ${showTerminalOutput.value} based on app settings`
      )
    }

    // Update UI state immediately, will be confirmed when we receive the event
    console.info(
      `Setting isModRunning to true in anticipation of successful launch`
    )
    isModRunning.value = true
  }
}

const clearModLogs = async (modId: string) => {
  if (!modId) return

  try {
    if (!props.mod?.save_terminal_output) {
      invoke('clear_all_mod_logs', { id: modId })
    } else {
      invoke('clear_mod_logs', { id: modId })
    }
    console.info(`Mod logs cleared for mod ID: ${modId}`)
  } catch (error) {
    console.error('Failed to clear mod logs:', error)
  }
}

const updateTitle = (newTitle: string) => {
  if (props.mod) {
    const updatedMod = { ...props.mod, name: newTitle }
    emit('update:mod', updatedMod)
  }
}

// Set up mod-terminated event listener
const setupModTerminatedListener = async () => {
  // Clean up existing listener first
  cleanupModTerminatedListener()

  if (props.mod) {
    console.info(`Setting up mod-terminated listener for mod: ${props.mod.id}`)
    modTerminatedListener = await listen<string>(
      'mod-terminated',
      async event => {
        console.info(
          `[ModDetails ${props.mod?.id}] Received mod-terminated event with payload: ${event.payload}`
        )

        // Check if this event is for the current mod
        if (event.payload === props.mod?.id) {
          console.info(
            `[ModDetails ${props.mod?.id}] Event is for current mod, updating UI state`
          )
          isModRunning.value = false
          showTerminalOutput.value = false

          // Display notification that the mod has terminated
          notificationService.modStopped(props.mod.name)

          await invoke('clear_mod_logs', { id: props.mod.id })
        } else {
          console.info(
            `[ModDetails ${props.mod?.id}] Event is for a different mod (${event.payload}), ignoring`
          )
        }
      }
    )
    console.info(
      `[ModDetails ${props.mod?.id}] Mod-terminated listener successfully set up`
    )
  }
}

// Clean up mod-terminated event listener
const cleanupModTerminatedListener = () => {
  if (modTerminatedListener) {
    console.info(
      `[ModDetails ${props.mod?.id}] Removing mod-terminated listener`
    )
    modTerminatedListener()
    modTerminatedListener = null
  }
}

// Check if a mod is currently running using the backend
const checkModRunningStatus = async (modId: string) => {
  try {
    console.info(`Checking if mod ${modId} is running...`)
    const running = await invoke<boolean>('is_mod_running', { id: modId })
    console.info(`Mod ${modId} running status from backend: ${running}`)
    return running
  } catch (error) {
    console.error('Error checking mod running status:', error)
    return false
  }
}

const handleOpenGamebanana = (id: number, model_type: string) => {
  // Ensure we're always passing valid values to the parent
  const numericId = Number(id) || 0
  const modelType = String(model_type || 'Mod')

  console.info(
    `Opening GameBanana mod with ID: ${numericId}, model type: ${modelType}`
  )
  emit('open-gamebanana-mod', {
    modId: numericId,
    modelType: modelType,
  })
}

// Watch for changes to props.mod.id
watch(
  () => props.mod?.id,
  async (newModId, oldModId) => {
    // Only reset state when the actual mod ID changes (meaning a different mod was selected)
    if (newModId !== oldModId) {
      console.info(`Mod changed from ${oldModId} to ${newModId}`)

      // Clean up previous listener first
      cleanupModTerminatedListener()

      if (newModId) {
        console.info(`Checking running status for new mod: ${newModId}`)
        // Check if the mod is currently running in the backend
        const isRunning = await checkModRunningStatus(newModId)
        console.info(
          `Setting isModRunning.value = ${isRunning} based on backend state check`
        )
        isModRunning.value = isRunning

        // If the mod is running, check terminal display preference
        if (isRunning && appSettings) {
          showTerminalOutput.value =
            (await appSettings.getSetting('showTerminalOutput')) || false
          console.info(
            `Terminal output visibility set to ${showTerminalOutput.value}`
          )
        } else {
          showTerminalOutput.value = false
          console.info(`Terminal output hidden because mod is not running`)
        }

        // Set up event listener for this mod
        await setupModTerminatedListener()
      } else {
        isModRunning.value = false
        showTerminalOutput.value = false
      }
    }
  },
  { immediate: true }
)

// Watch for app settings changes to update terminal visibility
watch(
  () => appSettings,
  async () => {
    if (props.mod && isModRunning.value) {
      const newValue = await appSettings.getSetting('showTerminalOutput')
      if (newValue !== undefined) {
        showTerminalOutput.value = newValue
      }
    }
  },
  { deep: true }
)

onMounted(async () => {
  // The watcher with immediate: true will handle the initial setup
  // No need to duplicate the listener setup here
  console.info(`ModDetails mounted with mod ID: ${props.mod?.id || 'none'}`)
})

onUnmounted(() => {
  console.info(
    `[ModDetails ${props.mod?.id}] Component unmounting, cleaning up listeners`
  )
  // Clean up event listener
  cleanupModTerminatedListener()
})
</script>

<style scoped>
.mod-details {
  padding: 0;
  color: var(--theme-text);
  width: 100%;
  height: 100%;
  border-radius: 0.5rem 0.5rem 0 0;
}

.mod-content-wrapper {
  display: flex;
  flex-direction: row;
  margin: 20px;
  flex-wrap: wrap; /* Allow items to wrap to next line on small screens */
  gap: 20px; /* Add gap between main content and sidebar when they wrap */
}

.mod-main-content {
  flex: 1;
  min-width: 300px; /* Ensure the main content has a reasonable minimum width */
}

.mod-sidebar {
  max-width: 250px;
}
.mod-actions {
  margin: 20px 0;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.action-buttons {
  display: flex;
  flex-direction: column;
}

.action-button {
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 200px;
  border-radius: 1rem;
  border: 2px solid var(--theme-border);
}

.settings-button {
  margin-left: 10px;
}

.mod-path {
  margin: 0 20px;
  font-size: 1rem;
  color: var(--theme-text);
}

.welcome-message {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  text-align: center;
  color: var(--theme-text);
}

.welcome-message h2 {
  font-size: 3rem;
  margin-bottom: 8px;
}

.welcome-message p {
  font-size: 1.25rem;
}

.error-message {
  color: red;
  font-size: 1rem;
  margin-top: 8px;
}

h6 {
  margin: 0;
  color: var(--theme-text);
}

.description {
  margin-bottom: 50px;
}

.description p {
  padding: 1rem;
  font-size: 1rem;
}
</style>
