<template>
  <div class="gamebanana-browser phantom-font">
    <div class="browser-header">
      <div class="text-h6"><img src="/images/gamebanana_logo.png"></div>
      <q-input 
        v-model="searchQuery" 
        outlined 
        dense
        placeholder="Search mods..."
        class="search-input"
        @keyup.enter="searchMods"
      >
        <template v-slot:append>
          <q-icon v-if="searchQuery.length > 0" name="close" class="cursor-pointer" @click="clearSearch" />
          <q-icon name="search" class="cursor-pointer" @click="searchMods" />
        </template>
      </q-input>
    </div>
    
    <!-- Search Results View -->
    <div v-if="activeView === 'search'">
      <div class="section-header">
        <div class="text-subtitle1">Search Results</div>
        <q-btn flat dense color="primary" icon="arrow_back" label="Back to Home" @click="clearSearch" />
      </div>
      
      <div class="loading-content" v-if="isLoadingSearch">
        <q-spinner color="primary" size="48px" />
        <div>Searching mods...</div>
      </div>
      
      <div class="mods-grid" v-else-if="searchResults.length > 0">
        <div v-for="mod in searchResults" :key="mod.id" class="mod-card">
          <q-img :src="mod.thumbnailUrl" class="mod-thumbnail" />
          <div class="mod-info">
            <div class="mod-title">{{ mod.name }}</div>
            <div class="mod-author">by {{ mod.owner }}</div>
            <div class="mod-stats">
              <span><q-icon name="downloading" size="xs" /> {{ formatNumber(mod.downloads) }}</span>
              <span><q-icon name="thumb_up" size="xs" /> {{ formatNumber(mod.likes) }}</span>
              <span><q-icon name="visibility" size="xs" /> {{ formatNumber(mod.views) }}</span>
            </div>
          </div>
          <q-btn color="primary" label="Download" class="download-btn" @click="downloadMod(mod)" />
        </div>
      </div>
      
      <div class="no-results" v-else>
        <q-icon name="search_off" size="48px" />
        <div>No mods found matching "{{ searchQuery }}"</div>
      </div>
      
      <div class="pagination" v-if="searchResults.length > 0">
        <q-pagination
          v-model="currentPage"
          :max="totalPages"
          direction-links
          boundary-links
          @update:model-value="changePage"
        />
      </div>
    </div>
    
    <!-- Home View with Featured and Latest Mods -->
    <div v-else class="scroll-container">
      <!-- Featured Mods Carousel Section -->
      <div class="mods-section">
        <div class="section-header">
          <div class="text-subtitle1">Featured Mods</div>
        </div>
        
        <div class="loading-content" v-if="isLoadingFeatured">
          <q-spinner color="primary" size="48px" />
          <div>Loading featured mods...</div>
        </div>
        
        <div v-else class="featured-carousel">
          <div class="carousel-container">
            <q-carousel
              v-model="currentFeaturedSlide"
              animated
              arrows
              autoplay
              navigation-position="bottom"
              infinite
              transition-prev="slide-right"
              transition-next="slide-left"
              class="featured-carousel-inner"
              height="50em"
              control-color="primary"
              navigation-icon="circle"
            >
              <q-carousel-slide 
                v-for="(mod, index) in featuredMods" 
                :key="mod.id" 
                :name="index"
                class="featured-slide"
              >
                <div class="featured-mod-card">
                  <q-img :src="mod.imageUrl" class="featured-thumbnail">
                    <div class="absolute-full featured-overlay"></div>
                    
                    <!-- Add author profile picture with name in top right -->
                    <div class="absolute-top-right q-pa-sm">
                      <div class="author-container">
                        <q-avatar size="40px">
                          <img :src="mod.submitterAvatarUrl || 'https://gamebanana.com/static/img/defaults/avatar.gif'" />
                        </q-avatar>
                        <span class="author-name">{{ mod.owner }}</span>
                      </div>
                    </div>

                    <!-- Move period tag to top left (unchanged) -->
                    <div class="absolute-top-left featured-period q-pa-sm">
                      <q-badge color="primary" text-color="white" class="mod-period-badge">{{ formatPeriod(mod.period) }}</q-badge>
                    </div>
                    
                    <div class="absolute-bottom featured-info">
                      <div class="featured-title">{{ mod.name }}</div>
                      <div class="featured-description">{{ mod.description.slice(0, 100) }}{{ mod.description.length > 100 ? '...' : '' }}</div>
                      
                      <!-- Move stats to the bottom right -->
                      <div class="featured-stats-container">
                        <q-btn color="primary" label="Download" class="featured-btn q-mt-sm" @click="downloadMod(mod)" />
                        <div class="featured-stats">
                          <span><q-icon name="message" size="sm" /> {{ formatNumber(mod.postCount) }}</span>
                          <span><q-icon name="thumb_up" size="sm" /> {{ formatNumber(mod.likes) }}</span>
                        </div>
                      </div>
                    </div>
                  </q-img>
                </div>
              </q-carousel-slide>
            </q-carousel>
          </div>
        </div>
      </div>
        <!-- Latest Mods Section -->
      <div class="mods-section">
        <div class="section-header">
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
            <div class="loading-content" v-if="isLoadingLatest">
              <q-spinner color="primary" size="48px" />
              <div>Loading latest mods...</div>
            </div>
            
            <div class="mods-grid" v-else>
              <div v-for="mod in latestMods" :key="mod.id" class="mod-card">
                <q-img 
                  :src="mod.previewImages && mod.previewImages.length > 0 ? mod.previewImages[0].baseUrl : mod.thumbnailUrl" 
                  class="mod-thumbnail" 
                />
                <div class="mod-info">
                  <div class="mod-title">{{ mod.name }}</div>
                  <div class="mod-author">by {{ mod.owner }}</div>
                  <div class="mod-stats">
                    <span><q-icon name="thumb_up" size="xs" /> {{ formatNumber(mod.likes) }}</span>
                    <span><q-icon name="messages" size="xs" /> {{ formatNumber(mod.postCount) }}</span>
                  </div>
                </div>
                <q-btn color="primary" label="Download" class="download-btn" @click="downloadMod(mod)" />
              </div>
            </div>
          </q-tab-panel>
          
          <!-- Psych Engine Modpacks Tab -->
          <q-tab-panel name="psychModpacks">
            <div class="loading-content" v-if="isLoadingPsychModpacks">
              <q-spinner color="primary" size="48px" />
              <div>Loading Psych Engine modpacks...</div>
            </div>
            
            <div class="mods-grid" v-else>
              <div v-for="mod in psychModpacks" :key="mod.id" class="mod-card">
                <q-img 
                  :src="mod.previewImages && mod.previewImages.length > 0 ? mod.previewImages[0].baseUrl : mod.thumbnailUrl" 
                  class="mod-thumbnail" 
                />
                <div class="mod-info">
                  <div class="mod-title">{{ mod.name }}</div>
                  <div class="mod-author">by {{ mod.owner }}</div>
                  <div class="mod-stats">
                    <span><q-icon name="thumb_up" size="xs" /> {{ formatNumber(mod.likes) }}</span>
                    <span><q-icon name="messages" size="xs" /> {{ formatNumber(mod.postCount) }}</span>
                  </div>
                </div>
                <q-btn color="primary" label="Download" class="download-btn" @click="downloadMod(mod)" />
              </div>
            </div>
          </q-tab-panel>
          
          <!-- V-Slice Modpacks Tab -->
          <q-tab-panel name="vsliceModpacks">
            <div class="loading-content" v-if="isLoadingVsliceModpacks">
              <q-spinner color="primary" size="48px" />
              <div>Loading V-Slice modpacks...</div>
            </div>
            
            <div class="mods-grid" v-else>
              <div v-for="mod in vsliceModpacks" :key="mod.id" class="mod-card">
                <q-img 
                  :src="mod.previewImages && mod.previewImages.length > 0 ? mod.previewImages[0].baseUrl : mod.thumbnailUrl" 
                  class="mod-thumbnail" 
                />
                <div class="mod-info">
                  <div class="mod-title">{{ mod.name }}</div>
                  <div class="mod-author">by {{ mod.owner }}</div>
                  <div class="mod-stats">
                    <span><q-icon name="thumb_up" size="xs" /> {{ formatNumber(mod.likes) }}</span>
                    <span><q-icon name="messages" size="xs" /> {{ formatNumber(mod.postCount) }}</span>
                  </div>
                </div>
                <q-btn color="primary" label="Download" class="download-btn" @click="downloadMod(mod)" />
              </div>
            </div>
          </q-tab-panel>
          
          <!-- Codename Engine Modpacks Tab -->
          <q-tab-panel name="codenameModpacks">
            <div class="loading-content" v-if="isLoadingCodenameModpacks">
              <q-spinner color="primary" size="48px" />
              <div>Loading Codename Engine modpacks...</div>
            </div>
            
            <div class="mods-grid" v-else>
              <div v-for="mod in codenameModpacks" :key="mod.id" class="mod-card">
                <q-img 
                  :src="mod.previewImages && mod.previewImages.length > 0 ? mod.previewImages[0].baseUrl : mod.thumbnailUrl" 
                  class="mod-thumbnail" 
                />
                <div class="mod-info">
                  <div class="mod-title">{{ mod.name }}</div>
                  <div class="mod-author">by {{ mod.owner }}</div>
                  <div class="mod-stats">
                    <span><q-icon name="thumb_up" size="xs" /> {{ formatNumber(mod.likes) }}</span>
                    <span><q-icon name="messages" size="xs" /> {{ formatNumber(mod.postCount) }}</span>
                  </div>
                </div>
                <q-btn color="primary" label="Download" class="download-btn" @click="downloadMod(mod)" />
              </div>
            </div>
          </q-tab-panel>
        </q-tab-panels>
        
        <div class="pagination">
          <q-pagination
            v-model="currentPage"
            :max="totalPages"
            direction-links
            boundary-links
            @update:model-value="changePage"
          />
        </div>
      </div>
    </div>    <!-- Download File Selector Dialog -->
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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useQuasar, Notify } from 'quasar';
import { useRouter } from 'vue-router';
import DownloadFileSelector from './DownloadFileSelector.vue';
import EngineSelectionDialog from './EngineSelectionDialog.vue';

