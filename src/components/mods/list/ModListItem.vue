<template>
  <q-item
    :key="mod.id"
    v-ripple
    clickable
    :active="isActive"
    active-class="active-mod"
    class="draggable-item cursor-move"
    :class="{ 'compact-mode': compactMode }"
    @click="$emit('select-mod', mod)"
    @contextmenu.prevent="showContextMenu"
  >
    <q-item-section v-if="mod.icon_data" avatar>
      <q-avatar size="32px" square>
        <div v-if="icon">
          <img
            :src="icon"
            alt="mod icon"
            style="
              object-fit: cover;
              border-radius: 4px;
              width: 32px;
              height: 32px;
            "
          />
        </div>
        <div v-else><img :src="mod.icon_data" alt="mod icon" /></div>
      </q-avatar>
    </q-item-section>
    <q-item-section v-else avatar>
      <q-avatar size="32px" icon="folder" square class="default-icon" />
    </q-item-section>
    <q-item-section v-if="!compactMode">
      <q-item-label>{{ mod.name }}</q-item-label>
      <q-item-label v-if="mod.version" caption class="version-text"
        >v{{ mod.version }}</q-item-label
      >
    </q-item-section>
    <q-tooltip v-if="compactMode" class="phantom-font">
      {{ mod.name }}{{ mod.version ? ` (v${mod.version})` : '' }}
    </q-tooltip>
  </q-item>
</template>

<script setup lang="ts">
import { Mod } from '@main-types'

const props = defineProps({
  mod: {
    type: Object as () => Mod,
    required: true,
  },
  icon: {
    type: String,
    default: '',
    required: false,
  },
  isActive: {
    type: Boolean,
    default: false,
  },
  compactMode: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits([
  'select-mod',
  'delete-mod',
  'open-mod-settings',
  'launch-mod',
  'super-delete-mod',
  'open-mod-folder',
])

// Context menu handler
const showContextMenu = (event: MouseEvent) => {
  event.preventDefault()
  event.stopPropagation()

  // Create context menu options
  const contextMenuOptions = [
    {
      icon: 'play_arrow',
      label: 'Launch Mod',
      action: () => {
        // Directly emit the launch-mod event with the mod's ID
        emit('launch-mod', props.mod.id)
      },
    },
    {
      icon: 'settings',
      label: 'Edit Settings',
      action: () => emit('open-mod-settings', props.mod),
    },
    {
      icon: 'folder_open',
      label: 'Open Mod Folder',
      action: () => emit('open-mod-folder', props.mod),
    },

    { separator: true },
    {
      icon: 'remove_circle_outline',
      label: 'Remove Mod',
      action: () => emit('delete-mod', props.mod),
    },
    {
      icon: 'delete_forever',
      label: 'Delete Mod',
      action: () => emit('super-delete-mod', props.mod),
      danger: true,
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
</script>

<style scoped>
.active-mod {
  background-color: var(--theme-surface);
  border-radius: 0 1rem 1rem 0;
}

.default-icon {
  color: var(--theme-text-secondary) !important;
  background-color: transparent;
  border-radius: 4px;
}

/* Target the inner icon within the q-avatar */
.default-icon .q-icon {
  color: var(--theme-text-secondary) !important;
}

.q-avatar img {
  object-fit: contain;
  background-color: transparent;
  image-rendering: pixelated;
  border-radius: 0;
}

.q-item {
  border-radius: 0 1rem 1rem 0;
  transition: padding 0.2s ease;
}

.draggable-item {
  transition: background-color 0.2s ease;
  position: relative;
}

.draggable-item:hover {
  background-color: var(--theme-surface);
}

.draggable-item * {
  user-select: none;
}

.cursor-move {
  cursor: grab;
}

.cursor-move:active {
  cursor: grabbing;
}

.delete-btn {
  opacity: 0;
  transition: opacity 0.2s ease;
}

.q-item:hover .delete-btn {
  opacity: 1;
}

.version-text {
  color: var(--theme-text-secondary) !important;
}

.compact-mode {
  padding: 4px 16px;
  min-height: 40px;
  max-width: 60px;
}
</style>
