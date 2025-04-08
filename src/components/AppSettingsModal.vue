<template>
  <q-dialog v-model="showModal" persistent>
    <q-card class="settings-modal phantom-font">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">App Settings</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-card-section>
        <div class="text-subtitle1 q-mb-md">Appearance</div>
        
        <q-select
          v-model="settings.accentColor"
          :options="accentColorOptions"
          label="Accent Color"
          outlined
          class="q-mb-md"
        >
          <template v-slot:option="scope">
            <q-item v-bind="scope.itemProps">
              <q-item-section avatar>
                <div class="color-preview" :style="{ backgroundColor: scope.opt.value }"></div>
              </q-item-section>
              <q-item-section>
                <q-item-label>{{ scope.opt.label }}</q-item-label>
              </q-item-section>
            </q-item>
          </template>
          
          <template v-slot:selected>
            <div class="row items-center">
              <div class="color-preview q-mr-sm" :style="{ 
                backgroundColor: typeof settings.accentColor === 'string' ? 
                  settings.accentColor : 
                  settings.accentColor?.value || '#FF0088'
              }"></div>
              <div>
                {{ 
                  typeof settings.accentColor === 'string' ? 
                  accentColorOptions.find(opt => opt.value === settings.accentColor)?.label : 
                  settings.accentColor?.label || 'Custom'
                }}
              </div>
            </div>
          </template>
        </q-select>
        
        <div class="text-subtitle1 q-mt-lg q-mb-md">Installation</div>
        
        <q-input 
          v-model="settings.installLocation" 
          label="Default Install Location" 
          outlined 
          readonly
          class="q-mb-md"
        >
          <template v-slot:append>
            <q-btn round flat icon="folder" @click="selectInstallLocation" />
          </template>
        </q-input>
        
        <div class="text-subtitle1 q-mt-lg q-mb-md">About</div>
        
        <div class="text-body2 q-mb-sm">
          FNF Mod Launcher v1.0.0
        </div>
        
        <div class="text-subtitle1 q-mt-lg q-mb-md">Acknowledgements</div>
        
        <div class="acknowledgements q-pa-md">
          <p>This application uses the following open source projects:</p>
          <ul>
            <li><strong>Tauri</strong> - Cross-platform app framework</li>
            <li><strong>Vue.js</strong> - Frontend framework</li>
            <li><strong>Quasar</strong> - UI component library</li>
            <li><strong>Rust</strong> - Backend language</li>
          </ul>
          <p>Special thanks to the Friday Night Funkin' community!</p>
        </div>
      </q-card-section>

      <q-card-actions align="right">
        <q-btn flat label="Cancel" color="primary" v-close-popup @click="cancel" />
        <q-btn flat label="Save" color="primary" @click="save" v-close-popup />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface ColorOption {
  label: string;
  value: string;
}

interface AppSettings {
  accentColor: string | ColorOption;
  installLocation: string;
}

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['update:modelValue', 'save']);

const settings = ref<AppSettings>({
  accentColor: '#DB2955', // Default magenta color
  installLocation: 'C:\\Users\\Public\\Documents\\FNF Mods'
});

const accentColorOptions = [
  { label: 'Pink', value: '#DB2955' },
  { label: 'Blue', value: '#235789' },
  { label: 'Green', value: '#35CE8D' },
  { label: 'Purple', value: '#C490D1' },
  { label: 'Orange', value: '#FE621D' },
  { label: 'Yellow', value: '#f7c548'},
  { label: 'Red', value: '#C03221' },
  { label: 'Cyan', value: '#39A9DB' }
];

const showModal = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
});

// Load saved settings when modal is opened
watch(() => props.modelValue, async (newVal) => {
  if (newVal) {
    await loadSettings();
  }
});

const loadSettings = async () => {
  try {
    if (!window.db) {
      console.warn('Database not initialized yet, using default settings');
      return;
    }
    
    // Load settings from database
    const accentColorResult = await window.db.select('SELECT value FROM settings WHERE key = $1', ['accentColor']);
    if (accentColorResult && accentColorResult.length > 0) {
      const colorValue = accentColorResult[0].value;
      console.log('Loaded accent color from DB:', colorValue);
      
      // Try to parse as JSON if it looks like an object
      if (typeof colorValue === 'string' && colorValue.startsWith('{') && colorValue.includes('value')) {
        try {
          const colorObj = JSON.parse(colorValue);
          console.log('Parsed color object:', colorObj);
          settings.value.accentColor = colorObj;
        } catch (e) {
          console.error('Failed to parse accent color as JSON:', e);
          settings.value.accentColor = colorValue;
        }
      } else {
        // It's a direct string value
        settings.value.accentColor = colorValue;
      }
    }
    
    const installLocationResult = await window.db.select('SELECT value FROM settings WHERE key = $1', ['installLocation']);
    if (installLocationResult && installLocationResult.length > 0) {
      settings.value.installLocation = installLocationResult[0].value;
    }
    
    console.log('Settings loaded from database:', settings.value);
  } catch (error) {
    console.error('Failed to load settings from database:', error);
  }
};

const selectInstallLocation = async () => {
  try {
    // Use the Rust backend to select a folder
    const folderPath = await invoke<string>('select_settings_folder');
    if (folderPath) {
      settings.value.installLocation = folderPath;
    }
  } catch (error) {
    console.error('Failed to select install location:', error);
  }
};

const save = async () => {
  try {
    if (!window.db) {
      console.warn('Database not initialized yet, cannot save settings');
      return;
    }
    
    // For accent color, we need to handle both string and object values
    const accentColorValue = typeof settings.value.accentColor === 'string' 
      ? settings.value.accentColor 
      : JSON.stringify(settings.value.accentColor);
    
    console.log('Saving accent color to DB:', accentColorValue);
    
    // Save settings to database
    await window.db.execute(
      'INSERT OR REPLACE INTO settings (key, value) VALUES ($1, $2)',
      ['accentColor', accentColorValue]
    );
    
    await window.db.execute(
      'INSERT OR REPLACE INTO settings (key, value) VALUES ($1, $2)',
      ['installLocation', settings.value.installLocation]
    );
    
    // Apply the accent color to CSS custom properties
    // Make sure we're setting a string value
    const colorValue = typeof settings.value.accentColor === 'string' 
      ? settings.value.accentColor 
      : (settings.value.accentColor?.value || '#FF0088');
    
    document.documentElement.style.setProperty('--q-primary', colorValue);
    
    console.log('Settings saved to database:', settings.value);
    emit('save', settings.value);
  } catch (error) {
    console.error('Failed to save settings to database:', error);
  }
};

const cancel = () => {
  // Reset any unsaved changes by reloading from DB
  loadSettings();
};

// Initialize settings on component creation
loadSettings();
</script>

<style scoped>
.settings-modal {
  width: 500px;
  max-width: 90vw;
}

.color-preview {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  border: 1px solid rgba(255, 255, 255, 0.3);
}

.acknowledgements {
  background-color: rgba(0, 0, 0, 0.1);
  border-radius: 4px;
  max-height: 200px;
  overflow-y: auto;
}

.acknowledgements ul {
  padding-left: 20px;
}

.acknowledgements li {
  margin-bottom: 4px;
}
</style>