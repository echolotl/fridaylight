<template>
  <q-item
    :key="folder.id"
    v-ripple
    clickable
    class="draggable-item cursor-move folder"
    :class="{ 'expanded-folder': isExpanded, 'compact-mode': compactMode }"
    :style="{ borderBottomColor: isExpanded ? folder.color : 'transparent' }"
    @click="toggleExpanded"
    @contextmenu.prevent="showContextMenu"
  >
    <q-item-section avatar>
      <q-avatar size="32px" square class="folder-icon">
        <q-icon name="folder" size="24px" :style="{ color: folder.color }" />
      </q-avatar>
    </q-item-section>
    <q-item-section v-if="!compactMode">
      <q-item-label>{{ folder.name }}</q-item-label>
    </q-item-section>
    <q-item-section v-if="!compactMode" side>
      <div class="row items-center">
        <q-btn
          flat
          round
          dense
          :icon="isExpanded ? 'expand_less' : 'expand_more'"
          style="color: var(--theme-text-secondary)"
          class="folder-action-btn q-mr-xs"
          @click.stop="toggleExpanded"
        />
      </div>
    </q-item-section>
    <q-tooltip v-if="compactMode" class="phantom-font">
      {{ folder.name }}
    </q-tooltip>
  </q-item>
  <!-- Expanded folder content with indentation -->
  <transition name="folder-expand" appear>
    <div
      v-if="isExpanded"
      class="folder-contents"
      :class="{ 'compact-folder-contents': compactMode }"
      :style="{ borderLeftColor: folder.color }"
    >
      <draggable
        :group="{ name: 'mods', pull: true, put: true }"
        :list="modsInFolder"
        item-key="id"
        :animation="200"
        ghost-class="sortable-ghost"
        chosen-class="sortable-chosen"
        drag-class="sortable-drag"
        :force-fallback="true"
        :fallback-on-body="true"
        :delay="100"
        @change="handleModsChange"
        @end="handleDragEnd"
      >
        <template #item="{ element }">
          <div :key="element.id" class="draggable-item cursor-move">
            <ModListItem
              :mod="element.data"
              :is-active="selectedModId === element.id"
              :compact-mode="compactMode"
              @select-mod="$emit('select-mod', element.data)"
              @delete-mod="$emit('delete-mod', element)"
              @super-delete-mod="$emit('super-delete-mod', element.data)"
              @open-mod-settings="$emit('open-mod-settings', $event)"
              @launch-mod="$emit('launch-mod', $event)"
            />
          </div>
        </template>
      </draggable>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import ModListItem from '@mods/list/ModListItem.vue'
import draggable from 'vuedraggable'
import { Mod, Folder } from '@main-types'
import { useI18n } from 'vue-i18n'

