<template>
  <q-layout view="lHh Lpr lFf" class="transparent-bg">
    <!-- Loading overlay shown during initialization -->
    <div class="loading-overlay" v-if="isInitializing">
      <div class="loading-container">
        <img class="logo" />
        <q-linear-progress
          :value="initProgress"
          size="md"
          rounded
          class="q-mt-md"
          color="primary"
        />
        <div class="text-caption q-mt-sm phantom-font">
          {{ initStatusText }}
        </div>
      </div>
    </div>
    <div v-if="!isInitializing">
      <Sidebar v-model="sidebarOpen" @resize="handleSidebarResize" />
      <!-- Engine selection for modpacks from deep links -->
      <EngineSelectionDialog
        v-model="showEngineSelectDialog"
        :compatible-engines="compatibleEngines"
        :engine-type="currentModpackType"
        :mod-name="currentModName"
        @select="onEngineSelected"
        @cancel="cancelDownload"
      />

      <!-- Missing mod path dialog -->
      <MessageDialog
        v-model="showModPathMissingDialog"
        title="Missing Mod"
        message="The following mod folder no longer exists:"
        icon="warning"
        icon-color="negative"
        confirm-label="Remove From List"
        confirm-color="negative"
        @confirm="removeMissingMod"
      >
        <div class="text-caption">{{ missingModPath }}</div>
        <p class="text-body2 q-mt-sm">
          Would you like to remove this mod from your library?
        </p>
      </MessageDialog>

      <!-- Global Context Menu -->
      <ContextMenu
        :visible="showContextMenu"
        :position="contextMenuPosition"
        :options="contextMenuOptions"
        @close="closeContextMenu"
      />
    </div>
  </q-layout>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, provide, reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { onOpenUrl } from "@tauri-apps/plugin-deep-link";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import { useQuasar, Notify } from "quasar";
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import Sidebar from "./components/layout/Sidebar.vue";
import EngineSelectionDialog from "./components/modals/EngineSelectionDialog.vue";
import MessageDialog from "./components/modals/MessageDialog.vue";
import ContextMenu from "./components/common/ContextMenu.vue";
import { DatabaseService } from "@services/dbService";
import { StoreService, DEFAULT_SETTINGS } from "@services/storeService";
import {
  gamebananaService,
  setupGameBananaEventListeners,
} from "@services/gamebananaService";
import { AppSettings } from "./types";

// Define window.db
declare global {
  interface Window {
    db: any;
  }
}

// Initialize Quasar
const $q = useQuasar();

// Ensure Notify is properly registered
Notify.create = Notify.create || (() => {});

// Initialization state variables
const isInitializing = ref(true);
const initProgress = ref(0);
const initStatusText = ref("Starting up...");

const sidebarOpen = ref(true);
const sidebarWidth = ref(250);

const showEngineSelectDialog = ref(false);
const compatibleEngines = ref<any[]>([]);
const currentModpackType = ref<string | null>(null);
const currentModName = ref<string>("Unknown Mod");
const currentDownloadUrl = ref<string>("");
const currentModId = ref<number | null>(null);
const handleSidebarResize = (width: number) => {
  sidebarWidth.value = width;
};

const cancelDownload = () => {
  showEngineSelectDialog.value = false;

  // Show cancellation notification
  $q.notify({
    type: "info",
    message: "Download cancelled",
    position: "bottom-right",
    timeout: 3000,
  });
};

const onEngineSelected = async (engine: any) => {
  try {
    if (!currentDownloadUrl.value || !currentModId.value) {
      throw new Error("Missing download information");
    }

    // Use the gamebananaService to handle modpack download with selected engine
    const result = await gamebananaService.downloadDeepLinkModpackWithEngine(
      currentDownloadUrl.value,
      currentModName.value,
      currentModId.value,
      engine
    );

    // Close the dialog
    showEngineSelectDialog.value = false;

    // Show an error if there was one
    if (!result.success) {
      $q.notify({
        type: "negative",
        message: "Failed to download modpack",
        caption: String(result.error),
        position: "bottom-right",
        timeout: 5000,
      });
    }
  } catch (error) {
    console.error("Failed to download modpack:", error);

    $q.notify({
      type: "negative",
      message: "Failed to download modpack",
      caption: String(error),
      position: "bottom-right",
      timeout: 5000,
    });

    // Close the dialog
    showEngineSelectDialog.value = false;
  }
};

