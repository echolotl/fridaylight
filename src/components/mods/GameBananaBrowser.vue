<template>
  <div class="gamebanana-browser phantom-font">
    <SearchBar
      :search-query="searchQuery"
      @update:search-query="searchQuery = $event"
      @search="searchMods"
      @clear="clearSearch"
    />

    <!-- Search Results View -->
    <div v-if="activeView === 'search'" class="phantom-font">
      <h6 class="phantom-font-difficulty q-mb-md q-mt-md">
        <div class="flex justify-between items-center">
          {{ $t('ui.search_results') }}
          <q-btn
            flat
            color="primary"
            class="phantom-font"
            icon="arrow_back"
            :label="$t('ui.actions.back')"
            @click="clearSearch"
          />
        </div>

        <hr />
      </h6>
    </div>
    <q-scroll-area v-if="activeView === 'search'" class="scroll-container">
      <ModGrid
        :mods="searchResults"
        :loading="isLoadingSearch"
        :loading-message="$t('ui.searching')"
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
              @show-details="openModDetails"
            />
          </div>
        </div>

        <!-- Latest Mods Section -->
        <div class="mods-section">
          <h6 class="phantom-font-difficulty q-mb-md q-mt-md">
            {{ $t('gamebanana.latest_mods') }}
            <hr />
          </h6>
          <!-- Tab navigation -->
          <q-tabs
            v-model="selectedModType"
            dense
            class="mod-tabs"
            active-color="primary"
            indicator-color="primary"
            narrow-indicator
          >
            <q-tab name="executables" :label="$t('mods.labels.executables')" />
            <q-tab
              name="psychModpacks"
              :label="'Psych Engine ' + $t('mods.labels.modpacks')"
            />
            <q-tab
              name="vsliceModpacks"
              :label="'V-Slice ' + $t('mods.labels.modpacks')"
            />
            <q-tab
              name="codenameModpacks"
              :label="'Codename Engine ' + $t('mods.labels.modpacks')"
            />
          </q-tabs>

          <q-tab-panels v-model="selectedModType" animated>
            <!-- Executables Tab -->
            <q-tab-panel name="executables">
              <ModGrid
                :mods="latestMods"
                :loading="isLoadingLatest"
                :loading-message="$t('gamebanana.loading_latest_mods')"
                :empty-message="$t('ui.no_results')"
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
                :loading-message="
                  $t('gamebanana.loading_modpacks', {
                    engineType: 'Psych Engine',
                  })
                "
                :empty-message="$t('ui.no_results')"
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
                :loading-message="
                  $t('gamebanana.loading_modpacks', { engineType: 'V-Slice' })
                "
                :empty-message="$t('ui.no_results')"
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
                :loading-message="
                  $t('gamebanana.loading_modpacks', {
                    engineType: 'Codename Engine',
                  })
                "
                :empty-message="$t('ui.no_results')"
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
      v-model="isFileSelectionModalOpen"
      :files="currentModalFiles"
      :mod-name="currentModalModName"
      :alternate-file-sources="currentModalAlternateFiles"
      @select="handleFileSelection"
      @cancel="handleFileSelectionCancel"
    />

    <!-- Engine Selection Dialog for Modpacks -->
    <EngineSelectionDialog
      v-model="isEngineSelectionModalOpen"
      :compatible-engines="currentModalCompatibleEngines"
      :engine-type="currentModalEngineType"
      :mod-name="currentModalModName"
      @select="handleEngineSelection"
      @cancel="handleEngineSelectionCancel"
    />
    <!-- Folder Exists Confirmation Dialog -->
    <FolderExistsDialog
      v-model="isFolderExistsModalOpen"
      :mod-name="currentModalModName"
      @update="handleFolderExistsUpdate"
      @download-anyway="handleFolderExistsDownloadAnyway"
      @cancel="handleFolderExistsCancel"
    />
    <!-- Mod Type Selection Modal -->
    <ModTypeSelectionModal
      v-model="isModTypeSelectionModalOpen"
      :mod-name="currentModalModName"
      @submit="handleModTypeSelection"
      @back="handleModTypeSelectionBack"
      @cancel="handleModTypeSelectionCancel"
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
import {
  GBAltFile,
  GBFile,
  GBProfilePage,
  type GBSubfeed,
  type GBSubfeedRecord,
  GBTopSubsItem,
} from '@custom-types/gamebanana'
import { Mod } from '@main-types'
import { gamebananaService } from '@services/gamebananaService'
import { formatEngineName } from '@utils/index'
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

