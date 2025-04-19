<template>
  <div class="gamebanana-browser phantom-font">
    <!-- Search Bar - Updated to handle custom-download event -->
    <SearchBar
      :searchQuery="searchQuery"
      @update:searchQuery="searchQuery = $event"
      @search="searchMods"
      @clear="clearSearch"
      @custom-download="showCustomUrlModal = true"
    />

    <!-- Search Results View -->
    <div v-if="activeView === 'search'">
      <div class="section-header phantom-font-difficulty">
        <div class="text-subtitle1">Search Results</div>
        <q-btn
          flat
          dense
          color="primary"
          icon="arrow_back"
          label="Back to Home"
          @click="clearSearch"
        />
      </div>

      <ModGrid
        :mods="searchResults"
        :loading="isLoadingSearch"
        loadingMessage="Searching mods..."
        :emptyMessage="`No mods found matching '${searchQuery}'`"
        :currentPage="currentPage"
        :totalPages="totalPages"
        :input-pagination="true"
        @download="downloadMod"
        @page-change="changePage"
      />
    </div>

    <!-- Home View with Featured and Latest Mods -->
    <div v-else class="scroll-container">
      <!-- Featured Mods Carousel Section -->
      <div class="section-header phantom-font-difficulty">
          <div class="text-subtitle1">Featured Mods</div>
        </div>
      <div class="mods-section">
        <FeaturedModsCarousel
          :mods="featuredMods"
          :loading="isLoadingFeatured"
          @download="downloadMod"
        />
      </div>
      <hr />
      
      <!-- Latest Mods Section -->
      <div class="mods-section">
        <div class="section-header phantom-font-difficulty">
          <div class="text-subtitle1">Latest Mods</div>
        </div>

        <!-- Tab navigation -->
        <q-tabs
          v-model="selectedModType"
          dense
          class="mod-tabs"
          active-color="primary"
          indicator-color="primary"
          align="justify"
          narrow-indicator
        >
          <q-tab name="executables" label="Executables" />
          <q-tab name="psychModpacks" label="Psych Engine Modpacks" />
          <q-tab name="vsliceModpacks" label="V-Slice Modpacks" />
          <q-tab name="codenameModpacks" label="Codename Engine Modpacks" />
        </q-tabs>

        <q-tab-panels v-model="selectedModType" animated>
          <!-- Executables Tab (Original Latest Mods) -->
          <q-tab-panel name="executables">
            <ModGrid
              :mods="latestMods"
              :loading="isLoadingLatest"
              loadingMessage="Loading latest mods..."
              emptyMessage="No mods found"
              :currentPage="currentPage"
              :totalPages="totalPages"
              :inputPagination="true"
              @download="downloadMod"
              @page-change="changePage"
            />
          </q-tab-panel>

          <!-- Psych Engine Modpacks Tab -->
          <q-tab-panel name="psychModpacks">
            <ModGrid
              :mods="psychModpacks"
              :loading="isLoadingPsychModpacks"
              loadingMessage="Loading Psych Engine modpacks..."
              emptyMessage="No Psych Engine modpacks found"
              :currentPage="currentPage"
              :totalPages="totalPages"
              :inputPagination="true"
              @download="downloadMod"
              @page-change="changePage"
            />
          </q-tab-panel>

          <!-- V-Slice Modpacks Tab -->
          <q-tab-panel name="vsliceModpacks">
            <ModGrid
              :mods="vsliceModpacks"
              :loading="isLoadingVsliceModpacks"
              loadingMessage="Loading V-Slice modpacks..."
              emptyMessage="No V-Slice modpacks found"
              :currentPage="currentPage"
              :totalPages="totalPages"
              :inputPagination="true"
              @download="downloadMod"
              @page-change="changePage"
            />
          </q-tab-panel>

          <!-- Codename Engine Modpacks Tab -->
          <q-tab-panel name="codenameModpacks">
            <ModGrid
              :mods="codenameModpacks"
              :loading="isLoadingCodenameModpacks"
              loadingMessage="Loading Codename Engine modpacks..."
              emptyMessage="No Codename Engine modpacks found"
              :currentPage="currentPage"
              :totalPages="totalPages"
              :inputPagination="true"
              @download="downloadMod"
              @page-change="changePage"
            />
          </q-tab-panel>
        </q-tab-panels>
      </div>
    </div>

    <!-- Engine Download Buttons -->
    <EngineDownloadButtons @download-engine="downloadEngine" />

    <!-- Download File Selector Dialog -->
    <DownloadFileSelector
      v-model="showFileSelector"
      :files="downloadFiles"
      :mod-name="currentDownloadMod?.name || ''"
      :alternate-file-sources="alternateFileSources"
      @select="onFileSelected"
      @cancel="cancelDownload"
    />
    
    <!-- Engine Selection Dialog for Modpacks -->
    <EngineSelectionDialog
      v-model="showEngineSelectDialog"
      :compatible-engines="currentModpackInfo?.compatibleEngines || []"
      :engine-type="currentModpackInfo?.type || undefined"
      :mod-name="currentModpackInfo?.mod?.name || ''"
      @select="onEngineSelected"
      @cancel="cancelDownload"
    />

    <!-- Custom URL Download Modal -->
    <CustomUrlDownloadModal
      v-model="showCustomUrlModal"
      @submit="onCustomUrlSubmit"
      @cancel="showCustomUrlModal = false"
    />

    <!-- Mod Type Selection Modal -->
    <ModTypeSelectionModal
      v-model="showModTypeModal"
      :modData="customModData"
      @submit="onModTypeSubmit"
      @back="showModTypeModal = false; showCustomUrlModal = true"
      @cancel="showModTypeModal = false; customModData = null"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useQuasar, Notify } from "quasar";
import { useRouter } from "vue-router";

