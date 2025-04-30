<template>
  <q-scroll-area class="mod-details phantom-font" v-if="mod">
    <ModBanner
      :mod="mod"
      @update:title="updateTitle"
      @open-settings="$emit('open-settings')"
      style="margin-bottom: 2rem"
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
              @click="handleModAction"
              :disabled="!mod.executable_path"
            >
              {{ isModRunning ? 'STOP' : 'PLAY' }}
            </q-btn>

            <div v-if="!mod.executable_path" class="error-message">
              No executable found in this mod folder
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
          :modId="mod.id"
          :isVisible="isModRunning && showTerminalOutput"
          @close="showTerminalOutput = false"
          @clear="clearModLogs"
        />
        
        <div class="mod-path" v-if="showDetails">
          <p>Location: {{ mod.path }}</p>
          <p v-if="mod.executable_path">Executable: {{ mod.executable_path }}</p>
          <p v-if="mod.version">Version: {{ mod.version }}</p>
          <p v-if="mod.engine_type">
            Engine: {{ formatEngineType(mod.engine_type) }}
          </p>
        </div>

        <div class="description" v-if="mod.description">
          <h5 class="phantom-font-difficulty">Description</h5>
          <hr />
          <p>{{ mod.description }}</p>
        </div>

        <!-- Show engine-specific mods list if mod has engine type and executable path -->
        <div
          v-if="
            mod.engine.engine_type && mod.executable_path && mod.engine.mods_folder
          "
        >
          <EngineModsList
            :executablePath="mod.executable_path"
            :engineType="mod.engine.engine_type"
            :customModsFolder="mod.engine.mods_folder_path"
          />
        </div>
      </div>

      <!-- Right column: Contributor information -->
      <div class="mod-sidebar" v-if="mod.contributors && mod.contributors.length > 0">
        <ContributorInfobox 
          :contributors="mod.contributors"
          :folder_path="mod.path"
        />
      </div>
    </div>
  </q-scroll-area>

  <!-- Show welcome message when no mod is selected -->
  <div v-else class="welcome-message phantom-font">
    <h2 class="phantom-font-display">Welcome to Fridaylight!</h2>
    <p>Select or add a mod from the sidebar to get started</p>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted} from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import ModBanner from "@mods/ModBanner.vue";
import EngineModsList from "@mods/EngineModsList.vue";
import TerminalOutput from "@common/TerminalOutput.vue";
import ContributorInfobox from "@common/ContributorInfobox.vue";
import { Mod} from "@main-types";
import { useQuasar } from "quasar";
import { StoreService } from "@services/storeService";

const appSettings = StoreService.getInstance();

const $q = useQuasar();
const props = defineProps({
  mod: {
    type: Object as () => Mod | null,
    default: null,
  },
  error: {
    type: String,
    default: "",
  },
});

const showDetails = ref(false);
const isModRunning = ref(false);
const showTerminalOutput = ref(false); 

const emit = defineEmits(["update:mod", "launch-mod", "open-settings", "stop-mod"]);
let modTerminatedListener: (() => void) | null = null;

// Function to check if the mod is currently running
const checkModRunningStatus = async () => {
  if (!props.mod) return;
  
  try {
    const running = await invoke("is_mod_running", { id: props.mod.id });
    console.log(`Checking if mod ${props.mod.name} (${props.mod.id}) is running:`, running);
    isModRunning.value = !!running;
    
    // If the mod is running, respect the app settings for terminal visibility
    if (isModRunning.value && appSettings) {
      showTerminalOutput.value = await appSettings.getSetting("showTerminalOutput");
      // If the setting is not found, default to false
      if (showTerminalOutput.value === undefined) {
        showTerminalOutput.value = false;
      }
      console.log(`Setting terminal visibility to ${showTerminalOutput.value} based on app settings`);
    } else if (!isModRunning.value) {
      showTerminalOutput.value = false;
    }
  } catch (error) {
    console.error("Error checking mod running status:", error);
    showTerminalOutput.value = false;
  }
};

