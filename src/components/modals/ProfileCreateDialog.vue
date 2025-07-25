<template>
  <q-dialog v-model="isOpen" persistent>
    <q-card class="profile-create-dialog phantom-font">
      <div class="profile-create">
        <q-card-section class="row items-center q-pb-none">
          <div class="text-h6 phantom-font-difficulty">
            {{
              props.isCreate
                ? $t('app.modals.profile_create.title')
                : $t('app.modals.profile_create.title_edit')
            }}
          </div>
          <q-space />
          <q-btn icon="close" flat round dense @click="handleCancel" />
        </q-card-section>
        <q-scroll-area class="profile-content-scroll">
          <q-card-section>
            <q-input
              v-model="localProfileName"
              :label="$t('app.modals.profile_create.profile_name')"
              class="q-mb-md"
            />
            <div class="text-subtitle2 q-mb-sm">
              {{ $t('app.modals.profile_create.icon') }}
            </div>
            <div class="preset-icons-container">
              <div class="preset-icons">
                <q-btn
                  v-for="icon in presetIcons"
                  :key="icon"
                  flat
                  round
                  :icon="icon"
                  :class="[
                    'preset-icon-button',
                    {
                      'selected-icon':
                        selectedIcon === icon && !showCustomInput,
                    },
                  ]"
                  @click="selectPresetIcon(icon)"
                />
                <q-btn
                  flat
                  :class="[
                    'preset-icon-button custom-button',
                    { 'selected-icon': showCustomInput },
                  ]"
                  @click="toggleCustomInput"
                >
                  {{ $t('app.modals.profile_create.custom') }}
                </q-btn>
              </div>
            </div>

            <div v-if="showCustomInput" class="custom-icon-input q-mb-md">
              <q-input
                v-model="customIconText"
                label="Custom Icon Name"
                bottom-slots
                @update:model-value="updateCustomIcon"
              >
                <template #hint>
                  <div
                    class="text-caption"
                    style="color: var(--theme-text-secondary)"
                  >
                    <i18n-t
                      keypath="app.modals.profile_create.enter_a_ma_link"
                      tag="span"
                    >
                      <template #link>
                        <a
                          class="link"
                          @click="
                            openUrl(
                              'https://fonts.google.com/icons?icon.set=Material+Icons&icon.style=Filled'
                            )
                          "
                          >Material Icons</a
                        >
                      </template>
                    </i18n-t>
                  </div>
                </template>
                <template #prepend>
                  <q-icon
                    :name="customIconText"
                    style="color: var(--theme-text)"
                    class="q-ml-sm"
                    size="md"
                  ></q-icon>
                </template>
              </q-input>
            </div>

            <div class="text-subtitle2 q-mb-sm">
              {{ $t('app.modals.profile_create.select_mods') }}
            </div>
            <div class="mod-states-list">
              <div
                v-for="(_, folderPath) in localModStates"
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
                      {{
                        getModByPath(folderPath)?.name || $t('misc.unknown_mod')
                      }}
                    </div>
                    <div class="mod-path text-caption">
                      {{ folderPath.split('/').pop() }}
                    </div>
                  </div>
                </div>
                <q-toggle
                  v-model="localModStates[folderPath]"
                  color="primary"
                />
              </div>
            </div>
          </q-card-section>
        </q-scroll-area>
        <q-card-actions align="right">
          <q-btn
            v-if="!props.isCreate"
            flat
            :label="$t('app.modals.profile_create.delete_profile')"
            color="negative"
            @click="emit('delete', props.profile)"
          />
          <q-btn
            flat
            :label="$t('ui.actions.cancel')"
            color="primary"
            @click="handleCancel"
          />
          <q-btn
            flat
            :label="$t('ui.actions.save')"
            color="primary"
            @click="handleSave"
          />
        </q-card-actions>
      </div>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { computed, ref, watch, withDefaults } from 'vue'
import type { EngineModProfile, ModMetadataFile } from '@main-types'
import { openUrl } from '@tauri-apps/plugin-opener'

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
const selectedIcon = ref(props.profile.icon_data || 'image_not_supported')
const showCustomInput = ref(false)
const customIconText = ref('')

