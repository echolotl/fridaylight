<template>
  <div class="gamebanana-browser phantom-font">
    <SearchBar
      :search-query="searchQuery"
      @update:search-query="searchQuery = $event"
      @search="searchMods"
      @clear="clearSearch"
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
      <div class="main-browser absolute fit">
        <div class="featured-header">
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
          <h6 class="phantom-font-difficulty q-mb-md q-mt-md">
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
    <!-- Folder Exists Confirmation Dialog -->
    <FolderExistsDialog
      v-model="showFolderExistsDialog"
      :mod-name="folderExistsModName"
      @update="updateFolderExistsMod"
      @download-anyway="continueFolderExistsDownload"
      @cancel="cancelDownload"
    />
    <!-- Mod Type Selection Modal -->
    <ModTypeSelectionModal
      v-model="showModTypeSelectionModal"
      :mod-name="currentDownloadMod?.name"
      @submit="onModTypeSelected"
      @back="onModTypeBack"
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
import FolderExistsDialog from '@modals/FolderExistsDialog.vue'
import ModTypeSelectionModal from '@modals/ModTypeSelectionModal.vue'
import ModDetailsModal from '@modals/ModDetailsModal.vue'
import { NotificationService } from '@services/notificationService'
import { formatEngineName } from '@utils/index'
import {
  type GBSubfeed,
  type GBSubfeedRecord,
  type GBTopSubs,
} from '@custom-types/gamebanana'

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

const notificationService = NotificationService.getInstance()

// Event listener cleanup
let eventListenerCleanup: (() => void) | undefined

// Search state
const searchQuery = ref('')
const isLoadingSearch = ref(false)
const hasSearched = ref(false)
const searchResults = ref<GBSubfeedRecord[]>([])

// Featured mods state
const featuredMods = ref<GBTopSubs>([])
const isLoadingFeatured = ref(false)

// Latest mods and modpacks state
const latestMods = ref<GBSubfeedRecord[]>([])
const psychModpacks = ref<GBSubfeedRecord[]>([])
const vsliceModpacks = ref<GBSubfeedRecord[]>([])
const codenameModpacks = ref<GBSubfeedRecord[]>([])
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
// Store the selected file for future use
const selectedFile = ref<any | null>(null)

// For modpack handling
const showEngineSelectDialog = ref(false)
const currentModpackInfo = ref<ModpackInfo | null>(null)
const selectedEngineMod = ref<any>(null)

// For folder existence confirmation
const showFolderExistsDialog = ref(false)
const folderExistsModName = ref('')

// For mod details modal
const selectedModId = ref<number>(0)
const currentModelType = ref<string>('')
const isModDetailsModalOpen = ref<boolean>(false)

const showModTypeSelectionModal = ref(false)

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
      const response = await invoke<GBSubfeed>(
        'fetch_gamebanana_mods_command',
        {
          query: '_psychmodpack',
          page: 1,
        }
      )
      totalPages.value = Math.ceil(
        response._aMetadata._nRecordCount / response._aMetadata._nPerpage
      )
    }
  } else if (newType === 'vsliceModpacks') {
    if (vsliceModpacks.value.length === 0) {
      await fetchVsliceModpacks()
    } else {
      // If data is already loaded, just update the totalPages
      const response = await invoke<GBSubfeed>(
        'fetch_gamebanana_mods_command',
        {
          query: '_vslicemodpack',
          page: 1,
        }
      )
      totalPages.value = Math.ceil(
        response._aMetadata._nRecordCount / response._aMetadata._nPerpage
      )
    }
  } else if (newType === 'codenameModpacks') {
    if (codenameModpacks.value.length === 0) {
      await fetchCodenameModpacks()
    } else {
      // If data is already loaded, just update the totalPages
      const response = await invoke<GBSubfeed>(
        'fetch_gamebanana_mods_command',
        {
          query: '_codenamemodpack',
          page: 1,
        }
      )
      totalPages.value = Math.ceil(
        response._aMetadata._nRecordCount / response._aMetadata._nPerpage
      )
    }
  } else if (newType === 'executables') {
    if (latestMods.value.length === 0) {
      await fetchLatestMods()
    } else {
      // If data is already loaded, just update the totalPages
      const response = await invoke<GBSubfeed>(
        'fetch_gamebanana_mods_command',
        {
          query: 'latest',
          page: 1,
        }
      )
      totalPages.value = Math.ceil(
        response._aMetadata._nRecordCount / response._aMetadata._nPerpage
      )
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
  }
  emit('resetModDetailsOnMount')
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
    const response = await invoke<GBTopSubs>('get_featured_mods_command')

    featuredMods.value = response
  } catch (error) {
    console.error('Failed to fetch featured mods:', error)
  } finally {
    isLoadingFeatured.value = false
  }
}