// Import local components
import SearchBar from "@components/common/SearchBar.vue";
import ModGrid from "@components/common/ModGrid.vue";
import FeaturedModsCarousel from "@mods/FeaturedModsCarousel.vue";
import EngineDownloadButtons from "@mods/EngineDownloadButtons.vue";
import DownloadFileSelector from "@modals/DownloadFileSelector.vue";
import EngineSelectionDialog from "@modals/EngineSelectionDialog.vue";
import CustomUrlDownloadModal from "@modals/CustomUrlDownloadModal.vue";
import ModTypeSelectionModal from "@modals/ModTypeSelectionModal.vue";
import { StoreService } from "../../services/storeService";

// Types
import type { GameBananaMod } from "@main-types";

// For managing global download state
import {
  downloadingMods,
  updateDownloadProgress,
  completeDownload,
  errorDownload,
} from "../../stores/downloadState";

// Declare db for TypeScript
declare global {
  interface Window {
    db: any;
  }
}

// Ensure Notify is properly registered
Notify.create = Notify.create || (() => {});

// Create a quasar instance at the top level of the script setup
const $q = useQuasar();
const router = useRouter();

// Event listeners
let removeDownloadStartedListener: (() => void) | undefined;
let removeDownloadProgressListener: (() => void) | undefined;
let removeDownloadFinishedListener: (() => void) | undefined;
let removeDownloadErrorListener: (() => void) | undefined;

// Search state
const searchQuery = ref("");
const isLoadingSearch = ref(false);
const hasSearched = ref(false);
const searchResults = ref<GameBananaMod[]>([]);

// Featured mods state
const featuredMods = ref<GameBananaMod[]>([]);
const isLoadingFeatured = ref(false);

// Latest mods and modpacks state
const latestMods = ref<GameBananaMod[]>([]);
const psychModpacks = ref<GameBananaMod[]>([]);
const vsliceModpacks = ref<GameBananaMod[]>([]);
const codenameModpacks = ref<GameBananaMod[]>([]);
const isLoadingLatest = ref(false);
const isLoadingPsychModpacks = ref(false);
const isLoadingVsliceModpacks = ref(false);
const isLoadingCodenameModpacks = ref(false);

// Pagination state
const currentPage = ref(1);
const totalPages = ref(1);
const itemsPerPage = 20;

// View state
const activeView = ref("all"); // 'all', 'search'
const selectedModType = ref("executables"); // For tabs

// For file selection
const showFileSelector = ref(false);
const downloadFiles = ref([]);
const alternateFileSources = ref([]);
const currentDownloadMod = ref<GameBananaMod | null>(null);
let pendingDownloadNotification: any = null;

// For modpack handling
interface ModpackInfo {
  mod: GameBananaMod;
  type: string | null;
  compatibleEngines: any[]; // List of compatible engine mods
}

const showEngineSelectDialog = ref(false);
const currentModpackInfo = ref<ModpackInfo | null>(null);
const selectedEngineMod = ref<any>(null);

// For custom URL download
const showCustomUrlModal = ref(false);
const showModTypeModal = ref(false);
const customModData = ref<any>(null);

// Watch for tab changes to load appropriate data
watch(selectedModType, async (newType) => {
  console.log("Tab changed to:", newType);
  currentPage.value = 1; // Reset to first page when changing tabs

  if (newType === "psychModpacks") {
    if (psychModpacks.value.length === 0) {
      await fetchPsychModpacks();
    } else {
      // If data is already loaded, just update the totalPages
      const response = await invoke<{ mods: GameBananaMod[]; total: number }>(
        "fetch_gamebanana_mods_command",
        {
          query: "_psychmodpack",
          page: 1,
        }
      );
      totalPages.value = Math.ceil(response.total / itemsPerPage);
    }
  } else if (newType === "vsliceModpacks") {
    if (vsliceModpacks.value.length === 0) {
      await fetchVsliceModpacks();
    } else {
      // If data is already loaded, just update the totalPages
      const response = await invoke<{ mods: GameBananaMod[]; total: number }>(
        "fetch_gamebanana_mods_command",
        {
          query: "_vslicemodpack",
          page: 1,
        }
      );
      totalPages.value = Math.ceil(response.total / itemsPerPage);
    }
  } else if (newType === "codenameModpacks") {
    if (codenameModpacks.value.length === 0) {
      await fetchCodenameModpacks();
    } else {
      // If data is already loaded, just update the totalPages
      const response = await invoke<{ mods: GameBananaMod[]; total: number }>(
        "fetch_gamebanana_mods_command",
        {
          query: "_codenamemodpack",
          page: 1,
        }
      );
      totalPages.value = Math.ceil(response.total / itemsPerPage);
    }
  } else if (newType === "executables") {
    if (latestMods.value.length === 0) {
      await fetchLatestMods();
    } else {
      // If data is already loaded, just update the totalPages
      const response = await invoke<{ mods: GameBananaMod[]; total: number }>(
        "fetch_gamebanana_mods_command",
        {
          query: "latest",
          page: 1,
        }
      );
      totalPages.value = Math.ceil(response.total / itemsPerPage);
    }
  }
});

