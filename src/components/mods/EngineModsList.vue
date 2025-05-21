<template>
  <div class="mods-title" v-if="mods.length >= 0">
    <div class="header">
      <h6 class="phantom-font-difficulty">Installed Mods</h6>
      <div class="scan-actions">
        <q-btn
          round
          color="primary"
          icon="refresh"
          @click="scanForMods"
          :loading="loading"
          flat
        />
        <q-btn
          color="transparent"
          text-color="var(--theme-text-secondary)"
          icon="folder"
          label="Open Mods Folder"
          flat
          @click="openModsFolder"
          class="button"
        />
      </div>
    </div>
    <hr />
  </div>

  <div class="engine-mods-container phantom-font" v-if="!isUnsupportedEngine">
    <div v-if="loading" class="loading">
      <q-spinner color="primary" size="36px" />
      <span>Scanning for installed mods...</span>
    </div>

    <div v-else-if="error" class="error-message">
      <q-icon name="error" color="negative" size="24px" />
      <span>{{ error }}</span>
    </div>

    <div v-else-if="mods.length === 0" class="no-mods">
      <q-icon name="folder_off" size="36px" />
      <span>No mods found for this engine</span>
    </div>

    <div v-else class="mods-list">
      <div
        v-for="mod in mods"
        :key="mod.folder_path"
        class="mod-item"
        @click="showModDetails(mod)"
      >
      <div class="mod-item-content">
        <div class="mod-icon">
          <q-img
            v-if="mod.icon_data"
            :src="mod.icon_data"
            spinner-color="primary"
            style="height: 48px; width: 48px"
            @error="handleImageError"
          >
            <template v-slot:error>
              <div class="fallback-icon">
                <q-icon name="image_not_supported" size="32px" />
              </div>
            </template>
          </q-img>
          <div v-else class="fallback-icon">
            <q-icon name="image_not_supported" size="32px" />
          </div>
        </div>
        <div class="mod-info">
          <div class="mod-name">
            {{ mod.name }}
            <div v-if="mod.version" class="text-caption">
              v{{ mod.version }}
            </div>
          </div>
          <div
            v-if="mod.description"
            class="mod-description"
            v-html="mod.description"
          ></div>
        </div>
        <div class="mod-actions">
          <q-toggle
            v-model="mod.enabled"
            :disable="
              engineType === 'codename' || toggleLoading[mod.folder_path]
            "
            :loading="toggleLoading[mod.folder_path]"
            @update:model-value="(val) => toggleModEnabled(mod, val)"
            color="primary"
            size="md"
            @click.stop
          />
        </div>
      </div>      
      <div v-if="mod.dependencies && Object.keys(mod.dependencies).length > 0">
        <div class="dependency-error-container q-mb-md" v-if="hasDependencyErrors(mod)" >
          <div v-for="(version, modName) in mod.dependencies" :key="modName">
            <div v-if="dependencyStates[modName]?.error" class="dependency-error-item">
            <div class="dependency-info">
              <span class="dependency-name" style="color: var(--red);">Requires <u>{{ modName }}</u>!</span>
              <span class="dependency-version" style="color: var(--red);">({{ version }})</span>
            </div>
            <div class="text-caption text-negative">
              {{ dependencyStates[modName]?.error }}
            </div>
          </div>
          </div>
        </div>
      </div>
      </div>
    </div>
  </div>
  <div v-else class="engine-mods-container phantom-font">
    <div class="loading">
      <span
        >Fridaylight can't scan for mods with the current engine type. You can
        disable this section in the mod settings.</span
      >
    </div>
  </div>

  <!-- Mod Details Dialog -->
  <MessageDialog
    v-model="showModDetailsDialog"
    title="Mod Details"
    confirmLabel="Close"
    v-if="selectedMod"
    :persistent="false"
    single-option
    disable-icon
  >
    <div class="mod-details">
      <div class="mod-header">
        <div class="mod-icon-large" v-if="selectedMod.icon_data">
          <q-img
            :src="selectedMod.icon_data"
            spinner-color="primary"
            style="height: 64px; width: 64px"
          />
        </div>
        <div class="mod-title">
          <h5 class="q-my-none">{{ selectedMod.name }}</h5>
          <div v-if="selectedMod.version" class="text-caption">
            Version: {{ selectedMod.version }}
          </div>
        </div>
      </div>

      <q-separator class="q-my-md" />

      <div v-if="selectedMod.description" class="q-mb-md">
        <div class="text-subtitle2">Description</div>
        <div v-html="selectedMod.description"></div>
      </div>

      <div v-if="selectedMod.homepage" class="q-mb-md">
        <div class="text-subtitle2">Homepage</div>
        <a @click="openUrl(selectedMod.homepage)">{{ selectedMod.homepage }}</a>
      </div>

      <div
        v-if="selectedMod.contributors && selectedMod.contributors.length > 0"
        class="q-mb-md"
      >
        <div class="text-subtitle2">Contributors</div>
        <ul>
          <li
            v-for="(contributor, index) in selectedMod.contributors"
            :key="index"
          >
            <div class="text-caption">
              {{ contributor.name }} - {{ contributor.role }}
              <span v-if="contributor.email"> ({{ contributor.email }})</span>
              <span v-if="contributor.url">
                (<a @click="openUrl(contributor.url)">{{
                  contributor.url
                }}</a>)
              </span>
            </div>
          </li>
        </ul>
      </div>

      <div v-if="selectedMod.license" class="q-mb-md">
        <div class="text-subtitle2">License</div>
        <div>{{ selectedMod.license }}</div>
      </div>      <div v-if="selectedMod.credits" class="q-mb-md">
        <div class="text-subtitle2">Credits</div>
        <div v-html="selectedMod.credits"></div>
      </div>

      <div v-if="selectedMod.dependencies && Object.keys(selectedMod.dependencies).length > 0" class="q-mb-md">
        <div class="text-subtitle2">Dependencies</div>
        <ul>
          <li v-for="(version, modName) in selectedMod.dependencies" :key="modName" class="dependency-item">
            <div class="dependency-info">
              <span class="dependency-name">{{ modName }}</span>
              <span class="dependency-version">({{ version }})</span>
              <q-spinner v-if="dependencyStates[modName]?.checking" color="primary" size="1em" />
              <q-icon v-else-if="dependencyStates[modName]?.installed" name="check_circle" color="positive" size="1.2em" />
              <q-icon v-else-if="dependencyStates[modName]?.installed === false" name="error" color="negative" size="1.2em" />
              <q-icon v-else name="help" color="warning" size="1.2em" />
            </div>
            <div v-if="dependencyStates[modName]?.error" class="text-caption text-negative">
              {{ dependencyStates[modName]?.error }}
            </div>
          </li>
        </ul>
      </div>

      <div class="text-subtitle2">Folder Path</div>
      <div class="text-caption q-mb-md">{{ selectedMod.folder_path }}</div>
    </div>
  </MessageDialog>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { openUrl, revealItemInDir } from "@tauri-apps/plugin-opener";
