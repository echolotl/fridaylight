<template>
  <q-item 
    :key="mod.id" 
    clickable 
    v-ripple 
    @click="$emit('select-mod', mod)"
    :active="isActive"
    active-class="active-mod"
    class="draggable-item cursor-move"
  >      
    <q-item-section avatar v-if="mod.icon_data">
      <q-avatar size="32px" square>
        <img :src="mod.icon_data" alt="mod icon" />
      </q-avatar>
    </q-item-section>
    <q-item-section avatar v-else>
      <q-avatar size="32px" icon="folder" square class="default-icon" />
    </q-item-section>
    <q-item-section>
      <q-item-label>{{ mod.name }}</q-item-label>
      <q-item-label caption class="version-text" v-if="mod.version">v{{ mod.version }}</q-item-label>
    </q-item-section>
    <q-item-section side>
      <q-btn
        flat
        round
        dense
        icon="delete"
        color="grey-5"
        @click.stop="$emit('delete-mod', mod)"
        class="delete-btn"
      />
    </q-item-section>
  </q-item>
</template>

<script setup lang="ts">
import { Mod } from '../../types';

const props = defineProps({
  mod: {
    type: Object as () => Mod,
    required: true
  },
  isActive: {
    type: Boolean,
    default: false
  }
});

defineEmits(['select-mod', 'delete-mod']);
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

.delete-btn {
  opacity: 0;
  transition: opacity 0.2s ease;
}

.q-item:hover .delete-btn {
  opacity: 1;
}

.version-text {
  color: #999999 !important;
}
</style>
