<template>
  <div class="mod-card">
    <div
      class="mod-card-content"
      @click="$emit('showDetails', mod._idRow, mod._sModelName)"
      @contextmenu.prevent="showContextMenu"
    >
      <q-img
        :src="'_sThumbnailUrl' in mod ? mod._sThumbnailUrl : previewImageUrl"
        :placeholder-src="'images/placeholder.png'"
        class="mod-thumbnail"
        :img-style="{
          filter: mod._sInitialVisibility === 'warn' ? 'blur(5px)' : 'none',
        }"
      >
        <img :src="mod._aRootCategory._sIconUrl" class="mod-category-icon" />
        <img
          v-if="mod._aSubmitter._sAvatarUrl"
          :src="mod._aSubmitter._sAvatarUrl"
          class="author-avatar"
        />
      </q-img>
      <div class="mod-info">
        <div class="mod-title">{{ mod._sName }}</div>
        <div class="mod-author-container">
          <div v-if="mod._aSubmitter._sUpicUrl" class="author-upic">
            <span>by</span
            ><img :src="mod._aSubmitter._sUpicUrl" alt="User Picture" />
          </div>
          <div v-else class="mod-author">by {{ mod._aSubmitter._sName }}</div>
        </div>

        <div class="mod-stats">
          <span v-if="mod._sInitialVisibility === 'warn'">
            <q-icon name="warning" size="xs" color="yellow" />
            Has sensitive content!
          </span>
          <span>
            <q-icon name="thumb_up" size="xs" />
            {{ formatNumber(mod._nLikeCount || 0) }}
          </span>
          <span v-if="mod._nPostCount !== undefined">
            <q-icon name="messages" size="xs" />
            {{ formatNumber(mod._nPostCount || 0) }}
          </span>
        </div>
      </div>
    </div>
    <q-btn
      v-if="'_bHasFiles' in mod && mod._bHasFiles && !nonDownloadable"
      color="primary"
      label="Download"
      class="download-btn"
      @click.stop="handleDownload(mod)"
    />
    <q-btn
      v-else
      color="primary"
      label="View Details"
      class="download-btn"
      @click.stop="$emit('showDetails', mod._idRow, mod._sModelName)"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import {
  GBProfilePage,
  type GBSubfeedRecord,
  type GBTopSubsItem,
} from '@custom-types/gamebanana'
import { openUrl } from '@tauri-apps/plugin-opener'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps({
  mod: {
    type: Object as () => GBSubfeedRecord | GBTopSubsItem,
    required: true,
  },
  nonDownloadable: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['download', 'showDetails'])

console.log('ModCard props:', props.mod)

const previewImageUrl = computed(() => {
  if (
    '_aPreviewMedia' in props.mod &&
    props.mod._aPreviewMedia &&
    !props.mod._aPreviewMedia._aImages
  ) {
    return 'images/placeholder.png'
  } else if (
    '_aPreviewMedia' in props.mod &&
    props.mod._aPreviewMedia &&
    props.mod._aPreviewMedia._aImages
  ) {
    return (
      props.mod._aPreviewMedia._aImages[0]._sBaseUrl +
      '/' +
      props.mod._aPreviewMedia._aImages[0]._sFile220
    )
  } else {
    return 'images/placeholder.png'
  }
})

// Helper function to format numbers (e.g., 1000 -> 1K)
const formatNumber = (num: number): string => {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + 'M'
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'K'
  }
  return num.toString()
}

// Context menu handler
const showContextMenu = (event: MouseEvent) => {
  event.preventDefault()
  event.stopPropagation()

  // Create context menu options
  const contextMenuOptions = [
    {
      icon: 'download',
      label: 'Download Mod',
      action: () => emit('download', props.mod),
    },
    {
      icon: 'info',
      label: 'Show Details',
      action: () => emit('showDetails', props.mod._idRow),
    },
    {
      icon: 'open_in_new',
      label: 'Open GameBanana Page',
      action: () => openUrl(props.mod._sProfileUrl),
    },
  ]

  // Create and dispatch custom event to show context menu
  const customEvent = new CustomEvent('show-context-menu', {
    detail: {
      position: { x: event.clientX, y: event.clientY },
      options: contextMenuOptions,
    },
    bubbles: true,
  })

  // Safely handle the case where event.target could be null
  if (event.target) {
    event.target.dispatchEvent(customEvent)
  } else {
    // Fallback to document if target is null
    document.dispatchEvent(customEvent)
  }
}
const handleDownload = async (mod: GBSubfeedRecord | GBTopSubsItem) => {
  const fullMod = await invoke<GBProfilePage>('get_mod_info_command', {
    modId: mod._idRow,
    modelName: mod._sModelName,
  })
  console.info('Full mod data:', fullMod)
  emit('download', fullMod)
}
</script>

<style scoped>
.mod-card {
  background: linear-gradient(
    180deg,
    var(--theme-surface) 0%,
    var(--theme-border) 100%
  );
  border-radius: 0.75rem;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  position: relative;
  transition:
    transform 0.2s,
    box-shadow 0.2s;
  border: 1px solid var(--theme-border);
  border-top: none;
  min-width: 300px;
  -webkit-backdrop-filter: blur(10px);
  backdrop-filter: blur(3px);
  isolation: isolate;
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
  border-radius: 0 0 0.75rem 0.75rem;
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