import { getEngineModsFolderPath } from "@utils/index";
import MessageDialog from "@components/modals/MessageDialog.vue";

interface ModMetadataFile {
  name: string;
  description?: string;
  folder_path: string;
  config_file_path?: string;
  icon_file_path?: string;
  icon_data?: string;
  enabled: boolean;
  version?: string;
  homepage?: string;
  contributors?: ContributorMetadata[];
  license?: string;
  credits?: string;
  dependencies?: DependencyMetadata[];
}

interface DependencyMetadata {
  [modName: string]: string; // Key: mod name, Value: version
}

interface ContributorMetadata {
  name: string;
  role: string;
  email?: string;
  url?: string;
}

interface EngineModsResponse {
  engine_type: string;
  executable_path: string;
  mods: ModMetadataFile[];
}

const props = defineProps({
  executablePath: {
    type: String,
    default: "",
  },
  engineType: {
    type: String,
    default: "",
  },
  autoScan: {
    type: Boolean,
    default: true,
  },
  customModsFolder: {
    type: String,
    default: null,
  },
});

const mods = ref<ModMetadataFile[]>([]);
const loading = ref(false);
const error = ref("");
const hasScanned = ref(false);
const toggleLoading = ref<Record<string, boolean>>({}); // Track loading state for each toggle

// Dialog state for mod details
const showModDetailsDialog = ref(false);
const selectedMod = ref<ModMetadataFile | null>(null);

// State for dependency checking
interface DependencyState {
  checking: boolean;
  installed?: boolean;
  error?: string;
}

const dependencyStates = ref<Record<string, DependencyState>>({});

const isUnsupportedEngine = computed(() => {
  return ["pre-vslice", "kade", "other", "unknown"].includes(props.engineType);
});

