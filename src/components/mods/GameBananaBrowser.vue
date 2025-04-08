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
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, onBeforeUnmount } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useQuasar, Notify } from 'quasar';
import { useRouter } from 'vue-router';

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
const searchResults = ref<GameBananaMod[]>([]);
const isLoadingFeatured = ref(false);
const isLoadingLatest = ref(false);
const isLoadingSearch = ref(false);
const hasSearched = ref(false);
const currentPage = ref(1);
const totalPages = ref(1);
const itemsPerPage = 20;
const activeView = ref('all'); // 'all', 'search'
const currentFeaturedSlide = ref(0); // For carousel

// Computed property to determine which mods to display
const displayedMods = computed(() => {
  if (activeView.value === 'search') {
    return searchResults.value;
  }
  return [];  // When in 'all' view, we show featured and latest separately
});

onMounted(async () => {
  fetchFeaturedMods();
  fetchLatestMods();
  
  // Set up event listeners for download events
  removeDownloadStartedListener = await listen<{
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
  });
  
  removeDownloadProgressListener = await listen<{
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
  });
  
  removeDownloadFinishedListener = await listen<{
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
  });
  
  removeDownloadErrorListener = await listen<{
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
  });
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
  }
};

const downloadMod = async (mod: GameBananaMod) => {
  try {
    // Show download starting notification
    const notification = $q.notify({
      type: 'ongoing',
      message: `Downloading "${mod.name}"...`,
      position: 'bottom-right',
      timeout: 0
    });
    
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
      if (window.db) {
        const result = await window.db.select('SELECT value FROM settings WHERE key = $1', ['installLocation']);
        if (result && result.length > 0) {
          installLocation = result[0].value;
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
      // Get mod info directly from the backend using the path
      console.log('Retrieving mod info for path:', modPath);
      
      // Get the list of mods from the backend to find the one we just added
      const allMods = await invoke<any[]>('get_mods');
      modInfo = allMods.find(m => m.path === modPath);
      
      // If we still don't have mod info, create a basic one
      if (!modInfo) {
        console.log('Creating default mod info for:', modPath);
        modInfo = {
          id: crypto.randomUUID(),
          name: mod.name,
          path: modPath,
          executable_path: null,
          icon_data: null,
          banner_data: mod.thumbnailUrl, // Use thumbnail URL as fallback banner
          version: mod.version || null,   // Store version from GameBanana mod
          engine_type: null
        };
      } else {
        console.log('Found mod info from backend:', modInfo);
      }
    }
    
    // Save the mod to the database
    if (modInfo) {
      console.log('Saving downloaded mod to database:', modInfo);
      await saveModToDatabase(modInfo);
    }
    
    // Dismiss loading notification
    notification();
    
    // Show success notification
    $q.notify({
      type: 'positive',
      message: `"${mod.name}" downloaded and installed successfully!`,
      caption: `Ready to play from the mods list`,
      position: 'bottom-right',
      timeout: 5000
    });
    
    console.log('Mod installed to:', modPath);
    
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
    console.error('Failed to download mod:', error);
  }
};

// Save a mod to the database
const saveModToDatabase = async (mod: any) => {
  try {
    // Check if db is initialized
    if (!window.db) {
      console.warn('Database not initialized yet, cannot save mod');
      return false;
    }
    
    console.log('Saving mod to database:', mod);
    
    await window.db.execute(
      `INSERT OR REPLACE INTO mods (id, name, path, executable_path, icon_data, banner_data, logo_data, version, engine_type)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`,
      [mod.id, mod.name, mod.path, mod.executable_path || null, mod.icon_data || null, mod.banner_data || null, mod.logo_data || null, mod.version || null, mod.engine_type || null]
    );
    
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
  // Add any other period values that might come from the API
};

const formatPeriod = (period: string): string => {
  return periodDisplayMap[period] || `Best of ${period}`;
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


.mod-thumbnail img {
  /* Apply mask only to the mod thumbnails in the grid */
  
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

.text-subtitle1 {
    color: var(--theme-text);
    font-size: 1.5rem;
}
</style>