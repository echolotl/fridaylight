<template>
  <div class="gamebanana-browser phantom-font">
    <SearchBar
      :search-query="searchQuery"
      @update:search-query="searchQuery = $event"
      @search="searchMods"
      @clear="clearSearch"
      @custom-download="showCustomUrlModal = true"
    />

    <!-- Search Results View -->
    <div v-if="activeView === 'search'" class="section-header phantom-font">
      <div class="text-subtitle1 phantom-font-difficulty">Search Results</div>
      <q-btn
        flat
        color="primary"
        icon="arrow_back"
        label="Back to Home"
        @click="clearSearch"
      />
    </div>
    <q-scroll-area v-if="activeView === 'search'" class="scroll-container">
      <ModGrid
        :mods="searchResults"
        :loading="isLoadingSearch"
        loading-message="Searching mods..."
        :empty-message="`No mods found matching '${searchQuery}'`"
        :current-page="currentPage"
        :total-pages="totalPages"
        :input-pagination="true"
        @download="downloadMod"
        @page-change="changePage"
        @show-details="openModDetails"
      />
    </q-scroll-area>

    <!-- Home View with Featured and Latest Mods -->
    <q-scroll-area v-else class="scroll-container">
      <div class="main-browser">
        <div class="featured-header">
          <h6 class="phantom-font-difficulty q-mb-xs">Featured Mods</h6>
          <div class="mods-section">
            <FeaturedModsCarousel
              :mods="featuredMods"
              :loading="isLoadingFeatured"
              @download="downloadMod"
              @show-details="openModDetails"
            />
          </div>
        </div>

        <!-- Latest Mods Section -->
        <div class="mods-section">
          <h6 class="phantom-font-difficulty q-mb-md">
            Latest Mods
            <hr />
          </h6>
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
            <!-- Executables Tab -->
            <q-tab-panel name="executables">
              <ModGrid
                :mods="latestMods"
                :loading="isLoadingLatest"
                loading-message="Loading latest mods..."
                empty-message="No mods found"
                :current-page="currentPage"
                :total-pages="totalPages"
                :input-pagination="true"
                @download="downloadMod"
                @page-change="changePage"
                @show-details="openModDetails"
              />
            </q-tab-panel>

            <!-- Psych Engine Modpacks Tab -->
            <q-tab-panel name="psychModpacks">
              <ModGrid
                :mods="psychModpacks"
                :loading="isLoadingPsychModpacks"
                loading-message="Loading Psych Engine modpacks..."
                empty-message="No Psych Engine modpacks found"
                :current-page="currentPage"
                :total-pages="totalPages"
                :input-pagination="true"
                @download="downloadMod"
                @page-change="changePage"
                @show-details="openModDetails"
              />
            </q-tab-panel>

            <!-- V-Slice Modpacks Tab -->
            <q-tab-panel name="vsliceModpacks">
              <ModGrid
                :mods="vsliceModpacks"
                :loading="isLoadingVsliceModpacks"
                loading-message="Loading V-Slice modpacks..."
                empty-message="No V-Slice modpacks found"
                :current-page="currentPage"
                :total-pages="totalPages"
                :input-pagination="true"
                @download="downloadMod"
                @page-change="changePage"
                @show-details="openModDetails"
              />
            </q-tab-panel>

            <!-- Codename Engine Modpacks Tab -->
            <q-tab-panel name="codenameModpacks">
              <ModGrid
                :mods="codenameModpacks"
                :loading="isLoadingCodenameModpacks"
                loading-message="Loading Codename Engine modpacks..."
                empty-message="No Codename Engine modpacks found"
                :current-page="currentPage"
                :total-pages="totalPages"
                :input-pagination="true"
                @download="downloadMod"
                @page-change="changePage"
                @show-details="openModDetails"
              />
            </q-tab-panel>
          </q-tab-panels>
        </div>
        <!-- Engine Download Buttons -->
        <EngineDownloadButtons @download-engine="downloadEngine" />
      </div>
    </q-scroll-area>

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
      :mod-data="customModData"
      @submit="onModTypeSubmit"
      @back="
        showModTypeModal =
          false && customModData?.isCustomUrl
            ? (showCustomUrlModal = true)
            : false
      "
      @cancel="handleModTypeCancel"
    />
    <!-- Folder Exists Confirmation Dialog -->
    <FolderExistsDialog
      v-model="showFolderExistsDialog"
      :mod-name="folderExistsModName"
      @update="updateFolderExistsMod"
      @download-anyway="continueFolderExistsDownload"
      @cancel="cancelDownload"
    />

    <!-- Mod Details Modal -->
    <ModDetailsModal
      :mod-id="selectedModId"
      :model-type="currentModelType"
      :is-open="isModDetailsModalOpen"
      @update:is-open="isModDetailsModalOpen = $event"
      @download="downloadMod"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useQuasar, Notify } from 'quasar'
