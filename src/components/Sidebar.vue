<template>
  <div class="layout-container">
    <!-- Sidebar with resize handle -->
    <div 
      class="sidebar"
      :style="{ width: `${sidebarWidth}px` }"
    >
      <ModList 
        :mods="mods" 
        :selectedModId="selectedMod?.id"
        @select-mod="selectMod"
        @add-mod="selectModFolder"
        @add-mods-folder="selectModsParentFolder"
        @delete-mod="deleteMod"
        @open-settings="openAppSettings"
        @reorder-mods="handleModsReorder"
        class="modlist"
      />
      
      <!-- GameBanana button at the bottom of the sidebar -->
      <div class="gamebanana-button-container phantom-font">
        <q-btn 
          :class="{'gamebanana-button': true, 'active-gamebanana': showGameBanana}"
          color="secondary" 
          icon="cloud_download" 
          label="Browse GameBanana" 
          @click="toggleGameBanana"
          flat
        />
      </div>
      
      <!-- Resize handle -->
      <div 
        class="resize-handle"
        @mousedown="startResize"
      >
        <div class="resize-handle-indicator"></div>
      </div>
    </div>
     <div class="main-content-area">
      <!-- Use transition for switching between components -->
      <Transition name="fade" mode="out-in" :duration="200">
        <!-- Show ModDetails when a mod is selected and GameBanana is not shown -->
        <component 
          :is="!showGameBanana ? ModDetails : GameBananaBrowser"
          v-bind="!showGameBanana ? {
            mod: selectedMod,
            error: launchError || '',
            'onUpdate:mod': updateModDetails,
            'onLaunch-mod': launchMod,
            'onOpen-settings': openSettingsModal,
            key: selectedMod?.id || 'no-mod'
          } : {
            key: 'gamebanana-browser'
          }"
        />
      </Transition>
    </div>

    <!-- Settings Modal -->
    <ModSettingsModal 
      v-model="showSettingsModal" 
      :mod="selectedMod"
      @save="saveModChanges"
      @change-folder="changeModFolder"
      @select-executable="selectExecutable"
    />
    
    <!-- App Settings Modal -->
    <AppSettingsModal
      v-model="showAppSettingsModal"
      @save="saveAppSettings"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import ModList from './mods/ModList.vue';
import ModDetails from './mods/ModDetails.vue';
import ModSettingsModal from './mods/ModSettingsModal.vue';
import AppSettingsModal from './AppSettingsModal.vue';
import GameBananaBrowser from './mods/GameBananaBrowser.vue';

// TypeScript declaration for db
declare global {
  interface Window {
    db: any;
  }
}

interface ModInfo {
  id: string;
  name: string;
  path: string;
  executable_path?: string;
  icon_data?: string;
  banner_data?: string;
  logo_data?: string;
  version?: string;
  engine_type?: string;
  display_order?: number;
}

// Use the props without storing in a variable to avoid the unused variable warning
defineProps<{
  modelValue: boolean
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void;
  (e: 'resize', width: number): void;
}>();

// Default width and min/max constraints
const sidebarWidth = ref(250);
const minWidth = 150;
const maxWidth = 400;

// Resize functionality
let isResizing = false;

const startResize = (_e: MouseEvent) => {
  isResizing = true;
  document.addEventListener('mousemove', handleResize);
  document.addEventListener('mouseup', stopResize);
  document.body.style.userSelect = 'none';
};

const handleResize = (e: MouseEvent) => {
  if (!isResizing) return;
  
  // Calculate new width based on mouse position
  let newWidth = e.clientX;
  
  // Apply constraints
  if (newWidth < minWidth) newWidth = minWidth;
  if (newWidth > maxWidth) newWidth = maxWidth;
  
  // Update width
  sidebarWidth.value = newWidth;
  
  // Emit resize event
  emit('resize', newWidth);
};

const stopResize = () => {
  isResizing = false;
  document.removeEventListener('mousemove', handleResize);
  document.removeEventListener('mouseup', stopResize);
  document.body.style.userSelect = '';
};

