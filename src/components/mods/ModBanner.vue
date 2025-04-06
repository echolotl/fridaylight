<template>
  <div class="mod-banner">
    <!-- Banner image - user uploaded or default -->
    <div 
      class="banner-image" 
      :style="mod && mod.banner_data ? { 
        backgroundImage: `url('${mod.banner_data}')`,
        backgroundSize: 'cover',
        backgroundPosition: 'center'
      } : {}"
    ></div>
    
    <!-- Title overlay on the banner -->
    <div class="title-overlay">
      <!-- Show logo if available -->
      <div v-if="mod.logo_data" class="logo-container">
        <img :src="mod.logo_data" alt="Mod Logo" class="mod-logo" @click="$emit('open-settings')" />
      </div>
      <!-- Otherwise show editable title -->
      <div v-else class="title-container">
        <h1 
          v-if="!isEditingTitle" 
          @click="startTitleEdit" 
          class="mod-title"
        >
          {{ mod.name }}
          <q-icon name="edit" size="sm" class="edit-icon" />
        </h1>
        <q-input
          v-else
          v-model="editedTitle"
          dense
          autofocus
          class="title-input phantom-font"
          @keyup.enter="saveTitle"
          @blur="saveTitle"
          ref="titleInput"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps({
  mod: {
    type: Object,
    required: true
  }
});

const emit = defineEmits(['update:title', 'open-settings']);

const isEditingTitle = ref(false);
const editedTitle = ref('');

const startTitleEdit = () => {
  isEditingTitle.value = true;
  editedTitle.value = props.mod.name || '';
};

const saveTitle = () => {
  emit('update:title', editedTitle.value);
  isEditingTitle.value = false;
};
</script>

<style scoped>
.mod-banner {
  position: relative;
  height: 300px;
  margin-bottom: 20px;
  border-radius: 8px 8px 0 0;
  overflow: hidden;
}

.banner-image {
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, #0575E6, #021B79);
  background-image: url('/images/menuBG.png');
  background-size: cover;
  background-position: center;
  position: relative;
  mask-image: linear-gradient(to bottom, black, transparent);
}

.banner-image::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 70%;
  background: linear-gradient(to top, rgba(0, 0, 0, 0.8), transparent);
  pointer-events: none;
}

.title-overlay {
  position: absolute;
  bottom: 20px;
  left: 20px;
  right: 20px;
  color: white;
  z-index: 2;
}

.mod-title {
  font-size: 2.5rem;
  margin: 0;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.6);
  cursor: pointer;
  display: flex;
  align-items: center;
}

.edit-icon {
  margin-left: 10px;
  opacity: 0.7;
  transition: opacity 0.2s;
}

.mod-title:hover .edit-icon {
  opacity: 1;
}

.title-input {
  width: 100%;
  font-size: 2rem;
  color: white;
  background: rgba(0, 0, 0, 0.5);
  border-radius: 4px;
}

.title-container {
  display: flex;
  align-items: center;
}

.logo-container {
  display: flex;
  justify-content: flex-start;
  align-items: center;
  max-width: 80%;
}

.mod-logo {
  max-width: 100%;
  max-height: 200px;
  cursor: pointer;
  filter: drop-shadow(2px 2px 4px rgba(0, 0, 0, 0.6));
  transition: transform 0.2s ease;
}

.mod-logo:hover {
  transform: scale(1.02);
}
</style>