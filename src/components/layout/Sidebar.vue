<template>
  <div class="layout-container">
    <!-- The Sidebar ended up becoming the wrapper around basically everything -->
    <!-- Actual Sidebar -->
    <div
      class="sidebar"
      :class="{ 'compact-mode': isCompactMode }"
      :style="{ width: isCompactMode ? '64px' : `${sidebarWidth}px` }"
    >
      <!-- Mod list -->
      <ModList
        :mods="mods"
        :folders="folders"
        :selectedModId="selectedMod?.id"
        :compact-mode="isCompactMode"
        :active-page="activePage"
        @select-mod="selectMod"
        @add-mod="selectModFolder"
        @add-mods-folder="selectModsParentFolder"
        @delete-mod="deleteMod"
        @delete-folder="deleteFolder"
        @create-folder="createFolder"
        @add-mod-to-folder="addModToFolder"
        @open-settings="openAppSettings"
        @open-mod-settings="openModSettings"
        @reorder-items="handleModsReorder"
        @update-mod="handleSaveMod"
        @update-folder="updateFolderDetails"
        @reorder-folder-mods="handleFolderModsReorder"
        @launch-mod="launchMod"
        @super-delete-mod="superDeleteMod"
        @open-mod-folder="openModFolder"
        @go-home="setActivePage('home')"
        class="modlist"
      />

      <!-- GameBanana button -->
      <div class="gamebanana-button-container phantom-font-difficulty">
        <q-btn
          :class="{
            'gamebanana-button': true,
            'active-gamebanana': activePage === 'gamebanana',
          }"
          color="primary"
          @click="setActivePage('gamebanana')"
          flat
        >
          <div class="gb-logo-container">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 318 32"
              class="gb-logo"
            >
              <defs>
                <linearGradient id="gradient" x2="0" y2="1">
                  <stop offset="0%" stop-color="var(--stop-color-1)" />
                  <stop offset="100%" stop-color="var(--stop-color-2)" />
                </linearGradient>
              </defs>
              <g shape-rendering="crispEdges">
                <g class="gb-logo2">
                  <polygon
                    points="31 3 30 3 29 3 28 3 27 3 26 3 25 3 24 3 23 3 22 3 21 3 20 3 19 3 18 3 17 3 16 3 15 3 14 3 13 3 12 3 11 3 10 3 9 3 8 3 7 3 6 3 5 3 4 3 3 3 3 4 3 5 3 6 3 7 3 8 3 9 3 10 3 11 3 12 3 13 3 14 3 15 3 16 3 17 3 18 3 19 3 20 3 21 3 22 3 23 3 24 3 25 3 26 3 27 3 28 3 29 4 29 5 29 6 29 7 29 8 29 9 29 10 29 11 29 12 29 13 29 14 29 15 29 16 29 17 29 18 29 19 29 20 29 21 29 22 29 23 29 24 29 25 29 26 29 27 29 28 29 29 29 30 29 31 29 32 29 33 29 33 28 33 27 33 26 33 25 33 24 33 23 33 22 33 21 33 20 33 19 33 18 33 17 33 16 33 15 33 14 33 13 32 13 31 13 30 13 29 13 28 13 27 13 26 13 25 13 24 13 23 13 22 13 21 13 20 13 19 13 19 14 19 15 19 16 19 17 19 18 19 19 20 19 21 19 22 19 23 19 24 19 25 19 26 19 27 19 27 20 27 21 27 22 27 23 26 23 25 23 24 23 23 23 22 23 21 23 20 23 19 23 18 23 17 23 16 23 15 23 14 23 13 23 12 23 11 23 10 23 9 23 9 22 9 21 9 20 9 19 9 18 9 17 9 16 9 15 9 14 9 13 9 12 9 11 9 10 9 9 10 9 11 9 12 9 13 9 14 9 15 9 16 9 17 9 18 9 19 9 20 9 21 9 22 9 23 9 24 9 25 9 26 9 27 9 28 9 29 9 30 9 31 9 32 9 33 9 33 8 33 7 33 6 33 5 33 4 33 3 32 3 31 3"
                  />
                  <path
                    d="M62,3h-28v26h6v-10h18v10h6V3h-2ZM58,10v3h-18v-4h18v1Z"
                  />
                  <polygon
                    points="99 3 98 3 97 3 96 3 95 3 94 3 93 3 92 3 91 3 90 3 89 3 88 3 87 3 86 3 85 3 84 3 83 3 82 3 81 3 80 3 79 3 78 3 77 3 76 3 75 3 74 3 73 3 72 3 71 3 70 3 69 3 68 3 67 3 66 3 65 3 65 4 65 5 65 6 65 7 65 8 65 9 65 10 65 11 65 12 65 13 65 14 65 15 65 16 65 17 65 18 65 19 65 20 65 21 65 22 65 23 65 24 65 25 65 26 65 27 65 28 65 29 66 29 67 29 68 29 69 29 70 29 71 29 71 28 71 27 71 26 71 25 71 24 71 23 71 22 71 21 71 20 71 19 71 18 71 17 71 16 71 15 71 14 71 13 71 12 71 11 71 10 71 9 72 9 73 9 74 9 75 9 76 9 77 9 78 9 79 9 80 9 80 10 80 11 80 12 80 13 80 14 80 15 80 16 80 17 80 18 80 19 80 20 80 21 80 22 80 23 80 24 80 25 80 26 80 27 80 28 80 29 81 29 82 29 83 29 84 29 85 29 86 29 86 28 86 27 86 26 86 25 86 24 86 23 86 22 86 21 86 20 86 19 86 18 86 17 86 16 86 15 86 14 86 13 86 12 86 11 86 10 86 9 87 9 88 9 89 9 90 9 91 9 92 9 93 9 94 9 95 9 95 10 95 11 95 12 95 13 95 14 95 15 95 16 95 17 95 18 95 19 95 20 95 21 95 22 95 23 95 24 95 25 95 26 95 27 95 28 95 29 96 29 97 29 98 29 99 29 100 29 101 29 101 28 101 27 101 26 101 25 101 24 101 23 101 22 101 21 101 20 101 19 101 18 101 17 101 16 101 15 101 14 101 13 101 12 101 11 101 10 101 9 101 8 101 7 101 6 101 5 101 4 101 3 100 3 99 3"
                  />
                  <polygon
                    points="128 3 127 3 126 3 125 3 124 3 123 3 122 3 121 3 120 3 119 3 118 3 117 3 116 3 115 3 114 3 113 3 112 3 111 3 110 3 109 3 108 3 107 3 106 3 105 3 104 3 103 3 102 3 102 4 102 5 102 6 102 7 102 8 102 9 102 10 102 11 102 12 102 13 102 14 102 15 102 16 102 17 102 18 102 19 102 20 102 21 102 22 102 23 102 24 102 25 102 26 102 27 102 28 102 29 103 29 104 29 105 29 106 29 107 29 108 29 109 29 110 29 111 29 112 29 113 29 114 29 115 29 116 29 117 29 118 29 119 29 120 29 121 29 122 29 123 29 124 29 125 29 126 29 127 29 128 29 129 29 130 29 130 28 130 27 130 26 130 25 130 24 130 23 129 23 128 23 127 23 126 23 125 23 124 23 123 23 122 23 121 23 120 23 119 23 118 23 117 23 116 23 115 23 114 23 113 23 112 23 111 23 110 23 109 23 108 23 108 22 108 21 108 20 108 19 109 19 110 19 111 19 112 19 113 19 114 19 115 19 116 19 117 19 118 19 119 19 120 19 121 19 122 19 123 19 124 19 125 19 126 19 127 19 128 19 129 19 130 19 130 18 130 17 130 16 130 15 130 14 130 13 129 13 128 13 127 13 126 13 125 13 124 13 123 13 122 13 121 13 120 13 119 13 118 13 117 13 116 13 115 13 114 13 113 13 112 13 111 13 110 13 109 13 108 13 108 12 108 11 108 10 108 9 109 9 110 9 111 9 112 9 113 9 114 9 115 9 116 9 117 9 118 9 119 9 120 9 121 9 122 9 123 9 124 9 125 9 126 9 127 9 128 9 129 9 130 9 130 8 130 7 130 6 130 5 130 4 130 3 129 3 128 3"
                  />
                  <path
                    d="M191,3h-28v26h6v-10h18v10h6V3h-2ZM187,10v3h-18v-4h18v1Z"
                  />
                  <polygon
                    points="221 3 220 3 219 3 218 3 217 3 216 3 215 3 214 3 213 3 212 3 211 3 210 3 209 3 208 3 207 3 206 3 205 3 204 3 203 3 202 3 201 3 200 3 199 3 198 3 197 3 196 3 195 3 194 3 194 4 194 5 194 6 194 7 194 8 194 9 194 10 194 11 194 12 194 13 194 14 194 15 194 16 194 17 194 18 194 19 194 20 194 21 194 22 194 23 194 24 194 25 194 26 194 27 194 28 194 29 195 29 196 29 197 29 198 29 199 29 200 29 200 28 200 27 200 26 200 25 200 24 200 23 200 22 200 21 200 20 200 19 200 18 200 17 200 16 200 15 200 14 200 13 200 12 200 11 200 10 200 9 201 9 202 9 203 9 204 9 205 9 206 9 207 9 208 9 209 9 210 9 211 9 212 9 213 9 214 9 215 9 216 9 217 9 217 10 217 11 217 12 217 13 217 14 217 15 217 16 217 17 217 18 217 19 217 20 217 21 217 22 217 23 217 24 217 25 217 26 217 27 217 28 217 29 218 29 219 29 220 29 221 29 222 29 223 29 223 28 223 27 223 26 223 25 223 24 223 23 223 22 223 21 223 20 223 19 223 18 223 17 223 16 223 15 223 14 223 13 223 12 223 11 223 10 223 9 223 8 223 7 223 6 223 5 223 4 223 3 222 3 221 3"
                  />
                  <path
                    d="M252,3h-28v26h6v-10h18v10h6V3h-2ZM248,10v3h-18v-4h18v1Z"
                  />
                  <polygon
                    points="282 3 281 3 280 3 279 3 278 3 277 3 276 3 275 3 274 3 273 3 272 3 271 3 270 3 269 3 268 3 267 3 266 3 265 3 264 3 263 3 262 3 261 3 260 3 259 3 258 3 257 3 256 3 255 3 255 4 255 5 255 6 255 7 255 8 255 9 255 10 255 11 255 12 255 13 255 14 255 15 255 16 255 17 255 18 255 19 255 20 255 21 255 22 255 23 255 24 255 25 255 26 255 27 255 28 255 29 256 29 257 29 258 29 259 29 260 29 261 29 261 28 261 27 261 26 261 25 261 24 261 23 261 22 261 21 261 20 261 19 261 18 261 17 261 16 261 15 261 14 261 13 261 12 261 11 261 10 261 9 262 9 263 9 264 9 265 9 266 9 267 9 268 9 269 9 270 9 271 9 272 9 273 9 274 9 275 9 276 9 277 9 278 9 278 10 278 11 278 12 278 13 278 14 278 15 278 16 278 17 278 18 278 19 278 20 278 21 278 22 278 23 278 24 278 25 278 26 278 27 278 28 278 29 279 29 280 29 281 29 282 29 283 29 284 29 284 28 284 27 284 26 284 25 284 24 284 23 284 22 284 21 284 20 284 19 284 18 284 17 284 16 284 15 284 14 284 13 284 12 284 11 284 10 284 9 284 8 284 7 284 6 284 5 284 4 284 3 283 3 282 3"
                  />
                  <path
                    d="M314,3h-29v26h6v-10h18v10h6V3h-1ZM309,10v3h-18v-4h18v1Z"
                  />
                  <path
                    d="M160,13V3h-27v26h29V13h-2ZM156,20v3h-17v-4h17v1ZM139,12v-3h15v4h-15v-1Z"
                  />
                </g>

                <g class="gb-logo1">
                  <polygon
                    points="153 9 152 9 151 9 150 9 149 9 148 9 147 9 146 9 145 9 144 9 143 9 142 9 141 9 140 9 139 9 139 10 139 11 139 12 139 13 140 13 141 13 142 13 143 13 144 13 145 13 146 13 147 13 148 13 149 13 150 13 151 13 152 13 153 13 154 13 154 12 154 11 154 10 154 9 153 9"
                  />
                  <polygon
                    points="186 9 185 9 184 9 183 9 182 9 181 9 180 9 179 9 178 9 177 9 176 9 175 9 174 9 173 9 172 9 171 9 170 9 169 9 169 10 169 11 169 12 169 13 170 13 171 13 172 13 173 13 174 13 175 13 176 13 177 13 178 13 179 13 180 13 181 13 182 13 183 13 184 13 185 13 186 13 187 13 187 12 187 11 187 10 187 9 186 9"
                  />
                  <polygon
                    points="247 9 246 9 245 9 244 9 243 9 242 9 241 9 240 9 239 9 238 9 237 9 236 9 235 9 234 9 233 9 232 9 231 9 230 9 230 10 230 11 230 12 230 13 231 13 232 13 233 13 234 13 235 13 236 13 237 13 238 13 239 13 240 13 241 13 242 13 243 13 244 13 245 13 246 13 247 13 248 13 248 12 248 11 248 10 248 9 247 9"
                  />
                  <polygon
                    points="308 9 307 9 306 9 305 9 304 9 303 9 302 9 301 9 300 9 299 9 298 9 297 9 296 9 295 9 294 9 293 9 292 9 291 9 291 10 291 11 291 12 291 13 292 13 293 13 294 13 295 13 296 13 297 13 298 13 299 13 300 13 301 13 302 13 303 13 304 13 305 13 306 13 307 13 308 13 309 13 309 12 309 11 309 10 309 9 308 9"
                  />
                  <polygon
                    points="155 19 154 19 153 19 152 19 151 19 150 19 149 19 148 19 147 19 146 19 145 19 144 19 143 19 142 19 141 19 140 19 139 19 139 20 139 21 139 22 139 23 140 23 141 23 142 23 143 23 144 23 145 23 146 23 147 23 148 23 149 23 150 23 151 23 152 23 153 23 154 23 155 23 156 23 156 22 156 21 156 20 156 19 155 19"
                  />
                  <g>
                    <polygon
                      class="cls-1"
                      points="56 9 55 9 54 9 53 9 52 9 51 9 50 9 49 9 48 9 47 9 46 9 45 9 44 9 43 9 42 9 41 9 40 9 40 10 40 11 40 12 40 13 41 13 42 13 43 13 44 13 45 13 46 13 47 13 48 13 49 13 50 13 51 13 52 13 53 13 54 13 55 13 56 13 57 13 58 13 58 12 58 11 58 10 58 9 57 9 56 9"
                    />
                    <path
                      class="cls-1"
                      d="M317,0H0v32h43v-10h12v10h19V12h3v20h12V12h3v20h80v-10h12v10h19V12h11v20h19v-10h12v10h19V12h11v20h19v-10h12v10h12V0h-1ZM33,4v5H9v14h18v-4h-8v-6h14v16H3V3h30v1ZM64,4v25h-6v-10h-18v10h-6V3h30v1ZM101,4v25h-6V9h-9v20h-6V9h-9v20h-6V3h36v1ZM130,4v5h-22v4h22v6h-22v4h22v6h-28V3h28v1ZM162,14v15h-29V3h27v10h2v1ZM193,4v25h-6v-10h-18v10h-6V3h30v1ZM223,4v25h-6V9h-17v20h-6V3h29v1ZM254,4v25h-6v-10h-18v10h-6V3h30v1ZM284,4v25h-6V9h-17v20h-6V3h29v1ZM315,4v25h-6v-10h-18v10h-6V3h30v1Z"
                    />
                  </g>
                </g>
              </g>
            </svg>
          </div>
        </q-btn>
      </div>

      <!-- Resize handle -->
      <div class="resize-handle" @mousedown="startResize">
        <div class="resize-handle-indicator"></div>
      </div>
    </div>
    <div class="main-content-area">
      <Transition name="fade" mode="out-in">
        <div
          :key="
            activePage === 'mods'
              ? selectedMod
                ? selectedMod.id
                : 'no-mod'
              : activePage === 'gamebanana'
              ? 'gamebanana-browser'
              : 'home-page'
          "
          class="main-content"
        >
          <!-- Can show ModDetails, HomePage, or GameBananaBrowser -->
          <component
            :is="
              activePage === 'mods'
                ? ModDetails
                : activePage === 'gamebanana'
                ? GameBananaBrowser
                : HomePage
            "
            :mod="selectedMod"
            :error="launchError || ''"
            @update:mod="handleSaveMod"
            @launch-mod="launchMod"
            @open-settings="openSettingsModal"
            @select-mod="selectMod"
            @open-mod-settings="openModSettings"
            @gamebanana-browser="openGamebananaBrowser"
          />
        </div>
      </Transition>
    </div>

    <!-- Settings Modal -->
    <ModSettingsModal
      v-model="showSettingsModal"
      :mod="selectedMod"
      @save="handleSaveMod"
      @change-folder="handleSelectModFolder"
      @select-executable="handleSelectExecutable"
    />

    <!-- App Settings Modal -->
    <AppSettingsModal v-model="showAppSettingsModal" @save="saveAppSettings" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import ModList from "@mods/ModList.vue";
