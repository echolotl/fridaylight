<template>
  <div class="mod-details phantom-font" v-if="mod">
    <ModBanner 
      :mod="mod" 
      @update:title="updateTitle"
      @open-settings="$emit('open-settings')"
      style="margin-bottom: 2rem;"
    />
    
    <div class="mod-actions">
      <div class="action-buttons">
        <q-btn 
          color="primary" 
          class="play-button phantom-font" 
          size="lg" 
          @click="$emit('launch-mod', mod.id)"
          :disabled="!mod.executable_path"
        >
          PLAY
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
      <div class="mod-path" v-if="showDetails">
      <p>Location: {{ mod.path }}</p>
      <p v-if="mod.executable_path">Executable: {{ mod.executable_path }}</p>
      <p v-if="mod.version">Version: {{ mod.version }}</p>
      <p v-if="mod.engine_type">Engine: {{ formatEngineType(mod.engine_type) }}</p>
    </div>

    <div class="description" v-if="mod.description">
      <h5 class="phantom-font-difficulty">Description</h5>
      <hr/>
      <p>{{ mod.description }}</p>
    </div>
    
    <!-- Show engine-specific mods list if mod has engine type and executable path -->
    <div v-if="mod.engine.engine_type && mod.executable_path && mod.engine.mods_folder">
      <EngineModsList
        :executablePath="mod.executable_path"
        :engineType="mod.engine.engine_type"
        :customModsFolder="mod.engine.mods_folder_path"
      />
    </div>
  </div>
  
  <!-- Show welcome message when no mod is selected -->
  <div v-else class="welcome-message phantom-font">
    <h2>Welcome to Friday Night Funkin' Mod Loader</h2>
    <p>Select a mod from the sidebar to get started</p>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import ModBanner from './ModBanner.vue';
import EngineModsList from './EngineModsList.vue';

interface Engine {
  engine_type: string;
  engine_name: string;
  engine_icon: string;
  mods_folder: boolean;
  mods_folder_path: string;
}

interface Mod {
  id: string;
  name: string;
  path: string;
  executable_path?: string;
  icon_data?: string;
  banner_data?: string;
  logo_data?: string | null;
  version?: string;
  engine_type?: string;  // Kept for backward compatibility (probably will remove for full release)
  engine: Engine;        // New extended engine information
  display_order?: number;
  description?: string;
}
const props = defineProps({
  mod: {
    type: Object as () => Mod | null,
    default: null
  },
  error: {
    type: String,
    default: ''
  }
});

const showDetails = ref(false);
const emit = defineEmits(['update:mod', 'launch-mod', 'open-settings']);

const updateTitle = (newTitle: string) => {
  if (props.mod) {
    const updatedMod = { ...props.mod, name: newTitle };
    emit('update:mod', updatedMod);
  }
};

const formatEngineType = (engineType: string) => {
  const engineTypes: Record<string, string> = {
    'vanilla': 'Vanilla',
    'psych': 'Psych Engine',
    'codename': 'Codename Engine',
    'kade': 'Kade Engine',
    'pre-vslice': 'Pre-VSlice',
    'other': 'Other'
  };
  
  return engineTypes[engineType] || engineType;
};
</script>

<style scoped>
.mod-details {
  padding: 0;
  color: var(--theme-text);
}

.mod-actions {
  margin: 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.action-buttons {
  display: flex;
  flex-direction: column;
}

.play-button {
  display: flex;
  align-items: center;
  justify-content: center;
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
  font-size: 1.5rem;
  margin-bottom: 8px;
}

.welcome-message p {
  font-size: 1rem;
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