import {
  gamebananaService,
  setupGameBananaEventListeners,
} from '@services/gamebananaService'
import type { ModpackInfo } from '@services/gamebananaService'

// Import local components
import SearchBar from '@components/common/SearchBar.vue'
import ModGrid from '@components/common/ModGrid.vue'
import FeaturedModsCarousel from '@mods/FeaturedModsCarousel.vue'
import EngineDownloadButtons from '@mods/EngineDownloadButtons.vue'
import DownloadFileSelector from '@modals/DownloadFileSelector.vue'
import EngineSelectionDialog from '@modals/EngineSelectionDialog.vue'
import CustomUrlDownloadModal from '@modals/CustomUrlDownloadModal.vue'
import ModTypeSelectionModal from '@modals/ModTypeSelectionModal.vue'
import FolderExistsDialog from '@modals/FolderExistsDialog.vue'
import ModDetailsModal from '@modals/ModDetailsModal.vue'
import { StoreService } from '../../services/storeService'

// Declare db for TypeScript
declare global {
  interface Window {
    db: any
  }
}

const props = defineProps<{
  openModDetailsOnMount?: { open: boolean; modId: number; modelType: string }
}>()

const emit = defineEmits(['resetModDetailsOnMount'])

// Ensure Notify is properly registered
Notify.create = Notify.create || (() => {})

// Create a quasar instance at the top level of the script setup
const $q = useQuasar()

// Event listener cleanup
let eventListenerCleanup: (() => void) | undefined

// Search state
const searchQuery = ref('')
const isLoadingSearch = ref(false)
const hasSearched = ref(false)
const searchResults = ref<any[]>([])

// Featured mods state
const featuredMods = ref<any[]>([])
const isLoadingFeatured = ref(false)

// Latest mods and modpacks state
const latestMods = ref<any[]>([])
const psychModpacks = ref<any[]>([])
const vsliceModpacks = ref<any[]>([])
const codenameModpacks = ref<any[]>([])
const isLoadingLatest = ref(false)
const isLoadingPsychModpacks = ref(false)
const isLoadingVsliceModpacks = ref(false)
const isLoadingCodenameModpacks = ref(false)

// Pagination state
const currentPage = ref(1)
const totalPages = ref(1)
const itemsPerPage = 20

// View state
const activeView = ref('all') // 'all', 'search'
const selectedModType = ref('executables') // For tabs

// For file selection
const showFileSelector = ref(false)
const downloadFiles = ref<any[]>([])
const alternateFileSources = ref<any[]>([])
const currentDownloadMod = ref<any | null>(null)
let pendingDownloadNotification: any = null

// For modpack handling
const showEngineSelectDialog = ref(false)
const currentModpackInfo = ref<ModpackInfo | null>(null)
const selectedEngineMod = ref<any>(null)

// For custom URL download
const showCustomUrlModal = ref(false)
const showModTypeModal = ref(false)
const customModData = ref<any>(null)

// For folder existence confirmation
const showFolderExistsDialog = ref(false)
const folderExistsModName = ref('')

// For mod details modal
const selectedModId = ref<number>(0)
const currentModelType = ref<string>('')
const isModDetailsModalOpen = ref<boolean>(false)

// Variables for handling folder existence check
const folderExistsDownloadContinueFunction = ref<(() => Promise<any>) | null>(
  null
)
const folderExistsUpdateFunction = ref<(() => Promise<any>) | null>(null)

watch(selectedModType, async newType => {
  console.log('Tab changed to:', newType)
  currentPage.value = 1 // Reset to first page when changing tabs

  if (newType === 'psychModpacks') {
    if (psychModpacks.value.length === 0) {
      await fetchPsychModpacks()
    } else {
      // If data is already loaded, just update the totalPages
      const response = await invoke<{ mods: any[]; total: number }>(
        'fetch_gamebanana_mods_command',
        {
          query: '_psychmodpack',
          page: 1,
        }
      )
      totalPages.value = Math.ceil(response.total / itemsPerPage)
    }
  } else if (newType === 'vsliceModpacks') {
    if (vsliceModpacks.value.length === 0) {
      await fetchVsliceModpacks()
    } else {
      // If data is already loaded, just update the totalPages
      const response = await invoke<{ mods: any[]; total: number }>(
        'fetch_gamebanana_mods_command',
        {
          query: '_vslicemodpack',
          page: 1,
        }
      )
      totalPages.value = Math.ceil(response.total / itemsPerPage)
    }
  } else if (newType === 'codenameModpacks') {
    if (codenameModpacks.value.length === 0) {
      await fetchCodenameModpacks()
    } else {
      // If data is already loaded, just update the totalPages
      const response = await invoke<{ mods: any[]; total: number }>(
        'fetch_gamebanana_mods_command',
        {
          query: '_codenamemodpack',
          page: 1,
        }
      )
      totalPages.value = Math.ceil(response.total / itemsPerPage)
    }
  } else if (newType === 'executables') {
    if (latestMods.value.length === 0) {
      await fetchLatestMods()
    } else {
      // If data is already loaded, just update the totalPages
      const response = await invoke<{ mods: any[]; total: number }>(
        'fetch_gamebanana_mods_command',
        {
          query: 'latest',
          page: 1,
        }
      )
      totalPages.value = Math.ceil(response.total / itemsPerPage)
    }
  }
})