import ModDetails from "@mods/ModDetails.vue";
import ModSettingsModal from "@modals/ModSettingsModal.vue";
import AppSettingsModal from "@modals/AppSettingsModal.vue";
import GameBananaBrowser from "@mods/GameBananaBrowser.vue";
import HomePage from "@mods/HomePage.vue";
import { Mod, Folder, DisplayItem } from "@main-types";
import { useQuasar } from "quasar";
import { StoreService } from "../../services/storeService";
import { DatabaseService } from "@services/dbService";
import { formatEngineName } from "@utils/index";
import { revealItemInDir } from "@tauri-apps/plugin-opener";
import { sep } from "@tauri-apps/api/path";

// Use the singleton directly instead of through a ref
const storeService = StoreService.getInstance();
const dbService = DatabaseService.getInstance();

// TypeScript declaration for db
declare global {
  interface Window {
    db: any;
  }
}

// So I didn't have to change anything
type ModInfo = Mod;

// Use the props without storing in a variable to avoid the unused variable warning
defineProps<{
  modelValue: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: boolean): void;
  (e: "resize", width: number): void;
}>();

// Default width and min/max constraints
const sidebarWidth = ref(250);
const minWidth = 240;
const maxWidth = 400;

// Resize functionality
let isResizing = false;

const startResize = (_e: MouseEvent) => {
  isResizing = true;
  document.addEventListener("mousemove", handleResize);
  document.addEventListener("mouseup", stopResize);
  document.body.style.userSelect = "none";
};