// For managing global download state
import { downloadingMods, updateDownloadProgress, completeDownload, errorDownload } from '../../stores/downloadState';

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

interface GameBananaMod {
  // Existing fields
  id: number;
  name: string;
  owner: string;
  description: string;
  thumbnailUrl: string;
  downloadUrl: string;
  views: number;
  downloads: number;
  likes: number;
  
  // New fields from GameBanana API
  modelName: string;
  profileUrl: string;
  imageUrl: string;
  initialVisibility: string;
  period: string;
  
  // Submitter details
  submitterId: number;
  submitterName: string;
  submitterIsOnline: boolean;
  submitterHasRipe: boolean;
  submitterProfileUrl: string;
  submitterAvatarUrl: string;
  submitterMoreByUrl: string;
  
  // Post count
  postCount: number;
  
  // Category details
  categoryName: string;
  categoryProfileUrl: string;
  categoryIconUrl: string;
  
  // Additional fields from normal mod data
  singularTitle: string;
  iconClasses: string;
  dateAdded: number;
  dateModified: number;
  dateUpdated: number;
  hasFiles: boolean;
  tags: string[];
  previewImages: GameBananaModImage[];
  version: string;
  isObsolete: boolean;
  hasContentRatings: boolean;
  viewCount: number;
  isOwnedByAccessor: boolean;
  wasFeatured: boolean;
}

