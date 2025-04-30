<template>
  <div class="contributor-infobox" v-if="contributors && contributors.length > 0">
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
              <img :src="contributor.icon" :alt="`${contributor.name}'s icon`" />
            </div>
            <div class="contributor-name">
              {{ contributor.name }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
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

// Group contributors by their title
const groupedContributors = computed(() => {
  const grouped: Record<string, Contributor[]> = {};
  
  if (!props.contributors || props.contributors.length === 0) {
    return grouped;
  }
  
  // First pass: collect all contributors with defined titles
  props.contributors.forEach(contributor => {

    const title = contributor.title || 'Contributor';
    if (!grouped[title]) {
      grouped[title] = [];
    }
    grouped[title].push(contributor);
  });
  
  // Second pass: convert icons to base64
  for (const title in grouped) {
    grouped[title] = grouped[title].map(contributor => {
      if (contributor.icon) {
        invoke('get_file_as_base64', { filePath: `${props.folder_path}${sep()}.flight${sep()}${contributor.icon}` })
          .then(base64 => {
            contributor.icon = base64 as string;
          });
      }
      return contributor;
    });
  }
  return grouped;
});
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

.contributor-name {
  font-size: 0.9rem;
  overflow: hidden;
  text-overflow: ellipsis;
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