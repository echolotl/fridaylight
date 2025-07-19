<template>
  <q-dialog v-model="dialogVisible" :persistent="false">
    <q-card
      class="phantom-font engine-mod-details-dialog"
      style="
        background: var(--theme-solid);
        color: var(--text-color);
        border: 2px solid var(--theme-border);
        backdrop-filter: blur(30px);
        display: flex;
        flex-direction: column;
      "
    >
      <q-card-section class="row items-center" style="flex-shrink: 0">
        <h6 class="dialog-title phantom-font-difficulty">
          {{ $t('mods.engine_mod_details.title') }}
        </h6>
        <q-space />
        <q-btn
          v-close-popup
          icon="close"
          flat
          round
          dense
          @click="dialogVisible = false"
        />
      </q-card-section>

      <q-card-section class="q-pt-none dialog-content">
        <div class="mod-details">
          <div class="mod-header">
            <div v-if="mod.icon_data" class="mod-icon-large">
              <q-img
                :src="mod.icon_data"
                spinner-color="primary"
                style="height: 64px; width: 64px"
              />
            </div>
            <div class="mod-title">
              <h5 class="q-my-none">{{ mod.name }}</h5>
              <div v-if="mod.version" class="text-caption">
                {{
                  $t('mods.engine_mod_details.version', {
                    version: mod.version,
                  })
                }}
              </div>
            </div>
          </div>

          <q-separator class="q-my-md" />

          <div v-if="mod.description" class="q-mb-md">
            <div class="text-subtitle2">
              {{ $t('mods.engine_mod_details.description') }}
            </div>
            <div v-html="mod.description"></div>
          </div>

          <div v-if="mod.homepage" class="q-mb-md">
            <div class="text-subtitle2">
              {{ $t('mods.engine_mod_details.homepage') }}
            </div>
            <a @click="openUrl(mod.homepage)">{{ mod.homepage }}</a>
          </div>

          <div
            v-if="mod.contributors && mod.contributors.length > 0"
            class="q-mb-md"
          >
            <div class="text-subtitle2">
              {{ $t('mods.engine_mod_details.contributors') }}
            </div>
            <ul>
              <li v-for="(contributor, index) in mod.contributors" :key="index">
                <div class="text-caption">
                  <a v-if="contributor.url" @click="openUrl(contributor.url)">{{
                    contributor.name
                  }}</a>
                  <span v-else>{{ contributor.name }}</span>
                  - {{ contributor.role }}
                  <span v-if="contributor.email">
                    ({{ contributor.email }})</span
                  >
                </div>
              </li>
            </ul>
          </div>

          <div v-if="mod.license" class="q-mb-md">
            <div class="text-subtitle2">
              {{ $t('mods.engine_mod_details.license') }}
            </div>
            <div>{{ mod.license }}</div>
          </div>

          <div
            v-if="mod.dependencies && Object.keys(mod.dependencies).length > 0"
            class="q-mb-md"
          >
            <div class="text-subtitle2">
              {{ $t('mods.engine_mod_details.dependencies') }}
            </div>
            <ul>
              <li
                v-for="(version, modName) in mod.dependencies"
                :key="modName"
                class="dependency-item"
              >
                <div class="dependency-info">
                  <span class="dependency-name">{{ modName }}</span>
                  <span class="dependency-version">({{ version }})</span>
                  <q-spinner
                    v-if="dependencyStates[modName]?.checking"
                    color="primary"
                    size="1em"
                  />
                  <q-icon
                    v-else-if="dependencyStates[modName]?.installed"
                    name="check_circle"
                    color="positive"
                    size="1.2em"
                  />
                  <q-icon
                    v-else-if="dependencyStates[modName]?.installed === false"
                    name="error"
                    color="negative"
                    size="1.2em"
                  />
                  <q-icon v-else name="help" color="warning" size="1.2em" />
                </div>
                <div
                  v-if="dependencyStates[modName]?.error"
                  class="text-caption text-negative"
                >
                  {{ dependencyStates[modName]?.error }}
                </div>
              </li>
            </ul>
          </div>

          <div class="text-subtitle2">
            {{ $t('mods.engine_mod_details.folder_path') }}
          </div>
          <div class="text-caption q-mb-md">{{ mod.folder_path }}</div>
        </div>
      </q-card-section>

      <q-card-actions align="right" style="flex-shrink: 0">
        <q-btn
          flat
          label="Add to List"
          color="primary"
          @click="emit('add-to-list', mod)"
        />
        <q-btn
          v-close-popup
          flat
          label="Close"
          color="primary"
          @click="dialogVisible = false"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { openUrl } from '@tauri-apps/plugin-opener'
import type { ModMetadataFile } from '@main-types'

interface DependencyState {
  checking: boolean
  installed?: boolean
  error?: string
}

interface Props {
  modelValue: boolean
  mod: ModMetadataFile
  dependencyStates: Record<string, DependencyState>
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void
  (e: 'add-to-list', value: ModMetadataFile): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const dialogVisible = computed({
  get: () => props.modelValue,
  set: (value: boolean) => emit('update:modelValue', value),
})
</script>

<style scoped>
/* Engine Mod Details Dialog Styles */
.engine-mod-details-dialog {
  width: 90vw;
  max-width: 600px;
  max-height: 80vh;
  overflow: hidden;
}

.dialog-title {
  margin: 0;
}

.dialog-content {
  flex: 1;
  min-height: 0; /* Important for flex child to shrink properly */
  overflow-y: auto;
}

.mod-details {
  max-width: 500px;
}

.mod-header {
  display: flex;
  align-items: center;
  margin-bottom: 10px;
}

.mod-icon-large {
  margin-right: 16px;
}

.mod-title {
  flex: 1;
}

.text-subtitle2 {
  font-weight: 500;
  color: var(--theme-text);
  margin-bottom: 4px;
}

.text-caption {
  font-size: 14px;
  color: var(--theme-text-secondary);
}

a {
  cursor: pointer;
}

.dependency-item {
  margin-bottom: 8px;
}

.dependency-info {
  display: flex;
  align-items: center;
  gap: 5px;
}

.dependency-name {
  font-weight: 500;
}

.dependency-version {
  color: var(--theme-text-secondary);
  font-size: 0.9em;
}
</style>