interface GameBananaModImage {
  imageType: string;
  baseUrl: string;
  fileName: string;
  file100: string;
  file220?: string;
  file530?: string;
  file800?: string;
  height100?: number;
  width100?: number;
  height220?: number;
  width220?: number;
  height530?: number;
  width530?: number;
  height800?: number;
  width800?: number;
}



const searchQuery = ref('');
const featuredMods = ref<GameBananaMod[]>([]);
const latestMods = ref<GameBananaMod[]>([]);
const psychModpacks = ref<GameBananaMod[]>([]);
const vsliceModpacks = ref<GameBananaMod[]>([]);
const codenameModpacks = ref<GameBananaMod[]>([]);
const searchResults = ref<GameBananaMod[]>([]);
const isLoadingFeatured = ref(false);
const isLoadingLatest = ref(false);
const isLoadingPsychModpacks = ref(false);
const isLoadingVsliceModpacks = ref(false);
const isLoadingCodenameModpacks = ref(false);
const isLoadingSearch = ref(false);
const hasSearched = ref(false);
const currentPage = ref(1);
const totalPages = ref(1);
const itemsPerPage = 20;
const activeView = ref('all'); // 'all', 'search'
const currentFeaturedSlide = ref(0); // For carousel
const selectedModType = ref('executables'); // For tabs

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

// Add a watcher to load the appropriate modpack data when tab changes
import { watch } from 'vue';

watch(selectedModType, (newType) => {
  console.log('Tab changed to:', newType);
  currentPage.value = 1; // Reset to first page when changing tabs
  
  if (newType === 'psychModpacks' && psychModpacks.value.length === 0) {
    fetchPsychModpacks();
  } else if (newType === 'vsliceModpacks' && vsliceModpacks.value.length === 0) {
    fetchVsliceModpacks();
  } else if (newType === 'codenameModpacks' && codenameModpacks.value.length === 0) {
    fetchCodenameModpacks();
  }
});

