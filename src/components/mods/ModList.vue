<template>
  <div>
    <q-scroll-area style="height: 100%">
      <q-list padding class="phantom-font" style="color: var(--theme-text)">
        <q-item-label header class="flex justify-between items-center" style="color: var(--theme-text-secondary)">
          Mods
          <div class="flex sticky">
            <q-btn
              flat
              round
              dense
              icon="settings"
              @click="$emit('open-settings')"
              class="q-mr-xs"
              style="color: var(--theme-text-secondary)"
            />
            <q-btn
              flat
              round
              dense
              icon="add_circle_outline"
              @click="$emit('add-mod')"
              class="q-mr-xs"
              style="color: var(--theme-text-secondary)"
            />
            <q-btn
              flat
              round
              dense
              icon="dashboard_customize"
              class="q-mr-xs"
              @click="$emit('add-mods-folder')"
              tooltip="Import Folder of Mods"
              style="color: var(--theme-text-secondary)"
            />
            <q-btn
              flat
              round
              dense
              icon="create_new_folder"
              @click="showCreateFolderDialog = true"
              class="q-mr-xs"
              tooltip="Create Folder"
              style="color: var(--theme-text-secondary)"
            />
          </div>
        </q-item-label>

        <!-- Downloading mods section -->
        <template v-if="Object.keys(downloadingMods).length > 0">
          <q-item-label header style="color: var(--theme-text-secondary)"> Downloading </q-item-label>

          <q-item
            v-for="download in downloadingMods"
            :key="`dl-${download.modId}`"
            class="downloading-mod"
          >
            <q-item-section avatar>
              <q-spinner
                size="32px"
                v-if="!download.isComplete && !download.isError"
                color="primary"
              />
              <q-icon
                name="check_circle"
                color="positive"
                size="32px"
                v-else-if="download.isComplete"
              />
              <q-icon
                name="error"
                color="negative"
                size="32px"
                v-else-if="download.isError"
              />
            </q-item-section>

            <q-item-section>
              <q-item-label>{{ download.name }}</q-item-label>
              <q-item-label caption style="color: var(--theme-text-secondary)">{{ download.step }}</q-item-label>

              <q-linear-progress
                v-if="!download.isComplete && !download.isError"
                :value="download.percentage / 100"
                color="primary"
                class="q-mt-xs"
                rounded
                size="8px"
              />

              <q-item-label
                caption
                class="text-negative"
                v-if="download.isError"
              >
                {{ download.error }}
              </q-item-label>
            </q-item-section>
          </q-item>

          <q-separator spaced />
        </template>

        <!-- Installed mods list -->
        <q-item-label header style="color: var(--theme-text-secondary)"> Installed </q-item-label>
        <!-- Sortable Mod list items -->
        <draggable
          v-model="displayItems"
          group="mods"
          item-key="id"
          @end="onDragEnd"
          @start="onDragStart"
          class="full-width"
          :animation="200"
          ghost-class="sortable-ghost"
          chosen-class="sortable-chosen"
          drag-class="sortable-drag"
          :force-fallback="true"
          :delay="50"
          :delayOnTouchOnly="true"
        >
          <template #item="{ element: item }">
            <div>
              <!-- Show folder if it's a folder type -->
              <FolderListItem
                v-if="item.type === 'folder'"
                :folder="item.data"
                :all-mods="mods"
                :selected-mod-id="selectedModId"
                @select-mod="$emit('select-mod', $event)"
                @delete-mod="confirmDelete($event)"
                @delete-folder="confirmDeleteFolder(item.data)"
                @update-folder-mods="handleFolderModsUpdate"
                @reorder-folder-mods="handleFolderModsReorder"
              />

              <!-- Show mod if it's not in any folder and is a mod type -->
              <ModListItem
                v-else-if="item.type === 'mod' && !isModInFolder(item.data.id)"
                :mod="item.data"
                :is-active="selectedModId === item.data.id"
                @select-mod="$emit('select-mod', item.data)"
                @delete-mod="confirmDelete(item.data)"
              />
            </div>
          </template>
        </draggable>

        <!-- Empty state when no mods -->
        <q-item v-if="mods.length === 0 || displayItems.length === 0">
          <q-item-section>
            <q-item-label caption>
              No mods added. Click the + button to add a mod folder.
            </q-item-label>
          </q-item-section>
        </q-item>
      </q-list>
    </q-scroll-area>
    <!-- Delete mod confirmation dialog -->
    <MessageDialog
      v-model="showDeleteDialog"
      title="Delete Mod"
      icon="warning"
      icon-color="negative"
      confirm-label="Delete"
      confirm-color="negative"
      @confirm="deleteMod"
      v-if="modToDelete"
    >
      <div class="text-h6">{{ modToDelete?.name }}</div>
      <div class="text-caption">{{ modToDelete?.path }}</div>
      <p class="text-body2 q-mt-sm">
        This will only remove the mod from the launcher. The mod files will
        not be deleted.
      </p>
    </MessageDialog>

    <!-- Delete folder confirmation dialog -->
    <MessageDialog
      v-model="showDeleteFolderDialog"
      title="Delete Folder"
      icon="warning"
      icon-color="negative"
      confirm-label="Delete"
      confirm-color="negative"
      @confirm="deleteFolder"
      v-if="folderToDelete"
    >
      <div class="text-h6">{{ folderToDelete?.name }}</div>
      <p class="text-body2 q-mt-sm">
        This will only delete the folder. The mods inside will not be
        deleted but will return to the main mod list.
      </p>
    </MessageDialog>

    <!-- Create Folder Modal -->
    <CreateFolderModal
      v-model="showCreateFolderDialog"
      @create-folder="handleCreateFolder"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { downloadingMods } from "@stores/downloadState";
