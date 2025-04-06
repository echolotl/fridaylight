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
        class="modlist"
      />
      
      <!-- Resize handle -->
      <div 
        class="resize-handle"
        @mousedown="startResize"
      >
        <div class="resize-handle-indicator"></div>
      </div>
    </div>
    
    <!-- Main content area -->
    <div class="main-content-area">
      <ModDetails 
        :mod="selectedMod" 
        :error="launchError"
        @update:mod="updateModDetails"
        @launch-mod="launchMod"
        @open-settings="openSettingsModal"
      />
    </div>

    <!-- Settings Modal -->
    <ModSettingsModal 
      v-model="showSettingsModal" 
      :mod="selectedMod"
      @save="saveModChanges"
      @change-folder="changeModFolder"
      @select-executable="selectExecutable"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import ModList from './mods/ModList.vue';
import ModDetails from './mods/ModDetails.vue';
import ModSettingsModal from './mods/ModSettingsModal.vue';

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
}

const props = defineProps<{
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

const startResize = (e: MouseEvent) => {
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

// Load mods on component mount
onMounted(async () => {
  emit('resize', sidebarWidth.value);
  try {
    await loadModsFromDatabase();
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
    
    const result = await window.db.select('SELECT * FROM mods');
    
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
    mods.value = modList;
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
      `INSERT OR REPLACE INTO mods (id, name, path, executable_path, icon_data, banner_data, logo_data, version, engine_type)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`,
      [mod.id, mod.name, mod.path, mod.executable_path || null, mod.icon_data || null, mod.banner_data || null, mod.logo_data || null, mod.version || null, mod.engine_type || null]
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

const selectMod = (mod: ModInfo) => {
  selectedMod.value = mod;
  launchError.value = null;
};

const updateModDetails = async (updatedMod: ModInfo) => {
  if (selectedMod.value) {
    // Find and update the mod in the mods array
    const modIndex = mods.value.findIndex(m => m.id === updatedMod.id);
    if (modIndex !== -1) {
      mods.value[modIndex] = updatedMod;
    }
    
    // Update the selected mod
    selectedMod.value = updatedMod;
    
    // Save to database
    await saveModToDatabase(updatedMod);
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
}

.main-content-area {
  flex-grow: 1;
  height: 100%;
  padding: 16px;
  box-sizing: border-box;
  overflow: auto;
  background: rgba(0, 0, 0, 0.25);
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
  border-right: 1px dashed rgba(0, 0, 0, 0.25);
}
.modlist {
  height: 100%;
}
</style>