// Function to set up event listeners - separated to avoid async in component setup
const setupEventListeners = () => {
  // Set up event listeners for download events
  listen<{
    modId: number;
    name: string;
    contentLength: number;
    thumbnailUrl: string;
  }>('download-started', (event) => {
    console.log('Download started:', event.payload);
    updateDownloadProgress({
      modId: event.payload.modId,
      name: event.payload.name,
      bytesDownloaded: 0,
      totalBytes: event.payload.contentLength,
      percentage: 0,
      step: 'Starting download...',
      thumbnailUrl: event.payload.thumbnailUrl,
      isComplete: false,
      isError: false
    });
  }).then(unsubscribe => {
    removeDownloadStartedListener = unsubscribe;
  });
  
  listen<{
    modId: number;
    name: string;
    bytesDownloaded: number;
    totalBytes: number;
    percentage: number;
    step: string;
  }>('download-progress', (event) => {
    console.log('Download progress:', event.payload);
    updateDownloadProgress({
      modId: event.payload.modId,
      name: event.payload.name,
      bytesDownloaded: event.payload.bytesDownloaded,
      totalBytes: event.payload.totalBytes,
      percentage: event.payload.percentage,
      step: event.payload.step,
      thumbnailUrl: downloadingMods[event.payload.modId]?.thumbnailUrl || '',
      isComplete: false,
      isError: false
    });
  }).then(unsubscribe => {
    removeDownloadProgressListener = unsubscribe;
  });
  
  listen<{
    modId: number;
    name: string;
    modInfo: any;
  }>('download-finished', (event) => {
    console.log('Download finished:', event.payload);
    updateDownloadProgress({
      modId: event.payload.modId,
      name: event.payload.name,
      bytesDownloaded: 100,
      totalBytes: 100,
      percentage: 100,
      step: 'Download complete',
      thumbnailUrl: downloadingMods[event.payload.modId]?.thumbnailUrl || '',
      isComplete: true,
      isError: false
    });
    
    // Remove from downloads list after a delay
    setTimeout(() => {
      completeDownload(event.payload.modId);
    }, 2000);
    
    // Refresh the mods list by forcing navigation to the home page
    // This will ensure that the newly added mod appears in the list
    setTimeout(() => {
      if (router.currentRoute.value.path !== '/') {
        router.push('/');
      } else {
        // If already on home page, emit a custom event to refresh mod list
        const event = new CustomEvent('refresh-mods');
        window.dispatchEvent(event);
      }
    }, 1000);
  }).then(unsubscribe => {
    removeDownloadFinishedListener = unsubscribe;
  });
  
  listen<{
    modId: number;
    name: string;
    error: string;
  }>('download-error', (event) => {
    console.log('Download error:', event.payload);
    updateDownloadProgress({
      modId: event.payload.modId,
      name: event.payload.name,
      bytesDownloaded: 0,
      totalBytes: 100,
      percentage: 0,
      step: 'Error',
      thumbnailUrl: downloadingMods[event.payload.modId]?.thumbnailUrl || '',
      isComplete: false,
      isError: true,
      error: event.payload.error
    });
    
    // Remove from downloads list after a delay
    setTimeout(() => {
      errorDownload(event.payload.modId, event.payload.error);
    }, 5000);
  }).then(unsubscribe => {
    removeDownloadErrorListener = unsubscribe;
  });
};