import draggable from "vuedraggable";
import ModListItem from "@mods/list/ModListItem.vue";
import FolderListItem from "@mods/list/FolderListItem.vue";
import CreateFolderModal from "@modals/CreateFolderModal.vue";
import MessageDialog from "@modals/MessageDialog.vue";
import { Mod, Folder, DisplayItem } from "@main-types";
import { v4 as uuidv4 } from "uuid";

const props = defineProps({
  mods: {
    type: Array as () => Mod[],
    required: true,
  },
  folders: {
    type: Array as () => Folder[],
    default: () => [],
  },
  selectedModId: {
    type: String,
    default: "",
  },
});

// Create a local reactive copy of the mods array for reordering
const modsList = ref<Mod[]>([]);
const foldersList = ref<Folder[]>([]);
const displayItems = ref<DisplayItem[]>([]);

// Initialize modsList and foldersList when component is first created
watch(
  () => props.mods,
  (newMods) => {
    if (newMods && newMods.length > 0) {
      console.log(
        "ModList received new mods from parent with display_order:",
        newMods.map((mod) => ({
          name: mod.name,
          display_order: mod.display_order,
        }))
      );
      
      // Ensure each mod has a valid display_order
      const modsWithValidOrder = newMods.map(mod => {
        // If display_order is undefined, null, or the problematic value 9999
        if (mod.display_order === undefined || mod.display_order === null || mod.display_order === 9999) {
          // Create a new mod with display_order set to a valid number
          // Use the index in the array as a fallback
          const index = newMods.findIndex(m => m.id === mod.id);
          return { ...mod, display_order: index >= 0 ? index : 0 };
        }
        return mod;
      });
      
      modsList.value = [...modsWithValidOrder];
    }
  },
  { immediate: true, deep: true }
);

watch(
  () => props.folders,
  (newFolders) => {
    if (newFolders && newFolders.length > 0) {
      console.log(
        "ModList received new folders from parent:",
        newFolders.map((folder) => ({
          name: folder.name,
          mods: folder.mods.length,
          display_order: folder.display_order,
        }))
      );
      
      // Ensure each folder has a valid display_order
      const foldersWithValidOrder = newFolders.map((folder, index) => {
        if (folder.display_order === undefined || folder.display_order === null || folder.display_order === 9999) {
          return { ...folder, display_order: index };
        }
        return folder;
      });
      
      foldersList.value = [...foldersWithValidOrder];
    }
  },
  { immediate: true, deep: true }
);

// Determine if a mod is inside any folder using the mod's folder_id
const isModInFolder = (modId: string) => {
  const mod = modsList.value.find((mod) => mod.id === modId);
  // Return true only if folder_id exists AND is not null/undefined/empty
  return (
    mod?.folder_id !== undefined &&
    mod?.folder_id !== null &&
    mod?.folder_id !== ""
  );
};