interface ModDownload {
  file: GBFile
  info: GBProfilePage
  folderName?: string
  update?: boolean
}
interface ModpackDownload {
  file: GBFile
  info: GBProfilePage
  engineInstallation: Mod
  folderName?: string
  update?: boolean
}

type DownloadItem = ModDownload | ModpackDownload

// Event listener cleanup
let eventListenerCleanup: (() => void) | undefined

// Search state
const searchQuery = ref('')
const isLoadingSearch = ref(false)
const hasSearched = ref(false)
const searchResults = ref<GBSubfeedRecord[]>([])

// Featured mods state
const featuredMods = ref<GBTopSubsItem[]>([])
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

// View state
const activeView = ref('all') // 'all', 'search'
const selectedModType = ref('executables') // For tabs

// Mod Details Modal state
const isModDetailsModalOpen = ref(false)
const selectedModId = ref(0)
const currentModelType = ref('Mod')

// Modal state management
const isFileSelectionModalOpen = ref(false)
const isEngineSelectionModalOpen = ref(false)
const isFolderExistsModalOpen = ref(false)
const isModTypeSelectionModalOpen = ref(false)

// Modal data
const currentModalFiles = ref<GBFile[]>([])
const currentModalAlternateFiles = ref<GBAltFile[]>([])
const currentModalModName = ref('')
const currentModalCompatibleEngines = ref<Mod[]>([])
const currentModalEngineType = ref('')

// Modal promise resolvers
let fileSelectionResolver:
  | ((value: GBFile | PromiseLike<GBFile>) => void)
  | null = null
let fileSelectionRejecter: ((reason?: any) => void) | null = null
let engineSelectionResolver: ((value: Mod | PromiseLike<Mod>) => void) | null =
  null
let engineSelectionRejecter: ((reason?: any) => void) | null = null
let folderExistsResolver:
  | ((
      value:
        | 'update'
        | 'download-anyway'
        | 'cancel'
        | PromiseLike<'update' | 'download-anyway' | 'cancel'>
    ) => void)
  | null = null
let folderExistsRejecter: ((reason?: any) => void) | null = null
let modTypeSelectionResolver:
  | ((
      value:
        | { modType: string; engineMod?: Mod; isAddon: boolean }
        | PromiseLike<{ modType: string; engineMod?: Mod; isAddon: boolean }>
    ) => void)
  | null = null
let modTypeSelectionRejecter: ((reason?: any) => void) | null = null