const $q = useQuasar();

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
  emit("resize", newWidth);
};

const stopResize = () => {
  isResizing = false;
  document.removeEventListener("mousemove", handleResize);
  document.removeEventListener("mouseup", stopResize);
  document.body.style.userSelect = "";
};

// Mod list and selection
const mods = ref<ModInfo[]>([]);
const folders = ref<Folder[]>([]);
const selectedMod = ref<ModInfo | null>(null);
const launchError = ref<string | null>(null);
const showSettingsModal = ref(false);
const activePage = ref("home"); // Default to showing home page
const showAppSettingsModal = ref(false);

// Add compact mode state
const isCompactMode = ref(false);

// Load mods on component mount
onMounted(async () => {
  // Initialize the StoreService
  await storeService.initialize();

  emit("resize", sidebarWidth.value);
  try {
    // Initialize dbService
    await dbService.initialize();
    // Assign the service instance to the window object if needed elsewhere
    window.db = { service: dbService };
    await loadModsFromDatabase();
    await loadFoldersFromDatabase(); // Load folders after mods

    // Load and apply app settings
    await loadAppSettings();

    activePage.value = "home"; // Show home page by default

    // Add event listener for refreshing the mods list
    window.addEventListener("refresh-mods", handleRefreshMods);

    // Add event listener for compact mode changes
    window.addEventListener("compact-mode-changed", (event: Event) => {
      const customEvent = event as CustomEvent;
      if (customEvent.detail && customEvent.detail.compactMode !== undefined) {
        isCompactMode.value = customEvent.detail.compactMode;
        console.log("Applied compact mode from event:", isCompactMode.value);
      }
    });
  } catch (error) {
    console.error("Error loading mods:", error);
  }
});