onMounted(() => {
  // Load initial data
  fetchFeaturedMods()
  fetchLatestMods()

  // Also load the first tab of mod packs data if needed
  if (selectedModType.value === 'psychModpacks') {
    fetchPsychModpacks()
  } else if (selectedModType.value === 'vsliceModpacks') {
    fetchVsliceModpacks()
  } else if (selectedModType.value === 'codenameModpacks') {
    fetchCodenameModpacks()
  }

  // Set up event listeners using the centralized function
  eventListenerCleanup = setupGameBananaEventListeners()

  // Handle initial mod details if provided
  if (props.openModDetailsOnMount && props.openModDetailsOnMount.open) {
    openModDetails(
      props.openModDetailsOnMount.modId,
      props.openModDetailsOnMount.modelType
    )
    emit('resetModDetailsOnMount')
  }
})

// Clean up event listeners when component is unmounted
onBeforeUnmount(() => {
  if (eventListenerCleanup) {
    eventListenerCleanup()
  }
})

// Data fetching functions
const fetchFeaturedMods = async () => {
  isLoadingFeatured.value = true
  try {
    const response = await invoke<{ mods: any[]; total: number }>(
      'fetch_gamebanana_mods_command',
      {
        query: 'featured',
        page: 1, // Always get first page for featured
      }
    )

    featuredMods.value = response.mods
  } catch (error) {
    console.error('Failed to fetch featured mods:', error)
  } finally {
    isLoadingFeatured.value = false
  }
}

const fetchLatestMods = async () => {
  isLoadingLatest.value = true
  try {
    const response = await invoke<{ mods: any[]; total: number }>(
      'fetch_gamebanana_mods_command',
      {
        query: 'latest',
        page: currentPage.value,
      }
    )

    latestMods.value = response.mods
    totalPages.value = Math.ceil(response.total / itemsPerPage)
  } catch (error) {
    console.error('Failed to fetch latest mods:', error)
  } finally {
    isLoadingLatest.value = false
  }
}

const fetchPsychModpacks = async () => {
  isLoadingPsychModpacks.value = true
  try {
    const response = await invoke<{ mods: any[]; total: number }>(
      'fetch_gamebanana_mods_command',
      {
        query: '_psychmodpack',
        page: currentPage.value,
      }
    )

    psychModpacks.value = response.mods
    if (selectedModType.value === 'psychModpacks') {
      totalPages.value = Math.ceil(response.total / itemsPerPage)
    }
  } catch (error) {
    console.error('Failed to fetch Psych Engine modpacks:', error)
  } finally {
    isLoadingPsychModpacks.value = false
  }
}

const fetchVsliceModpacks = async () => {
  isLoadingVsliceModpacks.value = true
  try {
    const response = await invoke<{ mods: any[]; total: number }>(
      'fetch_gamebanana_mods_command',
      {
        query: '_vslicemodpack',
        page: currentPage.value,
      }
    )

    vsliceModpacks.value = response.mods
    if (selectedModType.value === 'vsliceModpacks') {
      totalPages.value = Math.ceil(response.total / itemsPerPage)
    }
  } catch (error) {
    console.error('Failed to fetch V-Slice modpacks:', error)
  } finally {
    isLoadingVsliceModpacks.value = false
  }
}

const fetchCodenameModpacks = async () => {
  isLoadingCodenameModpacks.value = true
  try {
    const response = await invoke<{ mods: any[]; total: number }>(
      'fetch_gamebanana_mods_command',
      {
        query: '_codenamemodpack',
        page: currentPage.value,
      }
    )

    codenameModpacks.value = response.mods
    if (selectedModType.value === 'codenameModpacks') {
      totalPages.value = Math.ceil(response.total / itemsPerPage)
    }
  } catch (error) {
    console.error('Failed to fetch Codename Engine modpacks:', error)
  } finally {
    isLoadingCodenameModpacks.value = false
  }
}