// Check if a mod has any dependency errors
const hasDependencyErrors = (mod: ModMetadataFile): boolean => {
  if (!mod.dependencies) return false;
  
  // Check if any dependencies have been marked as not installed
  for (const dependencyName of Object.keys(mod.dependencies)) {
    if (dependencyStates.value[dependencyName]?.installed === false) {
      return true;
    }
  }
  
  return false;
};

const scanForMods = async () => {
  if (!props.executablePath || !props.engineType) {
    error.value = "Missing executable path or engine type";
    return;
  }

  // Handle "other" engine type as if it doesn't have a type
  if (props.engineType === "other" || props.engineType === "unknown") {
    error.value = "Custom engine types are not supported for mod scanning";
    return;
  }

  loading.value = true;
  error.value = "";
  mods.value = [];
  // Reset dependency states when refreshing
  dependencyStates.value = {};

  try {
    const response = await invoke<EngineModsResponse>("find_engine_mod_files", {
      executablePath: props.executablePath,
      engineType: props.engineType,
      modsFolder: props.customModsFolder,    });

    mods.value = response.mods;
    hasScanned.value = true;
    
    // Check dependencies for all mods after they are loaded
    if (mods.value.length > 0) {
      checkAllModsDependencies();
    }
  } catch (e: any) {
    console.error("Failed to scan for mods:", e);
    error.value = e.toString();
  } finally {
    loading.value = false;
  }
};

const handleImageError = () => {
  // This function can be expanded to handle image loading errors if needed
  console.warn("Failed to load mod icon");
};

// Show detailed information about a mod
const showModDetails = (mod: ModMetadataFile) => {
  selectedMod.value = mod;
  showModDetailsDialog.value = true;
  
  // Check dependencies if they exist and they haven't been checked yet
  if (mod.dependencies && Object.keys(mod.dependencies).length > 0) {
    checkDependencies(mod);
  }
};

// Check if a mod's dependencies are installed
const checkDependencies = async (mod: ModMetadataFile) => {
  if (!mod.dependencies) return;
  
  const modFolder = getEngineModsFolderPath(
    props.executablePath,
    props.customModsFolder
  );
  
  if (!modFolder) {
    console.error("Could not determine mods folder path");
    return;
  }

  // Check each dependency
  for (const [dependencyName, requiredVersion] of Object.entries(mod.dependencies)) {
    // Skip if already checking or if status is already known
    if (dependencyStates.value[dependencyName]?.checking) {
      continue;
    }
    
    // Set initial checking state if not already set
    if (!dependencyStates.value[dependencyName]) {
      dependencyStates.value[dependencyName] = { checking: true };
    }
    
    try {
      await invoke("check_mod_dependency", {
        modsFolderPath: modFolder,
        dependencyName: dependencyName,
        requiredVersion: requiredVersion
      });
      
      // If no error is thrown, the dependency is installed correctly
      dependencyStates.value[dependencyName] = { 
        checking: false, 
        installed: true 
      };
    } catch (e: any) {
      console.error(`Dependency check failed for ${dependencyName}:`, e);
      dependencyStates.value[dependencyName] = { 
        checking: false, 
        installed: false,
        error: e.toString()
      };
    }  }
};

// Check dependencies for all mods
const checkAllModsDependencies = async () => {
  const modFolder = getEngineModsFolderPath(
    props.executablePath,
    props.customModsFolder
  );
  
  if (!modFolder) {
    console.error("Could not determine mods folder path");
    return;
  }

  // Process each mod with dependencies
  for (const mod of mods.value) {
    if (mod.dependencies && Object.keys(mod.dependencies).length > 0) {
      for (const [dependencyName, requiredVersion] of Object.entries(mod.dependencies)) {
        if (!dependencyStates.value[dependencyName]) {
          dependencyStates.value[dependencyName] = { checking: true };
          
          try {
            await invoke("check_mod_dependency", {
              modsFolderPath: modFolder,
              dependencyName: dependencyName,
              requiredVersion: requiredVersion
            });
            
            dependencyStates.value[dependencyName] = { 
              checking: false, 
              installed: true 
            };
          } catch (e: any) {
            console.error(`Dependency check failed for ${dependencyName}:`, e);
            dependencyStates.value[dependencyName] = { 
              checking: false, 
              installed: false,
              error: e.toString()
            };
          }
        }
      }
    }
  }
};