watch(selectedModType, async newType => {
  console.info('Tab changed to:', newType)
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
    const response = await invoke<GBTopSubsItem[]>('get_featured_mods_command')

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
  console.info('Changing page to:', page)

  if (activeView.value === 'search') {
    isLoadingSearch.value = true
    try {
      const response = await invoke<GBSubfeed>(
        'fetch_gamebanana_mods_command',
        {
          query: searchQuery.value,
          page: currentPage.value,
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
        const response = await invoke<GBSubfeed>(
          'fetch_gamebanana_mods_command',
          {
            query: 'latest',
            page: page,
          }
        )

        latestMods.value = response._aRecords as GBSubfeedRecord[]
        // Make sure to update totalPages from the response
        totalPages.value = Math.ceil(
          response._aMetadata._nRecordCount / response._aMetadata._nPerpage
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
        const response = await invoke<GBSubfeed>(
          'fetch_gamebanana_mods_command',
          {
            query: '_codenamemodpack',
            page: page,
          }
        )

        codenameModpacks.value = response._aRecords as GBSubfeedRecord[]
        // Make sure to update totalPages from the response
        totalPages.value = Math.ceil(
          response._aMetadata._nRecordCount / response._aMetadata._nPerpage
        )
      } catch (error) {
        console.error('Failed to fetch Codename Engine modpacks page:', error)
      } finally {
        isLoadingCodenameModpacks.value = false
      }
    }
  }
}

// Download handling
const downloadMod = async (mod: GBProfilePage) => {
  const downloadItem: DownloadItem = {
    file: {} as GBFile,
    info: mod,
  }

  try {
    // Step 1: Check if mod has multiple files or alternate file sources
    if (
      (mod._aFiles && mod._aFiles.length > 1) ||
      (mod._aAlternateFileSources && mod._aAlternateFileSources.length > 0)
    ) {
      // Show File Selector Dialog
      const selectedFile = await showFileSelectionModal(
        mod._aFiles || [],
        mod._aAlternateFileSources || [],
        mod._sName
      )
      downloadItem.file = selectedFile
    } else {
      // Single file - use it directly
      if (mod._aFiles && mod._aFiles.length > 0) {
        downloadItem.file = mod._aFiles[0] as GBFile
      } else {
        throw new Error('No files available for download.')
      }
    }

    // Step 2: Check if file has executable
    if (downloadItem.file._aAnalysisWarnings?.contains_exe) {
      // File has executable so we treat it as a standalone mod, skip to folder existence check
      // No additional setup needed for standalone mods
    } else {
      // Step 3: Check if mod is labeled for a specific engine (modpack categories)
      let engineType = 'unknown'

      switch (mod._aCategory._idRow) {
        case 28367: // Psych Engine Modpack
          engineType = 'psych'
          break
        case 29202: // V-Slice Modpack
          engineType = 'vanilla'
          break
        case 34764: // Codename Engine Modpack
          engineType = 'codename'
          break
        default: // Not in a specific category
          engineType = 'generic'
          break
      }

      if (engineType !== 'unknown') {
        if (engineType === 'generic') {
          // Generic modpack - show mod type selection modal
          const modTypeResult = await showModTypeSelectionModal(mod._sName)
          console.info(
            `Mod "${mod._sName}"'s' user selected type:`,
            modTypeResult
          )
          if (
            (modTypeResult.modType === 'psych' ||
              modTypeResult.modType === 'codename' ||
              modTypeResult.modType === 'vanilla' ||
              modTypeResult.modType === 'fps-plus') &&
            modTypeResult.engineMod
          ) {
            ;(downloadItem as ModpackDownload).engineInstallation =
              modTypeResult.engineMod
          } else {
            // If the user picks "standalone", then we treat it as a regular mod download
            console.info(
              `Mod ${mod._sName} is a standalone modpack (picked by user), proceeding as regular mod download`
            )
          }
        } else {
          const compatibleEngines =
            await gamebananaService.getCompatibleEngines(engineType)

          if (compatibleEngines.length === 0) {
            throw new Error(
              `No compatible engines found for ${engineType} modpack`
            )
          }

          const selectedEngine = await showEngineSelectionModal(
            compatibleEngines,
            engineType,
            mod._sName
          )
          ;(downloadItem as ModpackDownload).engineInstallation = selectedEngine
        }
      } else {
        // If not a specific engine modpack, show mod type selection modal
        const modTypeResult = await showModTypeSelectionModal(mod._sName)

        if (modTypeResult.modType === 'modpack') {
          // User selected modpack - check if they provided an engine
          if (modTypeResult.engineMod) {
            ;(downloadItem as ModpackDownload).engineInstallation =
              modTypeResult.engineMod
          } else {
            // Need to show engine selection
            const compatibleEngines =
              await gamebananaService.getCompatibleEngines('unknown')
            const selectedEngine = await showEngineSelectionModal(
              compatibleEngines,
              'unknown',
              mod._sName
            )
            ;(downloadItem as ModpackDownload).engineInstallation =
              selectedEngine
          }
        }
        // If type is 'standalone', continue as regular mod download
      }
    }

    // Step 4: Check if folder already exists
    let folderExists = false
    if ('engineInstallation' in downloadItem) {
      // For modpacks, check in the engine's mods folder
      folderExists = await gamebananaService.checkModpackFolderExists(
        mod,
        (downloadItem as ModpackDownload).engineInstallation
      )
    } else {
      // For standalone mods, check in the main install location
      folderExists = await gamebananaService.checkModFolderExists(
        mod,
        await gamebananaService.getInstallLocation()
      )
    }
    console.info(`Folder exists for mod ${mod._sName}: ${folderExists}`)

    if (folderExists === true) {
      const folderAction = await showFolderExistsModal(mod._sName)

      switch (folderAction) {
        case 'cancel':
          return // User cancelled
        case 'update':
          downloadItem.update = true
          break
        case 'download-anyway':
          // This will create a separate folder for the download
          downloadItem.folderName = `${mod._sName}-${new Date().toISOString().slice(11, 19).replace(/:/g, '-')}`
          if ('engineInstallation' in downloadItem) {
            // For modpacks, check in the engine's mods folder
            if (
              await gamebananaService.checkModpackFolderExists(
                mod,
                (downloadItem as ModpackDownload).engineInstallation,
                downloadItem.folderName
              )
            ) {
              // Handle folder already exists case
              throw new Error(
                `Folder ${downloadItem.folderName} already exists. Trying again should generate a different one.`
              )
            }
          } else {
            // For standalone mods, check in the main install location
            if (
              await gamebananaService.checkModFolderExists(
                mod,
                await gamebananaService.getInstallLocation(),
                downloadItem.folderName
              )
            ) {
              // Handle folder already exists case
              throw new Error(
                `Folder ${downloadItem.folderName} already exists. Trying again should generate a different one.`
              )
            }
          }
          break
      }
    }

    // At this point, downloadItem is fully configured for download
    console.info('Download configured:', downloadItem)

    // Step 5: Perform the download
    // If downloadItem is a ModpackDownload, handle it differently
    if ('engineInstallation' in downloadItem) {
      // Handle modpack download
      await gamebananaService.downloadModpack(
        downloadItem.file,
        downloadItem.info,
        (downloadItem as ModpackDownload).engineInstallation,
        downloadItem.folderName,
        downloadItem.update
      )
    } else {
      // Handle regular mod download
      await gamebananaService.downloadMod(
        downloadItem.file,
        downloadItem.info,
        downloadItem.folderName,
        downloadItem.update
      )
    }
  } catch (error) {
    if (error instanceof Error && error.message.includes('cancelled')) {
      // User cancelled, don't show error notification
      return
    }
    notificationService.downloadError(mod._sName, String(error))
  }
}

const downloadEngine = async (engineType: string) => {
  try {
    let folderName: string | undefined = undefined
    let update = false
    // Check if the folder already exists
    const folderExists = await gamebananaService.checkEngineFolderExists(
      engineType,
      await gamebananaService.getInstallLocation()
    )
    if (folderExists) {
      const action = await showFolderExistsModal(
        await formatEngineName(engineType)
      )
      if (action === 'cancel') {
        return // User cancelled
      } else if (action === 'update') {
        update = true
      } else if (action === 'download-anyway') {
        // Create a new folder for the download
        folderName = `${formatEngineName(engineType)}-${new Date()
          .toISOString()
          .slice(11, 19)
          .replace(/:/g, '-')}`
      }
    }

    await gamebananaService.downloadEngine(engineType, folderName, update)
  } catch (error) {
    console.error('Error downloading engine:', error)
    if (error instanceof Error && error.message.includes('cancelled')) {
      // User cancelled, don't show error notification
      return
    }
    notificationService.downloadError(
      await formatEngineName(engineType),
      String(error)
    )
    throw new Error('Failed to download engine')
  }
}

// Modal handler functions
const showFileSelectionModal = (
  files: GBFile[],
  alternateFiles: GBAltFile[],
  modName: string
): Promise<GBFile> => {
  return new Promise<GBFile>((resolve, reject) => {
    currentModalFiles.value = files
    currentModalAlternateFiles.value = alternateFiles
    currentModalModName.value = modName
    isFileSelectionModalOpen.value = true

    fileSelectionResolver = resolve
    fileSelectionRejecter = reject
  })
}

const showEngineSelectionModal = (
  compatibleEngines: Mod[],
  engineType: string,
  modName: string
): Promise<Mod> => {
  return new Promise<Mod>((resolve, reject) => {
    currentModalCompatibleEngines.value = compatibleEngines
    currentModalEngineType.value = engineType
    currentModalModName.value = modName
    isEngineSelectionModalOpen.value = true

    engineSelectionResolver = resolve
    engineSelectionRejecter = reject
  })
}

const showFolderExistsModal = (
  modName: string
): Promise<'update' | 'download-anyway' | 'cancel'> => {
  return new Promise<'update' | 'download-anyway' | 'cancel'>(
    (resolve, reject) => {
      currentModalModName.value = modName
      isFolderExistsModalOpen.value = true

      folderExistsResolver = resolve
      folderExistsRejecter = reject
    }
  )
}

const showModTypeSelectionModal = (
  modName: string
): Promise<{ modType: string; engineMod?: Mod; isAddon: boolean }> => {
  return new Promise<{ modType: string; engineMod?: Mod; isAddon: boolean }>(
    (resolve, reject) => {
      currentModalModName.value = modName
      isModTypeSelectionModalOpen.value = true

      modTypeSelectionResolver = resolve
      modTypeSelectionRejecter = reject
    }
  )
}

// Modal event handlers
const handleFileSelection = (file: GBFile) => {
  isFileSelectionModalOpen.value = false
  if (fileSelectionResolver) {
    fileSelectionResolver(file)
    fileSelectionResolver = null
    fileSelectionRejecter = null
  }
}

const handleFileSelectionCancel = () => {
  isFileSelectionModalOpen.value = false
  if (fileSelectionRejecter) {
    fileSelectionRejecter(new Error('File selection cancelled'))
    fileSelectionResolver = null
    fileSelectionRejecter = null
  }
}

const handleEngineSelection = (engine: Mod) => {
  isEngineSelectionModalOpen.value = false
  if (engineSelectionResolver) {
    engineSelectionResolver(engine)
    engineSelectionResolver = null
    engineSelectionRejecter = null
  }
}

const handleEngineSelectionCancel = () => {
  isEngineSelectionModalOpen.value = false
  if (engineSelectionRejecter) {
    engineSelectionRejecter(new Error('Engine selection cancelled'))
    engineSelectionResolver = null
    engineSelectionRejecter = null
  }
}

const handleFolderExistsUpdate = () => {
  isFolderExistsModalOpen.value = false
  if (folderExistsResolver) {
    folderExistsResolver('update')
    folderExistsResolver = null
    folderExistsRejecter = null
  }
}

const handleFolderExistsDownloadAnyway = () => {
  isFolderExistsModalOpen.value = false
  if (folderExistsResolver) {
    folderExistsResolver('download-anyway')
    folderExistsResolver = null
    folderExistsRejecter = null
  }
}

const handleFolderExistsCancel = () => {
  isFolderExistsModalOpen.value = false
  if (folderExistsRejecter) {
    folderExistsRejecter(new Error('Folder exists dialog cancelled'))
    folderExistsResolver = null
    folderExistsRejecter = null
  }
}

const handleModTypeSelection = (result: {
  modType: string
  engineMod?: Mod
  isAddon: boolean
}) => {
  isModTypeSelectionModalOpen.value = false
  if (modTypeSelectionResolver) {
    modTypeSelectionResolver(result)
    modTypeSelectionResolver = null
    modTypeSelectionRejecter = null
  }
}

const handleModTypeSelectionBack = () => {
  // Handle back button - could reopen previous modal or cancel
  handleModTypeSelectionCancel()
}

const handleModTypeSelectionCancel = () => {
  isModTypeSelectionModalOpen.value = false
  if (modTypeSelectionRejecter) {
    modTypeSelectionRejecter(new Error('Mod type selection cancelled'))
    modTypeSelectionResolver = null
    modTypeSelectionRejecter = null
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