// Search functions
const searchMods = async () => {
  if (!searchQuery.value.trim()) {
    activeView.value = 'all'
    return
  }

  activeView.value = 'search'
  isLoadingSearch.value = true
  currentPage.value = 1

  try {
    const response = await invoke<{ mods: any[]; total: number }>(
      'fetch_gamebanana_mods_command',
      {
        query: searchQuery.value,
        page: currentPage.value,
      }
    )

    searchResults.value = response.mods
    totalPages.value = Math.ceil(response.total / itemsPerPage)
    hasSearched.value = true
  } catch (error) {
    console.error('Failed to search mods:', error)
  } finally {
    isLoadingSearch.value = false
  }
}

const clearSearch = () => {
  searchQuery.value = ''
  activeView.value = 'all'
  hasSearched.value = false
}

// Function to open mod details modal
const openModDetails = (
  modId: string | number | true,
  modelType: string | number | true
) => {
  console.log('Opening mod details for ID:', modId)
  // Convert modId to number to ensure type compatibility
  selectedModId.value = modId ? Number(modId) : 0
  currentModelType.value = modelType ? String(modelType) : 'Mod'
  console.log(
    'Selected mod ID:',
    selectedModId.value,
    'Model Type:',
    currentModelType.value
  )
  isModDetailsModalOpen.value = true
  console.log('Mod details modal opened')
}

// Pagination handling
const changePage = async (page: number) => {
  currentPage.value = page

  if (activeView.value === 'search') {
    isLoadingSearch.value = true
    try {
      const response = await invoke<{ mods: any[]; total: number }>(
        'fetch_gamebanana_mods_command',
        {
          query: searchQuery.value,
          page: page,
        }
      )

      searchResults.value = response.mods
      // Make sure to update totalPages from the response
      totalPages.value = Math.ceil(response.total / itemsPerPage)
    } catch (error) {
      console.error('Failed to fetch search page:', error)
    } finally {
      isLoadingSearch.value = false
    }
  } else {
    // Handle pagination based on the current active tab
    if (selectedModType.value === 'executables') {
      isLoadingLatest.value = true
      try {
        const response = await invoke<{ mods: any[]; total: number }>(
          'fetch_gamebanana_mods_command',
          {
            query: 'latest',
            page: page,
          }
        )

        latestMods.value = response.mods
        // Make sure to update totalPages from the response
        totalPages.value = Math.ceil(response.total / itemsPerPage)
      } catch (error) {
        console.error('Failed to fetch latest page:', error)
      } finally {
        isLoadingLatest.value = false
      }
    } else if (selectedModType.value === 'psychModpacks') {
      isLoadingPsychModpacks.value = true
      try {
        const response = await invoke<{ mods: any[]; total: number }>(
          'fetch_gamebanana_mods_command',
          {
            query: '_psychmodpack',
            page: page,
          }
        )

        psychModpacks.value = response.mods
        // Make sure to update totalPages from the response
        totalPages.value = Math.ceil(response.total / itemsPerPage)
      } catch (error) {
        console.error('Failed to fetch Psych Engine modpacks page:', error)
      } finally {
        isLoadingPsychModpacks.value = false
      }
    } else if (selectedModType.value === 'vsliceModpacks') {
      isLoadingVsliceModpacks.value = true
      try {
        const response = await invoke<{ mods: any[]; total: number }>(
          'fetch_gamebanana_mods_command',
          {
            query: '_vslicemodpack',
            page: page,
          }
        )

        vsliceModpacks.value = response.mods
        // Make sure to update totalPages from the response
        totalPages.value = Math.ceil(response.total / itemsPerPage)
      } catch (error) {
        console.error('Failed to fetch V-Slice modpacks page:', error)
      } finally {
        isLoadingVsliceModpacks.value = false
      }
    } else if (selectedModType.value === 'codenameModpacks') {
      isLoadingCodenameModpacks.value = true
      try {
        const response = await invoke<{ mods: any[]; total: number }>(
          'fetch_gamebanana_mods_command',
          {
            query: '_codenamemodpack',
            page: page,
          }
        )

        codenameModpacks.value = response.mods
        // Make sure to update totalPages from the response
        totalPages.value = Math.ceil(response.total / itemsPerPage)
      } catch (error) {
        console.error('Failed to fetch Codename Engine modpacks page:', error)
      } finally {
        isLoadingCodenameModpacks.value = false
      }
    }
  }
}

