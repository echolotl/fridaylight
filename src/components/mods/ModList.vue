<template>
  <div>
    <q-scroll-area style="height: 100%">
      <q-list padding class="phantom-font" style="color: var(--theme-text)">
        <q-item-label
          header
          class="flex justify-between items-center"
          style="color: var(--theme-text-secondary)"
          :class="{ 'compact-mode': compactMode }"
        >
          <div v-if="!compactMode">Mods</div>
          <div class="flex items-center">
            <div class="flex sticky">
              <q-btn
                flat
                round
                dense
                icon="settings"
                @click="$emit('open-settings')"
                class="q-mr-xs"
                style="color: var(--theme-text-secondary)"
              >
                <q-tooltip
                  anchor="bottom middle"
                  :offset="[0, 10]"
                  class="phantom-font"
                >
                  Settings
                </q-tooltip>
              </q-btn>
              <q-btn
                flat
                round
                dense
                icon="add_circle_outline"
                @click="$emit('add-mod')"
                class="q-mr-xs"
                style="color: var(--theme-text-secondary)"
              >
                <q-tooltip
                  anchor="bottom middle"
                  :offset="[0, 10]"
                  class="phantom-font"
                >
                  Add Mod
                </q-tooltip>
              </q-btn>
              <q-btn
                flat
                round
                dense
                icon="dashboard_customize"
                class="q-mr-xs"
                @click="$emit('add-mods-folder')"
                tooltip="Import Folder of Mods"
                style="color: var(--theme-text-secondary)"
              >
                <q-tooltip
                  anchor="bottom middle"
                  :offset="[0, 10]"
                  class="phantom-font"
                >
                  Bulk Add Mods
                </q-tooltip>
              </q-btn>
              <q-btn
                flat
                round
                dense
                icon="create_new_folder"
                @click="showCreateFolderDialog = true"
                class="q-mr-xs"
                tooltip="Create Folder"
                style="color: var(--theme-text-secondary)"
                v-if="sortBy === 'default' && !searchQuery.trim()"
              >
                <q-tooltip
                  anchor="bottom middle"
                  :offset="[0, 10]"
                  class="phantom-font"
                >
                  Create Folder
                </q-tooltip>
              </q-btn>
            </div>
          </div>
          <q-input
            v-if="!compactMode"
            v-model="searchQuery"
            dense
            placeholder="Search mods"
            class="search-input full-width"
            rounded
          >
            <template v-slot:prepend>
              <q-icon name="search" color="var(--theme-text-secondary)" />
            </template>
          </q-input>
        </q-item-label>

        <!-- Home button -->
        <q-item
          clickable
          v-ripple
          dense
          @click="$emit('go-home')"
          class="home-button q-my-sm"
          :active="activePage === 'home'"
          active-class="active-home"
          :class="{
            'compact-mode': compactMode
          }"
        >
          <q-item-section avatar>
            <q-icon name="home" style="margin-left: 4px" />
          </q-item-section>
          <q-item-section>Home</q-item-section>
        </q-item>

        <!-- Downloading mods section -->
        <template v-if="Object.keys(downloadingMods).length > 0">
          <q-item-label
            header
            style="color: var(--theme-text-secondary)"
            v-if="!compactMode"
          >
            Downloading
          </q-item-label>
          <q-separator spaced v-else />

          <DownloadListItem
            v-for="[id, download] in Object.entries(downloadingMods)"
            :key="id"
            :download="download"
            :compact-mode="compactMode"
          />

          <q-separator spaced v-if="!compactMode" />
        </template>

        <!-- Search results when search is active -->
        <template v-if="searchQuery.trim() !== ''">
          <q-item-label header style="color: var(--theme-text-secondary)">
            Search Results
          </q-item-label>

          <!-- Display search results -->
          <div v-if="filteredMods.length > 0">
            <ModListItem
              v-for="mod in filteredMods"
              :key="mod.id"
              :mod="mod"
              :is-active="selectedModId === mod.id"
              :compact-mode="compactMode"
              @select-mod="$emit('select-mod', mod)"
              @delete-mod="confirmDelete(mod)"
              @super-delete-mod="confirmSuperDelete(mod)"
              @open-mod-settings="openModSettings"
              @launch-mod="$emit('launch-mod', $event)"
              @open-mod-folder="$emit('open-mod-folder', mod)"
            />
          </div>

          <!-- No results message -->
          <q-item v-else>
            <q-item-section>
              <q-item-label caption style="color: var(--theme-text-secondary)">
                No mods found matching "{{ searchQuery }}"
              </q-item-label>
            </q-item-section>
          </q-item>
        </template>

        <!-- Installed mods list - show only when not searching -->
        <template v-else>
          <q-item-label
            header
            style="color: var(--theme-text-secondary)"
            v-if="!compactMode"
            class="cursor-pointer"
            @click="showSortMenu"
          >
            {{ sortHeaderText }} <q-icon name="arrow_drop_down" />
          </q-item-label>
          <q-separator spaced v-else />

          <!-- Show a regular list (non-draggable) when sorting is applied -->
          <template v-if="sortBy !== 'default'">
            <ModListItem
              v-for="item in sortedDisplayItems"
              :key="item.id"
              :mod="item.data as Mod"
              :is-active="selectedModId === item.data.id"
              :compact-mode="compactMode"
              @select-mod="$emit('select-mod', item.data as Mod)"
              @delete-mod="confirmDelete(item.data as Mod)"
              @super-delete-mod="confirmSuperDelete(item.data as Mod)"
              @open-mod-settings="openModSettings"
              @launch-mod="$emit('launch-mod', $event)"
              @open-mod-folder="$emit('open-mod-folder', item.data as Mod)"
            />
          </template>

          <!-- Use draggable when no sorting is applied -->
          <draggable
            v-else
            v-model="displayItems"
            group="mods"
            item-key="id"
            @end="onDragEnd"
            @start="onDragStart"
            class="full-width"
            ghost-class="sortable-ghost"
            chosen-class="sortable-chosen"
            drag-class="sortable-drag"
            :force-fallback="true"
            :delay="100"
          >
            <template #item="{ element: item }">
              <div>
                <!-- Show folder if it's a folder type -->
                <FolderListItem
                  v-if="item.type === 'folder'"
                  :folder="item.data"
                  :all-mods="mods"
                  :selected-mod-id="selectedModId"
                  :compact-mode="compactMode"
                  @select-mod="$emit('select-mod', $event)"
                  @delete-mod="confirmDelete($event)"
                  @delete-folder="confirmDeleteFolder(item.data)"
                  @update-folder-mods="handleFolderModsUpdate"
                  @reorder-folder-mods="handleFolderModsReorder"
                  @open-mod-settings="openModSettings"
                  @edit-folder="editFolder"
                  @launch-mod="$emit('launch-mod', $event)"
                />

                <!-- Show mod if it's not in any folder and is a mod type -->
                <ModListItem
                  v-else-if="
                    item.type === 'mod' && !isModInFolder(item.data.id)
                  "
                  :mod="item.data"
                  :is-active="selectedModId === item.data.id"
                  :compact-mode="compactMode"
                  @select-mod="$emit('select-mod', item.data)"
                  @delete-mod="confirmDelete(item.data)"
                  @super-delete-mod="confirmSuperDelete(item.data)"
                  @open-mod-settings="openModSettings"
                  @launch-mod="$emit('launch-mod', $event)"
                  @open-mod-folder="$emit('open-mod-folder', item.data as Mod)"
                />
              </div>
            </template>
          </draggable>

          <!-- Empty state when no mods -->
          <q-item v-if="mods.length === 0 || displayItems.length === 0">
            <q-item-section>
              <q-item-label caption style="color: var(--theme-text-secondary)">
                No mods added. Click the + button to add a mod folder.
              </q-item-label>
            </q-item-section>
          </q-item>
        </template>
      </q-list>
    </q-scroll-area>
    <!-- Delete mod confirmation dialog -->
    <MessageDialog
      v-model="showDeleteDialog"
      title="Remove Mod"
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
        Are you sure you want to remove this mod from Fridaylight? The mod files
        will remain on your system.
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
        This will only delete the folder. The mods inside will not be deleted
        but will return to the main mod list.
      </p>
    </MessageDialog>

    <!-- Super Delete mod confirmation dialog -->
    <MessageDialog
      v-model="showSuperDeleteDialog"
      title="Delete Mod"
      icon="delete_forever"
      icon-color="negative"
      confirm-label="Super Delete"
      confirm-color="negative"
      @confirm="superDeleteMod"
      v-if="modToSuperDelete"
    >
      <div class="text-h6">{{ modToSuperDelete?.name }}</div>
      <div class="text-caption">{{ modToSuperDelete?.path }}</div>
      <p class="text-body2 q-mt-sm">
        This will PERMANENTLY DELETE the mod folder and all its contents from
        your computer. This action cannot be undone!
      </p>
    </MessageDialog>

    <!-- Create Folder Modal -->
    <CreateFolderModal
      v-model="showCreateFolderDialog"
      @create-folder="handleCreateFolder"
    />

    <!-- Edit Folder Modal -->
    <EditFolderModal
      v-model="showEditFolderDialog"
      :folder="folderToEdit"
      @save-folder="handleSaveFolder"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { downloadingMods } from "@stores/downloadState";