// Function to set up event listeners for downloads
const setupEventListeners = () => {
  // Set up event listeners for download events
  listen<{
    modId: number;
    name: string;
    contentLength: number;
    thumbnailUrl: string;
  }>("download-started", (event) => {
    console.log("Download started:", event.payload);
    updateDownloadProgress({
      modId: event.payload.modId,
      name: event.payload.name,
      bytesDownloaded: 0,
      totalBytes: event.payload.contentLength,
      percentage: 0,
      step: "Starting download...",
      thumbnailUrl: event.payload.thumbnailUrl,
      isComplete: false,
      isError: false,
    });
  }).then((unsubscribe) => {
    removeDownloadStartedListener = unsubscribe;
  });

  listen<{
    modId: number;
    name: string;
    bytesDownloaded: number;
    totalBytes: number;
    percentage: number;
    step: string;
  }>("download-progress", (event) => {
    console.log("Download progress:", event.payload);
    updateDownloadProgress({
      modId: event.payload.modId,
      name: event.payload.name,
      bytesDownloaded: event.payload.bytesDownloaded,
      totalBytes: event.payload.totalBytes,
      percentage: event.payload.percentage,
      step: event.payload.step,
      thumbnailUrl: downloadingMods[event.payload.modId]?.thumbnailUrl || "",
      isComplete: false,
      isError: false,
    });
  }).then((unsubscribe) => {
    removeDownloadProgressListener = unsubscribe;
  });

  listen<{
    modId: number;
    name: string;
    modInfo: any;
  }>("download-finished", (event) => {
    console.log("Download finished:", event.payload);
    updateDownloadProgress({
      modId: event.payload.modId,
      name: event.payload.name,
      bytesDownloaded: 100,
      totalBytes: 100,
      percentage: 100,
      step: "Download complete",
      thumbnailUrl: downloadingMods[event.payload.modId]?.thumbnailUrl || "",
      isComplete: true,
      isError: false,
    });

    // Remove from downloads list after a delay
    setTimeout(() => {
      completeDownload(event.payload.modId);
    }, 2000);

    // Refresh the mods list by forcing navigation to the home page
    // This will ensure that the newly added mod appears in the list
    setTimeout(() => {
      if (router.currentRoute.value.path !== "/") {
        router.push("/");
      } else {
        // If already on home page, emit a custom event to refresh mod list
        const event = new CustomEvent("refresh-mods");
        window.dispatchEvent(event);
      }
    }, 1000);
  }).then((unsubscribe) => {
    removeDownloadFinishedListener = unsubscribe;
  });

  listen<{
    modId: number;
    name: string;
    error: string;
  }>("download-error", (event) => {
    console.log("Download error:", event.payload);
    updateDownloadProgress({
      modId: event.payload.modId,
      name: event.payload.name,
      bytesDownloaded: 0,
      totalBytes: 100,
      percentage: 0,
      step: "Error",
      thumbnailUrl: downloadingMods[event.payload.modId]?.thumbnailUrl || "",
      isComplete: false,
      isError: true,
      error: event.payload.error,
    });

    // Remove from downloads list after a delay
    setTimeout(() => {
      errorDownload(event.payload.modId, event.payload.error);
    }, 5000);
  }).then((unsubscribe) => {
    removeDownloadErrorListener = unsubscribe;
  });
};

onMounted(() => {
  // Load initial data
  fetchFeaturedMods();
  fetchLatestMods();

  // Also load the first tab of mod packs data if needed
  if (selectedModType.value === "psychModpacks") {
    fetchPsychModpacks();
  } else if (selectedModType.value === "vsliceModpacks") {
    fetchVsliceModpacks();
  } else if (selectedModType.value === "codenameModpacks") {
    fetchCodenameModpacks();
  }

  // Set up event listeners
  setupEventListeners();
});

// Clean up event listeners when component is unmounted
onBeforeUnmount(() => {
  removeDownloadStartedListener?.();
  removeDownloadProgressListener?.();
  removeDownloadFinishedListener?.();
  removeDownloadErrorListener?.();
});

// Data fetching functions
const fetchFeaturedMods = async () => {
  isLoadingFeatured.value = true;
  try {
    const response = await invoke<{ mods: GameBananaMod[]; total: number }>(
      "fetch_gamebanana_mods_command",
      {
        query: "featured",
        page: 1, // Always get first page for featured
      }
    );

    featuredMods.value = response.mods;
  } catch (error) {
    console.error("Failed to fetch featured mods:", error);
  } finally {
    isLoadingFeatured.value = false;
  }
};

const fetchLatestMods = async () => {
  isLoadingLatest.value = true;
  try {
    const response = await invoke<{ mods: GameBananaMod[]; total: number }>(
      "fetch_gamebanana_mods_command",
      {
        query: "latest",
        page: currentPage.value,
      }
    );

    latestMods.value = response.mods;
    totalPages.value = Math.ceil(response.total / itemsPerPage);
  } catch (error) {
    console.error("Failed to fetch latest mods:", error);
  } finally {
    isLoadingLatest.value = false;
  }
};

const fetchPsychModpacks = async () => {
  isLoadingPsychModpacks.value = true;
  try {
    const response = await invoke<{ mods: GameBananaMod[]; total: number }>(
      "fetch_gamebanana_mods_command",
      {
        query: "_psychmodpack",
        page: currentPage.value,
      }
    );

    psychModpacks.value = response.mods;
    if (selectedModType.value === "psychModpacks") {
      totalPages.value = Math.ceil(response.total / itemsPerPage);
    }
  } catch (error) {
    console.error("Failed to fetch Psych Engine modpacks:", error);
  } finally {
    isLoadingPsychModpacks.value = false;
  }
};

const fetchVsliceModpacks = async () => {
  isLoadingVsliceModpacks.value = true;
  try {
    const response = await invoke<{ mods: GameBananaMod[]; total: number }>(
      "fetch_gamebanana_mods_command",
      {
        query: "_vslicemodpack",
        page: currentPage.value,
      }
    );

    vsliceModpacks.value = response.mods;
    if (selectedModType.value === "vsliceModpacks") {
      totalPages.value = Math.ceil(response.total / itemsPerPage);
    }
  } catch (error) {
    console.error("Failed to fetch V-Slice modpacks:", error);
  } finally {
    isLoadingVsliceModpacks.value = false;
  }
};

const fetchCodenameModpacks = async () => {
  isLoadingCodenameModpacks.value = true;
  try {
    const response = await invoke<{ mods: GameBananaMod[]; total: number }>(
      "fetch_gamebanana_mods_command",
      {
        query: "_codenamemodpack",
        page: currentPage.value,
      }
    );

    codenameModpacks.value = response.mods;
    if (selectedModType.value === "codenameModpacks") {
      totalPages.value = Math.ceil(response.total / itemsPerPage);
    }
  } catch (error) {
    console.error("Failed to fetch Codename Engine modpacks:", error);
  } finally {
    isLoadingCodenameModpacks.value = false;
  }
};