const presetIcons = [
  'extension',
  'build',
  'sentiment_satisfied_alt',
  'code',
  'speaker',
  'local_florist',
  'emoji_events',
  'mic',
  'celebration',
  'monitor_heart',
  'cabin',
  'smart_toy',
  'wallpaper',
  'storm',
  'apartment',
  'electric_bolt',
]

// Functions to handle icon selection
const selectPresetIcon = (icon: string) => {
  selectedIcon.value = icon
  showCustomInput.value = false
  customIconText.value = ''
}

const toggleCustomInput = () => {
  showCustomInput.value = true
  if (customIconText.value) {
    selectedIcon.value = customIconText.value
  }
}

const updateCustomIcon = () => {
  if (customIconText.value.trim()) {
    selectedIcon.value = customIconText.value.trim()
  }
}

// Watch for prop changes to update local state
watch(
  () => props.profile,
  newProfile => {
    localProfileName.value = newProfile.name
    localModStates.value = { ...newProfile.mod_states }
    selectedIcon.value = newProfile.icon_data || 'image_not_supported'

    // Check if the icon is a custom one (not in preset list)
    const isPresetIcon = presetIcons.includes(
      newProfile.icon_data || 'image_not_supported'
    )
    if (!isPresetIcon && newProfile.icon_data) {
      showCustomInput.value = true
      customIconText.value = newProfile.icon_data
    } else {
      showCustomInput.value = false
      customIconText.value = ''
    }
  },
  { immediate: true }
)

// Handle save with updated data
const handleSave = () => {
  const updatedProfile: EngineModProfile = {
    ...props.profile,
    name: localProfileName.value,
    mod_states: { ...localModStates.value },
    icon_data: selectedIcon.value,
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
  width: 532px;
  max-width: 90vw;
  background-color: var(--theme-solid);
  color: var(--theme-text);
  border: var(--theme-border) 2px solid;
  backdrop-filter: blur(30px);
  overflow: hidden;
  height: 100%;
  max-height: 90vh;
}

.profile-content-scroll {
  height: calc(90vh - 120px);
  width: 100%;
  flex: 1;
}

.profile-content-scroll :deep(.q-scrollarea__content) {
  padding: 0;
  width: 100%;
  max-width: 100%;
  overflow-x: hidden;
}

.profile-content-scroll :deep(.q-card__section) {
  padding: 16px;
  box-sizing: border-box;
  width: 100%;
  max-width: 100%;
  overflow-x: hidden;
}

.profile-create {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.mod-states-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 100%;
  box-sizing: border-box;
}

.preset-icons-container {
  width: 100%;
  max-width: 100%;
  overflow-x: hidden;
  box-sizing: border-box;
}

.preset-icons {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 16px;
  width: 100%;
  max-width: 100%;
  box-sizing: border-box;
}

.preset-icon-button {
  width: 48px;
  height: 48px;
  border: 2px solid var(--theme-border);
  border-radius: 8px;
  transition: all 0.2s ease;
}

.custom-button {
  width: auto;
  min-width: 104px;
  font-size: 12px;
  font-weight: 500;
}

.preset-icon-button:hover {
  border-color: var(--q-primary);
  background-color: var(--theme-surface);
}

.preset-icon-button.selected-icon {
  border-color: var(--q-primary) !important;
  background-color: var(--theme-surface);
  color: var(--q-primary) !important;
}

.mod-state-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px;
  background: var(--theme-card);
  border-radius: 6px;
  border: 1px solid var(--theme-border);
  width: 100%;
  box-sizing: border-box;
  min-width: 0;
}

.mod-info {
  display: flex;
  align-items: center;
  flex: 1;
  gap: 12px;
  min-width: 0;
  overflow: hidden;
}

.mod-icon {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
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
  min-width: 0; /* Allow flex item to shrink */
  overflow: hidden;
}

.mod-name {
  font-weight: 500;
  color: var(--theme-text);
  margin-bottom: 2px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.mod-path {
  color: var(--theme-text-secondary);
  font-size: 12px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.q-field :deep(.q-field__label) {
  color: var(--theme-text) !important;
}

.q-field.q-field--outlined :deep(.q-field__control) {
  color: var(--theme-text);
}
:deep(.q-field__native) {
  color: var(--theme-text);
}
:deep(.q-field__messages) {
  color: var(--theme-text-secondary);
}
</style>
