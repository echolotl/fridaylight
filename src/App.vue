<template>
  <q-layout view="lHh Lpr lFf" class="transparent-bg">
    <!-- Redesigned Sidebar that contains both the sidebar and main content -->
    <Sidebar v-model="sidebarOpen" @resize="handleSidebarResize">
      <!-- Main content is now handled within the Sidebar component -->
    </Sidebar>

    <!-- Engine Selection Dialog for Modpacks from deep links -->
    <EngineSelectionDialog
      v-model="showEngineSelectDialog"
      :compatible-engines="compatibleEngines"
      :engine-type="currentModpackType"
      :mod-name="currentModName"
      @select="onEngineSelected"
      @cancel="cancelDownload"
    />
  </q-layout>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { onOpenUrl } from "@tauri-apps/plugin-deep-link";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useQuasar, Notify } from "quasar";
import Sidebar from "./components/layout/Sidebar.vue";
import EngineSelectionDialog from "./components/modals/EngineSelectionDialog.vue";
import { DatabaseService } from "./services/dbService";

// Define window.db
declare global {
  interface Window {
    db: any;
  }
}

// Initialize Quasar at the component setup level
const $q = useQuasar();

// Ensure Notify is properly registered
Notify.create = Notify.create || (() => {});

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

    // Show loading notification
    const downloadingNotification = $q.notify({
      type: "ongoing",
      message: `Downloading "${currentModName.value}" to ${formatEngineType(
        currentModpackType.value
      )}...`,
      position: "bottom-right",
      timeout: 0,
    });

    // Get the installation path for the selected engine's mods folder
    const modsFolderPath = getModsFolderPath(engine);
    if (!modsFolderPath) {
      throw new Error("Could not determine mods folder path");
    }

    console.log(
      `Starting download for "${currentModName.value}" modpack to ${modsFolderPath}`
    );

    // Fix the URL if needed
    let fixedUrl = currentDownloadUrl.value;
    if (fixedUrl.startsWith("//")) {
      fixedUrl = "https:" + fixedUrl;
    } else if (!fixedUrl.includes("://")) {
      fixedUrl = "https://" + fixedUrl;
    }

    // Call the backend to download the modpack to the engine's mods folder
    const result = await invoke<string>("download_gamebanana_mod_command", {
      url: fixedUrl,
      name: currentModName.value,
      modId: currentModId.value,
      installLocation: modsFolderPath,
    });

    console.log("Download result:", result);

    // Get the mod info from GameBanana API - we need this to process the result
    const modInfo = await invoke<any>("get_mod_info_command", {
      modId: currentModId.value,
    });

    // Process the result
    await processDownloadResult(
      result,
      currentModName.value,
      currentModId.value,
      modInfo,
      downloadingNotification
    );

    // Close the dialog
    showEngineSelectDialog.value = false;
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

// Apply theme based on system or user preference
const applyTheme = async (themeValue: string | boolean) => {
  // Convert legacy boolean value to theme string
  let activeThemeValue: string;
  if (typeof themeValue === 'boolean') {
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
          window: "main" // Main window label
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
    const dbService = DatabaseService.getInstance();
    const customCSS = await dbService.getSetting("customCSS");
    
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
    const dbService = DatabaseService.getInstance();
    const value = await dbService.getSetting("useSystemTheme");
    return value === "true";
  } catch (error) {
    console.error("Error fetching useSystemTheme setting:", error);
    return true; // Default to true on error
  }
};

