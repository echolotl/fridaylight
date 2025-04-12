<template>
  <div class="layout-container">
    <!-- Sidebar with resize handle -->
    <div 
      class="sidebar"
      :style="{ width: `${sidebarWidth}px` }"
    >      <ModList 
        :mods="mods"
        :folders="folders" 
        :selectedModId="selectedMod?.id"
        @select-mod="selectMod"
        @add-mod="selectModFolder"
        @add-mods-folder="selectModsParentFolder"
        @delete-mod="deleteMod"
        @delete-folder="deleteFolder"
        @create-folder="createFolder"
        @add-mod-to-folder="addModToFolder"
        @open-settings="openAppSettings"
        @reorder-items="handleModsReorder"
        @update-mod="updateModDetails"
        @update-folder="updateFolderDetails"
        class="modlist"
      />
      
      <!-- GameBanana button at the bottom of the sidebar -->
      <div class="gamebanana-button-container phantom-font-difficulty">
        <q-btn 
          :class="{'gamebanana-button': true, 'active-gamebanana': showGameBanana}"
          color="secondary" 
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
    </div>     <div class="main-content-area">
        <!-- Show ModDetails when a mod is selected and GameBanana is not shown -->
        <Transition name="fade" mode="out-in">
          <div :key="!showGameBanana ? (selectedMod ? selectedMod.id : 'no-mod') : 'gamebanana-browser'">
            <component 
            :is="!showGameBanana ? ModDetails : GameBananaBrowser"
            v-bind="!showGameBanana ? {
              mod: selectedMod,
              error: launchError || '',
              'onUpdate:mod': updateModDetails,
              'onLaunch-mod': launchMod,
              'onOpen-settings': openSettingsModal
            } : {}"
          />
          </div>
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
import { Mod, Folder, DisplayItem } from '../types';

// TypeScript declaration for db
declare global {
  interface Window {
    db: any;
  }
}

// Using ModInfo as an alias for Mod for backward compatibility in this file
type ModInfo = Mod;

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
const folders = ref<Folder[]>([]);
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
    await loadFoldersFromDatabase(); // Load folders after mods
    
    // Load and apply app settings
    await loadAppSettings();

    showGameBanana.value = false; // Hide GameBanana by default
    
    // Add event listener for refreshing the mods list
    window.addEventListener('refresh-mods', handleRefreshMods);
  } catch (error) {
    console.error('Error loading mods:', error);
  }
});

