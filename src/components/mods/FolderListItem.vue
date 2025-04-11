<template>
  <q-item 
    :key="folder.id" 
    clickable 
    v-ripple 
    @click="toggleExpanded"
    class="draggable-item cursor-move"
    :class="{ 'expanded-folder': isExpanded }"
  >      
    <q-item-section avatar>
      <q-avatar size="32px" square :style="{ backgroundColor: folder.color }" class="folder-icon">
        <q-icon name="folder" size="24px" color="white" />
      </q-avatar>
    </q-item-section>
    <q-item-section>
      <q-item-label>{{ folder.name }}</q-item-label>
      <q-item-label caption class="version-text">{{ modsInFolder.length }} mods</q-item-label>
    </q-item-section>
    <q-item-section side>
      <div class="row items-center">
        <q-btn
          flat
          round
          dense
          :icon="isExpanded ? 'expand_less' : 'expand_more'"
          color="grey-5"
          @click.stop="toggleExpanded"
          class="folder-action-btn q-mr-xs"
        />
        <q-btn
          flat
          round
          dense
          icon="delete"
          color="grey-5"
          @click.stop="$emit('delete-folder', folder)"
          class="delete-btn"
        />
      </div>
    </q-item-section>
  </q-item>

  <!-- Expanded folder content with indentation -->
  <div v-if="isExpanded" class="folder-contents">    <draggable
    group="mods"
    :list="modsInFolder"
    item-key="id"
    @change="handleModsChange">
    <template #item="{ element }">
      <div class="draggable-item" :key="element.id">
        <ModListItem 
          :mod="element.data" 
          :is-active="selectedModId === element.id"
          @select-mod="$emit('select-mod', element.data)"
          @delete-mod="$emit('delete-mod', element)"
        />
      </div>


</template>
</draggable>
</div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import ModListItem from './ModListItem.vue';
import draggable from 'vuedraggable';

interface Engine {
  engine_type: string;
  engine_name: string;
  engine_icon: string;
  mods_folder: boolean;
  mods_folder_path: string;
}

interface Mod {
  id: string;
  name: string;
  path: string;
  executable_path?: string;
  icon_data?: string;
  banner_data?: string;
  logo_data?: string | null;
  version?: string;
  engine_type?: string;
  engine: Engine;
  display_order?: number;
}

interface Folder {
  id: string;     // Unique ID for the folder
  mods: string[]; // Array of mod IDs
  color: string;  // Color for the folder icon
  name: string;   // Name of the folder
}

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
    .filter(mod => props.folder.mods.includes(mod.id))
    .map(mod => ({
      id: mod.id,
      data: mod  // Ensure each mod has a data property for ModListItem
    }));
});

// Toggle folder expanded state
const toggleExpanded = () => {
  isExpanded.value = !isExpanded.value;
};

const emit = defineEmits(['select-mod', 'delete-mod', 'delete-folder', 'update-folder-mods']);

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
  padding-left: 20px;
  margin-left: 16px;
  border-left: 2px solid var(--theme-border);
}

.folder-item-container {
  position: relative;
  margin-bottom: 2px;
}

.expanded-folder {
  background-color: var(--theme-surface-light);
  border-radius: 0 1rem 0 0;
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