// Get the manually set theme preference 
const getThemePreference = async (): Promise<string> => {
  try {
    const dbService = DatabaseService.getInstance();
    
    // First try to get the new theme format
    const themeValue = await dbService.getSetting("theme");
    if (themeValue) {
      return themeValue;
    }
    
    // Fallback to legacy enableLightTheme format
    const legacyValue = await dbService.getSetting("enableLightTheme");
    return legacyValue === "true" ? "light" : "dark";
  } catch (error) {
    console.error("Error fetching theme preference:", error);
    return "dark"; // Default to dark theme on error
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

    // Show notification that download is being prepared
    const pendingNotification = $q.notify({
      type: "ongoing",
      message: `Preparing to download mod...`,
      position: "bottom-right",
      timeout: 0,
    });

    // Get the mod info from GameBanana API to get the name and other details
    const modInfo = await invoke<any>("get_mod_info_command", { modId: modId });

    if (!modInfo || !modInfo._sName) {
      throw new Error("Failed to fetch mod information from GameBanana");
    }

    const modName = modInfo._sName;

    // Check if this is a modpack by examining tags, category, or description
    const isModpack = await determineIfModpack(modInfo);
    const modpackType = await determineModpackType(modInfo);

    console.log("Mod analysis:", { isModpack, modpackType });

    // Update notification with mod name
    pendingNotification();
    const downloadingNotification = $q.notify({
      type: "ongoing",
      message: `Preparing to download "${modName}"...`,
      position: "bottom-right",
      timeout: 0,
    });

    // If it's a modpack, check for compatible engines
    if (isModpack && modpackType) {
      // Get compatible engines for this modpack
      const engineMods = await getCompatibleEngineMods(modpackType);

      if (engineMods.length === 0) {
        // No compatible engine found
        downloadingNotification();

        $q.notify({
          type: "negative",
          message: `Cannot download ${modpackType} modpack`,
          caption: `You don't have any ${formatEngineType(
            modpackType
          )} installed. Please install it from the GameBanana browser first.`,
          position: "bottom-right",
          timeout: 5000,
        });

        return;
      } else {
        // Show the engine selection dialog
        compatibleEngines.value = engineMods;
        currentModpackType.value = modpackType;
        currentModName.value = modName;
        currentDownloadUrl.value = downloadUrl;
        currentModId.value = modId;
        showEngineSelectDialog.value = true;
        return;
      }
    }

    // If not a modpack, proceed with standard mod download
    // Get the install location from settings
    let installLocation: string | null = null;
    try {
      if (window.db && window.db.service) {
        installLocation = await window.db.service.getSetting("installLocation");
      }
    } catch (error) {
      console.warn("Could not get install location from settings:", error);
    }

    // Fix the URL if needed
    let fixedUrl = downloadUrl;
    if (fixedUrl.startsWith("//")) {
      fixedUrl = "https:" + fixedUrl;
    } else if (!fixedUrl.includes("://")) {
      fixedUrl = "https://" + fixedUrl;
    }

    console.log(`Starting download for "${modName}" from URL: ${fixedUrl}`);

    // Call the backend to download the mod
    const result = await invoke<string>("download_gamebanana_mod_command", {
      url: fixedUrl,
      name: modName,
      modId: modId,
      installLocation,
    });

    // Process the result and save to database
    processDownloadResult(
      result,
      modName,
      modId,
      modInfo,
      downloadingNotification
    );
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

// Helper function to process download result and save to database
const processDownloadResult = async (
  result: string,
  modName: string,
  modId: number,
  origModInfo: any,
  notification: any
) => {
  try {
    let modInfoResult: any;

    try {
      // Try to parse as JSON
      const parsed = JSON.parse(result);
      modInfoResult = parsed.mod_info;
    } catch (e) {
      // If parsing fails, assume it's just the path string
      const modPath = result;
      // Get mod info directly from the backend
      const allMods = await invoke<any[]>("get_mods");
      modInfoResult = allMods.find((m) => m.path === modPath);

      // If we still don't have mod info, create a basic one
      if (!modInfoResult) {
        modInfoResult = {
          id: crypto.randomUUID(),
          name: modName,
          path: modPath,
          executable_path: null,
          icon_data: null,
          banner_data: `https://gamebanana.com/mods/embeddables/${modId}`,
          version: origModInfo._sVersion || null,
          engine_type: null,
        };
      }
    }

    // Save the mod to the database
    if (modInfoResult) {
      try {
        if (window.db && window.db.service) {
          await window.db.service.saveMod(modInfoResult);
        }
      } catch (error) {
        console.error("Failed to save mod to database:", error);
      }
    }

    // Dismiss notification
    notification();

    // Show success notification
    $q.notify({
      type: "positive",
      message: `"${modName}" downloaded and installed successfully!`,
      caption: `Ready to play from the mods list`,
      position: "bottom-right",
      timeout: 5000,
    });

    // Trigger refresh event to update mod list
    const refreshEvent = new CustomEvent("refresh-mods");
    window.dispatchEvent(refreshEvent);
  } catch (error) {
    // Dismiss notification
    notification();

    // Show error notification
    $q.notify({
      type: "negative",
      message: `Failed to process download for "${modName}"`,
      caption: String(error),
      position: "bottom-right",
      timeout: 5000,
    });

    console.error("Error processing download result:", error);
  }
};

// Functions for checking modpack types using direct category IDs
const determineIfModpack = (modInfo: any): boolean => {
  // Check category ID directly - this is the most reliable method
  if (modInfo._aCategory && modInfo._aCategory._idRow) {
    const categoryId = parseInt(modInfo._aCategory._idRow);
    // These are the known modpack category IDs
    const modpackCategoryIds = [28367, 29202, 34764]; // Psych, V-Slice, Codename
    return modpackCategoryIds.includes(categoryId);
  }

  return false;
};

const determineModpackType = (modInfo: any): string | null => {
  // Determine type by category ID - most accurate method
  if (modInfo._aCategory && modInfo._aCategory._idRow) {
    const categoryId = parseInt(modInfo._aCategory._idRow);
    switch (categoryId) {
      case 28367:
        return "psych"; // Psych Engine
      case 29202:
        return "vanilla"; // V-Slice
      case 34764:
        return "codename"; // Codename Engine
    }
  }

  return null;
};

// Get a list of compatible engine mods
const getCompatibleEngineMods = async (
  engineType: string | null
): Promise<any[]> => {
  if (!engineType) return [];

  try {
    // Fetch all mods
    let allMods = [];
    if (window.db && window.db.service) {
      allMods = await window.db.service.getAllMods();
    } else {
      allMods = await invoke<any[]>("get_mods");
    }

    // Filter mods by engine type
    return allMods.filter(
      (mod: { engine: { engine_type: string }; engine_type: string }) => {
        // Check engine.engine_type first (new structure)
        if (mod.engine && mod.engine.engine_type) {
          return (
            mod.engine.engine_type.toLowerCase() === engineType.toLowerCase()
          );
        }

        // Fall back to legacy engine_type field
        return (
          mod.engine_type &&
          mod.engine_type.toLowerCase() === engineType.toLowerCase()
        );
      }
    );
  } catch (error) {
    console.error("Failed to get compatible engine mods:", error);
    return [];
  }
};

const formatEngineType = (engineType: string | null): string => {
  if (!engineType) return "Unknown";

  switch (engineType.toLowerCase()) {
    case "psych":
      return "Psych Engine";
    case "vanilla":
      return "V-Slice";
    case "codename":
      return "Codename Engine";
    case "fps-plus":
      return "FPS Plus";
    default:
      return engineType.charAt(0).toUpperCase() + engineType.slice(1);
  }
};

// Function to get the mods folder path for an engine mod
const getModsFolderPath = (engineMod: any): string => {
  // First check if the engine has a specified mods_folder_path in the new structure
  if (
    engineMod.engine &&
    engineMod.engine.mods_folder &&
    engineMod.engine.mods_folder_path
  ) {
    return engineMod.engine.mods_folder_path;
  }

  // If not, construct the default mods folder path
  const basePath = engineMod.path;
  const executablePath = engineMod.executable_path || "";

  if (!basePath) return "Unknown path";

  // Get parent directory of executable if it exists
  let baseDir = basePath;
  if (executablePath) {
    // Extract the directory from the executable path
    const lastSlashIndex = executablePath.lastIndexOf("/");
    if (lastSlashIndex > 0) {
      baseDir = executablePath.substring(0, lastSlashIndex);
    } else {
      const lastBackslashIndex = executablePath.lastIndexOf("\\");
      if (lastBackslashIndex > 0) {
        baseDir = executablePath.substring(0, lastBackslashIndex);
      }
    }
  }

  // Construct the path to the mods folder
  return `${baseDir}/mods`;
};

// Initialize the app
onMounted(async () => {
  try {
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
            const downloadUrl = parts[0];
            const modType = parts[1];
            const modId = parseInt(parts[2]);
            const archiveExt = parts[3];

            console.log("Parsed deep link:", {
              downloadUrl,
              modType,
              modId,
              archiveExt,
            });

            // Ensure this is a mod type we support
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

    // Initialize the database service
    const dbService = DatabaseService.getInstance();
    await dbService.initialize();

    // Make the database service available globally
    window.db = {
      // Legacy interface for compatibility
      select: async (query: string, params?: any[]) => {
        // Pass this to the actual database via dbService when needed
        const db = await dbService["db"];
        return db.select(query, params);
      },
      execute: async (query: string, params?: any[]) => {
        // Pass this to the actual database via dbService when needed
        const db = await dbService["db"];
        return db.execute(query, params);
      },
      // Add the dbService instance for direct access
      service: dbService,
    };

    // Load mods from the database
    const mods = await dbService.getAllMods();
    console.log("Loaded mods from database:", mods);
    if (mods && mods.length > 0) {
      console.log(`Loading ${mods.length} mods from database to backend`);
      // No need to sync with backend as the dbService handles this internally
    } else {
      console.log("No mods found in database");
    }

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

    // Listen for system theme changes
    const mediaQuery = window.matchMedia("(prefers-color-scheme: light)");
    mediaQuery.addEventListener("change", handleSystemThemeChange);

    console.log("App initialized and database tables updated");
  } catch (error) {
    console.error("Failed to initialize database:", error);
  }
});

onUnmounted(() => {
  // Clean up the event listener for system theme changes
  const mediaQuery = window.matchMedia("(prefers-color-scheme: light)");
  mediaQuery.removeEventListener("change", handleSystemThemeChange);
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

/* Make sure any Quasar containers are also transparent */
.q-layout,
.q-page-container,
.q-page {
  background: transparent !important;
}

/* Remove all shadows from Quasar components */
.q-shadow-1,
.q-shadow-2,
.q-shadow-3,
.q-shadow-4,
.q-shadow-5,
.q-shadow-6,
.q-shadow-7,
.q-shadow-8,
.q-shadow-9,
.q-shadow-10 {
  box-shadow: none !important;
}

/* Custom font face for phantom muff font */
@font-face {
  font-family: "PhantomMuffFull";
  src: url("./assets/fonts/PhantomMuffFull.ttf") format("truetype");
  font-weight: normal;
  font-style: normal;
}

@font-face {
  font-family: "PhantomMuffEmpty";
  src: url("./assets/fonts/PhantomMuffEmpty.ttf") format("truetype");
  font-weight: normal;
  font-style: normal;
}
</style>