// Update displayItems when mods or folders change
const updateDisplayItems = () => {
  const items: DisplayItem[] = [];

  // Add folders and standalone mods
  const allItems = [
    ...foldersList.value.map((folder) => ({
      id: folder.id,
      type: "folder" as const,
      data: folder,
      display_order: folder.display_order || 0, // Ensure display_order is not undefined
    })),
    ...modsList.value
      .filter((mod) => !mod.folder_id) // Only include mods not in folders
      .map((mod) => ({
        id: mod.id,
        type: "mod" as const,
        data: mod,
        display_order: mod.display_order || 0, // Ensure display_order is not undefined
      })),
  ];

  // Sort items by display_order
  items.push(...allItems.sort((a, b) => a.display_order - b.display_order));

  displayItems.value = items;
};

// Watch for changes in mods and folders to update displayItems
watch(
  [modsList, foldersList],
  () => {
    updateDisplayItems();
  },
  { deep: true, immediate: true }
);

const emit = defineEmits([
  "select-mod",
  "add-mod",
  "add-mods-folder",
  "delete-mod",
  "delete-folder",
  "open-settings",
  "reorder-items",
  "update-folder",
  "update-mod", // Added new emit type for mod updates
  "reorder-folder-mods", // Added new emit type for reordering mods within folders
]);

const showDeleteDialog = ref(false);
const showDeleteFolderDialog = ref(false);
const modToDelete = ref<Mod | null>(null);
const folderToDelete = ref<Folder | null>(null);

const confirmDelete = (mod: Mod) => {
  modToDelete.value = mod;
  showDeleteDialog.value = true;
};

const confirmDeleteFolder = (folder: Folder) => {
  folderToDelete.value = folder;
  showDeleteFolderDialog.value = true;
};

const deleteMod = () => {
  if (modToDelete.value) {
    // Immediately remove the mod from our local arrays to prevent UI ghosting
    modsList.value = modsList.value.filter((mod) => mod.id !== modToDelete.value?.id);
    
    // Update displayItems to remove this mod
    displayItems.value = displayItems.value.filter(
      (item) => !(item.type === "mod" && item.data.id === modToDelete.value?.id)
    );
    
    // Then emit to parent to handle database deletion
    emit("delete-mod", modToDelete.value.id);
    modToDelete.value = null;
  }
};

// Create folder dialog state
const showCreateFolderDialog = ref(false);

// Handle folder creation
const handleCreateFolder = (folderData: { name: string; color: string }) => {
  // Create a new folder object with a unique ID
  const newFolder: Folder = {
    id: uuidv4(),
    name: folderData.name,
    color: folderData.color,
    mods: [],
    display_order: displayItems.value.length, // Add at the end of the list
  };

  // Update our local foldersList
  foldersList.value.push(newFolder);

  // Emit the new folder to the parent component
  emit("update-folder", newFolder);

  // Update display items
  updateDisplayItems();
};

const deleteFolder = () => {
  if (folderToDelete.value) {
    // Immediately remove the folder from our local arrays to prevent UI ghosting
    foldersList.value = foldersList.value.filter((folder) => folder.id !== folderToDelete.value?.id);
    
    // Update displayItems to remove this folder
    displayItems.value = displayItems.value.filter(
      (item) => !(item.type === "folder" && item.data.id === folderToDelete.value?.id)
    );
    
    // Then emit to parent to handle database deletion
    emit("delete-folder", folderToDelete.value.id);
    folderToDelete.value = null;
  }
};

// Function to handle when drag starts
const onDragStart = () => {
  console.log(
    "Drag started, current display items:",
    displayItems.value.map(
      (item) =>
        `${item.type}:${item.type === "mod" ? item.data.name : item.data.name}`
    )
  );
};

// Function to handle when drag ends
const onDragEnd = () => {
  console.log(
    "Drag ended, new display items order:",
    displayItems.value.map(
      (item) =>
        `${item.type}:${item.type === "mod" ? item.data.name : item.data.name}`
    )
  );

  // Update display_order for all items based on their new position
  const updatedItems = displayItems.value.map((item, index) => {
    // Create a new object to avoid reactivity issues
    const updatedItem = { ...item };

    // Update the display_order in the data object
    if (item.type === "mod") {
      updatedItem.data = { ...item.data, display_order: index };
    } else if (item.type === "folder") {
      updatedItem.data = { ...item.data, display_order: index };
    }

    // Update the display_order in the item itself
    updatedItem.display_order = index;

    return updatedItem;
  });

  // Emit the updated items with their new display orders
  emit("reorder-items", updatedItems);

  console.log("Emitted reordered items with updated display_order values");
};

