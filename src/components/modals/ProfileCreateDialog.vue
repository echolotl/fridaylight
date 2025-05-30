<template>
  <q-dialog v-model="isOpen" persistent>
    <q-card class="profile-create-dialog phantom-font">
      <div class="profile-create">
        <q-card-section class="row items-center q-pb-none">
          <div class="text-h6 phantom-font-difficulty">
            {{ props.isCreate ? 'Create Profile' : 'Edit Profile' }}
          </div>
          <q-space />
          <q-btn icon="close" flat round dense @click="handleCancel" />
        </q-card-section>
        <q-card-section>
          <q-input
            v-model="localProfileName"
            label="Profile Name"
            class="q-mb-md"
          />

          <div class="text-subtitle2 q-mb-sm">Select Mods</div>
          <div class="mod-states-list">
            <div
              v-for="(enabled, folderPath) in localModStates"
              :key="folderPath"
              class="mod-state-item"
            >
              <div class="mod-info">
                <div class="mod-icon">
                  <q-img
                    v-if="getModByPath(folderPath)?.icon_data"
                    :src="getModByPath(folderPath)?.icon_data"
                    spinner-color="primary"
                    style="height: 32px; width: 32px"
                  />
                  <div v-else class="fallback-icon">
                    <q-icon name="image_not_supported" size="20px" />
                  </div>
                </div>
                <div class="mod-details">
                  <div class="mod-name">
                    {{ getModByPath(folderPath)?.name || 'Unknown Mod' }}
                  </div>
                  <div class="mod-path text-caption">
                    {{ folderPath.split('/').pop() }}
                  </div>
                </div>
              </div>
              <q-toggle v-model="localModStates[folderPath]" color="primary" />
            </div>
          </div>
        </q-card-section>
        <q-card-actions align="right">
          <q-btn
            v-if="!props.isCreate"
            flat
            label="Delete Profile"
            color="negative"
            @click="emit('delete', props.profile)"
          />
          <q-btn flat label="Cancel" color="primary" @click="handleCancel" />
          <q-btn flat label="Save" color="primary" @click="handleSave" />
        </q-card-actions>
      </div>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { computed, ref, watch, withDefaults } from 'vue'
import type { EngineModProfile, ModMetadataFile } from '@main-types'

interface Props {
  modelValue: boolean
  profile: EngineModProfile
  mods: ModMetadataFile[]
  isCreate?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isCreate: false,
})

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  save: [profile: EngineModProfile]
  cancel: []
  delete: [profile: EngineModProfile]
}>()

const isOpen = computed({
  get: () => props.modelValue,
  set: (value: boolean) => emit('update:modelValue', value),
})

// Create local reactive copies to avoid prop mutation
const localProfileName = ref(props.profile.name)
const localModStates = ref<Record<string, boolean>>({
  ...props.profile.mod_states,
})

// Watch for prop changes to update local state
watch(
  () => props.profile,
  newProfile => {
    localProfileName.value = newProfile.name
    localModStates.value = { ...newProfile.mod_states }
  },
  { immediate: true }
)

// Handle save with updated data
const handleSave = () => {
  const updatedProfile: EngineModProfile = {
    ...props.profile,
    name: localProfileName.value,
    mod_states: { ...localModStates.value },
    updated_at: Math.floor(Date.now() / 1000),
  }
  emit('save', updatedProfile)
}

// Helper function to find a mod by its folder path
const getModByPath = (folderPath: string): ModMetadataFile | undefined => {
  return props.mods.find(mod => mod.folder_path === folderPath)
}

// Handle cancel by closing dialog and emitting cancel event
const handleCancel = () => {
  isOpen.value = false
  emit('cancel')
}
</script>

<style scoped>
.profile-create-dialog {
  width: fit-content;
  background-color: var(--solid);
  color: var(--theme-text);
  border: var(--theme-border) 2px solid;
  backdrop-filter: blur(30px);
}

.profile-create {
  max-width: 50vw;
}

.mod-states-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.mod-state-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px;
  background: var(--theme-card);
  border-radius: 6px;
  border: 1px solid var(--theme-border);
}

.mod-info {
  display: flex;
  align-items: center;
  flex: 1;
  gap: 12px;
}

.mod-icon {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.fallback-icon {
  width: 32px;
  height: 32px;
  background: var(--theme-surface);
  border: 1px solid var(--theme-border);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--theme-text-secondary);
}

.mod-details {
  flex: 1;
}

.mod-name {
  font-weight: 500;
  color: var(--theme-text);
  margin-bottom: 2px;
}

.mod-path {
  color: var(--theme-text-secondary);
  font-size: 12px;
}
</style>