// Load mods from the database
const loadModsFromDatabase = async () => {
  try {
    // Use the DatabaseService to get all mods
    const processedMods = await dbService.getAllMods();

    if (processedMods && processedMods.length > 0) {
      mods.value = processedMods;

      // Sync mods from database to backend in-memory state
      console.log("Syncing mods from database to backend state on app start");
      await dbService.syncModsWithBackend();

      // If there's at least one mod, select the first one
      if (processedMods.length > 0 && !selectedMod.value) {
        selectMod(processedMods[0]);
      }
    } else {
      // If no mods in database, load from memory
      await loadModsFromMemory();
    }
  } catch (error) {
    console.error("Failed to load mods from database:", error);
    // Fallback to in-memory mods
    await loadModsFromMemory();
  }
};

// Load folders from the database
const loadFoldersFromDatabase = async () => {
  try {
    console.log("Loading folders from database");

    // Use the DatabaseService to get all folders
    const loadedFolders = await dbService.getAllFolders();

    if (loadedFolders && loadedFolders.length > 0) {
      console.log(`Found ${loadedFolders.length} folders in database`);

      // Update the reactive folders ref
      folders.value = loadedFolders;
      console.log("Folders loaded successfully:", folders.value);
    } else {
      console.log("No folders found in database");
      folders.value = [];
    }
  } catch (error) {
    console.error("Failed to load folders from database:", error);
    folders.value = [];
  }
};

// Load all mods from backend memory, if any
// This is a fallback if no mods are found in the database
const loadModsFromMemory = async () => {
  try {
    const modList = await invoke<ModInfo[]>("get_mods");

    // First, check for mods with display_order of 9999 or undefined and fix them
    let needsReindexing = false;
    const processedMods = modList.map((mod, index) => {
      // Check if display_order is problematic (undefined, null, 9999)
      if (
        mod.display_order === undefined ||
        mod.display_order === null ||
        mod.display_order === 9999
      ) {
        needsReindexing = true;
        // Assign a reasonable index based on position in array
        return { ...mod, display_order: index };
      }
      return mod;
    });

    // Sort mods by display_order
    mods.value = processedMods.sort((a, b) => {
      return (a.display_order ?? 0) - (b.display_order ?? 0);
    });

    // If we had to fix any mod display orders, save them back to the database
    if (needsReindexing) {
      console.log("Fixing incorrect display_order values in mods...");
      try {
        await dbService.saveMods(mods.value); // Use dbService to save all mods
        console.log("Successfully fixed display_order values");

        // Sync with backend so it's aware of our changes
        await dbService.syncModsWithBackend();
      } catch (error) {
        console.error("Failed to fix mod display_order values:", error);
      }
    }
  } catch (error) {
    console.error("Failed to load mods:", error);
  }
};

// Save a mod to the database
const saveModToDatabase = async (mod: ModInfo) => {
  try {
    // Ensure engine name is formatted before initial save
    if (mod.engine && !mod.engine.engine_name) {
      mod.engine.engine_name = formatEngineName(mod.engine.engine_type);
    } else if (!mod.engine) {
      mod.engine = {
        engine_type: "unknown",
        engine_name: formatEngineName("unknown"),
        engine_icon: "",
        mods_folder: false,
        mods_folder_path: "",
      };
    }
    await dbService.saveMod(mod);
    console.log("Mod saved successfully:", mod.name);
    await loadModsFromDatabase(); // Reload mods from DB
    await loadFoldersFromDatabase(); // Reload folders from DB
  } catch (error) {
    console.error("Failed to save mod:", error);
    $q.notify({
      type: "negative",
      message: "Failed to save mod",
      caption: String(error),
      position: "bottom-right",
      timeout: 3000,
    });
  }
};

