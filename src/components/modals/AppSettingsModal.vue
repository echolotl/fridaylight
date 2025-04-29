<template>
  <q-dialog v-model="showModal" persistent>
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
        <q-scroll-area class="settings-content">
          <!-- Appearance Section -->
          <q-card-section v-show="activeSection === 'appearance'">
            <div class="text-subtitle1 q-mb-md">Appearance</div>

            <div class="theme-toggle-container row q-mb-md">
              <!-- Use our ThemePreview component -->
              <ThemePreview
                :themeName="getThemeName()"
                :accentColor="getAccentColor()"
                :compactMode="settings.compactMode"
              />
              <q-item tag="label" class="full-width">
                <q-item-section>
                  <q-item-label>Use System Theme</q-item-label>
                  <q-item-label caption
                    >Automatically match your OS theme
                    (Light/Dark)</q-item-label
                  >
                </q-item-section>
                <q-item-section side>
                  <q-toggle v-model="settings.useSystemTheme" color="primary" />
                </q-item-section>
              </q-item>
            </div>

            <div class="theme-selector q-mb-md" v-if="!settings.useSystemTheme">
              <q-select
                v-model="settings.theme"
                :options="themeOptions"
                label="Theme"
                outlined
                class="q-mb-md selector"
              >
                <template v-slot:option="scope">
                  <q-item v-bind="scope.itemProps">
                    <q-item-section>
                      <q-item-label text-color="var(--theme-text)">{{
                        scope.opt.label
                      }}</q-item-label>
                    </q-item-section>
                  </q-item>
                </template>
              </q-select>
            </div>

            <q-item tag="label" class="q-mb-md">
              <q-item-section>
                <q-item-label>Compact Mode</q-item-label>
                <q-item-label caption
                  >Minimize sidebar width and show only icons</q-item-label
                >
              </q-item-section>
              <q-item-section side>
                <q-toggle v-model="settings.compactMode" color="primary" />
              </q-item-section>
            </q-item>

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
                    <div
                      class="color-preview"
                      :style="{ backgroundColor: scope.opt.value }"
                    ></div>
                  </q-item-section>
                  <q-item-section>
                    <q-item-label text-color="var(--theme-text)">{{
                      scope.opt.label
                    }}</q-item-label>
                  </q-item-section>
                </q-item>
              </template>

              <template v-slot:selected>
                <div class="row items-center">
                  <div
                    class="color-preview q-mr-sm"
                    :style="{
                      backgroundColor:
                        typeof settings.accentColor === 'string'
                          ? settings.accentColor
                          : settings.accentColor?.value || '#FF0088',
                    }"
                  ></div>
                  <div>
                    {{
                      typeof settings.accentColor === "string"
                        ? accentColorOptions.find(
                            (opt) => opt.value === settings.accentColor
                          )?.label
                        : settings.accentColor?.label || "Custom"
                    }}
                  </div>
                </div>
              </template>
            </q-select>

            <div class="q-mt-lg">
              <div class="text-subtitle2 q-mb-md">Custom CSS</div>
              <q-input
                v-model="settings.customCSS"
                type="textarea"
                outlined
                class="custom-css-editor"
                label="Custom CSS Rules"
                placeholder="/* Example: */&#10;:root {&#10;  --my-custom-color: #FF00FF;&#10;}&#10;&#10;.q-card {&#10;  border-radius: 16px !important;&#10;}"
                autogrow
                :rows="8"
                :max-rows="12"
              />
              <div class="text-caption q-mt-sm">
                Custom CSS allows you to override the application's default
                styles. Changes will apply after saving and restarting the app.
              </div>
            </div>
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
                  <q-btn
                    round
                    flat
                    icon="folder"
                    @click="selectInstallLocation"
                  />
                </div>
              </template>
            </q-input>

            <q-item tag="label" class="q-mt-md">
              <q-item-section>
                <q-item-label>Validate FNF Mods</q-item-label>
                <q-item-label caption>
                  Toggle validation of FNF mods before installation
                </q-item-label>
              </q-item-section>
              <q-item-section side>
                <q-toggle v-model="settings.validateFnfMods" color="primary" />
              </q-item-section>
            </q-item>

            <q-item tag="label" class="q-mt-md">
              <q-item-section>
                <q-item-label>Show Terminal Output</q-item-label>
                <q-item-label caption>
                  Display terminal output when running mods
                </q-item-label>
              </q-item-section>
              <q-item-section side>
                <q-toggle
                  v-model="settings.showTerminalOutput"
                  color="primary"
                />
              </q-item-section>
            </q-item>
          </q-card-section>

          <!-- About Section -->
          <q-card-section v-show="activeSection === 'about'">
            <div class="text-subtitle1 q-mb-md">About</div>

            <div class="text-body2 q-mb-sm">
              Fridaylight v0.8.4
              <br />
              A mod manager for Friday Night Funkin'.
              <br />
              <br />

              <div class="text-subtitle1 q-mb-md">Credits</div>
              <div class="acknowledgements q-mb-md">
                <ul style="background-color: transparent">
                  <li>
                    <a @click="openUrl('https://www.echolotl.lol')">echolotl</a>
                    - Coder, Designer, Director, Creator of Fridaylight
                  </li>
                  <li>
                    <a
                      @click="openUrl('https://gamebanana.com/members/1844732')"
                      >Cracsthor</a
                    >
                    - Creator of PhantomMuff Full + Difficulty fonts
                  </li>
                  <li>
                    <a
                      @click="openUrl('https://gamebanana.com/members/3083321')"
                      >NoizDynamic</a
                    >
                    - Creator of Tardling font
                  </li>
                </ul>
              </div>
              <div class="text-subtitle1 q-mb-md">Created with</div>
              <div class="center-credits">
                <div class="logo-grid">
                  <img
                    src="/images/vue.svg"
                    alt="Vue"
                    width="48"
                    height="48"
                    class="logo"
                    @click="openUrl('https://vuejs.org/')"
                  />
                  <img
                    src="/images/tauri.svg"
                    alt="Tauri"
                    width="48"
                    height="48"
                    class="logo"
                    @click="openUrl('https://v2.tauri.app/')"
                  />
                  <img
                    src="/images/quasar.svg"
                    alt="Quasar"
                    width="48"
                    height="48"
                    class="logo"
                    @click="openUrl('https://quasar.dev/')"
                  />
                </div>
              </div>

              <div class="settings-reset-section q-mt-lg">
                <q-separator class="q-my-md" />
                <div class="text-subtitle1 q-mb-md">Data Management</div>
                <q-btn
                  color="negative"
                  icon="restart_alt"
                  label="Reset to Default Settings"
                  class="full-width"
                  @click="showResetSettingsDialog = true"
                  outline
                />
                <div class="text-caption q-mt-sm">
                  This will reset all app settings to their default values.
                  You'll need to save for changes to take effect.
                </div>
                
                <q-btn
                  color="negative"
                  icon="delete_forever"
                  label="Reset App Data"
                  class="full-width q-mt-md"
                  @click="showResetAppDataDialog = true"
                  outline
                />
                <div class="text-caption q-mt-sm">
                  This will wipe the database and reset all application data.
                  All mod information will be lost, but files will not be deleted.
                </div>
              </div>
            </div>
            
            <AnimationPlayer jsonPath="/images/animations/characters/bf-holding-gf.json" 
  :width="500" 
  :height="500" 
  :scale="1"/>
          </q-card-section>
        </q-scroll-area>
      </div>

      <q-card-actions align="right">
        <q-btn
          flat
          label="Cancel"
          color="primary"
          v-close-popup
          @click="cancel"
        />
        <q-btn flat label="Save" color="primary" @click="save" v-close-popup />
      </q-card-actions>
    </q-card>
  </q-dialog>

  <!-- Reset Settings Confirmation Dialog -->
  <MessageDialog
    v-model="showResetSettingsDialog"
    title="Reset Settings"
    message="Are you sure you want to reset all settings to their default values?"
    icon="warning"
    icon-color="negative"
    confirm-label="Reset Settings"
    confirm-color="negative"
    @confirm="resetSettings"
  />

  <!-- Reset App Data Confirmation Dialog -->
  <MessageDialog
    v-model="showResetAppDataDialog"
    title="Reset App Data"
    message="This will wipe the database and reset ALL application data. All mod information will be lost, but files will not be deleted. Do you want to continue?"
    icon="warning"
    icon-color="negative"
    confirm-label="Reset App Data"
    confirm-color="negative"
    @confirm="resetAppData"
  />