// Handle the play/stop button click
const handleModAction = async () => {
  if (!props.mod) return;
  
  if (isModRunning.value) {
    // Stop the mod
    try {
      await invoke("stop_mod", { id: props.mod.id });
      isModRunning.value = false;
      showTerminalOutput.value = false; // Hide terminal immediately
      $q.notify({
        type: "positive",
        message: `Stopped ${props.mod.name}`,
        position: "bottom-right",
        timeout: 2000
      });
    } catch (error) {
      console.error("Failed to stop mod:", error);
      $q.notify({
        type: "negative",
        message: `Failed to stop ${props.mod.name}`,
        caption: String(error),
        position: "bottom-right",
        timeout: 3000,
      });
    }
  } else {
    // Launch the mod
    emit("launch-mod", props.mod.id);
    
    // Show terminal output based on user settings
    if (appSettings) {
      showTerminalOutput.value = await appSettings.getSetting("showTerminalOutput");
      // If the setting is not found, default to false
      if (showTerminalOutput.value === undefined) {
        showTerminalOutput.value = false;
      }
      console.log(`Setting terminal visibility to ${showTerminalOutput.value} based on app settings`);
    }
    
    // Update UI state immediately, will be confirmed when we receive the event
    isModRunning.value = true;
  }
};

const clearModLogs = async (modId: string) => {
  if (!modId) return;
  
  try {
    console.log(`Clearing logs for mod: ${modId}`);
    await invoke("clear_mod_logs", { id: modId });
    console.log(`Logs cleared for mod: ${modId}`);
  } catch (error) {
    console.error("Failed to clear mod logs:", error);
  }
};

const updateTitle = (newTitle: string) => {
  if (props.mod) {
    const updatedMod = { ...props.mod, name: newTitle };
    emit("update:mod", updatedMod);
  }
};

const formatEngineType = (engineType: string) => {
  const engineTypes: Record<string, string> = {
    vanilla: "Vanilla",
    psych: "Psych Engine",
    codename: "Codename Engine",
    kade: "Kade Engine",
    "pre-vslice": "Pre-VSlice",
    other: "Other",
  };

  return engineTypes[engineType] || engineType;
};

// Set up mod-terminated event listener
const setupModTerminatedListener = async () => {
  if (modTerminatedListener) {
    modTerminatedListener();
    modTerminatedListener = null;
  }
  
  if (props.mod) {
    modTerminatedListener = await listen<string>('mod-terminated', (event) => {
      // Check if this event is for the current mod
      if (event.payload === props.mod?.id) {
        console.log(`Received mod-terminated event for ${props.mod?.id}`);
        isModRunning.value = false;
        showTerminalOutput.value = false;
        
        // Display notification that the mod has terminated
        $q.notify({
          type: 'info',
          message: `${props.mod.name} has terminated`,
          position: "bottom-right",
          timeout: 2000,
        });
      }
    });
  }
};

// Watch for changes to props.mod and check running state
watch(() => props.mod, async (newMod) => {
  if (newMod) {
    // Check if the mod is running
    await checkModRunningStatus();
    
    // Set up event listener for this mod
    await setupModTerminatedListener();
  } else {
    isModRunning.value = false;
    
    // Clean up listener if no mod is selected
    if (modTerminatedListener) {
      modTerminatedListener();
      modTerminatedListener = null;
    }
  }
}, { immediate: true, deep: true });

// Watch for app settings changes to update terminal visibility
watch(
  () => props.mod,
  async (newMod) => {
    if (newMod && appSettings) {
      const newValue = await appSettings.getSetting("showTerminalOutput");
      if (newValue !== undefined && isModRunning.value) {
        showTerminalOutput.value = newValue;
      }
    }
  }
);

onMounted(async () => {
  // Check initial running state
  if (props.mod) {
    await checkModRunningStatus();
    await setupModTerminatedListener();
  }
});

onUnmounted(() => {
  // Clean up event listener
  if (modTerminatedListener) {
    modTerminatedListener();
    modTerminatedListener = null;
  }
});
</script>

<style scoped>
.mod-details {
  padding: 0;
  color: var(--theme-text);
  width: 100%;
  height: 100%;
  border-radius: .5rem .5rem 0 0;
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

h5 {
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
