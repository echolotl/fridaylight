<template>
  <div>
    <q-scroll-area style="height: 100%">
      <q-list padding class="text-white phantom-font">
        <q-item-label header class="flex justify-between items-center">
          Mods
          <q-btn flat round dense icon="add" @click="$emit('add-mod')" />
        </q-item-label>
        
        <!-- Mod list items -->
        <q-item 
          v-for="mod in mods" 
          :key="mod.id" 
          clickable 
          v-ripple 
          @click="$emit('select-mod', mod)"
          :active="selectedModId === mod.id"
          active-class="active-mod"
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
            <q-item-label caption v-if="mod.version">v{{ mod.version }}</q-item-label>
          </q-item-section>
        </q-item>
        
        <!-- Empty state when no mods -->
        <q-item v-if="mods.length === 0">
          <q-item-section>
            <q-item-label caption>
              No mods added. Click the + button to add a mod folder.
            </q-item-label>
          </q-item-section>
        </q-item>
      </q-list>
    </q-scroll-area>
  </div>
</template>

<script setup lang="ts">
defineProps({
  mods: {
    type: Array,
    required: true
  },
  selectedModId: {
    type: String,
    default: ''
  }
});

defineEmits(['select-mod', 'add-mod']);
</script>

<style scoped>
.active-mod {
  background-color: rgba(0, 0, 0, 0.25);
  border-radius: 0 1rem 1rem 0;
}

.default-icon {
  color: rgba(255, 255, 255, 0.7);
  background-color: transparent;
  border-radius: 4px;
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
</style>