// Mod list and selection
const mods = ref<ModInfo[]>([]);
const selectedMod = ref<ModInfo | null>(null);
const launchError = ref<string | null>(null);
const showSettingsModal = ref(false);
const showGameBanana = ref(false);
const showAppSettingsModal = ref(false);

// Load mods on component mount
onMounted(async () => {
  emit('resize', sidebarWidth.value);
  try {
    await loadModsFromDatabase();
    
    // Load and apply app settings
    await loadAppSettings();
    
    // Add event listener for refreshing the mods list
    window.addEventListener('refresh-mods', handleRefreshMods);
  } catch (error) {
    console.error('Error loading mods:', error);
  }
});

// Load mods from the database
const loadModsFromDatabase = async () => {
  try {
    // Check if db is initialized
    if (!window.db) {
      console.warn('Database not initialized yet, waiting...');
      // Wait a bit and check again
      await new Promise(resolve => setTimeout(resolve, 500));
      
      // If still not initialized, fall back to memory
      if (!window.db) {
        console.warn('Database still not initialized, falling back to memory');
        return await loadMods();
      }
    }
    
    const result = await window.db.select('SELECT * FROM mods ORDER BY display_order ASC');
    
    if (result && result.length > 0) {
      mods.value = result;
      
      // If there's at least one mod, select the first one
      if (result.length > 0) {
        selectMod(result[0]);
      }
    } else {
      // If no mods in database, load from memory
      await loadMods();
    }
  } catch (error) {
    console.error('Failed to load mods from database:', error);
    // Fallback to in-memory mods
    await loadMods();
  }
};

// Load all mods from backend memory
const loadMods = async () => {
  try {
    const modList = await invoke<ModInfo[]>('get_mods');
    
    // Sort mods by display_order if available, otherwise keep original order
    mods.value = modList.sort((a, b) => {
      if (a.display_order !== undefined && b.display_order !== undefined) {
        return a.display_order - b.display_order;
      }
      return 0;
    });
  } catch (error) {
    console.error('Failed to load mods:', error);
  }
};

// Save a mod to the database
const saveModToDatabase = async (mod: ModInfo) => {
  try {
    // Check if db is initialized
    if (!window.db) {
      console.warn('Database not initialized yet, cannot save mod');
      return;
    }
    
    await window.db.execute(
      `INSERT OR REPLACE INTO mods (id, name, path, executable_path, icon_data, banner_data, logo_data, version, engine_type, display_order)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)`,
      [mod.id, mod.name, mod.path, mod.executable_path || null, mod.icon_data || null, mod.banner_data || null, mod.logo_data || null, mod.version || null, mod.engine_type || null, mod.display_order || 9999]
    );
  } catch (error) {
    console.error('Failed to save mod to database:', error);
  }
};

const selectModFolder = async () => {
  try {
    const modFolderPath = await invoke<string>('select_mod_folder');
    if (modFolderPath) {
      const modInfo = await invoke<ModInfo>('add_mod', { path: modFolderPath });
      mods.value.push(modInfo);
      selectMod(modInfo);
      await saveModToDatabase(modInfo);
    }
  } catch (error) {
    console.error('Failed to select mod folder:', error);
  }
};

const selectModsParentFolder = async () => {
  try {
    // Call the Rust command to select a folder containing multiple mods
    const addedMods = await invoke<ModInfo[]>('select_mods_parent_folder');
    
    if (addedMods && addedMods.length > 0) {
      // Add each mod to the mods array
      for (const modInfo of addedMods) {
        mods.value.push(modInfo);
        await saveModToDatabase(modInfo);
      }
      
      // Select the first added mod
      selectMod(addedMods[0]);
      
      // Show a success message
      console.log(`Successfully added ${addedMods.length} mods`);
    }
  } catch (error) {
    console.error('Failed to select mods parent folder:', error);
  }
};

const selectMod = (mod: ModInfo) => {
  console.log('selectMod called with mod:', mod);
  selectedMod.value = mod;
  console.log('selectedMod.value after setting:', selectedMod.value);
  launchError.value = null;
  showGameBanana.value = false; // Switch to mod details view when selecting a mod
};

const toggleGameBanana = () => {
  showGameBanana.value = true;
};