// State for missing mod path dialog
const showModPathMissingDialog = ref(false);
const missingModPath = ref("");

// Function to remove a mod when it's missing
const removeMissingMod = async () => {
  if (!missingModPath.value) return;

  console.log("Removing missing mod with path:", missingModPath.value);

  try {
    // Find mod ID based on path
    const dbService = DatabaseService.getInstance();
    const mod = await dbService.getModByPath(missingModPath.value);

    if (mod) {
      // Delete the mod from the database
      await dbService.deleteMod(mod.id);

      // Show success notification
      $q.notify({
        type: "positive",
        message: `Removed missing mod from library`,
        position: "bottom-right",
        timeout: 3000,
      });

      // Refresh the mods list by dispatching an event
      const refreshEvent = new CustomEvent("refresh-mods");
      window.dispatchEvent(refreshEvent);
    } else {
      console.warn("Could not find mod with path:", missingModPath.value);
    }
  } catch (error) {
    console.error("Error removing missing mod:", error);
    $q.notify({
      type: "negative",
      message: "Failed to remove mod",
      caption: String(error),
      position: "bottom-right",
      timeout: 3000,
    });
  } finally {
    // Reset state
    showModPathMissingDialog.value = false;
    missingModPath.value = "";
  }
};

// State for context menu
const showContextMenu = ref(false);
const contextMenuPosition = ref({ x: 0, y: 0 });
const contextMenuOptions = ref<any[]>([]);

// Function to close the context menu
const closeContextMenu = () => {
  showContextMenu.value = false;
  contextMenuOptions.value = [];
};

// Global event listener for custom context menu
onMounted(() => {
  // Prevent default context menu
  document.addEventListener("contextmenu", (event) => {
    event.preventDefault();
  });

  // Listen for custom context menu events
  document.addEventListener("show-context-menu", ((event: CustomEvent) => {
    // Get position and options from the event
    const { position, options } = event.detail;

    // Update context menu state
    contextMenuPosition.value = position;
    contextMenuOptions.value = options;
    showContextMenu.value = true;
  }) as EventListener);
});

// Cleanup event listeners on unmount
onUnmounted(() => {
  document.removeEventListener("contextmenu", (event) => {
    event.preventDefault();
  });

  document.removeEventListener(
    "show-context-menu",
    (() => {}) as EventListener
  );
});

// Initialize application settings
const appSettings = reactive<AppSettings>({ ...DEFAULT_SETTINGS });

// Provide app settings to all components
provide("appSettings", appSettings);

// Load app settings from database and update the reactive object
const loadAppSettings = async () => {
  try {
    const storeService = StoreService.getInstance();

    // Load each setting individually
    const accentColor = await storeService.getSetting("accentColor");
    if (accentColor) appSettings.accentColor = accentColor;

    const installLocation = await storeService.getSetting("installLocation");
    if (installLocation) appSettings.installLocation = installLocation;

    const theme = await storeService.getSetting("theme");
    if (theme) appSettings.theme = theme;

    const useSystemTheme = await storeService.getSetting("useSystemTheme");
    if (useSystemTheme !== undefined)
      appSettings.useSystemTheme = useSystemTheme === true;

    const customCSS = await storeService.getSetting("customCSS");
    if (customCSS) appSettings.customCSS = customCSS;

    const validateFnfMods = await storeService.getSetting("validateFnfMods");
    if (validateFnfMods !== undefined)
      appSettings.validateFnfMods = validateFnfMods === true;

    const showTerminalOutput = await storeService.getSetting(
      "showTerminalOutput"
    );
    if (showTerminalOutput !== undefined)
      appSettings.showTerminalOutput = showTerminalOutput === true;

    console.log("Loaded app settings:", appSettings);
  } catch (error) {
    console.error("Error loading app settings:", error);
  }
};

