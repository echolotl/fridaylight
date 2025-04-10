<template>  <q-dialog v-model="showModal" persistent>
    <q-card class="settings-modal phantom-font">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6 phantom-font-difficulty">App Settings</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <div class="settings-layout">
        <!-- Sidebar Navigation -->
        <div class="settings-sidebar">
          <q-list padding>
            <q-item 
              v-for="section in settingsSections" 
              :key="section.id"
              clickable 
              :active="activeSection === section.id"
              @click="activeSection = section.id"
              active-class="settings-active-item"
            >
              <q-item-section avatar>
                <q-icon :name="section.icon" />
              </q-item-section>
              <q-item-section>
                {{ section.label }}
              </q-item-section>
            </q-item>
          </q-list>
        </div>

        <!-- Main Content Area -->
        <div class="settings-content">
          <!-- Appearance Section -->
          <q-card-section v-show="activeSection === 'appearance'">
            <div class="text-subtitle1 q-mb-md">Appearance</div>
            
            <div class="theme-toggle-container row q-mb-md">
              <q-item tag="label" class="full-width">
                <q-item-section>
                  <q-item-label>Use System Theme</q-item-label>
                  <q-item-label caption>Automatically match your OS theme</q-item-label>
                </q-item-section>
                <q-item-section side>
                  <q-toggle v-model="settings.useSystemTheme" color="primary" />
                </q-item-section>
              </q-item>
            </div>
            
            <div class="theme-toggle-container row q-mb-md" v-if="!settings.useSystemTheme">
              <q-item tag="label" class="full-width">
                <q-item-section>
                  <q-item-label>Enable Light Theme</q-item-label>
                  <q-item-label caption>Switch between light and dark mode</q-item-label>
                </q-item-section>
                <q-item-section side>
                  <q-toggle v-model="settings.enableLightTheme" color="primary" />
                </q-item-section>
              </q-item>
            </div>

            <q-select
              v-model="settings.accentColor"
              :options="accentColorOptions"
              label="Accent Color"
              outlined
              class="q-mb-md selector"
            >
              <template v-slot:option="scope">
                <q-item v-bind="scope.itemProps">
                  <q-item-section avatar>
                    <div class="color-preview" :style="{ backgroundColor: scope.opt.value }"></div>
                  </q-item-section>
                  <q-item-section>
                    <q-item-label text-color="var(--theme-text)">{{ scope.opt.label }}</q-item-label>
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
          </q-card-section>
          
          <!-- Installation Section -->
          <q-card-section v-show="activeSection === 'installation'">
            <div class="text-subtitle1 q-mb-md">Installation</div>
            
            <q-input 
              v-model="settings.installLocation" 
              label="Default Install Location" 
              outlined 
              readonly
              class="q-mb-md"
            >
              <template v-slot:append>
                <div class="icon">
                <q-btn round flat icon="folder" @click="selectInstallLocation" />
              </div>
              </template>
            </q-input>
          </q-card-section>
          
          <!-- About Section -->
          <q-card-section v-show="activeSection === 'about'">
            <div class="text-subtitle1 q-mb-md">About</div>
            
            <div class="text-body2 q-mb-sm">
              Fridaylight v0.2.0
              <br />
              A mod manager for Friday Night Funkin'.
              <br />
              <br />
              <div class="text-caption text-grey-6">Made with:</div>
              <div class="logo-grid">
                <img src="/images/vue.svg" alt="Vue" width="32" height="32" class="logo" @click="openUrl('https://vuejs.org/')"/>
                <img src="/images/tauri.svg" alt="Tauri" width="32" height="32" class="logo" @click="openUrl('https://v2.tauri.app/')" />
                <img src="/images/quasar.svg" alt="Quasar" width="32" height="32" class="logo" @click="openUrl('https://quasar.dev/')" />
              </div>
            </div>
          </q-card-section>
        </div>
      </div>

      <q-card-actions align="right">
        <q-btn flat label="Cancel" color="primary" v-close-popup @click="cancel" />
        <q-btn flat label="Save" color="primary" @click="save" v-close-popup />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { ref, watch, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { openUrl } from '@tauri-apps/plugin-opener';

interface ColorOption {
  label: string;
  value: string;
}

interface AppSettings {
  accentColor: string | ColorOption;
  installLocation: string;
  enableLightTheme: boolean;
  useSystemTheme: boolean;
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
  installLocation: 'C:\\Users\\Public\\Documents\\FNF Mods',
  enableLightTheme: false, // Default to dark theme
  useSystemTheme: true // Default to using system theme
});

// Sidebar navigation sections
const settingsSections = [
  { id: 'appearance', label: 'Appearance', icon: 'format_paint' },
  { id: 'installation', label: 'Installation', icon: 'folder' },
  { id: 'about', label: 'About', icon: 'info' }
];

// Track the active section
const activeSection = ref('appearance');

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
    
    const installLocationResult = await window.db.select('SELECT value FROM settings WHERE key = $1', ['installLocation']);    if (installLocationResult && installLocationResult.length > 0) {
      settings.value.installLocation = installLocationResult[0].value;
    }
      // Load theme setting
    const themeResult = await window.db.select('SELECT value FROM settings WHERE key = $1', ['enableLightTheme']);
    if (themeResult && themeResult.length > 0) {
      // Convert string 'true'/'false' to boolean
      settings.value.enableLightTheme = themeResult[0].value === 'true';
    }
    
    // Load system theme preference
    const systemThemeResult = await window.db.select('SELECT value FROM settings WHERE key = $1', ['useSystemTheme']);
    if (systemThemeResult && systemThemeResult.length > 0) {
      // Convert string 'true'/'false' to boolean
      settings.value.useSystemTheme = systemThemeResult[0].value === 'true';
    }
    
    // Apply theme immediately upon loading
    await updateTheme();
    
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

