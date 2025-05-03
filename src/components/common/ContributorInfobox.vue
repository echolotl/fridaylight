<template>
  <div class="contributor-infobox" v-if="contributorsData && contributorsData.length > 0">
    <h5 class="phantom-font-difficulty">Contributors</h5>
    <hr />
    <div class="contributor-groups">
      <div 
        v-for="(group, title) in groupedContributors" 
        :key="title"
        class="contributor-group"
      >
        <h6 class="contributor-title">{{ title }}</h6>
        <div class="contributor-list">
          <div 
            v-for="(contributor, index) in group" 
            :key="index"
            class="contributor-item"
          >
            <div class="contributor-icon" v-if="contributor.icon">
              <img :src="getIconSrc(contributor)" :alt="`${contributor.name}'s icon`" />
            </div>
            <div class="contributor-info">
              <div class="contributor-name">
                {{ contributor.name }}
              </div>
              <div class="contributor-role" v-if="contributor.role">
                {{ contributor.role }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, watch, reactive } from 'vue';
import { Contributor } from '@main-types';
import { sep } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  contributors: {
    type: Array as () => Contributor[],
    default: () => []
  },
  folder_path: {
    type: String,
    default: ''
  }
});

// Local state to store contributor data from metadata.json
const contributorsData = ref<Contributor[]>([]);
const metadataLoaded = ref(false);

// Map to store processed icon data
const iconCache = reactive(new Map<string, string>());

// Function to load metadata.json and extract contributors
const loadContributorsFromMetadata = async () => {
  if (!props.folder_path) return;
  
  try {
    console.log(`Attempting to load metadata from: ${props.folder_path}`);
    
    // Get metadata.json using the new Tauri command
    const metadata = await invoke('get_mod_metadata', { modPath: props.folder_path });
    console.log('Metadata loaded:', metadata);
    
    // Check if metadata has contributors array
    if (metadata && typeof metadata === 'object' && Array.isArray((metadata as any).contributors)) {
      console.log('Found contributors in metadata:', (metadata as any).contributors);
      contributorsData.value = (metadata as any).contributors;
      metadataLoaded.value = true;
      
      // Preload all icon data
      preloadIconData();
    } else {
      console.log('No contributors found in metadata, falling back to props:', props.contributors);
      // Fallback to props if metadata doesn't have contributors
      contributorsData.value = props.contributors;
      
      // Preload all icon data for props contributors
      preloadIconData();
    }
  } catch (error) {
    console.warn('Failed to load metadata.json for contributors:', error);
    // Fallback to props if there's an error
    contributorsData.value = props.contributors;
    
    // Preload all icon data for props contributors
    preloadIconData();
  }
};

// Function to preload all icon data
const preloadIconData = async () => {
  if (!contributorsData.value || !props.folder_path) return;
  const fileSeperator = sep();
  
  // Process all contributors to load their icons
  for (const contributor of contributorsData.value) {
    if (contributor.icon && !contributor.icon.startsWith('data:')) {
      const originalPath = contributor.icon;
      const iconPath = `${props.folder_path}${fileSeperator}.flight${fileSeperator}${originalPath}`;
      console.log(`Preloading icon from: ${iconPath}`);
      
      try {
        const base64Data = await invoke('get_file_as_base64', { filePath: iconPath }) as string;
        console.log(`Successfully loaded icon for ${contributor.name}`);
        iconCache.set(originalPath, base64Data);
      } catch (error) {
        console.warn(`Failed to load icon for contributor ${contributor.name}:`, error);
        // Keep the original path in the cache as a fallback
        iconCache.set(originalPath, originalPath);
      }
    }
  }
};

// Helper function to get the correct icon source
const getIconSrc = (contributor: Contributor): string => {
  if (!contributor.icon) return '';
  
  // If it's already a data URL, return it
  if (contributor.icon.startsWith('data:')) {
    return contributor.icon;
  }
  
  // Check if we have a cached version
  if (iconCache.has(contributor.icon)) {
    return iconCache.get(contributor.icon) || contributor.icon;
  }
  
  // Return the original path as fallback
  return contributor.icon;
};

// Group contributors by their title
const groupedContributors = computed(() => {
  const grouped: Record<string, Contributor[]> = {};
  
  if (!contributorsData.value || contributorsData.value.length === 0) {
    return grouped;
  }
  
  // Group all contributors by their title
  contributorsData.value.forEach(contributor => {
    const title = contributor.title || 'Contributor';
    if (!grouped[title]) {
      grouped[title] = [];
    }
    
    // Add the contributor to the appropriate group
    grouped[title].push(contributor);
  });
  
  return grouped;
});

// Load contributors data when component mounts or folder_path changes
onMounted(loadContributorsFromMetadata);
watch(() => props.folder_path, loadContributorsFromMetadata);
</script>

<style scoped>
.contributor-infobox {
  padding: 15px;
  height: fit-content;
  position: sticky;
  top: 20px;
}

.contributor-groups {
  margin-top: 15px;
}

.contributor-group {
  margin-bottom: 15px;
}

.contributor-title {
  color: var(--theme-primary);
  font-weight: bold;
  margin: 5px 0;
  font-size: 1rem;
}

.contributor-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin: 8px 0;
}

.contributor-item {
  display: flex;
  align-items: center;
  padding: 0 .9rem;
}

.contributor-item:last-child {
  border-bottom: none;
}

.contributor-icon {
  width: 32px;
  height: 32px;
  overflow: hidden;
  flex-shrink: 0;
  margin-right: .5rem;
}

.contributor-icon img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.contributor-info {
  display: flex;
  flex-direction: column;
}

.contributor-name {
  font-size: 0.9rem;
  overflow: hidden;
  text-overflow: ellipsis;
}

.contributor-role {
  font-size: 0.8rem;
  color: var(--theme-text-secondary);
  opacity: 0.85;
  line-height: 1.2;
}

hr {
  border-color: var(--theme-border);
  margin-top: 5px;
  margin-bottom: 10px;
}

h5 {
  margin: 0;
  color: var(--theme-text);
}
</style>