const fetchLatestMods = async () => {
  isLoadingLatest.value = true
  try {
    const response = await invoke<GBSubfeed>('fetch_gamebanana_mods_command', {
      query: 'latest',
      page: currentPage.value,
    })
    latestMods.value = response._aRecords as GBSubfeedRecord[]
    totalPages.value = Math.ceil(
      response._aMetadata._nRecordCount / response._aMetadata._nPerpage
    )
  } catch (error) {
    console.error('Failed to fetch latest mods:', error)
  } finally {
    isLoadingLatest.value = false
  }
}

const fetchPsychModpacks = async () => {
  isLoadingPsychModpacks.value = true
  try {
    const response = await invoke<GBSubfeed>('fetch_gamebanana_mods_command', {
      query: '_psychmodpack',
      page: currentPage.value,
    })

    psychModpacks.value = response._aRecords as GBSubfeedRecord[]
    if (selectedModType.value === 'psychModpacks') {
      totalPages.value = Math.ceil(
        response._aMetadata._nRecordCount / response._aMetadata._nPerpage
      )
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
    const response = await invoke<GBSubfeed>('fetch_gamebanana_mods_command', {
      query: '_vslicemodpack',
      page: currentPage.value,
    })

    vsliceModpacks.value = response._aRecords as GBSubfeedRecord[]
    if (selectedModType.value === 'vsliceModpacks') {
      totalPages.value = Math.ceil(
        response._aMetadata._nRecordCount / response._aMetadata._nPerpage
      )
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
    const response = await invoke<GBSubfeed>('fetch_gamebanana_mods_command', {
      query: '_codenamemodpack',
      page: currentPage.value,
    })

    codenameModpacks.value = response._aRecords as GBSubfeedRecord[]
    if (selectedModType.value === 'codenameModpacks') {
      totalPages.value = Math.ceil(
        response._aMetadata._nRecordCount / response._aMetadata._nPerpage
      )
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
    const response = await invoke<GBSubfeed>('fetch_gamebanana_mods_command', {
      query: searchQuery.value,
      page: currentPage.value,
    })

    searchResults.value = response._aRecords as GBSubfeedRecord[]
    totalPages.value = Math.ceil(
      response._aMetadata._nRecordCount / response._aMetadata._nPerpage
    )
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
  // Convert modId to number to ensure type compatibility
  selectedModId.value = modId ? Number(modId) : 0
  currentModelType.value = modelType ? String(modelType) : 'Mod'

  isModDetailsModalOpen.value = true
}

// Pagination handling
const changePage = async (page: number) => {
  currentPage.value = page

  if (activeView.value === 'search') {
    isLoadingSearch.value = true
    try {
      const response = await invoke<GBSubfeed>(
        'fetch_gamebanana_mods_command',
        {
          query: searchQuery.value,
          page: page,
        }
      )

      searchResults.value = response._aRecords as GBSubfeedRecord[]
      // Make sure to update totalPages from the response
      totalPages.value = Math.ceil(
        response._aMetadata._nRecordCount / response._aMetadata._nPerpage
      )
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
        const response = await invoke<{ mods: GBSubfeed; total: number }>(
          'fetch_gamebanana_mods_command',
          {
            query: 'latest',
            page: page,
          }
        )

        latestMods.value = response.mods._aRecords as GBSubfeedRecord[]
        // Make sure to update totalPages from the response
        totalPages.value = Math.ceil(
          response.mods._aMetadata._nRecordCount /
            response.mods._aMetadata._nPerpage
        )
      } catch (error) {
        console.error('Failed to fetch latest page:', error)
      } finally {
        isLoadingLatest.value = false
      }
    } else if (selectedModType.value === 'psychModpacks') {
      isLoadingPsychModpacks.value = true
      try {
        const response = await invoke<GBSubfeed>(
          'fetch_gamebanana_mods_command',
          {
            query: '_psychmodpack',
            page: page,
          }
        )

        psychModpacks.value = response._aRecords as GBSubfeedRecord[]
        // Make sure to update totalPages from the response
        totalPages.value = Math.ceil(
          response._aMetadata._nRecordCount / response._aMetadata._nPerpage
        )
      } catch (error) {
        console.error('Failed to fetch Psych Engine modpacks page:', error)
      } finally {
        isLoadingPsychModpacks.value = false
      }
    } else if (selectedModType.value === 'vsliceModpacks') {
      isLoadingVsliceModpacks.value = true
      try {
        const response = await invoke<GBSubfeed>(
          'fetch_gamebanana_mods_command',
          {
            query: '_vslicemodpack',
            page: page,
          }
        )

        vsliceModpacks.value = response._aRecords as GBSubfeedRecord[]
        // Make sure to update totalPages from the response
        totalPages.value = Math.ceil(
          response._aMetadata._nRecordCount / response._aMetadata._nPerpage
        )
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
const downloadMod = async (mod: any, mod_type?: string) => {
  try {
    currentDownloadMod.value = mod

    const result = await gamebananaService.downloadMod(mod, mod_type)

    // Handle different scenarios based on the result
    if ('showFileSelector' in result) {
      // If we need to show file selector
      downloadFiles.value = result.files
      alternateFileSources.value = result.alternateFileSources || []
      showFileSelector.value = true
      return
    }

    if ('showModTypeModal' in result) {
      // If we need to show mod type selection modal
      showModTypeSelectionModal.value = true
      return
    }

    if ('showEngineSelectDialog' in result) {
      // If we need to show engine selection dialog
      currentModpackInfo.value = result.modpackInfo
      showEngineSelectDialog.value = true
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
    notificationService.downloadError(mod.name, String(error))
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
      notificationService.installationFailed(
        currentModpackInfo.value.mod.name,
        result.error || 'Unknown error'
      )
    }
  } catch (error) {
    notificationService.installationFailed(
      currentModpackInfo.value.mod.name,
      String(error)
    )
  } finally {
    // Reset state
    currentModpackInfo.value = null
    selectedEngineMod.value = null
  }
}

// Function called when mod type is selected from the modal
const onModTypeSelected = async (selection: {
  modType: string
  engineMod: any
  isAddon?: boolean
}) => {
  if (!currentDownloadMod.value) return

  try {
    // Hide the modal immediately to provide feedback to the user
    showModTypeSelectionModal.value = false

    if (selection.modType === 'executable') {
      // For executables, just proceed with normal download
      const result = await gamebananaService.startDownload(
        currentDownloadMod.value
      )

      // Handle folder exists dialog if needed
      if ('showFolderExistsDialog' in result) {
        folderExistsModName.value = result.modName
        folderExistsDownloadContinueFunction.value = result.continueDownload
        folderExistsUpdateFunction.value = result.updateMod || null
        showFolderExistsDialog.value = true
        return
      }
    } else {
      // For modpacks, create modpack info and use engine selection
      const modpackInfo: ModpackInfo = {
        mod: currentDownloadMod.value,
        type: selection.modType,
        compatibleEngines: [selection.engineMod],
      }

      // For Codename Engine addons, we need to modify the installation path
      if (selection.modType === 'codename' && selection.isAddon) {
        // We'll handle this in the downloadModpackForEngine method
        const result = await gamebananaService.downloadModpackForEngine(
          modpackInfo,
          {
            ...selection.engineMod,
            isAddon: true,
          }
        )

        if (!result.success) {
          notificationService.installationFailed(
            currentDownloadMod.value.name,
            result.error || 'Unknown error'
          )
        }
      } else {
        // Regular modpack installation
        const result = await gamebananaService.downloadModpackForEngine(
          modpackInfo,
          selection.engineMod
        )

        if (!result.success) {
          notificationService.installationFailed(
            currentDownloadMod.value.name,
            result.error || 'Unknown error'
          )
        }
      }
    }
  } catch (error) {
    notificationService.installationFailed(
      currentDownloadMod.value.name,
      String(error)
    )
  } finally {
    // Reset state
    currentDownloadMod.value = null
  }
}

// Function called when user goes back from mod type selection modal
const onModTypeBack = () => {
  // For now, just close the modal since there's no previous step
  showModTypeSelectionModal.value = false
}

// Function called when a file is selected from the dialog
const onFileSelected = async (file: any) => {
  if (!currentDownloadMod.value) return

  try {
    // Save the selected file to the ref for future use
    selectedFile.value = file

    // Hide the file selector dialog immediately to provide user feedback
    showFileSelector.value = false // Use the centralized gamebananaService to handle file download with folder existence check
    const result = await gamebananaService.downloadModFile(
      currentDownloadMod.value,
      file
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

    if ('showFolderExistsDialog' in result) {
      // If the mod folder already exists, show confirmation dialog
      folderExistsModName.value = result.modName
      folderExistsDownloadContinueFunction.value = result.continueDownload
      folderExistsUpdateFunction.value = result.updateMod || null
      showFolderExistsDialog.value = true
      return
    }

    if ('success' in result && !result.success) {
      notificationService.downloadError(
        currentDownloadMod.value.name,
        String('error' in result ? result.error : 'Unknown error')
      )
    } // Trigger the refresh event to update the mod list
    const refreshEvent = new CustomEvent('refresh-mods')
    window.dispatchEvent(refreshEvent)

    // Reset current download mod and selected file
    currentDownloadMod.value = null
    selectedFile.value = null
  } catch (error) {
    notificationService.downloadError(
      currentDownloadMod.value.name,
      String(error)
    )

    console.error('Failed to download mod file:', error)
    currentDownloadMod.value = null
    selectedFile.value = null
  }
}

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
      notificationService.downloadError(
        await formatEngineName(engineType),
        String(result.error || 'Unknown error')
      )
    }
  } catch (error) {
    // Show error notification
    notificationService.downloadError(
      await formatEngineName(engineType),
      String(error)
    )

    console.error(`Failed to download ${engineType} engine:`, error)
  }
}

// Function to cancel the download
const cancelDownload = () => {
  // Show cancellation notification
  if (currentDownloadMod.value) {
    notificationService.downloadCancelled(currentDownloadMod.value.name)
  }

  // Reset current download mod and selected file
  currentDownloadMod.value = null
  selectedFile.value = null
}

// Function to continue download when folder exists
const continueFolderExistsDownload = async () => {
  showFolderExistsDialog.value = false

  try {
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
      } else if ('success' in result) {
        if (result.success) {
          // Show success notification
          notificationService.downloadSuccess(
            folderExistsModName.value,
            'Ready to play from the mods list'
          )

          // Trigger the refresh event to update the mod list
          const refreshEvent = new CustomEvent('refresh-mods')
          window.dispatchEvent(refreshEvent)
        } else {
          // Show error notification
          notificationService.downloadError(
            folderExistsModName.value,
            result.error || 'Unknown error'
          )
        }
      }
    }
  } catch (error) {
    // Show error notification
    notificationService.downloadError(folderExistsModName.value, String(error))

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
    // Call the update function that was stored
    if (folderExistsUpdateFunction.value) {
      const result = await folderExistsUpdateFunction.value()

      // Handle the result based on its type
      if ('success' in result) {
        if (result.success) {
          // Show success notification
          notificationService.updateSuccess(
            folderExistsModName.value,
            'Ready to play from the mods list'
          )

          // Trigger the refresh event to update the mod list
          const refreshEvent = new CustomEvent('refresh-mods')
          window.dispatchEvent(refreshEvent)
        } else {
          // Show error notification
          notificationService.updateError(
            folderExistsModName.value,
            result.error || 'Unknown error'
          )
        }
      }
    } else {
      // No update function available
      notificationService.updateNotAvailable(folderExistsModName.value)
      return
    }
  } catch (error) {
    // Show error notification
    notificationService.updateError(folderExistsModName.value, String(error))

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

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  width: 100%;
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
