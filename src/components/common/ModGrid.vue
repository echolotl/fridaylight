<template>
  <div>
    <div class="loading-content" v-if="loading">
      <q-spinner color="primary" size="48px" />
      <div>{{ loadingMessage }}</div>
    </div>

    <div class="no-results" v-else-if="mods.length === 0">
      <q-icon name="search_off" size="48px" />
      <div>{{ emptyMessage }}</div>
    </div>

    <div v-else class="mods-grid">
      <ModCard
        v-for="mod in mods"
        :key="mod.id"
        :mod="mod"
        @download="$emit('download', mod)"
        @showDetails="$emit('showDetails', mod.id, mod.model_name)"
      />
    </div>

    <div class="pagination" v-if="showPagination && mods.length > 0">
      <q-pagination
        v-model="currentPageModel"
        :max="totalPages"
        :min="1"
        direction-links
        boundary-links
        :input="inputPagination"
        @update:model-value="$emit('page-change', $event)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import type { GameBananaMod } from "@main-types";
import ModCard from "./ModCard.vue";

const props = defineProps({
  mods: {
    type: Array as () => GameBananaMod[],
    default: () => [],
  },
  loading: {
    type: Boolean,
    default: false,
  },
  loadingMessage: {
    type: String,
    default: "Loading mods...",
  },
  emptyMessage: {
    type: String,
    default: "No mods found",
  },
  currentPage: {
    type: Number,
    default: 1,
  },
  totalPages: {
    type: Number,
    default: 1,
  },
  showPagination: {
    type: Boolean,
    default: true,
  },
  inputPagination: {
    type: Boolean,
    default: false,
  },
});

// Use a computed property with getter/setter for the pagination model
const currentPageModel = computed({
  get: () => props.currentPage,
  set: (value) => {
    // Emit an event to update the page in the parent component
    emit("page-change", value);
  },
});

const emit = defineEmits(["download", "page-change", "showDetails"]);
</script>

<style scoped>
.mods-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
  padding: 8px;
}

.loading-content,
.no-results {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 200px;
  color: var(--theme-text-secondary);
}

.pagination {
  padding: 16px 0;
  display: flex;
  justify-content: center;
}
</style>