const selectModFolder = async () => {
  try {
    // Get current validation setting from database
    let validateFnfMods = true; // default to true if setting not found

    try {
      const validateResult = await storeService.getSetting("validateFnfMods");
      if (validateResult) {
        validateFnfMods = validateResult === true;
      }
    } catch (settingError) {
      console.warn(
        "Could not get validateFnfMods setting, using default:",
        settingError
      );
    }

    const modFolderPath = await invoke<string>("select_mod_folder");
    if (modFolderPath) {
      try {
        // Pass the validation setting to the backend
        const modInfo = await invoke<ModInfo>("add_mod", {
          path: modFolderPath,
          validate: validateFnfMods,
        });

        // Ensure the mod has an engine object before saving
        if (!modInfo.engine) {
          modInfo.engine = {
            engine_type: "unknown",
            engine_name: "",
            engine_icon: "",
            mods_folder: false,
            mods_folder_path: "",
          };
        }

        mods.value.push(modInfo);
        selectMod(modInfo);
        await saveModToDatabase(modInfo);

        // Show success notification
        $q.notify({
          type: "positive",
          message: `"${modInfo.name}" added successfully!`,
          position: "bottom-right",
          timeout: 3000,
        });
      } catch (error) {
        // This likely means the folder failed validation
        $q.notify({
          type: "negative",
          message: "Invalid FNF Mod",
          caption: String(error),
          position: "bottom-right",
          timeout: 5000,
        });
        console.error("Failed to add mod:", error);
      }
    }
  } catch (error) {
    console.error("Failed to select mod folder:", error);
  }
};

const selectModsParentFolder = async () => {
  try {
    // Get current validation setting from database
    let validateFnfMods = true; // default to true if setting not found

    try {
      const validateResult = await storeService.getSetting("validateFnfMods");
      if (validateResult) {
        validateFnfMods = validateResult === true;
      }
    } catch (settingError) {
      console.warn(
        "Could not get validateFnfMods setting, using default:",
        settingError
      );
    }

    // Show loading notification
    const loadingNotif = $q.notify({
      type: "ongoing",
      message: "Scanning for mods...",
      caption: "This may take a while for large folders",
      position: "bottom-right",
      timeout: 0,
      spinner: true,
    });

    // Call the Rust command to select a folder containing multiple mods
    const addedMods = await invoke<ModInfo[]>("select_mods_parent_folder", {
      validate: validateFnfMods,
    });

    // Dismiss loading notification
    loadingNotif();

    if (addedMods && addedMods.length > 0) {
      // Add each mod to the mods array
      for (const modInfo of addedMods) {
        // Ensure the mod has an engine object before saving
        if (!modInfo.engine) {
          modInfo.engine = {
            engine_type: "unknown",
            engine_name: "",
            engine_icon: "",
            mods_folder: false,
            mods_folder_path: "",
          };
        }
        mods.value.push(modInfo);
        await saveModToDatabase(modInfo);
      }

      // Select the first added mod
      selectMod(addedMods[0]);

      // Show a success message
      $q.notify({
        type: "positive",
        message: `Successfully added ${addedMods.length} mods`,
        position: "bottom-right",
        timeout: 3000,
      });
    } else {
      // Show a message when no mods were found
      $q.notify({
        type: "info",
        message: "No compatible mods found",
        caption: "Make sure the folder contains valid FNF mods",
        position: "bottom-right",
        timeout: 3000,
      });
    }
  } catch (error) {
    // This will catch validation errors or other issues
    $q.notify({
      type: "negative",
      message: "Failed to import mods",
      caption: String(error),
      position: "bottom-right",
      timeout: 5000,
    });
    console.error("Failed to select mods parent folder:", error);
  }
};

const selectMod = (mod: ModInfo) => {
  console.log("selectMod called with mod:", mod);
  selectedMod.value = mod;
  console.log("selectedMod.value after setting:", selectedMod.value);
  launchError.value = null;
  activePage.value = "mods"; // Switch to mod details view when selecting a mod
};

const setActivePage = (page: string) => {
  activePage.value = page;
};

// Function to handle saving changes to an existing mod (
const handleSaveMod = async (updatedMod: Mod) => {
  if (!updatedMod || !updatedMod.id) {
    console.error("handleSaveMod called with invalid mod data");
    return;
  }
  console.log(
    "SIDEBAR: handleSaveMod called for mod:",
    updatedMod.name,
    "with data:",
    JSON.stringify(updatedMod)
  ); // Log incoming data
  try {
    // Ensure engine object exists before saving
    if (!updatedMod.engine) {
      console.warn("Mod engine object missing, creating default.");
      updatedMod.engine = {
        engine_type: "unknown",
        engine_name: formatEngineName("unknown"), // Use util
        engine_icon: "",
        mods_folder: false,
        mods_folder_path: "",
      };
    } else {
      // Ensure engine_name is set correctly based on type if needed
      const defaultName = formatEngineName(updatedMod.engine.engine_type);
      // Only update name if it's empty or matches the default name of the *original* type
      const originalMod = mods.value.find((m) => m.id === updatedMod.id);
      const originalDefaultName = formatEngineName(
        originalMod?.engine?.engine_type || "unknown"
      );
      if (
        !updatedMod.engine.engine_name ||
        updatedMod.engine.engine_name === originalDefaultName
      ) {
        updatedMod.engine.engine_name = defaultName;
      }
    }

    console.log(
      "SIDEBAR: Calling dbService.saveMod with:",
      JSON.stringify(updatedMod)
    ); // Log data being sent to dbService
    await dbService.saveMod(updatedMod); // Use the main saveMod function
    console.log("Mod details updated successfully in DB:", updatedMod.name);

    // Update the local state
    const index = mods.value.findIndex((m) => m.id === updatedMod.id);
    if (index !== -1) {
      // Create a deep copy to avoid reactivity issues if the same object reference is used elsewhere
      const savedModCopy = JSON.parse(JSON.stringify(updatedMod));
      mods.value[index] = savedModCopy; // Update mod in the main list
      if (selectedMod.value && selectedMod.value.id === updatedMod.id) {
        selectedMod.value = savedModCopy; // Update selected mod details if it's the one being edited
      }
    }
  } catch (error) {
    console.error("Failed to update mod details:", error);
    $q.notify({
      type: "negative",
      message: `Failed to update "${updatedMod.name}".`,
      caption: String(error),
      position: "bottom-right",
      timeout: 3000,
    });
  }
};