// Load mods from the database
const loadModsFromDatabase = async () => {
  try {
    if (!window.db || !window.db.service) {
      console.warn('Database service not initialized yet, waiting...');
      // Wait a bit and check again
      await new Promise(resolve => setTimeout(resolve, 500));
      
      // If still not initialized, fall back to memory
      if (!window.db || !window.db.service) {
        console.warn('Database service still not initialized, falling back to memory');
        return await loadMods();
      }
    }
    
    // Use the DatabaseService to get all mods
    const dbService = window.db.service;
    const processedMods = await dbService.getAllMods();
    
    if (processedMods && processedMods.length > 0) {
      mods.value = processedMods;
      
      // If there's at least one mod, select the first one
      if (processedMods.length > 0) {
        selectMod(processedMods[0]);
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

// Load folders from the database
const loadFoldersFromDatabase = async () => {
  try {
    if (!window.db || !window.db.service) {
      console.warn('Database service not initialized yet when loading folders');
      return;
    }
    
    console.log('Loading folders from database');
    
    // Use the DatabaseService to get all folders
    const dbService = window.db.service;
    const loadedFolders = await dbService.getAllFolders();
    
    if (loadedFolders && loadedFolders.length > 0) {
      console.log(`Found ${loadedFolders.length} folders in database`);
      
      // Update the reactive folders ref
      folders.value = loadedFolders;
      console.log('Folders loaded successfully:', folders.value);
    } else {
      console.log('No folders found in database');
      folders.value = [];
    }
  } catch (error) {
    console.error('Failed to load folders from database:', error);
    folders.value = [];
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
    if (!window.db || !window.db.service) {
      console.warn('Database service not initialized yet, cannot save mod');
      return;
    }
    
    // Use the DatabaseService to save the mod
    const dbService = window.db.service;
    await dbService.saveMod(mod);
    console.log('Mod saved successfully to database:', mod.name);
  } catch (error) {
    console.error('Failed to save mod to database:', error);
  }
};

const selectModFolder = async () => {
  try {
    const modFolderPath = await invoke<string>('select_mod_folder');
    if (modFolderPath) {
      const modInfo = await invoke<ModInfo>('add_mod', { path: modFolderPath });
      
      // Ensure the mod has an engine object before saving
      if (!modInfo.engine) {
        modInfo.engine = {
          engine_type: modInfo.engine_type || 'unknown',
          engine_name: '',
          engine_icon: '',
          mods_folder: false,
          mods_folder_path: ''
        };
      }
      
      mods.value.push(modInfo);
      selectMod(modInfo);
      await saveModToDatabase(modInfo);
      console.log('Mod added and saved to database successfully:', modInfo.name);
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
  console.log('Updating mod details:', updatedMod.name, 'folder_id:', updatedMod.folder_id);
  
  // Find and update the mod in the mods array regardless of whether it's selected
  const modIndex = mods.value.findIndex(m => m.id === updatedMod.id);
  if (modIndex !== -1) {
    // Preserve display_order to prevent mod from moving in the list
    const currentDisplayOrder = mods.value[modIndex].display_order;
    updatedMod.display_order = currentDisplayOrder;
    
    // Update the mod in the array
    mods.value[modIndex] = updatedMod;
    console.log('Updated mod in array at index', modIndex);
    
    // If this is the selected mod, update that reference too
    if (selectedMod.value && selectedMod.value.id === updatedMod.id) {
      selectedMod.value = updatedMod;
    }
    
    // Save to database and sync with backend
    await saveModToDatabase(updatedMod);
    
    // Sync the updated mods list with the backend
    await syncModsWithBackend();
  } else {
    console.warn('Could not find mod with ID', updatedMod.id, 'in mods array');
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
    
  // Delete mod from the database using the service
    if (window.db && window.db.service) {
      await window.db.service.deleteMod(modId);
      // No need to sync with backend separately since the service handles it
    }
  } catch (error) {
    console.error('Failed to delete mod:', error);
  }
};

// Function to handle refreshing mods list
const handleRefreshMods = async () => {
  console.log('Refreshing mods list');
  await loadModsFromDatabase();
  await loadFoldersFromDatabase();
};

// Create a new folder
const createFolder = async (folder: Folder) => {
  console.log('Creating new folder:', folder);
  try {
    if (!window.db || !window.db.service) {
      console.error('Database service not initialized, cannot create folder');
      return;
    }
    
    // Use the DatabaseService to save the folder
    await window.db.service.saveFolder(folder);
    
    console.log('Folder created successfully in database');
    
    // Add the folder to the local folders array
    folders.value.push({
      ...folder,
      mods: [] // Ensure mods is initialized as an empty array
    });
    
    console.log('Updated folders list:', folders.value);
  } catch (error) {
    console.error('Error creating folder:', error);
  }
};

// Function to handle folder updates, saving to the database
const updateFolderDetails = async (updatedFolder: Folder) => {
  console.log('Updating folder details:', updatedFolder.name, 'mods count:', updatedFolder.mods.length);
  
  // Find and update the folder in the folders array
  const folderIndex = folders.value.findIndex(f => f.id === updatedFolder.id);
  
  try {
    if (!window.db || !window.db.service) {
      console.error('Database service not initialized, cannot save folder');
      return;
    }
    
    if (folderIndex !== -1) {
      // Existing folder: preserve display_order to prevent folder from moving in the list
      const currentDisplayOrder = folders.value[folderIndex].display_order;
      updatedFolder.display_order = currentDisplayOrder;
      
      // Update the folder in the array
      folders.value[folderIndex] = updatedFolder;
      console.log('Updated existing folder in array at index', folderIndex);
    } else {
      // New folder: add it to the folders array
      console.log('Adding new folder to array:', updatedFolder.name);
      folders.value.push(updatedFolder);
    }
    
    // Save to database
    await window.db.service.saveFolder(updatedFolder);
    console.log('Folder saved successfully to database:', updatedFolder.name);
    
    // No need to explicitly update mods here, since the updateModDetails function
    // handles that when mods are dragged in/out of folders
  } catch (error) {
    console.error('Failed to save folder to database:', error);
  }
};

// Add a mod to a folder
const addModToFolder = async (data: { modId: string, folderId: string }) => {
  const { modId, folderId } = data;
  console.log(`Adding mod ${modId} to folder ${folderId}`);
  
  try {
    if (!window.db || !window.db.service) {
      console.error('Database service not initialized, cannot add mod to folder');
      return;
    }
    
    // Use the DatabaseService to move the mod to the folder
    await window.db.service.moveModToFolder(modId, folderId);
    
    console.log('Mod added to folder in database');
    
    // Update the local folders array
    const folderIndex = folders.value.findIndex(f => f.id === folderId);
    if (folderIndex !== -1) {
      // Add the mod ID to the folder's mods array if it's not already there
      if (!folders.value[folderIndex].mods.includes(modId)) {
        folders.value[folderIndex].mods.push(modId);
      }
      console.log(`Updated folder ${folderId} with mod ${modId}`);
      
      // Update the mod's folder_id in our local mods array
      const modIndex = mods.value.findIndex(m => m.id === modId);
      if (modIndex !== -1) {
        mods.value[modIndex].folder_id = folderId;
      }
      
      // Force a refresh of the UI
      folders.value = [...folders.value];
    }
  } catch (error) {
    console.error('Error adding mod to folder:', error);
  }
};

// Delete a folder
const deleteFolder = async (folderId: string) => {
  console.log(`Deleting folder with ID: ${folderId}`);
  
  try {
    if (!window.db || !window.db.service) {
      console.error('Database service not initialized, cannot delete folder');
      return;
    }
    
    // Use the DatabaseService to delete the folder
    await window.db.service.deleteFolder(folderId);
    
    console.log('Folder deleted from database');
    
    // Remove the folder from the local folders array
    folders.value = folders.value.filter(f => f.id !== folderId);
    
    // Update any mods that were in this folder
    mods.value.forEach(mod => {
      if (mod.folder_id === folderId) {
        mod.folder_id = undefined;
      }
    });
    
    console.log('Updated folders list after deletion');
  } catch (error) {
    console.error('Error deleting folder:', error);
  }
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
const handleModsReorder = async (newOrder: DisplayItem[]) => {
  console.log('Reordering display items, new order:', 
    newOrder.map(item => `${item.type}:${item.type === 'mod' ? item.data.name : item.data.name}`));
  
  // Process the reordered items by type (mods or folders)
  for (let index = 0; index < newOrder.length; index++) {
    const item = newOrder[index];
    
    if (item.type === 'mod') {
      // Find the corresponding mod in our mods array and update its display_order
      const modIndex = mods.value.findIndex(m => m.id === item.data.id);
      if (modIndex !== -1) {
        mods.value[modIndex].display_order = index;
      }
    } else if (item.type === 'folder') {
      // Find the corresponding folder in our folders array and update its display_order
      const folderIndex = folders.value.findIndex(f => f.id === item.data.id);
      if (folderIndex !== -1) {
        folders.value[folderIndex].display_order = index;
      }
    }
  }
  
  // Force reactivity update
  mods.value = [...mods.value];
  folders.value = [...folders.value];
    try {
    // Check if db is initialized
    if (!window.db) {
      console.warn('Database not initialized yet, cannot save mod order');
      return;
    }
    
    // Start a transaction to update all mods and folders at once
    await window.db.execute('BEGIN TRANSACTION');
    
    // Save the updated mod order to the database
    for (const mod of mods.value) {
      await window.db.execute(
        `UPDATE mods SET display_order = $1 WHERE id = $2`,
        [mod.display_order, mod.id]
      );
    }
    
    // Save the updated folder order to the database
    for (const folder of folders.value) {
      await window.db.execute(
        `UPDATE folders SET display_order = $1 WHERE id = $2`,
        [folder.display_order, folder.id]
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

.main-content-area {
  scrollbar-width: none;
}
</style>