onMounted(() => {
  fetchFeaturedMods();
  fetchLatestMods();
  
  // Also load the first tab of mod packs data
  if (selectedModType.value === 'psychModpacks') {
    fetchPsychModpacks();
  } else if (selectedModType.value === 'vsliceModpacks') {
    fetchVsliceModpacks();
  } else if (selectedModType.value === 'codenameModpacks') {
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

const fetchFeaturedMods = async () => {
  isLoadingFeatured.value = true;
  try {
    const response = await invoke<{ mods: GameBananaMod[], total: number }>('fetch_gamebanana_mods_command', { 
      query: 'featured',
      page: 1 // Always get first page for featured
    });
    
    // Take just the top 5 mods for the featured section
    featuredMods.value = response.mods;
  } catch (error) {
    console.error('Failed to fetch featured mods:', error);
  } finally {
    isLoadingFeatured.value = false;
  }
};

const fetchLatestMods = async () => {
  isLoadingLatest.value = true;
  try {
    // Using "latest" as the query to get the latest mods
    const response = await invoke<{ mods: GameBananaMod[], total: number }>('fetch_gamebanana_mods_command', { 
      query: 'latest',
      page: currentPage.value
    });
    
    latestMods.value = response.mods;
    totalPages.value = Math.ceil(response.total / itemsPerPage);
  } catch (error) {
    console.error('Failed to fetch latest mods:', error);
  } finally {
    isLoadingLatest.value = false;
  }
};

// Fetch Psych Engine modpacks
const fetchPsychModpacks = async () => {
  isLoadingPsychModpacks.value = true;
  try {
    const response = await invoke<{ mods: GameBananaMod[], total: number }>('fetch_gamebanana_mods_command', { 
      query: '_psychmodpack',
      page: currentPage.value
    });
    
    psychModpacks.value = response.mods;
    if (selectedModType.value === 'psychModpacks') {
      totalPages.value = Math.ceil(response.total / itemsPerPage);
    }
  } catch (error) {
    console.error('Failed to fetch Psych Engine modpacks:', error);
  } finally {
    isLoadingPsychModpacks.value = false;
  }
};

// Fetch V-Slice modpacks
const fetchVsliceModpacks = async () => {
  isLoadingVsliceModpacks.value = true;
  try {
    const response = await invoke<{ mods: GameBananaMod[], total: number }>('fetch_gamebanana_mods_command', { 
      query: '_vslicemodpack',
      page: currentPage.value
    });
    
    vsliceModpacks.value = response.mods;
    if (selectedModType.value === 'vsliceModpacks') {
      totalPages.value = Math.ceil(response.total / itemsPerPage);
    }
  } catch (error) {
    console.error('Failed to fetch V-Slice modpacks:', error);
  } finally {
    isLoadingVsliceModpacks.value = false;
  }
};

// Fetch Codename Engine modpacks
const fetchCodenameModpacks = async () => {
  isLoadingCodenameModpacks.value = true;
  try {
    const response = await invoke<{ mods: GameBananaMod[], total: number }>('fetch_gamebanana_mods_command', { 
      query: '_codenamemodpack',
      page: currentPage.value
    });
    
    codenameModpacks.value = response.mods;
    if (selectedModType.value === 'codenameModpacks') {
      totalPages.value = Math.ceil(response.total / itemsPerPage);
    }
  } catch (error) {
    console.error('Failed to fetch Codename Engine modpacks:', error);
  } finally {
    isLoadingCodenameModpacks.value = false;
  }
};

const searchMods = async () => {
  if (!searchQuery.value.trim()) {
    activeView.value = 'all';
    return;
  }
  
  activeView.value = 'search';
  isLoadingSearch.value = true;
  currentPage.value = 1;
  
  try {
    const response = await invoke<{ mods: GameBananaMod[], total: number }>('fetch_gamebanana_mods_command', { 
      query: searchQuery.value,
      page: currentPage.value 
    });
    
    searchResults.value = response.mods;
    totalPages.value = Math.ceil(response.total / itemsPerPage);
    hasSearched.value = true;
  } catch (error) {
    console.error('Failed to search mods:', error);
  } finally {
    isLoadingSearch.value = false;
  }
};

const clearSearch = () => {
  searchQuery.value = '';
  activeView.value = 'all';
  hasSearched.value = false;
};

const changePage = async (page: number) => {
  currentPage.value = page;
  
  if (activeView.value === 'search') {
    isLoadingSearch.value = true;
    try {
      const response = await invoke<{ mods: GameBananaMod[], total: number }>('fetch_gamebanana_mods_command', { 
        query: searchQuery.value,
        page: page
      });
      
      searchResults.value = response.mods;
    } catch (error) {
      console.error('Failed to fetch search page:', error);
    } finally {
      isLoadingSearch.value = false;
    }
  } else {
    // Handle pagination based on the current active tab
    if (selectedModType.value === 'executables') {
      isLoadingLatest.value = true;
      try {
        const response = await invoke<{ mods: GameBananaMod[], total: number }>('fetch_gamebanana_mods_command', { 
          query: 'latest',
          page: page
        });
        
        latestMods.value = response.mods;
      } catch (error) {
        console.error('Failed to fetch latest page:', error);
      } finally {
        isLoadingLatest.value = false;
      }
    } else if (selectedModType.value === 'psychModpacks') {
      isLoadingPsychModpacks.value = true;
      try {
        const response = await invoke<{ mods: GameBananaMod[], total: number }>('fetch_gamebanana_mods_command', { 
          query: '_psychmodpack',
          page: page
        });
        
        psychModpacks.value = response.mods;
      } catch (error) {
        console.error('Failed to fetch Psych Engine modpacks page:', error);
      } finally {
        isLoadingPsychModpacks.value = false;
      }
    } else if (selectedModType.value === 'vsliceModpacks') {
      isLoadingVsliceModpacks.value = true;
      try {
        const response = await invoke<{ mods: GameBananaMod[], total: number }>('fetch_gamebanana_mods_command', { 
          query: '_vslicemodpack',
          page: page
        });
        
        vsliceModpacks.value = response.mods;
      } catch (error) {
        console.error('Failed to fetch V-Slice modpacks page:', error);
      } finally {
        isLoadingVsliceModpacks.value = false;
      }
    } else if (selectedModType.value === 'codenameModpacks') {
      isLoadingCodenameModpacks.value = true;
      try {
        const response = await invoke<{ mods: GameBananaMod[], total: number }>('fetch_gamebanana_mods_command', { 
          query: '_codenamemodpack',
          page: page
        });
        
        codenameModpacks.value = response.mods;
      } catch (error) {
        console.error('Failed to fetch Codename Engine modpacks page:', error);
      } finally {
        isLoadingCodenameModpacks.value = false;
      }
    }
  }
};

const downloadMod = async (mod: GameBananaMod) => {
  try {
    currentDownloadMod.value = mod;
    
    // Show loading notification
    pendingDownloadNotification = $q.notify({
      type: 'ongoing',
      message: `Preparing to download "${mod.name}"...`,
      position: 'bottom-right',
      timeout: 0
    });
    
    // First check if this mod has multiple download options
    const downloadInfo = await invoke<any>('get_mod_download_files_command', { modId: mod.id });
    
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
      const engineMods = await getCompatibleEngineMods(modpackType);
      
      if (engineMods.length === 0) {
        // No compatible engine found, show error
        if (pendingDownloadNotification) {
          pendingDownloadNotification();
          pendingDownloadNotification = null;
        }
        
        $q.notify({
          type: 'negative',
          message: `Cannot download ${modpackType} modpack`,
          caption: `You don't have any ${formatEngineType(modpackType)} installed. Please install it.`,
          position: 'bottom-right',
          timeout: 5000
        });
        
        return;
      } else {
        // Compatible engine found, proceed with showing engine selection dialog
        // Store current modpack info for later use in the dialog
        currentModpackInfo.value = {
          mod,
          type: modpackType,
          compatibleEngines: engineMods
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
      type: 'negative',
      message: `Failed to prepare download for "${mod.name}"`,
      caption: String(error),
      position: 'bottom-right',
      timeout: 5000
    });
    
    // Dismiss any pending notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
      pendingDownloadNotification = null;
    }
    
    console.error('Failed to prepare mod download:', error);
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
      type: 'ongoing',
      message: `Downloading "${currentModpackInfo.value.mod.name}"...`,
      position: 'bottom-right',
      timeout: 0
    });
    
    const mod = currentModpackInfo.value.mod;
    
    // Get the installation path for the selected engine's mods folder
    const modsFolderPath = getModsFolderPath(selectedEngineMod.value);
    if (!modsFolderPath) {
      throw new Error("Could not determine mods folder path");
    }
      // Check if we already have a specific download URL from the file selector
    // This happens when the user first selected a file, then selected an engine
    if (mod.downloadUrl && mod.downloadUrl !== currentModpackInfo.value.mod.downloadUrl) {
      // A specific file was already chosen in the file selector
      console.log('Using previously selected file URL:', mod.downloadUrl);
      
      // Use the specific file URL that was already selected
      const result = await invoke<string>('download_gamebanana_mod_command', { 
        url: mod.downloadUrl, 
        name: mod.name,
        modId: mod.id,
        installLocation: modsFolderPath
      });
      
      console.log('Download result:', result);
    } 
    // Not a specific file selection, use the default download URL
    else {
      console.log('Using default download URL:', mod.downloadUrl);
      
      // Use the default download URL for direct download
      const result = await invoke<string>('download_gamebanana_mod_command', { 
        url: mod.downloadUrl, 
        name: mod.name,
        modId: mod.id,
        installLocation: modsFolderPath
      });
      
      console.log('Download result:', result);
    }
  } catch (error) {
    // Show error notification
    $q.notify({
      type: 'negative',
      message: `Failed to install modpack`,
      caption: String(error),
      position: 'bottom-right',
      timeout: 5000
    });
    
    // Dismiss any pending notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
      pendingDownloadNotification = null;
    }
    
    console.error('Failed to install modpack:', error);
  } finally {
    // Reset state
    showEngineSelectDialog.value = false;
    currentModpackInfo.value = null;
    selectedEngineMod.value = null;
    currentDownloadMod.value = null;
  }
};  // Function called when a file is selected from the dialog
const onFileSelected = async (selectedFile: any) => {
  if (!currentDownloadMod.value) return;
  
  try {
    // Create a new notification for the download process
    pendingDownloadNotification = $q.notify({
      type: 'ongoing',
      message: `Preparing "${currentDownloadMod.value.name}"...`,
      position: 'bottom-right',
      timeout: 0
    });
    
    // Use the specific download URL from the selected file
    const mod = currentDownloadMod.value;
    
    // Clear any existing pending modpack path
    localStorage.removeItem('pendingModpackInstallPath');
    
    // After file selection, check if this is a modpack
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
          type: 'negative',
          message: `Cannot download ${modpackType} modpack`,
          caption: `You don't have any ${formatEngineType(modpackType)} installed. Please install it.`,
          position: 'bottom-right',
          timeout: 5000
        });
        
        return;
      } else {
        // Compatible engine found, store the selected file URL for later use
        currentModpackInfo.value = {
          mod: {...mod, downloadUrl: selectedFile._sDownloadUrl},  // Override with selected URL
          type: modpackType,
          compatibleEngines: engineMods
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
    
    // If not a modpack, proceed with standard download
    // Update notification to downloading
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
    }
    pendingDownloadNotification = $q.notify({
      type: 'ongoing',
      message: `Downloading "${mod.name}"...`,
      position: 'bottom-right',
      timeout: 0
    });
    
    // Get the install location from settings
    let installLocation: string | null = null;
    try {
      if (window.db && window.db.service) {
        installLocation = await window.db.service.getSetting('installLocation');
      }
    } catch (error) {
      console.warn('Could not get install location from settings:', error);
    }
    
    console.log('Using selected file URL:', selectedFile._sDownloadUrl);
    console.log('Using installation location:', installLocation);
    
    // Call backend to download using the specific file URL
    const result = await invoke<string>('download_gamebanana_mod_command', { 
      url: selectedFile._sDownloadUrl,
      name: mod.name,
      modId: mod.id,
      installLocation
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
      const allMods = await invoke<any[]>('get_mods');
      modInfo = allMods.find(m => m.path === modPath);
      
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
          engine_type: null
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
      type: 'positive',
      message: `"${mod.name}" downloaded and installed successfully!`,
      caption: `Ready to play from the mods list`,
      position: 'bottom-right',
      timeout: 5000
    });
    
    // Trigger the refresh event to update the mod list
    const refreshEvent = new CustomEvent('refresh-mods');
    window.dispatchEvent(refreshEvent);
    
    // Reset current download mod
    currentDownloadMod.value = null;
    
  } catch (error) {
    // Show error notification
    $q.notify({
      type: 'negative',
      message: `Failed to download "${currentDownloadMod.value?.name || 'Mod'}"`,
      caption: String(error),
      position: 'bottom-right',
      timeout: 5000
    });
    
    // Dismiss any pending notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
      pendingDownloadNotification = null;
    }
    
    console.error('Failed to download mod:', error);
    
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
      type: 'info',
      message: `Download of "${currentDownloadMod.value.name}" cancelled`,
      position: 'bottom-right',
      timeout: 3000
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
      type: 'info',
      message: `Starting download of "${mod.name}"`,
      position: 'bottom-right',
      timeout: 2000
    });
    
    // Get the install location from settings
    let installLocation: string | null = null;
    try {
      if (window.db && window.db.service) {
        installLocation = await window.db.service.getSetting('installLocation');
        if (installLocation) {
          console.log('Using custom install location from settings:', installLocation);
        }
      }
    } catch (error) {
      console.warn('Could not get install location from settings:', error);
    }
    
    // Pass mod ID along with URL, name, and install location
    const result = await invoke<string>('download_gamebanana_mod_command', { 
      url: mod.downloadUrl, 
      name: mod.name,
      modId: mod.id,
      installLocation
    });
    console.log('download_gamebanana_mod_command result', result);
    
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
      const allMods = await invoke<any[]>('get_mods');
      modInfo = allMods.find(m => m.path === modPath);
      
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
          engine_type: null
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
      type: 'positive',
      message: `"${mod.name}" downloaded and installed successfully!`,
      caption: `Ready to play from the mods list`,
      position: 'bottom-right',
      timeout: 5000
    });
    
    // Trigger the refresh event to update the mod list
    const refreshEvent = new CustomEvent('refresh-mods');
    window.dispatchEvent(refreshEvent);
  } catch (error) {
    // Show error notification
    $q.notify({
      type: 'negative',
      message: `Failed to download "${mod.name}"`,
      caption: String(error),
      position: 'bottom-right',
      timeout: 5000
    });
    
    // Dismiss any pending notification
    if (pendingDownloadNotification) {
      pendingDownloadNotification();
      pendingDownloadNotification = null;
    }
    
    console.error('Failed to download mod:', error);
  }
};