import draggable from "vuedraggable";
import ModListItem from "@mods/list/ModListItem.vue";
import FolderListItem from "@mods/list/FolderListItem.vue";
import DownloadListItem from "@mods/list/DownloadListItem.vue";
import CreateFolderModal from "@modals/CreateFolderModal.vue";
import EditFolderModal from "@modals/EditFolderModal.vue";
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
  compactMode: {
    type: Boolean,
    default: false,
  },
  activePage: {
    type: String,
    default: "mods",
  },
});

// Create a local reactive copy of the mods array for reordering
const modsList = ref<Mod[]>([]);
const foldersList = ref<Folder[]>([]);
const displayItems = ref<DisplayItem[]>([]);
const searchQuery = ref("");

// Create folder dialog state
const showCreateFolderDialog = ref(false);
const showEditFolderDialog = ref(false);
const folderToEdit = ref<Folder | null>(null);

// Sorting state
const sortBy = ref("default");
const sortDirection = ref("asc");
const sortHeaderText = computed(() => {
  switch (sortBy.value) {
    case "name":
      return "Name";
    case "date_added":
      return "Date Added";
    case "last_played":
      return "Last Played";
    default:
      return "Installed";
  }
});

// Show sort options dropdown
const showSortMenu = (event: MouseEvent) => {
  // Prevent the default click behavior
  event.preventDefault();
  event.stopPropagation();

  // Create options for the context menu
  const sortOptions = [
    {
      icon: "sort",
      label: "Default Order",
      action: () => setSortOption("default"),
    },
    {
      icon: "sort_by_alpha",
      label: "Sort by Name",
      action: () => setSortOption("name"),
    },
    {
      icon: "date_range",
      label: "Sort by Date Added",
      action: () => setSortOption("date_added"),
    },
    {
      icon: "play_circle",
      label: "Sort by Last Played",
      action: () => setSortOption("last_played"),
    },
    { separator: true },
    {
      icon: sortDirection.value === "asc" ? "arrow_upward" : "arrow_downward",
      label: sortDirection.value === "asc" ? "Ascending" : "Descending",
      action: () => toggleSortDirection(),
    },
  ];

  // Create and dispatch the context menu event
  const customEvent = new CustomEvent("show-context-menu", {
    detail: {
      position: { x: event.clientX, y: event.clientY },
      options: sortOptions,
    },
    bubbles: true,
  });

  if (event.target) {
    event.target.dispatchEvent(customEvent);
  } else {
    document.dispatchEvent(customEvent);
  }
};

