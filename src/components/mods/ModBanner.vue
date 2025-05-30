<template>
  <div class="mod-banner">
    <!-- Banner image -->

    <div
      class="banner-image"
      :style="
        mod && mod.banner_data
          ? {
              backgroundImage: `url('${mod.banner_data}')`,
              backgroundSize: 'cover',
              backgroundPosition: 'center',
            }
          : {}
      "
    ></div>

    <!-- Title overlay on the banner -->
    <div class="title-overlay">
      <!-- Show logo if available -->
      <div v-if="mod.logo_data" class="logo-container">
        <img
          :src="mod.logo_data"
          alt="Mod Logo"
          class="mod-logo"
          :class="{
            'logo-left-bottom':
              !mod.logo_position || mod.logo_position === 'left_bottom',
            'logo-left-middle': mod.logo_position === 'left_middle',
            'logo-middle': mod.logo_position === 'middle',
          }"
          @click="$emit('open-settings')"
        />
      </div>
      <!-- Otherwise show editable title -->
      <div v-else class="title-container">
        <h1 v-if="!isEditingTitle" class="mod-title" @click="startTitleEdit">
          {{ mod.name }}
          <q-icon name="edit" size="sm" class="edit-icon" />
        </h1>
        <q-input
          v-else
          ref="titleInput"
          v-model="editedTitle"
          dense
          autofocus
          class="title-input phantom-font"
          @keyup.enter="saveTitle"
          @blur="saveTitle"
        />
      </div>
      <!-- Show version and engine type if available -->
      <div
        v-if="
          mod.version ||
          (mod.engine && (mod.engine.engine_type || mod.engine.engine_icon))
        "
        class="mod-info-overlay"
      >
        <span v-if="mod.version" class="version-tag phantom-font-difficulty"
          ><b>v{{ mod.version }}</b></span
        >
        <span
          v-if="
            mod.engine &&
            mod.engine.engine_icon &&
            mod.engine.engine_type !== 'unknown'
          "
          class="engine-tag"
        >
          <img
            :src="mod.engine.engine_icon"
            alt="Engine Icon"
            class="custom-engine-icon"
          />
          <q-tooltip
            v-if="mod.engine.engine_name"
            class="phantom-font"
            anchor="top middle"
          >
            {{ mod.engine.engine_name }}
          </q-tooltip>
        </span>
        <span
          v-else-if="
            mod.engine &&
            mod.engine.engine_type &&
            mod.engine.engine_type !== 'unknown'
          "
          class="engine-tag"
        >
          <img
            :src="
              '/images/engine_icons/' +
              formatEngineType(mod.engine.engine_type) +
              '.webp'
            "
          />
          <q-tooltip
            v-if="mod.engine.engine_name"
            anchor="top middle"
            class="phantom-font"
            :offset="[0, 40]"
          >
            {{ mod.engine.engine_name }}
          </q-tooltip>
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Mod } from '@main-types'

const props = defineProps({
  mod: {
    type: Object as () => Mod,
    required: true,
  },
})

const emit = defineEmits(['update:title', 'open-settings'])

const isEditingTitle = ref(false)
const editedTitle = ref('')

const startTitleEdit = () => {
  isEditingTitle.value = true
  editedTitle.value = props.mod.name || ''
}

const saveTitle = () => {
  emit('update:title', editedTitle.value)
  isEditingTitle.value = false
}

const formatEngineType = (engineType: string) => {
  return engineType.charAt(0).toUpperCase() + engineType.slice(1)
}
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
  background: linear-gradient(135deg, #0575e6, #021b79);
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
  background: linear-gradient(to top, var(--theme-bg), transparent);
  pointer-events: none;
}

.title-overlay {
  position: absolute;
  bottom: 20px;
  left: 20px;
  right: 20px;
  color: var(--theme-text);
  z-index: 2;
}

.mod-title {
  font-size: 2.5rem;
  margin: 0;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.6);
  line-height: 1.2;
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
  color: var(--theme-text);
  background: var(--theme-surface);
  border-radius: 4px;
}

.title-container {
  display: flex;
  align-items: center;
}

.logo-container {
  display: relative;
  align-items: center;
  width: 100%;
  height: 300px;
}

/* Logo position styles */
.logo-left-bottom {
  position: absolute;
  bottom: 0px;
  left: 0px;
}

.logo-left-bottom:hover {
  transform: scale(1.05);
}

.logo-left-middle {
  position: absolute;
  left: 0px;
  bottom: 0px;
  transform: translate(0, -25%);
  align-items: center;
}

.logo-left-middle:hover {
  transform: translateY(-50%) scale(1.05);
}

.logo-middle {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  max-width: 50%;
  justify-content: center;
}

.logo-middle:hover {
  transform: translate(-50%, -50%) scale(1.05);
}

.mod-logo {
  max-width: 35vw;
  max-height: 200px;
  cursor: pointer;
  transition:
    transform 0.2s ease,
    filter 0.2s ease;
  transform-origin: center;
  filter: drop-shadow(0 0 5px rgba(0, 0, 0, 0));
}

.mod-logo:hover {
  filter: drop-shadow(0 0px 5px rgba(0, 0, 0, 0.5));
}

.mod-info-overlay {
  margin-top: 10px;
  display: flex;
  position: absolute;
  bottom: 10px;
  right: 10px;
  gap: 10px;
  align-items: center;
}

.engine-tag {
  color: var(--q-primary);
  border-radius: 4px;
  font-size: 0.9rem;
  object-fit: contain;
}

.engine-tag img {
  max-height: 2rem;
  width: auto;
}

.engine-tag .custom-engine-icon {
  max-height: 2rem;
  width: auto;
  object-fit: contain;
}
.version-tag {
  color: var(--q-primary);
  transform: translateY(1px);
  padding: 5px 10px;
  border-radius: 4px;
  font-size: 0.8rem;
}
</style>