// Save a mod to the database
const saveModToDatabase = async (mod: any) => {
  try {
    // Check if DatabaseService is initialized
    if (!window.db || !window.db.service) {
      console.warn('Database service not initialized yet, cannot save mod');
      return false;
    }
    
    console.log('Saving mod to database using DatabaseService:', mod);
    
    // Make sure the mod has an engine field required by the type
    if (!mod.engine) {
      mod.engine = {
        engine_type: mod.engine_type || 'unknown',
        engine_name: mod.engine_type || 'Unknown Engine',
        engine_icon: '',
        mods_folder: false,
        mods_folder_path: ''
      };
    }
    
    // Use the DatabaseService to save the mod
    await window.db.service.saveMod(mod);
    
    console.log('Mod saved successfully to database:', mod.name);
    return true;
  } catch (error) {
    console.error('Failed to save mod to database:', error);
    return false;
  }
};

const formatNumber = (num: number): string => {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + 'M';
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'K';
  }
  return num.toString();
};

const periodDisplayMap: Record<string, string> = {
  'today': 'Best of Today',
  'week': 'Best of this Week',
  'month': 'Best of this Month',
  '3month': 'Best of 3 Months',
  '6month': 'Best of 6 Months',
  'year': 'Best of this Year',
  'alltime': 'Best of All Time',
};