// Download handling
const downloadMod = async (mod: any) => {
  try {
    currentDownloadMod.value = mod

    const result = await gamebananaService.downloadMod(mod)

    // Handle different scenarios based on the result
    if ('showFileSelector' in result) {
      // If we need to show file selector
      downloadFiles.value = result.files
      alternateFileSources.value = result.alternateFileSources || []
      showFileSelector.value = true
      return
    }

    if ('showEngineSelectDialog' in result) {
      // If we need to show engine selection dialog
      currentModpackInfo.value = result.modpackInfo
      showEngineSelectDialog.value = true
      return
    }
    if ('showModTypeModal' in result) {
      // If we need to show mod type selection
      customModData.value = result.customModData
      showModTypeModal.value = true
      return
    }

    if ('showFolderExistsDialog' in result) {
      // If the mod folder already exists, show confirmation dialog
      folderExistsModName.value = result.modName
      folderExistsDownloadContinueFunction.value = result.continueDownload
      folderExistsUpdateFunction.value = result.updateMod || null
      showFolderExistsDialog.value = true
      return
    }
  } catch (error) {
    // Show error notification
    $q.notify({
      type: 'negative',
      message: `Failed to prepare download for "${mod.name}"`,
      caption: String(error),
      position: 'bottom-right',
      timeout: 5000,
    })

    console.error('Failed to prepare mod download:', error)
  }
}

// Function called when an engine is selected from the dialog
const onEngineSelected = async (engine: any) => {
  if (!currentModpackInfo.value) return

  try {
    // Hide the dialog immediately to provide feedback to the user
    showEngineSelectDialog.value = false

    // Use the centralized gamebananaService to handle modpack download
    const result = await gamebananaService.downloadModpackForEngine(
      currentModpackInfo.value,
      engine
    )

    if (!result.success) {
      $q.notify({
        type: 'negative',
        message: `Failed to install modpack`,
        caption: String(result.error),
        position: 'bottom-right',
        timeout: 5000,
      })
    }
  } catch (error) {
    $q.notify({
      type: 'negative',
      message: `Failed to install modpack`,
      caption: String(error),
      position: 'bottom-right',
      timeout: 5000,
    })
  } finally {
    // Reset state
    currentModpackInfo.value = null
    selectedEngineMod.value = null
  }
}

// Function called when a file is selected from the dialog
const onFileSelected = async (selectedFile: any) => {
  if (!currentDownloadMod.value) return

  try {
    // Hide the file selector dialog immediately to provide user feedback
    showFileSelector.value = false

    // Use the centralized gamebananaService to handle file download with folder existence check
    const result = await gamebananaService.downloadModFile(
      currentDownloadMod.value,
      selectedFile
    )

    // Handle different scenarios based on the result
    if ('showFileSelector' in result) {
      // If we need to show file selector again (shouldn't happen in this context)
      downloadFiles.value = result.files
      alternateFileSources.value = result.alternateFileSources || []
      showFileSelector.value = true
      return
    }

    if ('showEngineSelectDialog' in result) {
      // If we need to show engine selection dialog
      currentModpackInfo.value = result.modpackInfo
      showEngineSelectDialog.value = true
      return
    }

    if ('showModTypeModal' in result) {
      // If we need to show mod type selection
      customModData.value = result.customModData
      showModTypeModal.value = true
      return
    }
    if ('showFolderExistsDialog' in result) {
      // If the mod folder already exists, show confirmation dialog
      folderExistsModName.value = result.modName
      folderExistsDownloadContinueFunction.value = result.continueDownload
      folderExistsUpdateFunction.value = result.updateMod || null
      showFolderExistsDialog.value = true
      return
    }

    if ('success' in result && !result.success) {
      $q.notify({
        type: 'negative',
        message: `Failed to download "${currentDownloadMod.value.name}"`,
        caption: String('error' in result ? result.error : 'Unknown error'),
        position: 'bottom-right',
        timeout: 5000,
      })
    }

    // Trigger the refresh event to update the mod list
    const refreshEvent = new CustomEvent('refresh-mods')
    window.dispatchEvent(refreshEvent)

    // Reset current download mod
    currentDownloadMod.value = null
  } catch (error) {
    $q.notify({
      type: 'negative',
      message: `Failed to download mod`,
      caption: String(error),
      position: 'bottom-right',
      timeout: 5000,
    })

    console.error('Failed to download mod file:', error)
    currentDownloadMod.value = null
  }
}

// Function to cancel the download
const cancelDownload = () => {
  // Dismiss any pending notification
  if (pendingDownloadNotification) {
    pendingDownloadNotification()
    pendingDownloadNotification = null
  }

  // Show cancellation notification
  if (currentDownloadMod.value) {
    $q.notify({
      type: 'info',
      message: `Download of "${currentDownloadMod.value.name}" cancelled`,
      position: 'bottom-right',
      timeout: 3000,
    })
  }

  // Reset current download mod
  currentDownloadMod.value = null
}