// Function to sync mods with backend
const syncModsWithBackend = async () => {
  try {
    console.log('Syncing mods with backend...');
    await invoke('sync_mods_from_database', { modsData: mods.value });
    console.log('Successfully synced mods with backend');
  } catch (error) {
    console.error('Failed to sync mods with backend:', error);
  }
};

// Function to handle mod updates, saving to the database and syncing with backend
const updateModDetails = async (updatedMod: ModInfo) => {
  if (selectedMod.value) {
    // Find and update the mod in the mods array
    const modIndex = mods.value.findIndex(m => m.id === updatedMod.id);
    if (modIndex !== -1) {
      mods.value[modIndex] = updatedMod;
    }
    
    // Update the selected mod
    selectedMod.value = updatedMod;
    
    // Save to database and sync with backend
    await saveModToDatabase(updatedMod);
    
    // Sync the updated mods list with the backend
    await syncModsWithBackend();
  }
};

const launchMod = async (modId: string) => {
  try {
    await invoke('launch_mod', { id: modId });
  } catch (error) {
    console.error('Failed to launch mod:', error);
    launchError.value = 'Failed to launch mod';
  }
};

const openSettingsModal = () => {
  showSettingsModal.value = true;
};

const saveModChanges = async (updatedMod: ModInfo) => {
  await updateModDetails(updatedMod);
};

const changeModFolder = async () => {
  try {
    const modFolderPath = await invoke<string>('select_mod_folder');
    if (modFolderPath && selectedMod.value) {
      selectedMod.value.path = modFolderPath;
      await saveModToDatabase(selectedMod.value);
    }
  } catch (error) {
    console.error('Failed to change mod folder:', error);
  }
};

const selectExecutable = async () => {
  try {
    const executablePath = await invoke<string>('select_executable');
    if (executablePath && selectedMod.value) {
      selectedMod.value.executable_path = executablePath;
      await saveModToDatabase(selectedMod.value);
    }
  } catch (error) {
    console.error('Failed to select executable:', error);
  }
};

const deleteMod = async (modId: string) => {
  try {
    // Remove mod from the mods array
    mods.value = mods.value.filter(mod => mod.id !== modId);
    
    // If the deleted mod was the selected mod, clear the selection
    if (selectedMod.value?.id === modId) {
      selectedMod.value = null;
    }
    
    // Delete mod from the database
    if (window.db) {
      await window.db.execute('DELETE FROM mods WHERE id = $1', [modId]);
    }
    
    // Sync the updated mods list with the backend
    await syncModsWithBackend();
  } catch (error) {
    console.error('Failed to delete mod:', error);
  }
};

// Function to handle refreshing mods list
const handleRefreshMods = async () => {
  console.log('Refreshing mods list');
  await loadModsFromDatabase();
};

// Function to open app settings modal
const openAppSettings = () => {
  showAppSettingsModal.value = true;
};

// Function to save app settings
const saveAppSettings = (settings: any) => {
  console.log('App settings saved:', settings);
  
  // Apply the accent color immediately - ensure it's a string
  const colorValue = typeof settings.accentColor === 'string' 
    ? settings.accentColor 
    : (settings.accentColor?.value || '#FF0088');
  
  document.documentElement.style.setProperty('--q-primary', colorValue);
  
  // If we need to use the install location in other parts of the app,
  // we could store it in a ref or emit an event here
};

// Load and apply app settings
const loadAppSettings = async () => {
  try {
    if (!window.db) {
      console.warn('Database not initialized yet, cannot load settings');
      return;
    }
    
    // Load accent color
    const accentColorResult = await window.db.select('SELECT value FROM settings WHERE key = $1', ['accentColor']);
    if (accentColorResult && accentColorResult.length > 0) {
      const accentColorValue = accentColorResult[0].value;
      
      let colorValue = '#FF0088'; // Default color
      
      // Check if the value is a JSON string and parse it if needed
      if (typeof accentColorValue === 'string') {
        try {
          if (accentColorValue.startsWith('{') && accentColorValue.includes('value')) {
            // It's a JSON object string
            const colorObj = JSON.parse(accentColorValue);
            if (colorObj && colorObj.value) {
              colorValue = colorObj.value;
            }
          } else {
            // It's a direct color string
            colorValue = accentColorValue;
          }
        } catch (e) {
          console.error('Failed to parse accent color:', e);
          // Use the string value directly if parsing fails
          colorValue = accentColorValue;
        }
      }
      
      // Apply the accent color to CSS custom properties
      document.documentElement.style.setProperty('--q-primary', colorValue);
      console.log('Applied accent color from settings:', colorValue);
    }
  } catch (error) {
    console.error('Failed to load and apply app settings:', error);
  }
};