// Toggle a mod's enabled state
const toggleModEnabled = async (mod: ModMetadataFile, enable: boolean) => {
  if (!props.executablePath || !props.engineType) {
    console.error("Missing executable path or engine type");
    return;
  }

  // Skip for Codename Engine
  if (props.engineType === "codename") {
    console.warn("Mod enabling/disabling not supported for Codename Engine");
    return;
  }

  // Set loading state for this specific toggle
  toggleLoading.value[mod.folder_path] = true;

  try {
    const result = await invoke<{
      success: boolean;
      enabled: boolean;
      message: string;
    }>("toggle_mod_enabled", {
      executablePath: props.executablePath,
      modFolderPath: mod.folder_path,
      engineType: props.engineType,
      enable,
    });

    console.log(
      `Toggled mod ${mod.name} to ${result.enabled ? "enabled" : "disabled"}`
    );

    // Update the mod's enabled state based on the result
    mod.enabled = result.enabled;
  } catch (e: any) {
    console.error(`Failed to toggle mod ${mod.name}:`, e);
    // Reset the toggle to its previous state on error
    mod.enabled = !enable;
  } finally {
    // Clear loading state
    toggleLoading.value[mod.folder_path] = false;
  }
};

// Open mods folder in file explorer
const openModsFolder = async () => {
  try {
    const folderPath = getEngineModsFolderPath(
      props.executablePath,
      props.customModsFolder
    );
    if (!folderPath) {
      error.value = "Could not determine mods folder path";
      return;
    }

    console.log("Opening mods folder:", folderPath);
    await revealItemInDir(folderPath);
  } catch (e: any) {
    console.error("Failed to open mods folder:", e);
    error.value = `Failed to open mods folder: ${e.toString()}`;
  }
};

// Watch for changes in executable path or engine type
watch(
  () => [props.executablePath, props.engineType],
  () => {
    if (props.autoScan && props.executablePath && props.engineType) {
      scanForMods();
    }
  },
  { immediate: true }
);

onMounted(() => {
  if (props.autoScan && props.executablePath && props.engineType) {
    scanForMods();
  }
});
</script>

<style scoped>
.button {
  border-radius: 1rem;
}
.engine-mods-container {
  margin-top: 0.25rem;
  background: var(--theme-surface);
  border-radius: 8px;
  padding: 16px;
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

h6 {
  margin: 0;
  margin-top: 2rem;
  color: var(--theme-text);
  width: fit-content;
  display: inline-flex;
}

.engine-badge {
  font-size: 14px;
  padding: 4px 8px;
}

.mods-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 16px;
}

.mod-item-content {
  display: flex;
  align-items: center;
  padding: 10px;
  background: var(--theme-card);
  border-radius: 6px;
  transition: background-color 0.2s;
  border: 1px solid var(--theme-border);
  cursor: pointer;
}

.mod-item-content:hover {
  background: var(--theme-border);
}

.mod-icon {
  width: 48px;
  height: 48px;
  margin-right: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.fallback-icon {
  width: 48px;
  height: 48px;
  background: var(--theme-surface);
  border: 1px solid var(--theme-border);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--theme-text-secondary);
}

.mod-info {
  flex: 1;
}

.mod-name {
  font-weight: bold;
  font-size: 16px;
  color: var(--theme-text);
  margin-bottom: 4px;
}

.mod-description {
  font-size: 14px;
  color: var(--theme-text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
}

.loading,
.no-mods,
.error-message {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 32px;
  color: var(--theme-text-secondary);
  text-align: center;
  gap: 12px;
}

.error-message {
  color: #ff5252;
}

.dependency-error-container {
  margin: 0 1rem;
  padding: 0.5rem 1rem;
  border: 1px solid var(--red);
  border-top: none;
  border-radius: 0 0 1rem 1rem;
  background: var(--theme-background);
}

.scan-actions {
  display: inline-flex;
  gap: 0.5rem;
  margin-left: auto;
  margin-top: 1.5rem;
}

/* Mod Details Dialog Styles */
.mod-details {
  max-width: 500px;
}

.mod-header {
  display: flex;
  align-items: center;
  margin-bottom: 10px;
}

.mod-icon-large {
  margin-right: 16px;
}

.mod-title {
  flex: 1;
}

.text-subtitle2 {
  font-weight: 500;
  color: var(--theme-text);
  margin-bottom: 4px;
}

.text-caption {
  font-size: 14px;
  color: var(--theme-text-secondary);
}

a {
  cursor: pointer;
}

.dependency-item {
  margin-bottom: 8px;
}

.dependency-info {
  display: flex;
  align-items: center;
  gap: 5px;
}

.dependency-name {
  font-weight: 500;
}

.dependency-version {
  color: var(--theme-text-secondary);
  font-size: 0.9em;
}
</style>