// Save a mod to the database
const saveModToDatabase = async (mod: any) => {
  try {
    // Check if DatabaseService is initialized
    if (!window.db || !window.db.service) {
      console.warn('Database service not initialized yet, cannot save mod')
      return false
    }

    console.log('Saving mod to database using DatabaseService:', mod)

    // Make sure the mod has an engine field required by the type
    if (!mod.engine) {
      mod.engine = {
        engine_type: 'unknown',
        engine_name: '',
        engine_icon: '',
        mods_folder: false,
        mods_folder_path: '',
      }
    }

    // Use the DatabaseService to save the mod
    await window.db.service.saveMod(mod)

    console.log('Mod saved successfully to database:', mod.name)
    return true
  } catch (error) {
    console.error('Failed to save mod to database:', error)
    return false
  }
}

const formatEngineType = (engineType: string | null): string => {
  if (!engineType) return 'Unknown'

  switch (engineType.toLowerCase()) {
    case 'psych':
      return 'Psych Engine'
    case 'vanilla':
      return 'V-Slice'
    case 'codename':
      return 'Codename Engine'
    default:
      return engineType.charAt(0).toUpperCase() + engineType.slice(1)
  }
}

// Function to get the mods folder path for an engine mod
const getModsFolderPath = (engineMod: any): string => {
  // Get base directory first in all cases
  const basePath = engineMod.path
  const executablePath = engineMod.executable_path || ''

  if (!basePath) return 'Unknown path'

  // Get parent directory of executable if it exists
  let baseDir = basePath
  if (executablePath) {
    // Extract the directory from the executable path
    const lastSlashIndex = executablePath.lastIndexOf('/')
    if (lastSlashIndex > 0) {
      baseDir = executablePath.substring(0, lastSlashIndex)
    } else {
      const lastBackslashIndex = executablePath.lastIndexOf('\\')
      if (lastBackslashIndex > 0) {
        baseDir = executablePath.substring(0, lastBackslashIndex)
      }
    }
  }

  // Then check if the engine has a specified custom mods folder path
  if (
    engineMod.engine &&
    engineMod.engine.mods_folder &&
    engineMod.engine.mods_folder_path
  ) {
    // Combine the base directory with the custom mods folder path
    return `${baseDir}/${engineMod.engine.mods_folder_path}`
  }

  // If no custom path specified, use default mods folder
  return `${baseDir}/mods`
}

// Add new method to handle direct engine downloads
const downloadEngine = async (engineType: string) => {
  try {
    // Use the centralized gamebananaService to download the engine
    const result = await gamebananaService.downloadEngine(engineType) // Check if we need to show the folder exists dialog
    if ('showFolderExistsDialog' in result) {
      // If the engine folder already exists, show confirmation dialog
      folderExistsModName.value = result.modName
      folderExistsDownloadContinueFunction.value = result.continueDownload
      folderExistsUpdateFunction.value = result.updateMod || null
      showFolderExistsDialog.value = true
      return
    }

    // Otherwise, the download was successful or failed without folder conflict
    if ('success' in result && !result.success) {
      $q.notify({
        type: 'negative',
        message: `Failed to download ${formatEngineType(engineType)}`,
        caption: result.error || 'Unknown error',
        position: 'bottom-right',
        timeout: 5000,
      })
    }
  } catch (error) {
    // Show error notification
    $q.notify({
      type: 'negative',
      message: `Failed to download ${formatEngineType(engineType)}`,
      caption: String(error),
      position: 'bottom-right',
      timeout: 5000,
    })

    console.error(`Failed to download ${engineType} engine:`, error)
  }
}

// Get the install location from settings
const getInstallLocation = async (): Promise<string | null> => {
  try {
    const storeService = StoreService.getInstance()
    return await storeService.getSetting('installLocation')
  } catch (error) {
    console.warn('Could not get install location from settings:', error)
    return null
  }
}

// Custom URL download flow
const onCustomUrlSubmit = (formData: any) => {
  console.log('Custom URL form submitted:', formData)
  // Add isCustomUrl flag to identify the source of this mod data
  customModData.value = {
    ...formData,
    isCustomUrl: true,
  }
  showCustomUrlModal.value = false
  showModTypeModal.value = true
}