// Set the sort option
const setSortOption = (option: string) => {
  sortBy.value = option;
  // Reset to default order when selecting default
  if (option === "default") {
    updateDisplayItems();
  }
};

// Toggle sort direction between ascending and descending
const toggleSortDirection = () => {
  sortDirection.value = sortDirection.value === "asc" ? "desc" : "asc";
};

const sortedDisplayItems = computed(() => {
  if (sortBy.value === "default") {
    return displayItems.value;
  }

  // Filter out only mods (we don't sort folders)
  const onlyMods = displayItems.value.filter((item) => item.type === "mod");

  // Sort the mods based on the selected criteria
  const sortedMods = [...onlyMods].sort((a, b) => {
    let comparison = 0;

    if (sortBy.value === "name") {
      // Sort by name (alphabetically)
      comparison = a.data.name.localeCompare(b.data.name);
    } else if (sortBy.value === "date_added") {
      // Sort by date_added (if available, otherwise by ID which is usually chronological)
      // Type casting to ensure TypeScript knows we're only dealing with mod data
      const modA = a.data as Mod;
      const modB = b.data as Mod;
      const dateA = modA.date_added ? modA.date_added : 0;
      const dateB = modB.date_added ? modB.date_added : 0;
      // Default sort should be newest first (descending), so we use B - A
      comparison = dateB - dateA;
    } else if (sortBy.value === "last_played") {
      // Sort by last_played timestamp (if available)
      const modA = a.data as Mod;
      const modB = b.data as Mod;
      const lastPlayedA = modA.last_played ? modA.last_played : 0;
      const lastPlayedB = modB.last_played ? modB.last_played : 0;
      // Default sort should be most recently played first (descending), so we use B - A
      comparison = lastPlayedB - lastPlayedA;
    }
    // For date_added and last_played, we've already set the default comparison to be newest/most recent first
    // So we invert the sort direction logic for those fields
    if (sortBy.value === "date_added" || sortBy.value === "last_played") {
      return sortDirection.value === "desc" ? comparison : -comparison;
    } else {
      // For other fields like name, use the normal sort direction
      return sortDirection.value === "asc" ? comparison : -comparison;
    }
  });

  return sortedMods;
});