// Search functions
const searchMods = async () => {
  if (!searchQuery.value.trim()) {
    activeView.value = "all";
    return;
  }

  activeView.value = "search";
  isLoadingSearch.value = true;
  currentPage.value = 1;

  try {
    const response = await invoke<{ mods: GameBananaMod[]; total: number }>(
      "fetch_gamebanana_mods_command",
      {
        query: searchQuery.value,
        page: currentPage.value,
      }
    );

    searchResults.value = response.mods;
    totalPages.value = Math.ceil(response.total / itemsPerPage);
    hasSearched.value = true;
  } catch (error) {
    console.error("Failed to search mods:", error);
  } finally {
    isLoadingSearch.value = false;
  }
};

const clearSearch = () => {
  searchQuery.value = "";
  activeView.value = "all";
  hasSearched.value = false;
};

// Pagination handling
const changePage = async (page: number) => {
  currentPage.value = page;

  if (activeView.value === "search") {
    isLoadingSearch.value = true;
    try {
      const response = await invoke<{ mods: GameBananaMod[]; total: number }>(
        "fetch_gamebanana_mods_command",
        {
          query: searchQuery.value,
          page: page,
        }
      );

      searchResults.value = response.mods;
      // Make sure to update totalPages from the response
      totalPages.value = Math.ceil(response.total / itemsPerPage);
    } catch (error) {
      console.error("Failed to fetch search page:", error);
    } finally {
      isLoadingSearch.value = false;
    }
  } else {
    // Handle pagination based on the current active tab
    if (selectedModType.value === "executables") {
      isLoadingLatest.value = true;
      try {
        const response = await invoke<{ mods: GameBananaMod[]; total: number }>(
          "fetch_gamebanana_mods_command",
          {
            query: "latest",
            page: page,
          }
        );

        latestMods.value = response.mods;
        // Make sure to update totalPages from the response
        totalPages.value = Math.ceil(response.total / itemsPerPage);
      } catch (error) {
        console.error("Failed to fetch latest page:", error);
      } finally {
        isLoadingLatest.value = false;
      }
    } else if (selectedModType.value === "psychModpacks") {
      isLoadingPsychModpacks.value = true;
      try {
        const response = await invoke<{ mods: GameBananaMod[]; total: number }>(
          "fetch_gamebanana_mods_command",
          {
            query: "_psychmodpack",
            page: page,
          }
        );

        psychModpacks.value = response.mods;
        // Make sure to update totalPages from the response
        totalPages.value = Math.ceil(response.total / itemsPerPage);
      } catch (error) {
        console.error("Failed to fetch Psych Engine modpacks page:", error);
      } finally {
        isLoadingPsychModpacks.value = false;
      }
    } else if (selectedModType.value === "vsliceModpacks") {
      isLoadingVsliceModpacks.value = true;
      try {
        const response = await invoke<{ mods: GameBananaMod[]; total: number }>(
          "fetch_gamebanana_mods_command",
          {
            query: "_vslicemodpack",
            page: page,
          }
        );

        vsliceModpacks.value = response.mods;
        // Make sure to update totalPages from the response
        totalPages.value = Math.ceil(response.total / itemsPerPage);
      } catch (error) {
        console.error("Failed to fetch V-Slice modpacks page:", error);
      } finally {
        isLoadingVsliceModpacks.value = false;
      }
    } else if (selectedModType.value === "codenameModpacks") {
      isLoadingCodenameModpacks.value = true;
      try {
        const response = await invoke<{ mods: GameBananaMod[]; total: number }>(
          "fetch_gamebanana_mods_command",
          {
            query: "_codenamemodpack",
            page: page,
          }
        );

        codenameModpacks.value = response.mods;
        // Make sure to update totalPages from the response
        totalPages.value = Math.ceil(response.total / itemsPerPage);
      } catch (error) {
        console.error("Failed to fetch Codename Engine modpacks page:", error);
      } finally {
        isLoadingCodenameModpacks.value = false;
      }
    }
  }
};

// Download handling
const downloadMod = async (mod: GameBananaMod) => {
  try {
    currentDownloadMod.value = mod;

    // Show loading notification
    pendingDownloadNotification = $q.notify({
      type: "ongoing",
      message: `Preparing to download "${mod.name}"...`,
      position: "bottom-right",
      timeout: 0,
    });

    // First check if this mod has multiple download options
    const downloadInfo = await invoke<any>("get_mod_download_files_command", {
      modId: mod.id,
    });

    // Check if there are multiple files
    if (downloadInfo._aFiles && downloadInfo._aFiles.length > 1) {
      // Set the download files for the selector
      downloadFiles.value = downloadInfo._aFiles;

      // Set alternate sources if available
      alternateFileSources.value = downloadInfo._aAlternateFileSources || [];

      // Show the file selector dialog first
      showFileSelector.value = true;

      // Dismiss the loading notification
      if (pendingDownloadNotification) {
        pendingDownloadNotification();
        pendingDownloadNotification = null;
      }

      return; // Wait for user selection in the file selector
    }

    // If there's only one file or no file selection needed, continue
    // Determine if this is a modpack
    const isModpack = determineIfModpack(mod);
    const modpackType = determineModpackType(mod); // 'psych', 'vslice', 'codename', or null

    if (isModpack) {
      // Handle modpack download logic
      console.log("Modpack detected:", modpackType);
      const engineMods = await getCompatibleEngineMods(modpackType);

      if (engineMods.length === 0) {
        // No compatible engine found, show error
        if (pendingDownloadNotification) {
          pendingDownloadNotification();
          pendingDownloadNotification = null;
        }

        $q.notify({
          type: "negative",
          message: `Cannot download ${modpackType} modpack`,
          caption: `You don't have any ${formatEngineType(
            modpackType
          )} installed. Please install it.`,
          position: "bottom-right",
          timeout: 5000,
        });

        return;
      } else {
        // Compatible engine found, proceed with showing engine selection dialog
        // Store current modpack info for later use in the dialog
        currentModpackInfo.value = {
          mod,
          type: modpackType,
          compatibleEngines: engineMods,
        };

        // Show engine selection dialog
        showEngineSelectDialog.value = true;

        // Dismiss the loading notification
        if (pendingDownloadNotification) {
          pendingDownloadNotification();
          pendingDownloadNotification = null;
        }

        return; // Wait for user selection of an engine
      }
    } else {
      // If there's only one file or no files available and it's not a modpack, proceed with normal download
      await startDownload(mod);
    }
  } catch (error) {
    // Show error notification
    $q.notify({
      type: "negative",
      message: `Failed to prepare download for "${mod.name}"`,
      caption: String(error),
      position: "bottom-right",
      timeout: 5000,
    });

    // Dismiss any pending notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
      pendingDownloadNotification = null;
    }

    console.error("Failed to prepare mod download:", error);
  }
};