const props = defineProps({
  folder: {
    type: Object as () => Folder,
    required: true,
  },
  allMods: {
    type: Array as () => Mod[],
    required: true,
  },
  selectedModId: {
    type: String,
    default: '',
  },
  compactMode: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits([
  'select-mod',
  'delete-mod',
  'super-delete-mod',
  'delete-folder',
  'update-folder-mods',
  'reorder-folder-mods',
  'open-mod-settings',
  'edit-folder',
  'launch-mod',
])

const { t } = useI18n()
const isExpanded = ref(false)

// Compute the list of mods that belong to this folder
const modsInFolder = computed(() => {
  return (
    props.allMods
      // Double check both the folder.mods array AND the mod's folder_id to avoid ghost items
      .filter(
        mod =>
          props.folder.mods.includes(mod.id) &&
          mod.folder_id === props.folder.id
      )
      .sort((a, b) => {
        // Sort by display_order_in_folder if available
        const orderA = a.display_order_in_folder ?? 999
        const orderB = b.display_order_in_folder ?? 999
        return orderA - orderB
      })
      .map(mod => ({
        id: mod.id,
        data: mod, // Ensure each mod has a data property for ModListItem
      }))
  )
})

// Toggle folder expanded state
const toggleExpanded = () => {
  isExpanded.value = !isExpanded.value
}

// Context menu handler
const showContextMenu = (event: MouseEvent) => {
  event.preventDefault()
  event.stopPropagation()

  // Create context menu options
  const contextMenuOptions = [
    {
      icon: 'edit',
      label: t('ui.actions.edit_folder'),
      action: () => emit('edit-folder', props.folder),
    },
    {
      icon: isExpanded.value ? 'expand_less' : 'expand_more',
      label: isExpanded.value
        ? t('ui.actions.collapse_folder')
        : t('ui.actions.expand_folder'),
      action: () => toggleExpanded(),
    },
    { separator: true },
    {
      icon: 'folder_delete',
      label: t('ui.actions.delete_folder'),
      action: () => emit('delete-folder', props.folder),
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

// Handle drag end to update mod order within folder
const handleDragEnd = () => {
  console.log('Reordering mods within folder:', props.folder.name)

  // Emit an event to update the order with the batch update function
  emit('reorder-folder-mods', {
    folderId: props.folder.id,
    updatedMods: modsInFolder.value.map(item => item.data),
  })
}

// Handle changes to the mods list when dragging
const handleModsChange = (event: any) => {
  console.log('Drag event:', event)

  // When items are added to this folder
  if (event.added) {
    const addedMod = event.added.element
    // Make sure we have a valid mod ID
    if (addedMod && addedMod.id) {
      emit('update-folder-mods', {
        folderId: props.folder.id,
        action: 'add',
        modId: addedMod.id,
      })
    } else if (addedMod && addedMod.data && addedMod.data.id) {
      // Fallback in case the structure is different
      emit('update-folder-mods', {
        folderId: props.folder.id,
        action: 'add',
        modId: addedMod.data.id,
      })
    }
  }

  // When items are removed from this folder
  if (event.removed) {
    const removedMod = event.removed.element
    // Make sure we have a valid mod ID
    if (removedMod && removedMod.id) {
      emit('update-folder-mods', {
        folderId: props.folder.id,
        action: 'remove',
        modId: removedMod.id,
      })
    } else if (removedMod && removedMod.data && removedMod.data.id) {
      // Fallback in case the structure is different
      emit('update-folder-mods', {
        folderId: props.folder.id,
        action: 'remove',
        modId: removedMod.data.id,
      })
    }
  }
}
</script>

<style scoped>
.folder-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
}

.folder-contents {
  margin-left: 31px;
  border-left: 2px dashed var(--theme-border);
}

.folder-item-container {
  position: relative;
  margin-bottom: 2px;
}

.folder {
  min-height: 12px;
  padding: 2px 16px !important;
  border-bottom-width: 0px;
  border-bottom-style: solid;
}

.expanded-folder {
  background-color: var(--theme-surface-light);
  border-radius: 0 1rem 0 0 !important;
  border-bottom-width: 2px;
  border-bottom-style: solid;
}

.folder-action-btn {
  opacity: 0.7;
  transition: opacity 0.2s ease;
}

.delete-btn {
  opacity: 0;
  transition: opacity 0.2s ease;
}

.q-item:hover .delete-btn,
.q-item:hover .folder-action-btn {
  opacity: 1;
}

.draggable-item {
  transition: background-color 0.2s ease;
  position: relative;
  border-radius: 0 1rem 1rem 0;
}

.draggable-item:hover {
  background-color: var(--theme-surface);
}

.draggable-item * {
  user-select: none;
}

.cursor-move {
  cursor: move;
  cursor: grab;
}

.cursor-move:active {
  cursor: grabbing;
}

.version-text {
  color: #999999 !important;
}

.sortable-ghost {
  opacity: 0;
}

/* Add styles for when an item is being dropped */
.sortable-chosen {
  background-color: var(--theme-surface) !important;
  box-shadow: 0 0 10px var(--theme-border);
  border-radius: 0 1rem 1rem 0;
  font-family: 'PhantomMuffFull';
}

/* Compact mode styles */
.compact-mode {
  padding: 4px 12px;
  min-height: 20px;
  max-width: 60px;
}

.compact-folder-contents {
  margin-left: 0px;
  border-left-style: solid;
}

/* Folder expand/collapse transitions */
.folder-expand-enter-active,
.folder-expand-leave-active {
  transition: all 0.3s ease;
  transform-origin: top;
}

.folder-expand-enter-from {
  opacity: 0;
  transform: scaleY(0);
  max-height: 0;
}

.folder-expand-leave-to {
  opacity: 0;
  transform: scaleY(0);
  max-height: 0;
}

.folder-expand-enter-to,
.folder-expand-leave-from {
  opacity: 1;
  transform: scaleY(1);
  max-height: 1000px; /* Adjust based on expected content height */
  transform-origin: top;
}
</style>