// Get all mods, including those in folders
const getAllMods = () => {
  return modsList.value;
};

// Filter mods based on search query
const filteredMods = computed(() => {
  if (!searchQuery.value.trim()) {
    return getAllMods();
  }

  const query = searchQuery.value.toLowerCase().trim();
  return getAllMods().filter(
    (mod) =>
      mod.name.toLowerCase().includes(query) ||
      (mod.description && mod.description.toLowerCase().includes(query))
  );
});

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
      const modsWithValidOrder = newMods.map((mod) => {
        // If display_order is undefined, null, or the problematic value 9999
        if (
          mod.display_order === undefined ||
          mod.display_order === null ||
          mod.display_order === 9999
        ) {
          // Create a new mod with display_order set to a valid number
          // Use the index in the array as a fallback
          const index = newMods.findIndex((m) => m.id === mod.id);
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
        if (
          folder.display_order === undefined ||
          folder.display_order === null ||
          folder.display_order === 9999
        ) {
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
  "update-mod",
  "reorder-folder-mods",
  "open-mod-settings",
  "launch-mod",
  "super-delete-mod",
  "open-mod-folder",
  "go-home",
]);

const showDeleteDialog = ref(false);
const showDeleteFolderDialog = ref(false);
const showSuperDeleteDialog = ref(false);
const modToDelete = ref<Mod | null>(null);
const modToSuperDelete = ref<Mod | null>(null);
const folderToDelete = ref<Folder | null>(null);

const confirmDelete = (mod: Mod) => {
  modToDelete.value = mod;
  showDeleteDialog.value = true;
};

const confirmSuperDelete = (mod: Mod) => {
  modToSuperDelete.value = mod;
  showSuperDeleteDialog.value = true;
};

const confirmDeleteFolder = (folder: Folder) => {
  folderToDelete.value = folder;
  showDeleteFolderDialog.value = true;
};

const deleteMod = () => {
  if (modToDelete.value) {
    // Immediately remove the mod from our local arrays to prevent UI ghosting
    modsList.value = modsList.value.filter(
      (mod) => mod.id !== modToDelete.value?.id
    );

    // Update displayItems to remove this mod
    displayItems.value = displayItems.value.filter(
      (item) => !(item.type === "mod" && item.data.id === modToDelete.value?.id)
    );

    // Then emit to parent to handle database deletion
    emit("delete-mod", modToDelete.value.id);
    modToDelete.value = null;
  }
};

const superDeleteMod = () => {
  if (modToSuperDelete.value) {
    // Get the ID before deleting the reference
    const modId = modToSuperDelete.value.id;

    // We'll start by removing the mod from our local arrays to update the UI
    modsList.value = modsList.value.filter((mod) => mod.id !== modId);

    // Update displayItems to remove this mod
    displayItems.value = displayItems.value.filter(
      (item) => !(item.type === "mod" && item.data.id === modId)
    );

    // Then emit to parent to handle the super delete (physical file deletion)
    emit("super-delete-mod", modId);

    // Clean up the reference
    modToSuperDelete.value = null;
  }
};

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
    foldersList.value = foldersList.value.filter(
      (folder) => folder.id !== folderToDelete.value?.id
    );

    // Update displayItems to remove this folder
    displayItems.value = displayItems.value.filter(
      (item) =>
        !(item.type === "folder" && item.data.id === folderToDelete.value?.id)
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

  // Update all mods with their new display_order_in_folder values (locally)
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
    }
  });

  // Update the folder in our array
  foldersList.value[folderIndex] = updatedFolder;

  // Emit event to update the folder and all mods in a batch operation
  emit("reorder-folder-mods", {
    folderId: event.folderId,
    updatedMods: event.updatedMods,
  });

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