// Apply theme based on system or user preference
const applyTheme = async (themeValue: string | boolean) => {
  // Convert legacy boolean value to theme string
  let activeThemeValue: string;
  if (typeof themeValue === "boolean") {
    activeThemeValue = themeValue ? "light" : "dark";
  } else {
    activeThemeValue = themeValue;
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

      // Remove transparent background styles
      document.documentElement.style.setProperty(
        "--transparent-bg-override",
        "none"
      );

      // Set background to solid color instead of transparent
      const bgColor = `var(--theme-bg)`;
      // Apply background explicitly instead of using style property
      document.documentElement.style.setProperty("background", bgColor);
      document.body.style.removeProperty("background");
      document.body.style.backgroundColor = bgColor;
      document
        .querySelector(".q-layout")
        ?.setAttribute("style", "background: " + bgColor + " !important");
    } else {
      // For other themes on non-Windows 11, don't use solid-theme
      document.body.classList.remove("solid-theme");

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

      // Apply Mica effect based on theme (Windows 11 only)
      try {
        await invoke("change_mica_theme", {
          window: "main",
          dark: activeThemeValue !== "light",
        });
        console.log(
          "Applied Mica theme effect:",
          activeThemeValue !== "light" ? "dark" : "light"
        );
      } catch (error) {
        console.error("Failed to apply Mica effect:", error);
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

      // Remove Mica effect for non-standard themes
      try {
        await invoke("remove_mica_theme", {
          window: "main",
        });
        console.log("Removed Mica effect for theme:", activeThemeValue);
      } catch (error) {
        console.error("Failed to remove Mica effect:", error);
      }
    }
  }
};

// Get the current system theme (light or dark)
const getSystemTheme = (): boolean => {
  return (
    window.matchMedia &&
    window.matchMedia("(prefers-color-scheme: light)").matches
  );
};

// Apply custom CSS from settings
const loadCustomCSS = async () => {
  try {
    const storeService = StoreService.getInstance();
    const customCSS = await storeService.getSetting("customCSS");

    if (customCSS) {
      // Create a style element for custom CSS
      const styleElement = document.createElement("style");
      styleElement.id = "custom-user-css";
      styleElement.textContent = customCSS;
      document.head.appendChild(styleElement);
      console.log("Applied custom CSS from settings");
    }
  } catch (error) {
    console.error("Failed to load custom CSS:", error);
  }
};

// Handle system theme changes
const handleSystemThemeChange = async (event: MediaQueryListEvent) => {
  // Only react to system theme changes if that setting is enabled
  const useSystemTheme = await getUseSystemThemeSetting();
  if (useSystemTheme) {
    const isLightTheme = event.matches;
    applyTheme(isLightTheme);
  }
};

// Get the current "use system theme" setting from the database
const getUseSystemThemeSetting = async (): Promise<boolean> => {
  try {
    const storeService = StoreService.getInstance();
    return await storeService.getSetting("useSystemTheme");
  } catch (error) {
    console.error("Error fetching useSystemTheme setting:", error);
    return true; // Default to true on error
  }
};

// Get the manually set theme preference
const getThemePreference = async (): Promise<string> => {
  try {
    const storeService = StoreService.getInstance();

    // Get theme from the store
    const theme = await storeService.getSetting("theme");
    if (theme) {
      return theme;
    }

    return "dark"; // Default to dark theme
  } catch (error) {
    console.error("Error fetching theme preference:", error);
    return "dark"; // Default to dark theme
  }
};