const formatPeriod = (period: string): string => {
  return periodDisplayMap[period] || `Best of ${period}`;
};

// Function to determine if a mod is a modpack based on the current tab or mod properties
const determineIfModpack = (mod: GameBananaMod): boolean => {
  // Check if we're in a modpack tab
  if (['psychModpacks', 'vsliceModpacks', 'codenameModpacks'].includes(selectedModType.value)) {
    return true;
  }
  
  // Check mod category if available
  if (mod.categoryName) {
    const lowerCaseCategoryName = mod.categoryName.toLowerCase();
    return lowerCaseCategoryName.includes('modpack') || 
           lowerCaseCategoryName.includes('mod pack') ||
           lowerCaseCategoryName.includes('mods folder');
  }
  
  return false;
};

// Function to determine modpack type (psych, vslice, codename, or null if not a modpack)
const determineModpackType = (mod: GameBananaMod): string | null => {
  // First check the current tab
  if (selectedModType.value === 'psychModpacks') return 'psych';
  if (selectedModType.value === 'vsliceModpacks') return 'vanilla';
  if (selectedModType.value === 'codenameModpacks') return 'codename';
  
  // Then check mod tags or description
  const lowerCaseDesc = mod.description?.toLowerCase() || '';
  const tags = mod.tags || [];
  const lowerCaseTags = tags.map(tag => tag.toLowerCase());
  
  if (lowerCaseDesc.includes('psych engine') || lowerCaseTags.some(tag => tag.includes('psych'))) {
    return 'psych';
  } else if (lowerCaseDesc.includes('v-slice') || lowerCaseTags.some(tag => tag.includes('vslice') || tag.includes('v-slice'))) {
    return 'vslice';
  } else if (lowerCaseDesc.includes('codename engine') || lowerCaseTags.some(tag => tag.includes('codename'))) {
    return 'codename';
  }
  
  return null;
};