// Handle updates from the folder when mods are dragged in or out
// Handle reordering of mods within a specific folder
const handleFolderModsReorder = (event: {
  folderId: string;
  updatedMods: Mod[];
}) => {
  console.log(
    `Reordering mods within folder ${event.folderId}:`,
    event.updatedMods.map((mod) => mod.name)
  );

  // Find the folder
  const folderIndex = foldersList.value.findIndex(
    (folder) => folder.id === event.folderId
  );
  if (folderIndex === -1) {
    console.error("Could not find folder with ID", event.folderId);
    return;
  }

  // Create a new folder object with updated mods array in the desired order
  // Important: preserve all original properties, especially display_order
  const updatedFolder = {
    ...foldersList.value[folderIndex],
    mods: event.updatedMods.map((mod) => mod.id),
  };

  // Update the display_order_in_folder for each mod
  event.updatedMods.forEach((mod, index) => {
    // Find the mod in modsList to update it
    const modIndex = modsList.value.findIndex((m) => m.id === mod.id);
    if (modIndex !== -1) {
      // Create a new mod object with updated display_order_in_folder
      const updatedMod = {
        ...modsList.value[modIndex],
        display_order_in_folder: index,
      };

      // Update the mod in our array
      modsList.value[modIndex] = updatedMod;

      // Emit event to update the mod in parent component
      emit("update-mod", updatedMod);
    }
  });

  // Update the folder in our array
  foldersList.value[folderIndex] = updatedFolder;

  // Emit event to update the folder in parent component
  emit("update-folder", updatedFolder);

  // Update display items
  updateDisplayItems();
};

const handleFolderModsUpdate = (event: {
  folderId: string;
  action: "add" | "remove";
  modId: string;
}) => {
  console.log("Folder mods update:", event);

  // Find the folder that needs to be updated
  const folderIndex = foldersList.value.findIndex(
    (folder) => folder.id === event.folderId
  );
  if (folderIndex === -1) return;

  const folder = { ...foldersList.value[folderIndex] };
  // Find the mod that's being updated
  const modIndex = modsList.value.findIndex((mod) => mod.id === event.modId);
  if (modIndex === -1) return;
  // Update the folder's mod list based on the action
  if (event.action === "add") {
    // Only add if it's not already in the folder
    if (!folder.mods.includes(event.modId)) {
      folder.mods.push(event.modId);

      // Get the new display_order_in_folder (use the folder's current mods length as the index)
      const newDisplayOrderInFolder = folder.mods.length - 1;

      // Update the mod's folder_id AND display_order_in_folder properties
      const updatedMod = {
        ...modsList.value[modIndex],
        folder_id: event.folderId,
        display_order_in_folder: newDisplayOrderInFolder,
      };
      const newModsList = [...modsList.value];
      newModsList[modIndex] = updatedMod;
      modsList.value = newModsList;

      // Emit the mod update to the parent component
      emit("update-mod", updatedMod);
    }
  } else if (event.action === "remove") {
    folder.mods = folder.mods.filter((id) => id !== event.modId);

    // Clear both folder_id and display_order_in_folder properties to make it show in the main list again
    const updatedMod = {
      ...modsList.value[modIndex],
      folder_id: null,
      display_order_in_folder: 0,
    };
    const newModsList = [...modsList.value];
    newModsList[modIndex] = updatedMod;
    modsList.value = newModsList;

    // Emit the mod update to the parent component
    emit("update-mod", updatedMod);
  }

  // Create a new folders array to maintain reactivity
  const updatedFolders = [...foldersList.value];
  updatedFolders[folderIndex] = folder;
  foldersList.value = updatedFolders;

  // Emit the updated folder to the parent component
  emit("update-folder", folder);

  // Update the display items to reflect the changes
  updateDisplayItems();
};
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
  cursor: move; /* fallback for older browsers */
  cursor: grab;
}

.cursor-move:active {
  cursor: grabbing;
}

/* Add styles for when an item is being dragged */
.sortable-ghost {
  background-color: var(--theme-surface) !important;
  border-radius: 0 1rem 1rem 0;
  opacity: 0.5;
}

/* Add styles for when an item is being dropped */
.sortable-chosen {
  background-color: var(--theme-surface) !important;
  box-shadow: 0 0 10px var(--theme-border);
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

.downloading-mod {
  margin-bottom: 8px;
}

.sortable-list {
  background: transparent !important;
}
.q-item__label {
  line-height: 1.2;
}
</style>