// Process deep link mod download
const processDeepLinkModDownload = async (
  downloadUrl: string,
  modId: number,
  archiveExt: string
) => {
  try {
    console.log("Processing mod download from deep link:", {
      downloadUrl,
      modId,
      archiveExt,
    });

    // Use the gamebananaService to handle the deep link download
    const result = await gamebananaService.downloadModFromDeepLink(
      downloadUrl,
      modId
    );

    // Type guard to check if we need to show the engine selection dialog
    if ("showEngineSelectDialog" in result && result.showEngineSelectDialog) {
      // Show the engine selection dialog with the data from the result
      compatibleEngines.value = result.compatibleEngines;
      currentModpackType.value = result.modpackType;
      currentModName.value = result.modName;
      currentDownloadUrl.value = result.downloadUrl;
      currentModId.value = result.modId;
      showEngineSelectDialog.value = true;
    } else if ("success" in result) {
      // Handle regular operation result
      if (!result.success && result.error) {
        $q.notify({
          type: "negative",
          message: "Failed to download mod",
          caption: result.error,
          position: "bottom-right",
          timeout: 5000,
        });
      }
    }
  } catch (error) {
    console.error("Failed to download mod from deep link:", error);

    // Show error notification
    $q.notify({
      type: "negative",
      message: "Failed to download mod",
      caption: String(error),
      position: "bottom-right",
      timeout: 5000,
    });
  }
};

// Initialize the app
let cleanupEventListeners: (() => void) | undefined;