// Function to handle selecting (updating/changing) a mod folder
const handleSelectModFolder = async (callback?: (newPath: string) => void) => {
  if (!selectedMod.value) return;
  try {
    const result = await open({
      directory: true,
      title: "Select Mod Folder",
    });

    if (result) {
      // Update the mod's path with the new folder path
      console.log("Selected mod folder:", result);
      const updatedMod = { ...selectedMod.value, path: result };

      // If a callback was provided, call it with the new path
      if (callback && typeof callback === "function") {
        callback(result);
      }

      // Save the updated mod
      await handleSaveMod(updatedMod);

      // Show success notification
      $q.notify({
        type: "positive",
        message: "Mod folder updated",
        position: "bottom-right",
        timeout: 3000,
      });
    }
  } catch (error) {
    console.error("Error selecting mod folder:", error);
    $q.notify({
      type: "negative",
      message: "Error changing mod folder.",
      caption: String(error),
      position: "bottom-right",
      timeout: 3000,
    });
  }
};

// Function to handle selecting (updating/changing) an executable for the mod
const handleSelectExecutable = async (
  callback?: (newExecutablePath: string) => void
) => {
  if (!selectedMod.value) return;
  try {
    const result = await open({
      title: "Select Mod Executable",
      filters: [
        { name: "Executables", extensions: ["exe"] },
        { name: "All Files", extensions: ["*"] },
      ],
      defaultPath: selectedMod.value.path, // Start in the mod's directory
    });

    if (result) {
      // Update the mod's executable_path with the new executable path
      console.log("Selected mod executable:", result);
      const updatedMod = { ...selectedMod.value, executable_path: result };

      // If a callback was provided, call it with the new path
      if (callback && typeof callback === "function") {
        callback(result);
      }

      // Save the updated mod
      await handleSaveMod(updatedMod);

      // Show success notification
      $q.notify({
        type: "positive",
        message: "Executable path updated",
        position: "bottom-right",
        timeout: 3000,
      });
    }
  } catch (error) {
    console.error("Error selecting executable:", error);
    $q.notify({
      type: "negative",
      message: "Error selecting executable.",
      caption: String(error),
      position: "bottom-right",
      timeout: 3000,
    });
  }
};

// Function that launches the selected mod
const launchMod = async (modId: string) => {
  try {
    // Now invoke the backend to launch the mod
    await invoke("launch_mod", { id: modId });

    // Find the mod in our local array by ID
    const mod = mods.value.find((m) => m.id === modId);

    if (mod) {      // Update the last_played timestamp before launching
      const updatedMod = {
        ...mod,
        last_played: Math.trunc(Date.now() / 1000), // Current Unix timestamp in seconds
      };

      // Save to database to persist the last_played value
      await dbService.saveMod(updatedMod);

      // Update the local state
      const index = mods.value.findIndex((m) => m.id === modId);
      if (index !== -1) {
        mods.value[index] = updatedMod;
      }

      // If this is the currently selected mod, update that reference too
      if (selectedMod.value?.id === modId) {
        selectedMod.value = updatedMod;
      }

      console.log(
        `Updated last_played timestamp for mod ${updatedMod.name} to ${updatedMod.last_played}`
      );
    }
  } catch (error) {
    console.error("Failed to launch mod:", error);
    launchError.value = "Failed to launch mod";
  }
};

// Function that opens the setting modal
const openSettingsModal = () => {
  showSettingsModal.value = true;
};

// Function to delete a mod
const deleteMod = async (modId: string) => {
  try {
    // Remove mod from the mods array
    mods.value = mods.value.filter((mod) => mod.id !== modId);

    // If the deleted mod was the selected mod, clear the selection
    if (selectedMod.value?.id === modId) {
      selectedMod.value = null;
    }

    // Delete mod from the database using the service
    await dbService.deleteMod(modId);

    // Force a refresh of the UI to ensure the mod is removed from displayItems
    const refreshEvent = new CustomEvent("refresh-mods");
    window.dispatchEvent(refreshEvent);
  } catch (error) {
    console.error("Failed to delete mod:", error);
  }
};

// Function to super delete a mod (completely remove its folder from disk)
const superDeleteMod = async (modId: string) => {
  try {
    // Show a loading notification
    const loadingNotif = $q.notify({
      type: "ongoing",
      message: "Super deleting mod...",
      position: "bottom-right",
      timeout: 0,
      spinner: true,
    });

    // Call the Rust backend to perform super delete
    await invoke("super_delete_mod", { id: modId });

    // Dismiss loading notification
    loadingNotif();

    // Remove mod from the mods array
    mods.value = mods.value.filter((mod) => mod.id !== modId);

    // If the deleted mod was the selected mod, clear the selection
    if (selectedMod.value?.id === modId) {
      selectedMod.value = null;
    }

    // Delete mod from the database using the service
    await dbService.deleteMod(modId);

    // Show success notification
    $q.notify({
      type: "positive",
      message: "Mod completely deleted",
      caption: "The mod folder and all its contents have been deleted",
      position: "bottom-right",
      timeout: 3000,
    });

    // Force a refresh of the UI to ensure the mod is removed from displayItems
    const refreshEvent = new CustomEvent("refresh-mods");
    window.dispatchEvent(refreshEvent);
  } catch (error) {
    console.error("Failed to super delete mod:", error);

    // Show error notification
    $q.notify({
      type: "negative",
      message: "Failed to delete mod files",
      caption: String(error),
      position: "bottom-right",
      timeout: 5000,
    });
  }
};