// Get the current system theme (light or dark)
const getSystemTheme = (): boolean => {
  return window.matchMedia && window.matchMedia('(prefers-color-scheme: light)').matches;
};

// Apply theme function
const updateTheme = async () => {
  // Determine if we should use light theme based on settings
  let useLightTheme = settings.value.enableLightTheme;
  
  // If using system theme, override with the system preference
  if (settings.value.useSystemTheme) {
    useLightTheme = getSystemTheme();
  }
  
  console.log('Applying theme:', useLightTheme ? 'light' : 'dark');
  
  // First check if we're running on Windows 11
  const isWindows11 = await invoke<boolean>('is_windows_11');
  console.log('Is Windows 11:', isWindows11);
  
  // Apply CSS classes for theme
  if (useLightTheme) {
    document.body.classList.add('light-theme');
    document.body.classList.remove('dark-theme');
  } else {
    document.body.classList.remove('light-theme');
    document.body.classList.add('dark-theme');
  }
  
  // Apply solid theme if not on Windows 11
  if (!isWindows11) {
    document.body.classList.add('solid-theme');
    
    // Remove transparent background styles
    document.documentElement.style.setProperty('--transparent-bg-override', 'none');
    
    // Set background to solid color instead of transparent
    const bgColor = useLightTheme ? 'var(--theme-bg)' : 'var(--theme-bg)';
    document.body.style.background = bgColor;
    document.querySelector('.q-layout')?.setAttribute('style', 'background: ' + bgColor + ' !important');
  } else {
    document.body.classList.remove('solid-theme');
    document.documentElement.style.setProperty('--transparent-bg-override', 'transparent');
    document.body.style.background = 'transparent';
    
    // Call the Rust backend to apply Mica effect (Windows only)
    try {
      await invoke('change_mica_theme', { 
        window: 'main',  // Main window label
        dark: !useLightTheme  // Invert because true = dark theme in the Rust function
      });
      console.log('Applied Mica theme effect:', !useLightTheme ? 'dark' : 'light');
    } catch (error) {
      console.error('Failed to apply Mica effect:', error);
      // Non-fatal error, the app will still work without Mica
    }
  }
  
  // Dispatch an event so other components can know about theme changes
  window.dispatchEvent(new CustomEvent('theme-changed', { 
    detail: { 
      isLightTheme: useLightTheme,
      useSystemTheme: settings.value.useSystemTheme
    } 
  }));
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
      // Save theme settings
    await window.db.execute(
      'INSERT OR REPLACE INTO settings (key, value) VALUES ($1, $2)',
      ['enableLightTheme', String(settings.value.enableLightTheme)]
    );
    
    await window.db.execute(
      'INSERT OR REPLACE INTO settings (key, value) VALUES ($1, $2)',
      ['useSystemTheme', String(settings.value.useSystemTheme)]
    );
    
    // Apply the accent color to CSS custom properties
    // Make sure we're setting a string value
    const colorValue = typeof settings.value.accentColor === 'string' 
      ? settings.value.accentColor 
      : (settings.value.accentColor?.value || '#FF0088');
      document.documentElement.style.setProperty('--q-primary', colorValue);
    
    // Apply theme
    await updateTheme();
    
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

onMounted(() => {
  // Really silly fix for dropdown background, oh Quasar why
  const style = document.createElement('style');
  style.innerHTML = `
    .q-menu {
      background-color: var(--theme-card) !important;
      color: var(--theme-text) !important;
    }
    .q-item {
      color: var(--theme-text) !important;
    }
  `;
  document.head.appendChild(style);
});

// Initialize settings on component creation
loadSettings();
</script>

<style scoped>
.settings-modal {
  width: 700px;
  height: 500px;
  max-width: 90vw;
  max-height: 90vh;
  background-color: var(--theme-border);
  color: var(--theme-text);
  border: var(--theme-border) 2px solid;
  backdrop-filter: blur(10px);
  scrollbar-width: none;
}

.settings-layout {
  display: flex;
  height: calc(100% - 100px); /* Account for header and footer */
  overflow: hidden;
}

.settings-sidebar {
  width: 200px;
  border-right: 1px solid var(--theme-border);
  overflow-y: auto;
}

.settings-content {
  flex: 1;
  overflow-y: auto;
  padding-right: 8px;
}

.settings-active-item {
  background-color: var(--q-primary) !important;
  color: white !important;
}

.settings-sidebar .q-icon {
  color: var(--theme-text-secondary) !important;
}

.settings-active-item .q-icon {
  color: white !important;
}

.color-preview {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  border: 1px solid var(--theme-border);
}

.acknowledgements {
  background-color: var(--theme-surface);
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

.text-caption {
  color: var(--theme-text-secondary);
}

.logo-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(32px, 1fr));
  gap: 8px;
  margin-top: 16px;
}

.logo {
  cursor: pointer;
  transition: transform 0.2s;
}

.logo:hover {
  transform: scale(1.05);
}
</style>