// Function called when an engine is selected from the dialog
const onEngineSelected = async (engine: any) => {
  if (!currentModpackInfo.value) return;

  // Store the selected engine
  selectedEngineMod.value = engine;

  // Proceed with downloading the modpack
  await downloadModpackForSelectedEngine();
};

// Function to download modpack for the selected engine
const downloadModpackForSelectedEngine = async () => {
  if (!currentModpackInfo.value || !selectedEngineMod.value) return;

  try {
    // Create a new notification for the download process
    pendingDownloadNotification = $q.notify({
      type: "ongoing",
      message: `Downloading "${currentModpackInfo.value.mod.name}"...`,
      position: "bottom-right",
      timeout: 0,
    });

    const mod = currentModpackInfo.value.mod;

    // Get the installation path for the selected engine's mods folder
    const modsFolderPath = getModsFolderPath(selectedEngineMod.value);
    if (!modsFolderPath) {
      throw new Error("Could not determine mods folder path");
    }
    // Check if we already have a specific download URL from the file selector
    // This happens when the user first selected a file, then selected an engine
    if (
      mod.downloadUrl &&
      mod.downloadUrl !== currentModpackInfo.value.mod.downloadUrl
    ) {
      // A specific file was already chosen in the file selector
      console.log("Using previously selected file URL:", mod.downloadUrl);

      // Use the specific file URL that was already selected
      const result = await invoke<string>("download_gamebanana_mod_command", {
        url: mod.downloadUrl,
        name: mod.name,
        modId: mod.id,
        installLocation: modsFolderPath,
      });

      console.log("Download result:", result);
    }
    // Not a specific file selection, use the default download URL
    else {
      console.log("Using default download URL:", mod.downloadUrl);

      // Use the default download URL for direct download
      const result = await invoke<string>("download_gamebanana_mod_command", {
        url: mod.downloadUrl,
        name: mod.name,
        modId: mod.id,
        installLocation: modsFolderPath,
      });

      console.log("Download result:", result);
    }

    // Dismiss the loading notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
      pendingDownloadNotification = null;
    }

    // Show success notification
    $q.notify({
      type: "positive",
      message: `"${mod.name}" installed successfully!`,
      caption: `Ready to play from the mods list`,
      position: "bottom-right",
      timeout: 5000,
    });

    // Trigger the refresh event to update the mod list
    const refreshEvent = new CustomEvent("refresh-mods");
    window.dispatchEvent(refreshEvent);
  } catch (error) {
    // Show error notification
    $q.notify({
      type: "negative",
      message: `Failed to install modpack`,
      caption: String(error),
      position: "bottom-right",
      timeout: 5000,
    });

    // Dismiss any pending notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
      pendingDownloadNotification = null;
    }

    console.error("Failed to install modpack:", error);
  } finally {
    // Reset state
    showEngineSelectDialog.value = false;
    currentModpackInfo.value = null;
    selectedEngineMod.value = null;
    currentDownloadMod.value = null;
  }
};