// Get a list of compatible engine mods
const getCompatibleEngineMods = async (engineType: string | null): Promise<any[]> => {
  if (!engineType) return [];
  
  try {
    // Fetch all mods
    let allMods = [];
    if (window.db && window.db.service) {
      allMods = await window.db.service.getAllMods();
    } else {
      allMods = await invoke<any[]>('get_mods');
    }
    
    // Filter mods by engine type
    return allMods.filter((mod: { engine: { engine_type: string; }; engine_type: string; }) => {
      // Check engine.engine_type first (new structure)
      if (mod.engine && mod.engine.engine_type) {
        return mod.engine.engine_type.toLowerCase() === engineType.toLowerCase();
      }
      
      // Fall back to legacy engine_type field
      return mod.engine_type && mod.engine_type.toLowerCase() === engineType.toLowerCase();
    });
  } catch (error) {
    console.error('Failed to get compatible engine mods:', error);
    return [];
  }
};

const formatEngineType = (engineType: string | null): string => {
  if (!engineType) return 'Unknown';
  
  switch(engineType.toLowerCase()) {
    case 'psych': return 'Psych Engine';
    case 'vanilla': return 'V-Slice';
    case 'codename': return 'Codename Engine';
    default: return engineType.charAt(0).toUpperCase() + engineType.slice(1);
  }
};

// Function to get the mods folder path for an engine mod
const getModsFolderPath = (engineMod: any): string => {
  // First check if the engine has a specified mods_folder_path in the new structure
  if (engineMod.engine && engineMod.engine.mods_folder && engineMod.engine.mods_folder_path) {
    return engineMod.engine.mods_folder_path;
  }
  
  // If not, construct the default mods folder path
  const basePath = engineMod.path;
  const executablePath = engineMod.executable_path || '';
  
  if (!basePath) return 'Unknown path';
  
  // Get parent directory of executable if it exists
  let baseDir = basePath;
  if (executablePath) {
    // Extract the directory from the executable path
    const lastSlashIndex = executablePath.lastIndexOf('/');
    if (lastSlashIndex > 0) {
      baseDir = executablePath.substring(0, lastSlashIndex);
    } else {
      const lastBackslashIndex = executablePath.lastIndexOf('\\');
      if (lastBackslashIndex > 0) {
        baseDir = executablePath.substring(0, lastBackslashIndex);
      }
    }
  }
  
  // Construct the path to the mods folder
  return `${baseDir}/mods`;
};
</script>

<style scoped>
.gamebanana-browser {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.browser-header {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 16px;
}

.search-input {
  width: 100%;
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

.featured-carousel-inner {
  width: 100%;
}

.featured-slide {
  display: flex;
  justify-content: center;
  align-items: center;
}

.featured-mod-card {
  position: relative;
  width: 100%;
    height: 100%;
  border-radius: 8px;
  overflow: hidden;
}

.featured-thumbnail {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.featured-overlay {
  background: linear-gradient(to top, rgba(0, 0, 0, 0.8), transparent);
}

.featured-info {
  padding: 12px;
  color: white;
  line-height: 1.2;
}

.featured-title {
  font-weight: bold;
  font-size: 2rem;
  margin-bottom: 4px;
}

.featured-author {
  font-size: 0.9rem;
  margin-bottom: 8px;
}

.featured-description {
  font-size: 1.1rem;
  margin-bottom: 8px;
}

.featured-stats-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.featured-stats {
  display: flex;
  gap: 12px;
  font-size: 1rem;
  align-items: center;
}

.featured-btn {
  margin-top: 8px;
}

.featured-period {
  font-size: 0.9rem;
  z-index: 10;
}

.mod-period-badge {
  font-size: 0.8rem;
  padding: 0.5rem 0.5rem;
}

.absolute-top-right {
  z-index: 10;
}

.author-container {
  display: flex;
  align-items: center;
  gap: 8px;
}

.author-name {
  font-size: 0.9rem;
  color: white;
}

.mods-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
  overflow-y: auto;
  padding: 8px;
}

.mod-card {
  background: var(--theme-card);
  border-radius: 8px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  transition: transform 0.2s;
}

.mod-card:hover {
  transform: translateY(-5px);
}

.mod-thumbnail {
  height: 160px;
  object-fit: cover;
}

.mod-info {
  padding: 12px;
  flex-grow: 1;
}

.mod-title {
  font-weight: bold;
  font-size: 1.1rem;
  margin-bottom: 4px;
  color: var(--theme-text);
}

.mod-author {
  font-size: 0.9rem;
  color: var(--theme-text-secondary);
  margin-bottom: 8px;
}

.mod-stats {
  display: flex;
  gap: 12px;
  font-size: 0.8rem;
  color: var(--theme-text-secondary);
}

.download-btn {
  margin: 0 12px 12px 12px;
}

.loading-content, .no-results {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 200px;
  color: var(--theme-text-secondary);
}

.pagination {
  padding: 16px 0;
  display: flex;
  justify-content: center;
}
.q-carousel {
    background-color: transparent;
}

.q-img__content > div {
    background-color: transparent;
}

.q-tab-panel, .q-tab-panels, .q-panel{
  background-color: transparent !important;
}

.text-subtitle1 {
    color: var(--theme-text);
    font-size: 1.5rem;
}
</style>