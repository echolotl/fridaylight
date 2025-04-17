<template>
  <q-item 
    :key="folder.id" 
    clickable 
    v-ripple 
    @click="toggleExpanded"
    class="draggable-item cursor-move folder"
    :class="{ 'expanded-folder': isExpanded }"
    :style="{ borderBottomColor: isExpanded ? folder.color : 'transparent' }"
  >      
    <q-item-section avatar>
      <q-avatar size="32px" square class="folder-icon">
        <q-icon name="folder" size="24px" :style="{ color: folder.color}" />
      </q-avatar>
    </q-item-section>
    <q-item-section>
      <q-item-label>{{ folder.name }}</q-item-label>
    </q-item-section>
    <q-item-section side>
      <div class="row items-center">
        <q-btn
          flat
          round
          dense
          :icon="isExpanded ? 'expand_less' : 'expand_more'"
          style="color: var(--theme-text-secondary)"
          @click.stop="toggleExpanded"
          class="folder-action-btn q-mr-xs"
        />
        <q-btn
          flat
          round
          dense
          icon="delete"
          style="color: var(--theme-text-secondary)"
          @click.stop="$emit('delete-folder', folder)"
          class="delete-btn"
        />
      </div>
    </q-item-section>
  </q-item>
  <!-- Expanded folder content with indentation -->
  <div v-if="isExpanded" class="folder-contents" :style="{ borderLeftColor: folder.color}">            
    <draggable 
            :group="{ name: 'mods', pull: true, put: true }" 
            :list="modsInFolder" 
            item-key="id" 
            @change="handleModsChange"
            @end="handleDragEnd"
            :animation="200" 
            ghost-class="sortable-ghost" 
            chosen-class="sortable-chosen"
            drag-class="sortable-drag" 
            :force-fallback="true" 
            :fallback-on-body="true"
            :delay="50" 
            :delayOnTouchOnly="true">
            <template #item="{ element }">
                <div class="draggable-item cursor-move" :key="element.id">
                    <ModListItem 
                        :mod="element.data" 
                        :is-active="selectedModId === element.id"
                        @select-mod="$emit('select-mod', element.data)" 
                        @delete-mod="$emit('delete-mod', element)" />
                </div>
</template>
</draggable>
</div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import ModListItem from '@mods/list/ModListItem.vue';
import draggable from 'vuedraggable';
import { Mod, Folder } from '@main-types';

const props = defineProps({
  folder: {
    type: Object as () => Folder,
    required: true
  },
  allMods: {
    type: Array as () => Mod[],
    required: true
  },
  selectedModId: {
    type: String,
    default: ''
  }
});

const isExpanded = ref(false);

// Compute the list of mods that belong to this folder
const modsInFolder = computed(() => {
  return props.allMods
    // Double check both the folder.mods array AND the mod's folder_id to avoid ghost items
    .filter(mod => 
      props.folder.mods.includes(mod.id) && 
      mod.folder_id === props.folder.id
    )
    .sort((a, b) => {
      // Sort by display_order_in_folder if available
      const orderA = a.display_order_in_folder ?? 999;
      const orderB = b.display_order_in_folder ?? 999;
      return orderA - orderB;
    })
    .map(mod => ({
      id: mod.id,
      data: mod  // Ensure each mod has a data property for ModListItem
    }));
});

// Toggle folder expanded state
const toggleExpanded = () => {
  isExpanded.value = !isExpanded.value;
};

const emit = defineEmits(['select-mod', 'delete-mod', 'delete-folder', 'update-folder-mods', 'reorder-folder-mods']);

// Handle drag end to update mod order within folder
const handleDragEnd = () => {
  console.log('Reordering mods within folder:', props.folder.name);
  
  // Emit an event to update the order
  emit('reorder-folder-mods', {
    folderId: props.folder.id,
    updatedMods: modsInFolder.value.map(item => item.data)
  });
};

// Handle changes to the mods list when dragging
const handleModsChange = (event: any) => {
  console.log('Drag event:', event);
  
  // When items are added to this folder
  if (event.added) {
    const addedMod = event.added.element;
    // Make sure we have a valid mod ID
    if (addedMod && addedMod.id) {
      emit('update-folder-mods', {
        folderId: props.folder.id,
        action: 'add',
        modId: addedMod.id
      });
    } else if (addedMod && addedMod.data && addedMod.data.id) {
      // Fallback in case the structure is different
      emit('update-folder-mods', {
        folderId: props.folder.id,
        action: 'add',
        modId: addedMod.data.id
      });
    }
  }
  
  // When items are removed from this folder
  if (event.removed) {
    const removedMod = event.removed.element;
    // Make sure we have a valid mod ID
    if (removedMod && removedMod.id) {
      emit('update-folder-mods', {
        folderId: props.folder.id,
        action: 'remove',
        modId: removedMod.id
      });
    } else if (removedMod && removedMod.data && removedMod.data.id) {
      // Fallback in case the structure is different
      emit('update-folder-mods', {
        folderId: props.folder.id,
        action: 'remove',
        modId: removedMod.data.id
      });
    }
  }
};
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
.folder {
  min-height: 12px;
  padding: 2px 16px !important
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
</style>
