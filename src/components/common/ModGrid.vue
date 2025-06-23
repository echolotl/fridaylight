<template>
  <div>
    <div v-if="loading" class="mods-grid">
      <SkeletonModCard v-for="n in 15" :key="n" />
    </div>

    <div v-else-if="mods.length === 0" class="no-results">
      <q-icon name="search_off" size="48px" />
      <div>{{ emptyMessage }}</div>
    </div>

    <div v-else class="mods-grid">
      <ModCard
        v-for="mod in mods"
        :key="mod._idRow"
        :mod="mod"
        @download="$emit('download', mod)"
        @show-details="$emit('showDetails', mod._idRow, mod._sModelName)"
      />
    </div>

    <div v-if="showPagination && mods.length > 0" class="pagination">
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
import { computed } from 'vue'
import type { GBSubfeedRecord } from '@custom-types/gamebanana'
import ModCard from './ModCard.vue'
import SkeletonModCard from '@components/skeletons/SkeletonModCard.vue'

const props = defineProps({
  mods: {
    type: Array as () => GBSubfeedRecord[],
    default: () => [],
  },
  loading: {
    type: Boolean,
    default: false,
  },
  loadingMessage: {
    type: String,
    default: 'Loading mods...',
  },
  emptyMessage: {
    type: String,
    default: 'No mods found',
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
})

const emit = defineEmits(['download', 'page-change', 'showDetails'])

// Use a computed property with getter/setter for the pagination model
const currentPageModel = computed({
  get: () => props.currentPage,
  set: value => {
    // Emit an event to update the page in the parent component
    emit('page-change', value)
  },
})
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