// Function to handle refreshing mods list
const handleRefreshMods = async () => {
  console.log("Refreshing mods list");
  await loadModsFromDatabase();
  await loadFoldersFromDatabase();
};

// Function that creates a new folder
const createFolder = async (folder: Folder) => {
  console.log("Creating new folder:", folder);
  try {
    await dbService.saveFolder(folder);

    console.log("Folder created successfully in database");

    // Add the folder to the local folders array
    folders.value.push({
      ...folder,
      mods: [], // Ensure mods is initialized as an empty array
    });

    console.log("Updated folders list:", folders.value);
  } catch (error) {
    console.error("Error creating folder:", error);
  }
};

// Function to handle folder updates, saving to the database
const updateFolderDetails = async (updatedFolder: Folder) => {
  console.log(
    "Updating folder details:",
    updatedFolder.name,
    "mods count:",
    updatedFolder.mods.length
  );

  // Find and update the folder in the folders array
  const folderIndex = folders.value.findIndex((f) => f.id === updatedFolder.id);

  try {
    if (folderIndex !== -1) {
      // Existing folder: preserve display_order to prevent folder from moving in the list
      const currentDisplayOrder = folders.value[folderIndex].display_order;
      updatedFolder.display_order = currentDisplayOrder;

      // Update the folder in the array
      folders.value[folderIndex] = updatedFolder;
      console.log("Updated existing folder in array at index", folderIndex);
    } else {
      // New folder: add it to the folders array
      console.log("Adding new folder to array:", updatedFolder.name);
      folders.value.push(updatedFolder);
    }

    // Save to database
    await dbService.saveFolder(updatedFolder);
    console.log("Folder saved successfully to database:", updatedFolder.name);
  } catch (error) {
    console.error("Failed to save folder to database:", error);
  }
};

// Function to add a mod to a folder
const addModToFolder = async (data: { modId: string; folderId: string }) => {
  const { modId, folderId } = data;
  console.log(`Adding mod ${modId} to folder ${folderId}`);

  try {
    await dbService.moveModToFolder(modId, folderId);

    console.log("Mod added to folder in database");

    // Update the local folders array
    const folderIndex = folders.value.findIndex((f) => f.id === folderId);
    if (folderIndex !== -1) {
      // Add the mod ID to the folder's mods array if it's not already there
      if (!folders.value[folderIndex].mods.includes(modId)) {
        folders.value[folderIndex].mods.push(modId);
      }
      console.log(`Updated folder ${folderId} with mod ${modId}`);

      // Update the mod's folder_id in our local mods array
      const modIndex = mods.value.findIndex((m) => m.id === modId);
      if (modIndex !== -1) {
        mods.value[modIndex].folder_id = folderId;
      }

      // Force a refresh of the UI
      folders.value = [...folders.value];
    }
  } catch (error) {
    console.error("Error adding mod to folder:", error);
  }
};

// Function to delete a folder
const deleteFolder = async (folderId: string) => {
  console.log(`Deleting folder with ID: ${folderId}`);

  try {
    await dbService.deleteFolder(folderId);

    console.log("Folder deleted from database");

    // Remove the folder from the local folders array
    folders.value = folders.value.filter((f) => f.id !== folderId);

    // Update any mods that were in this folder
    mods.value.forEach((mod) => {
      if (mod.folder_id === folderId) {
        mod.folder_id = undefined;
      }
    });

    console.log("Updated folders list after deletion");
  } catch (error) {
    console.error("Error deleting folder:", error);
  }
};

// Function to open app settings modal
const openAppSettings = () => {
  showAppSettingsModal.value = true;
};

// Function to save app settings
const saveAppSettings = async (settings: any) => {
  console.log("App settings saved:", settings);

  // Apply the accent color immediately - ensure it's a string
  let colorValue = settings.accentColor || "#DB2955";

  // Handle if accentColor is stored as a JSON string (from previous version)
  if (
    typeof colorValue === "string" &&
    colorValue.trim().startsWith("{") &&
    colorValue.includes("value")
  ) {
    try {
      // Validate JSON format before parsing
      if (/^[\s]*\{.*\}[\s]*$/.test(colorValue)) {
        const parsedColor = JSON.parse(colorValue);
        if (
          parsedColor &&
          parsedColor.value &&
          typeof parsedColor.value === "string"
        ) {
          colorValue = parsedColor.value;

          // Also fix it in the database by saving it back as a string
          await storeService.saveSetting("accentColor", colorValue);
          console.log("Fixed accent color format in database:", colorValue);
        } else {
          // If parsed but invalid structure, use default
          colorValue = "#DB2955";
          console.log(
            "Parsed color doesn't have a valid value property, using default"
          );
        }
      } else {
        // Not valid JSON object format
        colorValue = "#DB2955";
        console.log("Invalid JSON format for color value, using default");
      }
    } catch (e) {
      console.error("Failed to parse accent color JSON:", e);
      // Save the default color to fix the database
      colorValue = "#DB2955";
      await storeService.saveSetting("accentColor", colorValue);
      console.log("Saved default accent color to fix invalid data");
    }
  }
  // Handle if accentColor is an object with a value property
  else if (typeof colorValue !== "string" && colorValue?.value) {
    colorValue = colorValue.value;
  }

  // Ensure colorValue is always a valid CSS color string
  if (typeof colorValue !== "string" || !colorValue.startsWith("#")) {
    colorValue = "#DB2955"; // Fallback to default if invalid
  }
  document.documentElement.style.setProperty("--q-primary", colorValue);
  console.log("Applied accent color from settings:", colorValue);

  window.dispatchEvent(new CustomEvent("settings-saved", { detail: settings }));
};