// Function called when a file is selected from the dialog
const onFileSelected = async (selectedFile: any) => {
  if (!currentDownloadMod.value) return;

  try {
    // Create a new notification for the download process
    pendingDownloadNotification = $q.notify({
      type: "ongoing",
      message: `Preparing "${currentDownloadMod.value.name}"...`,
      position: "bottom-right",
      timeout: 0,
    });

    // Use the specific download URL from the selected file
    const mod = currentDownloadMod.value;

    // Clear any existing pending modpack path
    localStorage.removeItem("pendingModpackInstallPath");

    // Check if the selected file contains an executable
    // If it does, treat it as an executable mod regardless of other factors
    if (selectedFile._bContainsExe) {
      console.log("Selected file contains an executable, treating as standard mod");
      
      // Update notification to downloading
      if (pendingDownloadNotification) {
        pendingDownloadNotification();
      }
      pendingDownloadNotification = $q.notify({
        type: "ongoing",
        message: `Downloading "${mod.name}"...`,
        position: "bottom-right",
        timeout: 0,
      });

      // Get the install location from settings
      let installLocation: string | null = null;
      try {
        installLocation = await getInstallLocation();
      } catch (error) {
        console.warn("Could not get install location from settings:", error);
      }

      console.log("Using selected file URL:", selectedFile._sDownloadUrl);
      console.log("Using installation location:", installLocation);

      // Call backend to download using the specific file URL
      const result = await invoke<string>("download_gamebanana_mod_command", {
        url: selectedFile._sDownloadUrl,
        name: mod.name,
        modId: mod.id,
        installLocation,
      });
      
      // Process the result
      let modInfo: any;
      let modPath: string;

      try {
        // Try to parse as JSON first
        const parsed = JSON.parse(result);
        modPath = parsed.path;
        modInfo = parsed.mod_info;
      } catch (e) {
        // If parsing fails, assume it's just the path string
        modPath = result;
        // Get mod info directly from the backend
        const allMods = await invoke<any[]>("get_mods");
        modInfo = allMods.find((m) => m.path === modPath);

        // If we still don't have mod info, create a basic one
        if (!modInfo) {
          modInfo = {
            id: crypto.randomUUID(),
            name: mod.name,
            path: modPath,
            executable_path: null,
            icon_data: null,
            banner_data: mod.thumbnailUrl,
            version: mod.version || null,
            engine_type: null,
          };
        }
      }

      // Save the mod to the database
      if (modInfo) {
        await saveModToDatabase(modInfo);
      }

      // Dismiss loading notification
      if (pendingDownloadNotification) {
        pendingDownloadNotification();
        pendingDownloadNotification = null;
      }

      // Show success notification
      $q.notify({
        type: "positive",
        message: `"${mod.name}" downloaded and installed successfully!`,
        caption: `Ready to play from the mods list`,
        position: "bottom-right",
        timeout: 5000,
      });

      // Trigger the refresh event to update the mod list
      const refreshEvent = new CustomEvent("refresh-mods");
      window.dispatchEvent(refreshEvent);

      // Reset current download mod
      currentDownloadMod.value = null;
      return;
    }

    // If file doesn't contain an executable, check if this is a modpack
    const isModpack = determineIfModpack(mod);
    const modpackType = determineModpackType(mod);

    if (isModpack) {
      // Handle modpack download logic for selected file
      const engineMods = await getCompatibleEngineMods(modpackType);

      if (engineMods.length === 0) {
        // No compatible engine found, show error
        if (pendingDownloadNotification) {
          pendingDownloadNotification();
          pendingDownloadNotification = null;
        }

        $q.notify({
          type: "negative",
          message: `Cannot download ${modpackType} modpack`,
          caption: `You don't have any ${formatEngineType(
            modpackType
          )} installed. Please install it.`,
          position: "bottom-right",
          timeout: 5000,
        });

        return;
      } else {
        // Compatible engine found, store the selected file URL for later use
        currentModpackInfo.value = {
          mod: { ...mod, downloadUrl: selectedFile._sDownloadUrl }, // Override with selected URL
          type: modpackType,
          compatibleEngines: engineMods,
        };

        // Show engine selection dialog
        showEngineSelectDialog.value = true;

        // Dismiss the loading notification
        if (pendingDownloadNotification) {
          pendingDownloadNotification();
          pendingDownloadNotification = null;
        }

        return; // Wait for user selection of an engine
      }
    }

    // If not a modpack and doesn't contain an executable, proceed with standard download
    // Update notification to downloading
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
    }
    pendingDownloadNotification = $q.notify({
      type: "ongoing",
      message: `Downloading "${mod.name}"...`,
      position: "bottom-right",
      timeout: 0,
    });

    // Get the install location from settings
    let installLocation: string | null = null;
    try {
      installLocation = await getInstallLocation();
    } catch (error) {
      console.warn("Could not get install location from settings:", error);
    }

    console.log("Using selected file URL:", selectedFile._sDownloadUrl);
    console.log("Using installation location:", installLocation);

    // Call backend to download using the specific file URL
    const result = await invoke<string>("download_gamebanana_mod_command", {
      url: selectedFile._sDownloadUrl,
      name: mod.name,
      modId: mod.id,
      installLocation,
    });
    // Process the result
    let modInfo: any;
    let modPath: string;

    try {
      // Try to parse as JSON first
      const parsed = JSON.parse(result);
      modPath = parsed.path;
      modInfo = parsed.mod_info;
    } catch (e) {
      // If parsing fails, assume it's just the path string
      modPath = result;
      // Get mod info directly from the backend
      const allMods = await invoke<any[]>("get_mods");
      modInfo = allMods.find((m) => m.path === modPath);

      // If we still don't have mod info, create a basic one
      if (!modInfo) {
        modInfo = {
          id: crypto.randomUUID(),
          name: mod.name,
          path: modPath,
          executable_path: null,
          icon_data: null,
          banner_data: mod.thumbnailUrl,
          version: mod.version || null,
          engine_type: null,
        };
      }
    }

    // Save the mod to the database
    if (modInfo) {
      await saveModToDatabase(modInfo);
    }

    // Dismiss loading notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
      pendingDownloadNotification = null;
    }

    // Show success notification
    $q.notify({
      type: "positive",
      message: `"${mod.name}" downloaded and installed successfully!`,
      caption: `Ready to play from the mods list`,
      position: "bottom-right",
      timeout: 5000,
    });

    // Trigger the refresh event to update the mod list
    const refreshEvent = new CustomEvent("refresh-mods");
    window.dispatchEvent(refreshEvent);

    // Reset current download mod
    currentDownloadMod.value = null;
  } catch (error) {
    // Show error notification
    $q.notify({
      type: "negative",
      message: `Failed to download "${
        currentDownloadMod.value?.name || "Mod"
      }"`,
      caption: String(error),
      position: "bottom-right",
      timeout: 5000,
    });

    // Dismiss any pending notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
      pendingDownloadNotification = null;
    }

    console.error("Failed to download mod:", error);

    // Reset current download mod
    currentDownloadMod.value = null;
  }
};

// Function to cancel the download
const cancelDownload = () => {
  // Dismiss any pending notification
  if (pendingDownloadNotification) {
    pendingDownloadNotification();
    pendingDownloadNotification = null;
  }

  // Show cancellation notification
  if (currentDownloadMod.value) {
    $q.notify({
      type: "info",
      message: `Download of "${currentDownloadMod.value.name}" cancelled`,
      position: "bottom-right",
      timeout: 3000,
    });
  }

  // Reset current download mod
  currentDownloadMod.value = null;
};

