<template>
  <div class="mod-card">
    <q-img
      :src="previewImageUrl"
      class="mod-thumbnail"
    />
    <div class="mod-info">
      <div class="mod-title">{{ mod.name }}</div>
      <div class="mod-author">by {{ mod.owner }}</div>
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
    <q-btn
      color="primary"
      label="Download"
      class="download-btn"
      @click="$emit('download', mod)"
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

defineEmits(['download']);

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
  border-radius: 8px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  transition: transform 0.2s;
  border: 1px solid var(--theme-border);
}

.mod-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
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
</style>