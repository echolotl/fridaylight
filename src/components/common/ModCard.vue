<template>
  <div class="mod-card">
    <div class="mod-card-content" @click="$emit('showDetails', mod.id)">
      <q-img
        :src="previewImageUrl"
        class="mod-thumbnail"
      >
      <img :src="mod.categoryIconUrl" class="mod-category-icon" />
      <img :src="mod.submitterAvatarUrl" class="author-avatar" v-if="mod.submitterAvatarUrl" />
      </q-img>
      <div class="mod-info">
        <div class="mod-title">{{ mod.name }}</div>
        <div class="mod-author-container">
          <div class="author-upic" v-if="mod.submitterUPic">
            <span>by</span><img :src="mod.submitterUPic" alt="User Picture" />
          </div>
          <div class="mod-author" v-else>by {{ mod.owner }}</div>
        </div>
        
        <div class="mod-stats">
          <span>
            <q-icon name="thumb_up" size="xs" />
            {{ formatNumber(mod.likes) }}
          </span>
          <span v-if="mod.postCount !== undefined">
            <q-icon name="messages" size="xs" />
            {{ formatNumber(mod.postCount) }}
          </span>
        </div>
      </div>
    </div>
    <q-btn
      color="primary"
      label="Download"
      class="download-btn"
      @click.stop="$emit('download', mod)"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { GameBananaMod } from "@main-types";

const props = defineProps({
  mod: {
    type: Object as () => GameBananaMod,
    required: true
  }
});

defineEmits(['download', 'showDetails']);

const previewImageUrl = computed(() => {
  return props.mod.previewImages && props.mod.previewImages.length > 0
    ? props.mod.previewImages[0].baseUrl
    : props.mod.thumbnailUrl;
});

// Helper function to format numbers (e.g., 1000 -> 1K)
const formatNumber = (num: number): string => {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + "M";
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + "K";
  }
  return num.toString();
};
</script>

<style scoped>
.mod-card {
  background: var(--theme-card);
  border-radius: .75rem;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  transition: transform 0.2s, box-shadow 0.2s;
  border-bottom: 2px solid var(--theme-border);
}

.mod-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 0px 20px rgba(0, 0, 0, 0.2);
}

.mod-card-content {
  cursor: pointer;
  display: flex;
  flex-direction: column;
  flex-grow: 1;
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

.author-upic {
  display: flex;
  align-items: center;
  gap: 4px;
  color: var(--theme-text-secondary);
  margin-bottom: 8px;
  font-size: 0.9rem;

}

.author-avatar {
  position: absolute;
  top: 8px;
  right: 8px;
  width: 32px;
  height: 32px;
  margin-left: 4px;
}

.mod-stats {
  display: flex;
  gap: 12px;
  font-size: 0.8rem;
  color: var(--theme-text-secondary);
}

.download-btn {
  margin: 0 12px 12px 12px;
  border: 2px solid var(--theme-border);
  border-radius: .5rem;
}

.mod-category-icon {
  position: absolute;
  top: 8px;
  left: 8px;
  width: auto;
  height: 32px;
  padding: 2px;
}
</style>