// Original download function, renamed to startDownload
const startDownload = async (mod: GameBananaMod) => {
  try {
    // First notification for downloading
    $q.notify({
      type: "info",
      message: `Starting download of "${mod.name}"`,
      position: "bottom-right",
      timeout: 2000,
    });

    // Get the install location from settings
    let installLocation: string | null = null;
    try {
      installLocation = await getInstallLocation();
    } catch (error) {
      console.warn("Could not get install location from settings:", error);
    }

    // Pass mod ID along with URL, name, and install location
    const result = await invoke<string>("download_gamebanana_mod_command", {
      url: mod.downloadUrl,
      name: mod.name,
      modId: mod.id,
      installLocation,
    });
    console.log("download_gamebanana_mod_command result", result);

    // Parse the response result - it might be a simple path string or a JSON object
    let modInfo: any;
    let modPath: string;

    try {
      // Try to parse as JSON first
      const parsed = JSON.parse(result);
      modPath = parsed.path;
      modInfo = parsed.mod_info;
    } catch (e) {
      // If parsing fails, assume it's just the path string
      modPath = result;
      // Get mod info directly from the backend
      const allMods = await invoke<any[]>("get_mods");
      modInfo = allMods.find((m) => m.path === modPath);

      // If we still don't have mod info, create a basic one
      if (!modInfo) {
        modInfo = {
          id: crypto.randomUUID(),
          name: mod.name,
          path: modPath,
          executable_path: null,
          icon_data: null,
          banner_data: mod.thumbnailUrl,
          version: mod.version || null,
          engine_type: null,
        };
      }
    }

    // Save the mod to the database
    if (modInfo) {
      await saveModToDatabase(modInfo);
    }

    // Dismiss loading notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
      pendingDownloadNotification = null;
    }

    // Show success notification
    $q.notify({
      type: "positive",
      message: `"${mod.name}" downloaded and installed successfully!`,
      caption: `Ready to play from the mods list`,
      position: "bottom-right",
      timeout: 5000,
    });

    // Trigger the refresh event to update the mod list
    const refreshEvent = new CustomEvent("refresh-mods");
    window.dispatchEvent(refreshEvent);
  } catch (error) {
    // Show error notification
    $q.notify({
      type: "negative",
      message: `Failed to download "${mod.name}"`,
      caption: String(error),
      position: "bottom-right",
      timeout: 5000,
    });

    // Dismiss any pending notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
      pendingDownloadNotification = null;
    }

    console.error("Failed to download mod:", error);
  }
};

// Save a mod to the database
const saveModToDatabase = async (mod: any) => {
  try {
    // Check if DatabaseService is initialized
    if (!window.db || !window.db.service) {
      console.warn("Database service not initialized yet, cannot save mod");
      return false;
    }

    console.log("Saving mod to database using DatabaseService:", mod);

    // Make sure the mod has an engine field required by the type
    if (!mod.engine) {
      mod.engine = {
        engine_type: mod.engine_type || "unknown",
        engine_name: mod.engine_type || "Unknown Engine",
        engine_icon: "",
        mods_folder: false,
        mods_folder_path: "",
      };
    }

    // Use the DatabaseService to save the mod
    await window.db.service.saveMod(mod);

    console.log("Mod saved successfully to database:", mod.name);
    return true;
  } catch (error) {
    console.error("Failed to save mod to database:", error);
    return false;
  }
};

// Determine if a mod is a modpack based on the current tab or mod properties
const determineIfModpack = (mod: GameBananaMod): boolean => {
  // Check if we're in a modpack tab
  if (
    ["psychModpacks", "vsliceModpacks", "codenameModpacks"].includes(
      selectedModType.value
    )
  ) {
    return true;
  }

  // Check mod category if available
  if (mod.categoryName) {
    const lowerCaseCategoryName = mod.categoryName.toLowerCase();
    if (lowerCaseCategoryName.includes("executables")) return false;
    if (lowerCaseCategoryName.includes("psych")) return true;
    if (lowerCaseCategoryName.includes("v-slice")) return true;
    if (lowerCaseCategoryName.includes("codename")) return true;
  }

  return false;
};

// Determine modpack type (psych, vslice, codename, or null if not a modpack)
const determineModpackType = (mod: GameBananaMod): string | null => {
  // First check the current tab
  if (selectedModType.value === "psychModpacks") return "psych";
  if (selectedModType.value === "vsliceModpacks") return "vanilla";
  if (selectedModType.value === "codenameModpacks") return "codename";


  console.log("Mod category:", mod.categoryName);

  // Check mod category if available
  if (mod.categoryName) {
    const lowerCaseCategoryName = mod.categoryName.toLowerCase();
    if (lowerCaseCategoryName.includes("psych")) return "psych";
    if (lowerCaseCategoryName.includes("v-slice")) return "vanilla";
    if (lowerCaseCategoryName.includes("codename")) return "codename";
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
    let allMods: any[] = [];
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

// Add new method to handle direct engine downloads
const downloadEngine = async (engineType: string) => {
  try {
    // Show loading notification
    pendingDownloadNotification = $q.notify({
      type: "ongoing",
      message: `Preparing to download ${formatEngineType(engineType)}...`,
      position: "bottom-right",
      timeout: 0,
    });

    // Get the install location from settings
    let installLocation: string | null = null;
    try {
      if (window.db && window.db.service) {
        installLocation = await window.db.service.getSetting("installLocation");
      }
    } catch (error) {
      console.warn("Could not get install location from settings:", error);
    }

    console.log(
      `Downloading ${engineType} engine to ${
        installLocation || "default location"
      }`
    );

    // Call backend to download the engine directly
    const result = await invoke<string>("download_engine_command", {
      engineType,
      installLocation,
    });

    // Process the result
    let modInfo: any;

    try {
      // Try to parse as JSON
      const parsed = JSON.parse(result);
      modInfo = parsed.mod_info;
    } catch (e) {
      // If parsing fails, assume it's just the path string
      const modPath = result;
      // Get mod info directly from the backend
      const allMods = await invoke<any[]>("get_mods");
      modInfo = allMods.find((m) => m.path === modPath);
    }

    // Save the mod to the database
    if (modInfo) {
      await saveModToDatabase(modInfo);
    }

    // Dismiss loading notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
      pendingDownloadNotification = null;
    }

    // Show success notification
    $q.notify({
      type: "positive",
      message: `${formatEngineType(engineType)} installed successfully!`,
      caption: `Ready to play from the mods list`,
      position: "bottom-right",
      timeout: 5000,
    });

    // Trigger the refresh event to update the mod list
    const refreshEvent = new CustomEvent("refresh-mods");
    window.dispatchEvent(refreshEvent);
  } catch (error) {
    // Show error notification
    $q.notify({
      type: "negative",
      message: `Failed to download ${formatEngineType(engineType)}`,
      caption: String(error),
      position: "bottom-right",
      timeout: 5000,
    });

    // Dismiss any pending notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
      pendingDownloadNotification = null;
    }

    console.error(`Failed to download ${engineType} engine:`, error);
  }
};