// Function to handle reordering mods
const handleModsReorder = async (newOrder: ModInfo[]) => {
  console.log('Reordering mods, new order:', newOrder.map(mod => mod.name));
  console.log('Before update display_order values:', newOrder.map(mod => ({ name: mod.name, display_order: mod.display_order })));
  
  // Update the display_order field for each mod based on its position in the array
  const updatedMods = newOrder.map((mod, index) => {
    return {
      ...mod,
      display_order: index
    };
  });
  
  console.log('After update display_order values:', updatedMods.map(mod => ({ name: mod.name, display_order: mod.display_order })));
  
  // Update the mods array with the new order and display_order values
  mods.value = updatedMods;
  
  try {
    // Check if db is initialized
    if (!window.db) {
      console.warn('Database not initialized yet, cannot save mod order');
      return;
    }
    
    // Start a transaction to update all mods at once
    await window.db.execute('BEGIN TRANSACTION');
    
    // Save the updated order to the database
    for (const mod of mods.value) {
      await window.db.execute(
        `UPDATE mods SET display_order = $1 WHERE id = $2`,
        [mod.display_order, mod.id]
      );
    }
    
    // Commit the transaction
    await window.db.execute('COMMIT');
    console.log('Successfully saved mod order to database');
    
    // Sync the updated mods list with the backend
    await syncModsWithBackend();
  } catch (error) {
    // Rollback if there's an error
    if (window.db) {
      await window.db.execute('ROLLBACK');
    }
    console.error('Failed to save mod order:', error);
  }
};

// Watch for changes to the mods list
watch(mods, async (newMods) => {
  // Update all mods in the database when the list changes
  for (const mod of newMods) {
    await saveModToDatabase(mod);
  }
}, { deep: true });

// Clean up event listeners
onUnmounted(() => {
  if (isResizing) {
    document.removeEventListener('mousemove', handleResize);
    document.removeEventListener('mouseup', stopResize);
    document.body.style.userSelect = '';
  }
  
  // Remove refresh-mods event listener
  window.removeEventListener('refresh-mods', handleRefreshMods);
});
</script>

<style scoped>
.layout-container {
  display: flex;
  width: 100%;
  height: 100vh;
  overflow: hidden;
}

.sidebar {
  height: 100%;
  background-color: transparent;
  position: relative;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
}

.main-content-area {
  flex-grow: 1;
  height: 100%;
  padding: 16px;
  box-sizing: border-box;
  overflow: auto;
  background: var(--theme-surface);
  border-radius: 1rem 1rem 0 0;
  margin: 8px;
  }

.resize-handle {
  position: absolute;
  top: 0;
  right: -5px;
  width: 10px;
  height: 100%;
  cursor: col-resize;
  z-index: 100;
  display: flex;
  justify-content: center;
  border-radius: 1rem;
}

.resize-handle-indicator {
  width: 1px;
  height: 100%;
  border-right: 1px dotted transparent;
  transition: border-right 0.2s ease-out;
  border-radius: 1rem;
}

.resize-handle:hover .resize-handle-indicator,
.resize-handle:active .resize-handle-indicator {
  border-right: 1px dashed var(--theme-border);
}

.modlist {
  flex: 1;
  overflow-y: auto;
}

.gamebanana-button-container {
  padding: 12px;
  text-align: center;
  border-top: 1px solid var(--theme-border);
  margin-top: auto;
}

.gamebanana-button {
  width: 100%;
  transition: all 0.2s ease;
}
</style>