// Function that loads and applies app settings
const loadAppSettings = async () => {
  try {
    if (!storeService) {
      console.warn("Store service not initialized yet, cannot load settings");
      return;
    }

    // Get all settings at once through the StoreService
    const settings = await storeService.getAllSettings();

    // Apply compact mode setting
    isCompactMode.value = settings.compactMode === true;
    console.log("Applied compact mode setting:", isCompactMode.value);

    // Apply the accent color to CSS custom properties
    let colorValue = settings.accentColor || "#DB2955";

    // Handle if accentColor is stored as a JSON string (from previous version)
    if (
      typeof colorValue === "string" &&
      colorValue.trim().startsWith("{") &&
      colorValue.includes("value")
    ) {
      try {
        // Validate JSON format before parsing
        if (/^[\s]*\{.*\}[\s]*$/.test(colorValue)) {
          const parsedColor = JSON.parse(colorValue);
          if (
            parsedColor &&
            parsedColor.value &&
            typeof parsedColor.value === "string"
          ) {
            colorValue = parsedColor.value;

            // Also fix it in the database by saving it back as a string
            await storeService.saveSetting("accentColor", colorValue);
            console.log("Fixed accent color format in database:", colorValue);
          } else {
            // If parsed but invalid structure, use default
            colorValue = "#DB2955";
            console.log(
              "Parsed color doesn't have a valid value property, using default"
            );
          }
        } else {
          // Not valid JSON object format
          colorValue = "#DB2955";
          console.log("Invalid JSON format for color value, using default");
        }
      } catch (e) {
        console.error("Failed to parse accent color JSON:", e);
        // Save the default color to fix the database
        colorValue = "#DB2955";
        await storeService.saveSetting("accentColor", colorValue);
        console.log("Saved default accent color to fix invalid data");
      }
    }
    // Handle if accentColor is an object with a value property
    else if (typeof colorValue !== "string" && colorValue?.value) {
      const originalValue = colorValue;
      colorValue = colorValue.value;

      // Also fix it in the database
      await storeService.saveSetting("accentColor", colorValue);
      console.log(
        "Fixed accent color format in database:",
        originalValue,
        "→",
        colorValue
      );
    }

    // Ensure colorValue is always a valid CSS color string
    if (typeof colorValue !== "string" || !colorValue.startsWith("#")) {
      colorValue = "#DB2955"; // Fallback to default if invalid
    }    document.documentElement.style.setProperty("--q-primary", colorValue);
    console.log("Applied accent color from settings:", colorValue);
  } catch (error) {
    console.error("Failed to load and apply app settings:", error);
  }
};

// Function to handle reordering mods
const handleModsReorder = async (newOrder: DisplayItem[]) => {
  console.log(
    "Reordering display items, new order:",
    newOrder.map(
      (item) =>
        `${item.type}:${item.type === "mod" ? item.data.name : item.data.name}`
    )
  );

  // Process the reordered items by type (mods or folders)
  for (let index = 0; index < newOrder.length; index++) {
    const item = newOrder[index];

    if (item.type === "mod") {
      // Find the corresponding mod in our mods array and update its display_order
      const modIndex = mods.value.findIndex((m) => m.id === item.data.id);
      if (modIndex !== -1) {
        mods.value[modIndex].display_order = index;
      }
    } else if (item.type === "folder") {
      // Find the corresponding folder in our folders array and update its display_order
      const folderIndex = folders.value.findIndex((f) => f.id === item.data.id);
      if (folderIndex !== -1) {
        folders.value[folderIndex].display_order = index;
      }
    }
  }

  // Force reactivity update
  mods.value = [...mods.value];
  folders.value = [...folders.value];

  try {
    // Use the DatabaseService to update display order
    await dbService.updateDisplayOrder(newOrder);
    console.log("Successfully saved display order to database");
  } catch (error) {
    console.error("Failed to save mod order:", error);
  }
};

// Function to handle reordering mods within a folder
const handleFolderModsReorder = async (data: {
  folderId: string;
  updatedMods: Mod[];
}) => {
  console.log(
    `Handling folder mods reorder for folder: ${data.folderId}, mods count: ${data.updatedMods.length}`
  );

  try {
    await dbService.updateDisplayOrderInFolder(data.folderId, data.updatedMods);
    console.log(
      `Successfully updated display order for ${data.updatedMods.length} mods in folder`
    );

    // Update the local mods array with the new display_order_in_folder values
    data.updatedMods.forEach((updatedMod, index) => {
      const modIndex = mods.value.findIndex((m) => m.id === updatedMod.id);
      if (modIndex !== -1) {
        mods.value[modIndex].display_order_in_folder = index;
      }
    });

    // Force reactivity update
    mods.value = [...mods.value];
  } catch (error) {
    console.error("Failed to save folder mods order:", error);
  }
};

// Function to handle opening settings for a specific mod
const openModSettings = (mod: ModInfo) => {
  // Set the selected mod and open the settings modal
  selectedMod.value = mod;
  showSettingsModal.value = true;
};

const openModFolder = async (mod: ModInfo) => {
  // Open the mod folder in the file explorer
  await revealItemInDir(mod.path + sep());
};

const openGamebananaBrowser = () => {
  setActivePage("gamebanana");
};

// Clean up event listeners
onUnmounted(() => {
  if (isResizing) {
    document.removeEventListener("mousemove", handleResize);
    document.removeEventListener("mouseup", stopResize);
    document.body.style.userSelect = "";
  }

  // Remove refresh-mods event listener
  window.removeEventListener("refresh-mods", handleRefreshMods);
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
  background-image: url("/images/menuTransparent.png");
  background-size: cover;
  background-position: center;
  background-blend-mode: overlay;
  background-color: var(--theme-surface);
  border-radius: 1rem 1rem 0 0;
  margin: 8px;
}

.main-content {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.resize-handle {
  position: absolute;
  top: 0;
  right: -5px;
  width: 10px;
  height: 100%;
  cursor: ew-resize;
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
  margin-top: auto;
}

.gamebanana-button {
  width: 100%;
  transition: background-color 0.2s ease;
  border-radius: 0.5rem;
}

.main-content-area {
  scrollbar-width: none;
}

.gb-logo-container {
  width: 100%;
  height: 100%;
  max-width: 250px;
  display: flex;
  justify-content: center;
  align-items: center;
}
.gb-logo1 {
  fill: var(--theme-bg);
}
.gb-logo2 {
  fill: url(#gradient);
}

#gradient {
  --stop-color-1: var(--q-primary);
  --stop-color-2: color-mix(in oklab, var(--q-primary) 75%, black 25%);
}
</style>