onMounted(async () => {
  try {
    // Set up download event listeners
    cleanupEventListeners = setupGameBananaEventListeners();

    // Update progress bar - Step 1: Initialize deep link handler
    initStatusText.value = "Setting up deep link handler...";
    initProgress.value = 0.1;

    // Set up deep link handler
    onOpenUrl(async (url) => {
      console.log("Deep link received:", url[0]);
      await getCurrentWindow().setFocus();

      // Process the deep link URL
      if (url[0].startsWith("flmod://")) {
        try {
          // Remove the protocol prefix
          const data = url[0].substring(8); // Remove "flmod://"

          // Split the URL by $ delimiter
          const parts = data.split("$");
          if (parts.length >= 4) {
            let downloadUrl = parts[0];
            const modType = parts[1];
            const modId = parseInt(parts[2]);
            const archiveExt = parts[3];

            downloadUrl = downloadUrl.replace("https//", "https://"); // Fix URL format

            console.log("Parsed deep link:", {
              downloadUrl,
              modType,
              modId,
              archiveExt,
            });

            // Ensure this is a mod type we support (this should always be 'Mod')
            if (modType === "Mod" && !isNaN(modId)) {
              // Process the download - fetch the mod info first
              processDeepLinkModDownload(downloadUrl, modId, archiveExt);
            } else {
              console.error(
                "Unsupported mod type or invalid mod ID in deep link"
              );
            }
          } else {
            console.error("Invalid deep link format, missing required parts");
          }
        } catch (error) {
          console.error("Failed to process deep link:", error);
        }
      }
    });

    // Update progress bar - Step 2: Initialize database
    initStatusText.value = "Initializing database...";
    initProgress.value = 0.2;

    // Initialize the database service
    const dbService = DatabaseService.getInstance();
    await dbService.initialize();

    // Make the database service available globally
    window.db = {
      // Direct access to the database service
      select: async (query: string, params?: any[]) => {
        const db = await dbService["db"];
        return db.select(query, params);
      },
      execute: async (query: string, params?: any[]) => {
        const db = await dbService["db"];
        return db.execute(query, params);
      },
      service: dbService,
    };

    // Update progress bar - Step 3: Load mods
    initStatusText.value = "Loading mods...";
    initProgress.value = 0.4;

    // Load mods from the database
    const mods = await dbService.getAllMods();
    console.log("Loaded mods from database:", mods);
    if (mods && mods.length > 0) {
      console.log(`Loading ${mods.length} mods from database to backend`);
      // No need to sync with backend as the dbService handles this internally
    } else {
      console.log("No mods found in database");
    }

    // Update progress bar - Step 4: Apply theme
    initStatusText.value = "Applying theme...";
    initProgress.value = 0.6;

    // Apply initial theme based on system or user preference
    const useSystemTheme = await getUseSystemThemeSetting();
    if (useSystemTheme) {
      // If using system theme, apply light or dark based on system preference
      const isSystemLight = getSystemTheme();
      applyTheme(isSystemLight ? "light" : "dark");
    } else {
      // If using custom theme, apply the saved theme directly
      const themeValue = await getThemePreference();
      applyTheme(themeValue);
    }

    // Apply custom CSS
    await loadCustomCSS();

    // Update progress bar - Step 5: Load settings
    initStatusText.value = "Loading settings...";
    initProgress.value = 0.8;

    // Load app settings
    await loadAppSettings();

    // Set up listener for missing mod paths
    await listen<string>("mod-path-missing", (event) => {
      console.log("Received mod-path-missing event:", event);
      missingModPath.value = event.payload;
      showModPathMissingDialog.value = true;
    });

    // Listen for system theme changes
    const mediaQuery = window.matchMedia("(prefers-color-scheme: light)");
    mediaQuery.addEventListener("change", handleSystemThemeChange);

    // Update progress bar - Step 6: Check for updates
    initStatusText.value = "Checking for updates...";
    initProgress.value = 0.9;

    // Check for updates
    try {
      const updateResult = await check();
      if (updateResult !== null) {
        // Update progress bar - Updates available
        initStatusText.value = "Installing update...";

        // Install the update if there is one
        await updateResult.downloadAndInstall((event) => {
          switch (event.event) {
            case "Started":
              let contentLength = event.data.contentLength;
              break;
            case "Progress":
              let downloaded = 0;
              downloaded += event.data.chunkLength;
              let percent = contentLength
                ? Math.round((downloaded / contentLength) * 100)
                : 0;
              initProgress.value = 0.9 + percent / 1000;
              break;
            case "Finished":
              initProgress.value = 1.0;
              break;
          }
        });

        await relaunch();
      } else {
        console.log("No updates available");
      }
    } catch (error) {
      console.error("Failed to check for updates:", error);
    }

    // Update progress bar - Step 7: Complete
    initStatusText.value = "Ready!";
    initProgress.value = 1.0;

    // Hide the loading overlay after a small delay to ensure transitions are smooth
    setTimeout(() => {
      isInitializing.value = false;
    }, 500);

    console.log("App initialized and database tables updated");
  } catch (error) {
    console.error("Failed to initialize database:", error);

    // Update progress bar with error state
    initStatusText.value = "Error during initialization: " + String(error);
    initProgress.value = 1.0;

    // Hide the loading overlay after a delay
    setTimeout(() => {
      isInitializing.value = false;
    }, 2000);
    onUnmounted(() => {
      // Clean up the event listener for system theme changes
      const mediaQuery = window.matchMedia("(prefers-color-scheme: light)");
      mediaQuery.removeEventListener("change", handleSystemThemeChange);

      // Clean up download event listeners
      if (typeof cleanupEventListeners === "function") {
        cleanupEventListeners();
      }
    });
  }
});
</script>

<style>
html,
body {
  margin: 0;
  padding: 0;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  color: var(--theme-text);
}

.transparent-bg {
  background: transparent !important;
}

/* Loading overlay styles */
.loading-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 9999;
}

.loading-container {
  width: 400px;
  text-align: center;
  padding: 24px;
  border-radius: 12px;
  background-color: var(--theme-card);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  border: 2px solid var(--theme-border);
}
.logo {
  width: 350px;
  height: 10vh;
  mask: url("/images/fridaylight.svg") no-repeat center;
  -webkit-mask: url("/images/fridaylight.svg") no-repeat center;
  background-color: var(--theme-text);
  image-rendering: optimizeQuality;
}

loading-container h2 {
  margin-top: 0;
  margin-bottom: 24px;
}

.loading-container .text-caption {
  color: var(--theme-text-secondary, #aaaaaa);
  margin-top: 12px;
}
</style>