const onModTypeSubmit = async (typeData: any) => {
  console.log('Mod type selected:', typeData)
  if (!customModData.value) return

  try {
    // Show loading notification
    pendingDownloadNotification = $q.notify({
      type: 'ongoing',
      message: `Preparing to download "${customModData.value.name}"...`,
      position: 'bottom-right',
      timeout: 0,
    })

    // Determine install location based on mod type
    let installLocation: string | null = null

    if (typeData.modType === 'executable') {
      // For standalone mods, use the standard install location
      installLocation = await getInstallLocation()
    } else {
      // For modpacks, use the engine's mods folder
      if (typeData.engineMod) {
        installLocation = getModsFolderPath(typeData.engineMod)
      } else {
        throw new Error(
          `No ${formatEngineType(typeData.modType)} installation found`
        )
      }
    }

    console.log(
      `Downloading ${customModData.value.name} to ${
        installLocation || 'default location'
      }`
    )

    // Generate a random modId for tracking the download
    const modId = Math.floor(Math.random() * 1000000)

    // Call backend to download using the custom mod command instead
    const result = await invoke<string>('download_custom_mod_command', {
      url: customModData.value.url,
      name: customModData.value.name,
      modId,
      installLocation,
      thumbnailUrl: customModData.value.bannerData,
      description: customModData.value.description,
      version: customModData.value.version,
    })

    // Process the result
    let modInfo: any
    let modPath: string

    try {
      // Try to parse as JSON
      const parsed = JSON.parse(result)
      modPath = parsed.path
      modInfo = parsed.mod_info
    } catch {
      // If parsing fails, assume it's just the path string
      modPath = result
      // Get mod info directly from the backend
      const allMods = await invoke<any[]>('get_mods')
      modInfo = allMods.find(m => m.path === modPath)
    }

    // If we still don't have mod info, create one with custom data
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
        engine_type:
          typeData.modType !== 'executable' ? typeData.modType : null,
        engine:
          typeData.modType !== 'executable'
            ? {
                engine_type: typeData.modType,
                engine_name: formatEngineType(typeData.modType),
                mods_folder: true,
                mods_folder_path: 'mods',
              }
            : null,
      }
    } else {
      // Update the mod info with custom data
      modInfo.name = customModData.value.name
      modInfo.banner_data =
        customModData.value.bannerData || modInfo.banner_data
      modInfo.logo_data = customModData.value.logoData || modInfo.logo_data
      modInfo.description =
        customModData.value.description || modInfo.description
      modInfo.version = customModData.value.version || modInfo.version

      if (typeData.modType !== 'executable') {
        modInfo.engine_type = typeData.modType
        modInfo.engine = {
          ...(modInfo.engine || {}),
          engine_type: typeData.modType,
          engine_name: formatEngineType(typeData.modType),
          mods_folder: true,
          mods_folder_path: 'mods',
        }
      }
    }

    // Save the mod to the database
    if (modInfo) {
      await saveModToDatabase(modInfo)
    }

    // Dismiss loading notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification()
      pendingDownloadNotification = null
    }

    // Show success notification
    $q.notify({
      type: 'positive',
      message: `"${customModData.value.name}" downloaded and installed successfully!`,
      caption: `Ready to play from the mods list`,
      position: 'bottom-right',
      timeout: 5000,
    })

    // Trigger the refresh event to update the mod list
    const refreshEvent = new CustomEvent('refresh-mods')
    window.dispatchEvent(refreshEvent)

    // Reset state
    showModTypeModal.value = false
    customModData.value = null
  } catch (error) {
    // Show error notification
    $q.notify({
      type: 'negative',
      message: `Failed to download "${customModData.value.name}"`,
      caption: String(error),
      position: 'bottom-right',
      timeout: 5000,
    })

    // Dismiss any pending notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification()
      pendingDownloadNotification = null
    }

    console.error('Failed to download custom mod:', error)

    // Reset state but keep the modal open to allow for corrections
    showModTypeModal.value = false
    showCustomUrlModal.value = true
  }
}

// Handle cancel from mod type selection modal
const handleModTypeCancel = () => {
  // Check the source of the mod type modal - custom URL or regular download
  if (customModData.value?.isCustomUrl) {
    // If from custom URL, go back to custom URL modal
    showModTypeModal.value = false
    showCustomUrlModal.value = true
  } else {
    // If from regular mod download, just close the modal and clean up
    showModTypeModal.value = false
    customModData.value = null

    // Show cancellation notification
    $q.notify({
      type: 'info',
      message: 'Download cancelled',
      position: 'bottom-right',
      timeout: 3000,
    })
  }
}