// Handle saving an edited folder
const handleSaveFolder = (folderData: {
  id: string;
  name: string;
  color: string;
}) => {
  // Find the folder index
  const folderIndex = foldersList.value.findIndex(
    (folder) => folder.id === folderData.id
  );
  if (folderIndex === -1) return;

  // Create an updated folder object while preserving all existing properties
  const updatedFolder = {
    ...foldersList.value[folderIndex],
    name: folderData.name,
    color: folderData.color,
  };

  // Update our local folders list
  const newFoldersList = [...foldersList.value];
  newFoldersList[folderIndex] = updatedFolder;
  foldersList.value = newFoldersList;

  // Emit the updated folder to the parent component
  emit("update-folder", updatedFolder);

  // Update display items to reflect the changes
  updateDisplayItems();
};

// Handle context menu actions
const openModSettings = (mod: Mod) => {
  // This will forward the mod to the parent component which should handle opening the settings modal
  emit("open-mod-settings", mod);
};

// Handle edit folder action from context menu
const editFolder = (folder: Folder) => {
  // Set the folder to edit and open the edit folder modal
  folderToEdit.value = folder;
  showEditFolderDialog.value = true;
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

.compact-mode {
  padding: 4px 12px;
  min-height: 40px;
  position: relative;
  max-width: 60px;
}

.search-input {
  width: 100%;
  margin-top: 16px;
}

.search-input :deep(.q-field__control) {
  background-color: var(--theme-surface);
  border-radius: 1rem;
}

.search-input :deep(.q-field__native) {
  color: var(--theme-text);
}
.search-input :deep(.q-field__marginal) {
  color: var(--theme-text-secondary);
  padding: 0 8px;
}

.home-button {
  margin: 16px 0;
  border-radius: 0 1rem 1rem 0;
  transition: all 0.2s ease;
  background-color: none;
  color: var(--theme-text-secondary);
}

.home-button.compact-mode {
  
}

.active-home {
  background-color: var(--theme-surface);
  border-radius: 0 1rem 1rem 0;
}
</style>
