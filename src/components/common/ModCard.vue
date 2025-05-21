<template>
  <div class="mod-card">
    <div
      class="mod-card-content"
      @click="$emit('showDetails', mod.id, mod.model_name)"
      @contextmenu.prevent="showContextMenu"
    >
      <q-img :src="previewImageUrl" class="mod-thumbnail" :img-style="{ filter: mod.initial_visibility == 'warn' ? 'blur(5px)' : 'none' }">
        <img :src="mod.category_icon_url" class="mod-category-icon" />
        <img
          :src="mod.submitter_avatar_url"
          class="author-avatar"
          v-if="mod.submitter_avatar_url"
        />
      </q-img>
      <div class="mod-info">
        <div class="mod-title">{{ mod.name }}</div>
        <div class="mod-author-container">
          <div class="author-upic" v-if="mod.submitter_u_pic">
            <span>by</span><img :src="mod.submitter_u_pic" alt="User Picture" />
          </div>
          <div class="mod-author" v-else>by {{ mod.owner }}</div>
        </div>

        <div class="mod-stats">
          <span>
            <q-icon name="thumb_up" size="xs" />
            {{ formatNumber(mod.likes) }}
          </span>
          <span v-if="mod.post_count !== undefined">
            <q-icon name="messages" size="xs" />
            {{ formatNumber(mod.post_count) }}
          </span>
        </div>
      </div>
    </div>
    <q-btn
      color="primary"
      label="Download"
      class="download-btn"
      @click.stop="$emit('download', mod)"
      v-if="mod.has_files"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import type { GameBananaMod } from "@main-types";
import { openUrl } from "@tauri-apps/plugin-opener";

const props = defineProps({
  mod: {
    type: Object as () => GameBananaMod,
    required: true,
  },
});

const emit = defineEmits(["download", "showDetails"]);

const previewImageUrl = computed(() => {
  console.log(props.mod);
  return props.mod.image_url ||props.mod.thumbnail_url;
});

console.log(props.mod);

// Helper function to format numbers (e.g., 1000 -> 1K)
const formatNumber = (num: number): string => {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + "M";
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + "K";
  }
  return num.toString();
};

// Context menu handler
const showContextMenu = (event: MouseEvent) => {
  event.preventDefault();
  event.stopPropagation();

  // Create context menu options
  const contextMenuOptions = [
    {
      icon: "download",
      label: "Download Mod",
      action: () => emit("download", props.mod),
    },
    {
      icon: "info",
      label: "Show Details",
      action: () => emit("showDetails", props.mod.id),
    },
    {
      icon: "open_in_new",
      label: "Open GameBanana Page",
      action: () => openUrl(props.mod.profile_url),
    },
  ];

  // Create and dispatch custom event to show context menu
  const customEvent = new CustomEvent("show-context-menu", {
    detail: {
      position: { x: event.clientX, y: event.clientY },
      options: contextMenuOptions,
    },
    bubbles: true,
  });

  // Safely handle the case where event.target could be null
  if (event.target) {
    event.target.dispatchEvent(customEvent);
  } else {
    // Fallback to document if target is null
    document.dispatchEvent(customEvent);
  }
};
</script>

<style scoped>
.mod-card {
  background: var(--theme-card);
  border-radius: 0.75rem;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  transition: transform 0.2s, box-shadow 0.2s;
  border-bottom: 2px solid var(--theme-border);
  min-width: 300px;
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
  border-bottom: 2px solid var(--theme-border);
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
  border-radius: 0.5rem;
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