// Function to continue download when folder exists
const continueFolderExistsDownload = async () => {
  showFolderExistsDialog.value = false

  try {
    // Show downloading notification
    pendingDownloadNotification = $q.notify({
      type: 'ongoing',
      message: `Downloading "${folderExistsModName.value}"...`,
      position: 'bottom-right',
      timeout: 0,
    })

    // Call the continue download function that was stored
    if (folderExistsDownloadContinueFunction.value) {
      const result = await folderExistsDownloadContinueFunction.value()

      // Handle the result based on its type
      if ('showFileSelector' in result) {
        // If we need to show file selector
        downloadFiles.value = result.files
        alternateFileSources.value = result.alternateFileSources || []
        showFileSelector.value = true
      } else if ('showEngineSelectDialog' in result) {
        // If we need to show engine selection dialog
        currentModpackInfo.value = result.modpackInfo
        showEngineSelectDialog.value = true
      } else if ('showModTypeModal' in result) {
        // If we need to show mod type selection
        customModData.value = result.customModData
        showModTypeModal.value = true
      } else if ('success' in result) {
        // If it's a direct download result
        if (pendingDownloadNotification) {
          pendingDownloadNotification()
          pendingDownloadNotification = null
        }

        if (result.success) {
          // Show success notification
          $q.notify({
            type: 'positive',
            message: `"${folderExistsModName.value}" downloaded and installed successfully!`,
            caption: `Ready to play from the mods list`,
            position: 'bottom-right',
            timeout: 5000,
          })

          // Trigger the refresh event to update the mod list
          const refreshEvent = new CustomEvent('refresh-mods')
          window.dispatchEvent(refreshEvent)
        } else {
          // Show error notification
          $q.notify({
            type: 'negative',
            message: `Failed to download "${folderExistsModName.value}"`,
            caption: result.error || 'Unknown error',
            position: 'bottom-right',
            timeout: 5000,
          })
        }
      }
    }
  } catch (error) {
    // Show error notification
    $q.notify({
      type: 'negative',
      message: `Failed to download "${folderExistsModName.value}"`,
      caption: String(error),
      position: 'bottom-right',
      timeout: 5000,
    })

    // Dismiss any pending notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification()
      pendingDownloadNotification = null
    }
    console.error('Failed to download mod:', error)
  } finally {
    // Reset state
    folderExistsModName.value = ''
    folderExistsDownloadContinueFunction.value = null
    folderExistsUpdateFunction.value = null
  }
}

// Function to update mod when folder exists
const updateFolderExistsMod = async () => {
  showFolderExistsDialog.value = false

  try {
    // Show updating notification
    pendingDownloadNotification = $q.notify({
      type: 'ongoing',
      message: `Updating "${folderExistsModName.value}"...`,
      position: 'bottom-right',
      timeout: 0,
    })

    // Call the update function that was stored
    if (folderExistsUpdateFunction.value) {
      const result = await folderExistsUpdateFunction.value()

      // Handle the result based on its type
      if ('success' in result) {
        // If it's a direct update result
        if (pendingDownloadNotification) {
          pendingDownloadNotification()
          pendingDownloadNotification = null
        }

        if (result.success) {
          // Show success notification
          $q.notify({
            type: 'positive',
            message: `"${folderExistsModName.value}" updated successfully!`,
            caption: `Ready to play from the mods list`,
            position: 'bottom-right',
            timeout: 5000,
          })

          // Trigger the refresh event to update the mod list
          const refreshEvent = new CustomEvent('refresh-mods')
          window.dispatchEvent(refreshEvent)
        } else {
          // Show error notification
          $q.notify({
            type: 'negative',
            message: `Failed to update "${folderExistsModName.value}"`,
            caption: result.error || 'Unknown error',
            position: 'bottom-right',
            timeout: 5000,
          })
        }
      }
    } else {
      // No update function available
      $q.notify({
        type: 'warning',
        message: 'Update not available',
        caption: 'This mod cannot be updated',
        position: 'bottom-right',
        timeout: 3000,
      })
    }
  } catch (error) {
    // Show error notification
    $q.notify({
      type: 'negative',
      message: `Failed to update "${folderExistsModName.value}"`,
      caption: String(error),
      position: 'bottom-right',
      timeout: 5000,
    })

    // Dismiss any pending notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification()
      pendingDownloadNotification = null
    }

    console.error('Failed to update mod:', error)
  } finally {
    // Reset state
    folderExistsModName.value = ''
    folderExistsDownloadContinueFunction.value = null
    folderExistsUpdateFunction.value = null
  }
}
</script>

<style scoped>
.gamebanana-browser {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.scroll-container {
  flex-grow: 1;
  display: block;
}

.main-browser {
  width: 100%;
}

.mods-section {
  margin-bottom: 24px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  width: 100%;
}

.featured-header {
  background: linear-gradient(to bottom, var(--theme-card), transparent);
  padding: 16px;
  border-radius: 0.5rem;
  h6 {
    margin: 0;
  }
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
  background-color: transparent;
  border-radius: 0 0 1rem 1rem;
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
:deep(.q-tab) {
  border-radius: 8px 8px 0 0;
}
:deep(.q-tabs__content) {
  gap: 8px;
}
</style>