</template>

<script setup lang="ts">
import { ref, watch, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import { AppSettings } from "@main-types";
import ThemePreview from "../common/ThemePreview.vue";
import MessageDialog from "./MessageDialog.vue";
import { StoreService } from "../../services/storeService";
import { DatabaseService } from "../../services/dbService";

// Use the singleton directly instead of through a ref
const storeService = StoreService.getInstance();

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(["update:modelValue", "save"]);

const settings = ref<AppSettings>({
  accentColor: "#DB2955", // Default magenta color
  installLocation: "C:\\Users\\Public\\Documents\\FNF Mods",
  theme: "dark", // Default to dark theme
  useSystemTheme: true, // Default to using system theme
  customCSS: "", // Default to no custom CSS
  validateFnfMods: true, // Default to validating FNF mods
  showTerminalOutput: true, // Default to showing terminal output
  compactMode: false, // Default to not using compact mode
});

// Sidebar navigation sections
const settingsSections = [
  { id: "appearance", label: "Appearance", icon: "palette" },
  { id: "installation", label: "Installation", icon: "folder" },
  { id: "about", label: "About", icon: "info" },
];

// Track the active section
const activeSection = ref("appearance");

// Theme options - "doe" is hidden until unlocked
const hasUnlockedExtraThemes = ref(false);
const themeOptions = computed(() => {
  const baseOptions = [
    { label: "Light", value: "light" },
    { label: "Dark", value: "dark" },
    { label: "Shaggy", value: "shaggy" },
    { label: "Hotline", value: "hotline" },
    { label: "Yourself", value: "yourself" },
    { label: "Corruption", value: "corruption" },
    { label: "QT", value: "qt" },
    { label: "Garcello", value: "garcello" },
    { label: "Pump", value: "pump" },
    { label: "Boo", value: "boo" },
  ];

  // Only add Extra themes if they are unlocked
  if (hasUnlockedExtraThemes.value) {
    baseOptions.push({ label: "Doe", value: "doe" });
  }

  return baseOptions;
});

const accentColorOptions = [
  { label: "Pink", value: "#DB2955" },
  { label: "Blue", value: "#235789" },
  { label: "Green", value: "#35CE8D" },
  { label: "Purple", value: "#C490D1" },
  { label: "Orange", value: "#FE621D" },
  { label: "Yellow", value: "#f7c548" },
  { label: "Red", value: "#C03221" },
  { label: "Cyan", value: "#39dbce" },
];

// Helper function to get current theme name
const getThemeName = () => {
  if (settings.value.useSystemTheme) {
    let value = getSystemTheme();
    return value.value;
  }
  // Ensure we're working with a string value
  if (typeof settings.value.theme === "string") {
    return settings.value.theme;
  }
  // Handle case where theme is an object
  let value = settings.value.theme as unknown as {
    label: string;
    value: string;
  };
  return value.value;
};

// Helper function to get current accent color
const getAccentColor = () => {
  if (typeof settings.value.accentColor === "string") {
    return settings.value.accentColor;
  }
  return settings.value.accentColor?.value || "#DB2955";
};

const showModal = computed({
  get: () => props.modelValue,
  set: (value) => emit("update:modelValue", value),
});

const showResetSettingsDialog = ref(false);
const showResetAppDataDialog = ref(false);

// Load saved settings when modal is opened
watch(
  () => props.modelValue,
  async (newVal) => {
    if (newVal) {
      await loadSettings();
    }
  }
);

const loadSettings = async () => {
  try {
    if (!storeService) {
      console.warn("Store service not initialized yet, using default settings");
      return;
    }

    // Get all settings from StoreService
    const storedSettings = await storeService.getAllSettings();

    // Apply stored settings to our local settings ref
    settings.value = { ...settings.value, ...storedSettings };

    console.log("Settings loaded from store service:", settings.value);

    // Apply theme immediately upon loading
    await updateTheme();
  } catch (error) {
    console.error("Failed to load settings:", error);
  }
};

const selectInstallLocation = async () => {
  try {
    // Use the Rust backend to select a folder
    const folderPath = await invoke<string>("select_settings_folder");
    if (folderPath) {
      settings.value.installLocation = folderPath;
    }
  } catch (error) {
    console.error("Failed to select install location:", error);
  }
};

// Get the current system theme (light or dark)
const getSystemTheme = (): { label: string; value: string } => {
  return window.matchMedia &&
    window.matchMedia("(prefers-color-scheme: light)").matches
    ? { label: "Light", value: "light" }
    : { label: "Dark", value: "dark" };
};

// Apply theme function
const updateTheme = async () => {
  // Determine which theme to use based on settings
  let activeThemeValue: string;

  // If using system theme, override with the system preference
  if (settings.value.useSystemTheme) {
    activeThemeValue = getSystemTheme().value;
  } else {
    // Handle both string and object theme values
    activeThemeValue =
      typeof settings.value.theme === "string"
        ? settings.value.theme
        : (settings.value.theme as unknown as { value: string })?.value ||
          "dark";
  }

  console.log("Applying theme:", activeThemeValue);

  // First check if we're running on Windows 11
  const isWindows11 = await invoke<boolean>("is_windows_11");
  console.log("Is Windows 11:", isWindows11, "Theme:", activeThemeValue);

  // Apply CSS classes for theme by first removing all theme classes
  document.body.classList.remove(
    "light-theme",
    "dark-theme",
    "yourself-theme",
    "hotline-theme",
    "corruption-theme",
    "shaggy-theme",
    "boo-theme",
    "qt-theme",
    "garcello-theme",
    "pump-theme",
    "doe-theme"
  );

  // Then add the active theme class
  document.body.classList.add(`${activeThemeValue}-theme`);

  // Apply solid theme if not on Windows 11
  if (!isWindows11) {
    // Only apply solid-theme to light and dark themes
    if (activeThemeValue === "light" || activeThemeValue === "dark") {
      document.body.classList.add("solid-theme");
      console.log(
        "Using solid background for non-Windows 11 theme:",
        activeThemeValue
      );

      // Remove transparent background styles
      document.documentElement.style.setProperty(
        "--transparent-bg-override",
        "none"
      );

      // Set background to solid color based on the theme
      const bgColor = `var(--theme-bg)`;
      document.documentElement.style.setProperty("background", bgColor);
      document.body.style.removeProperty("background");
      document.body.style.backgroundColor = bgColor;
      document
        .querySelector(".q-layout")
        ?.setAttribute("style", "background: " + bgColor + " !important");
    } else {
      // For other themes on non-Windows 11, don't use solid-theme
      document.body.classList.remove("solid-theme");
      console.log(
        "Using transparent background for non-Windows 11 theme:",
        activeThemeValue
      );

      // Use the semi-transparent theme variables directly
      const bgColor = `var(--theme-bg)`;
      document.documentElement.style.setProperty("background", bgColor);
      document.body.style.removeProperty("background");
      document.body.style.backgroundColor = bgColor;
      document
        .querySelector(".q-layout")
        ?.setAttribute("style", "background: " + bgColor + " !important");
    }
  } else {
    // On Windows 11, only light and dark themes should be transparent for Mica
    if (activeThemeValue === "light" || activeThemeValue === "dark") {
      document.body.classList.remove("solid-theme");
      document.documentElement.style.setProperty(
        "--transparent-bg-override",
        "transparent"
      );
      // Fix for background style being commented out
      document.body.style.removeProperty("background");
      document.body.setAttribute("style", "background: transparent !important");

      // Make sure q-layout is also transparent for Mica to work properly
      const qLayout = document.querySelector(".q-layout");
      if (qLayout) {
        qLayout.removeAttribute("style");
        qLayout.setAttribute("style", "background: transparent !important");
      }

      // Call the Rust backend to apply Mica effect (Windows only)
      try {
        // Only light and dark themes should use the Mica effect
        const isDarkMica = activeThemeValue !== "light";

        await invoke("change_mica_theme", {
          window: "main", // Main window label
          dark: isDarkMica, // true for dark themes, false for light theme
        });
        console.log(
          "Applied Mica theme effect:",
          isDarkMica ? "dark" : "light"
        );
      } catch (error) {
        console.error("Failed to apply/remove Mica effect:", error);
        // Non-fatal error, the app will still work without Mica
      }
    } else {
      document.body.classList.remove("solid-theme");
      document.documentElement.style.setProperty(
        "--transparent-bg-override",
        "none"
      );

      // Set background to solid color based on the theme
      const bgColor = `var(--theme-bg)`;
      document.documentElement.style.setProperty("background", bgColor);
      document.body.style.removeProperty("background");
      document.body.style.backgroundColor = bgColor;
      document
        .querySelector(".q-layout")
        ?.setAttribute("style", "background: " + bgColor + " !important");
    }
  }

  // Dispatch an event so other components can know about theme changes
  window.dispatchEvent(
    new CustomEvent("theme-changed", {
      detail: {
        theme: activeThemeValue,
        useSystemTheme: settings.value.useSystemTheme,
      },
    })
  );
};

const save = async () => {
  try {
    if (!storeService) {
      console.warn("Store service not initialized yet, cannot save settings");
      return;
    }

    // Process the accent color for storage - ensure it's stored as a string
    // For accent color, we need to handle both string and object values
    if (typeof settings.value.accentColor !== "string") {
      // If it's an object with a value property
      if (settings.value.accentColor?.value) {
        console.log("Converting accent color object to string value:", settings.value.accentColor);
        settings.value.accentColor = settings.value.accentColor.value;
      }
      // If it's null or undefined or another non-string type
      else {
        console.log("Using default accent color");
        settings.value.accentColor = "#DB2955"; // Default
      }
    }

    // Ensure accentColor is a valid CSS color value (starts with #)
    if (typeof settings.value.accentColor !== 'string' || !settings.value.accentColor.startsWith('#')) {
      console.log("Fixing invalid accent color format");
      settings.value.accentColor = "#DB2955"; // Default
    }

    // Process the theme value to ensure it's a string
    if (typeof settings.value.theme === "object" && settings.value.theme !== null && "value" in settings.value.theme) {
      console.log("Converting theme object to string value");
      settings.value.theme = (settings.value.theme as { value: string }).value;
    }

    console.log("Saving settings with accent color:", settings.value.accentColor);

    // Save all settings at once using the StoreService
    await storeService.saveSettings(settings.value);
    console.log("Settings saved via StoreService:", settings.value);

    // Apply the accent color to CSS custom properties
    document.documentElement.style.setProperty("--q-primary", settings.value.accentColor);

    // Apply theme
    await updateTheme();
    
    // Dispatch compact mode changed event
    window.dispatchEvent(
      new CustomEvent("compact-mode-changed", {
        detail: {
          compactMode: settings.value.compactMode
        },
      })
    );

    // Emit save event
    emit("save", settings.value);
  } catch (error) {
    console.error("Failed to save settings:", error);
  }
};

const cancel = () => {
  // Reset any unsaved changes by reloading from DB
  loadSettings();
};

const resetSettings = () => {
  // Reset all settings to default values
  settings.value = {
    accentColor: "#DB2955",
    installLocation: "C:\\Users\\Public\\Documents\\FNF Mods",
    theme: "dark",
    useSystemTheme: true,
    customCSS: "",
    validateFnfMods: true,
    showTerminalOutput: true,
    compactMode: false,
  };
  console.log("Settings have been reset to default values.");
};

const resetAppData = async () => {
  try {
    console.log("Resetting application data...");
    
    // Get instances of our services
    const dbService = DatabaseService.getInstance();
    
    // First, reset settings to default values
    await storeService.clearSettings();
    console.log("Settings have been cleared successfully");
    
    // Then clear the database (delete all data from tables)
    await dbService.clearDatabase();
    console.log("Database has been cleared successfully");
    
    // Reload the application to apply changes
    window.location.reload();
  } catch (error) {
    console.error("Failed to reset app data:", error);
    window.alert(`Failed to reset app data: ${error}`);
  }
};

onMounted(async () => {
  // Initialize StoreService
  await storeService.initialize();

  // Really silly fix for dropdown background, oh Quasar why
  const style = document.createElement("style");
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
  width: 90vw;
  height: 90vh;
  max-width: 90vw;
  max-height: 90vh;
  background-color: var(--solid);
  color: var(--theme-text);
  border: var(--theme-border) 2px solid;
  backdrop-filter: blur(30px);
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
  background-color: var(--q-primary);
  color: white;
}

.settings-sidebar .q-icon {
  color: var(--theme-text-secondary);
}

.settings-active-item .q-icon {
  color: white;
}

.color-preview {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  border: 1px solid var(--theme-border);
}

.acknowledgements {
  max-height: 200px;
  overflow-y: auto;
  line-height: 1.5;
}

a {
  cursor: pointer;
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
  display: flex;
  gap: 8px;
}

.logo {
  cursor: pointer;
  transition: transform 0.2s;
  display: inline-block;
}

.logo:hover {
  transform: scale(1.05);
}

.center-credits {
  display: flex;
  flex-direction: column;
  margin-top: 16px;
}

:deep(.q-field__messages, .q-field__native::placeholder, .q-field__label) {
  color: var(--theme-text-secondary);
}

:deep(.q-field__native::placeholder) {
  color: var(--theme-text-secondary);
}
</style>