// Get the install location from settings
const getInstallLocation = async (): Promise<string | null> => {
  try {
    const storeService = StoreService.getInstance();
    return await storeService.getSetting("installLocation");
  } catch (error) {
    console.warn("Could not get install location from settings:", error);
    return null;
  }
};

// Custom URL download flow
const onCustomUrlSubmit = (formData: any) => {
  console.log("Custom URL form submitted:", formData);
  customModData.value = formData;
  showCustomUrlModal.value = false;
  showModTypeModal.value = true;
};

const onModTypeSubmit = async (typeData: any) => {
  console.log("Mod type selected:", typeData);
  if (!customModData.value) return;

  try {
    // Show loading notification
    pendingDownloadNotification = $q.notify({
      type: "ongoing",
      message: `Preparing to download "${customModData.value.name}"...`,
      position: "bottom-right",
      timeout: 0,
    });

    // Determine install location based on mod type
    let installLocation: string | null = null;
    
    if (typeData.modType === 'executable') {
      // For standalone mods, use the standard install location
      installLocation = await getInstallLocation();
    } else {
      // For modpacks, use the engine's mods folder
      if (typeData.engineMod) {
        installLocation = getModsFolderPath(typeData.engineMod);
      } else {
        throw new Error(`No ${formatEngineType(typeData.modType)} installation found`);
      }
    }

    console.log(`Downloading ${customModData.value.name} to ${installLocation || 'default location'}`);

    // Generate a random modId for tracking the download
    const modId = Math.floor(Math.random() * 1000000);

    // Call backend to download using the custom mod command instead
    const result = await invoke<string>("download_custom_mod_command", {
      url: customModData.value.url,
      name: customModData.value.name,
      modId,
      installLocation,
      thumbnailUrl: customModData.value.bannerData,
      description: customModData.value.description,
      version: customModData.value.version
    });

    // Process the result
    let modInfo: any;
    let modPath: string;

    try {
      // Try to parse as JSON
      const parsed = JSON.parse(result);
      modPath = parsed.path;
      modInfo = parsed.mod_info;
    } catch (e) {
      // If parsing fails, assume it's just the path string
      modPath = result;
      // Get mod info directly from the backend
      const allMods = await invoke<any[]>("get_mods");
      modInfo = allMods.find((m) => m.path === modPath);
    }

    // If we still don't have mod info, create one with our custom data
    if (!modInfo) {
      modInfo = {
        id: crypto.randomUUID(),
        name: customModData.value.name,
        path: modPath,
        executable_path: null,
        icon_data: null,
        banner_data: customModData.value.bannerData,
        logo_data: customModData.value.logoData,
        description: customModData.value.description,
        version: customModData.value.version || null,
        engine_type: typeData.modType !== 'executable' ? typeData.modType : null,
        engine: typeData.modType !== 'executable' ? {
          engine_type: typeData.modType,
          engine_name: formatEngineType(typeData.modType),
          mods_folder: true,
          mods_folder_path: "mods"
        } : null
      };
    } else {
      // Update the mod info with custom data
      modInfo.name = customModData.value.name;
      modInfo.banner_data = customModData.value.bannerData || modInfo.banner_data;
      modInfo.logo_data = customModData.value.logoData || modInfo.logo_data;
      modInfo.description = customModData.value.description || modInfo.description;
      modInfo.version = customModData.value.version || modInfo.version;
      
      if (typeData.modType !== 'executable') {
        modInfo.engine_type = typeData.modType;
        modInfo.engine = {
          ...(modInfo.engine || {}),
          engine_type: typeData.modType,
          engine_name: formatEngineType(typeData.modType),
          mods_folder: true,
          mods_folder_path: "mods"
        };
      }
    }

    // Save the mod to the database
    if (modInfo) {
      await saveModToDatabase(modInfo);
    }

    // Dismiss loading notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
      pendingDownloadNotification = null;
    }

    // Show success notification
    $q.notify({
      type: "positive",
      message: `"${customModData.value.name}" downloaded and installed successfully!`,
      caption: `Ready to play from the mods list`,
      position: "bottom-right",
      timeout: 5000,
    });

    // Trigger the refresh event to update the mod list
    const refreshEvent = new CustomEvent("refresh-mods");
    window.dispatchEvent(refreshEvent);

    // Reset state
    showModTypeModal.value = false;
    customModData.value = null;

  } catch (error) {
    // Show error notification
    $q.notify({
      type: "negative",
      message: `Failed to download "${customModData.value.name}"`,
      caption: String(error),
      position: "bottom-right",
      timeout: 5000,
    });

    // Dismiss any pending notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
      pendingDownloadNotification = null;
    }

    console.error("Failed to download custom mod:", error);

    // Reset state but keep the modal open to allow for corrections
    showModTypeModal.value = false;
    showCustomUrlModal.value = true;
  }
};
</script>

<style scoped>
.gamebanana-browser {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.scroll-container {
  overflow-y: auto;
  flex-grow: 1;
}

.mods-section {
  margin-bottom: 24px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.text-subtitle1 {
  color: var(--theme-text);
  font-size: 1.5rem;
  margin-top: 1rem;
}

.q-carousel {
  background-color: transparent;
}

.q-img__content > div {
  background-color: transparent;
}

.q-tab-panel,
.q-tab-panels,
.q-panel {
  background-color: transparent !important;
}

/* Style the tabs to use theme colors */
:deep(.q-tab) {
  color: var(--theme-text-secondary);
}

:deep(.q-tab--active) {
  color: var(--theme-text);
}

:deep(.q-field__native),
:deep(.q-field__input) {
  color: var(--theme-text) !important;
}

:deep(.q-field__label) {
  color: var(--theme-text-secondary) !important;
}

:deep(.q-field__marginal) {
  color: var(--theme-text-secondary);
}

:deep(input::placeholder) {
  color: var(--theme-text-secondary) !important;
}

:deep(.q-field__messages) {
  color: var(--theme-text-secondary) !important;
}
</style>
