<template>
  <q-dialog v-model="showModal" persistent>
    <q-card class="folder-modal phantom-font">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6 phantom-font-difficulty">
          {{ $t('ui.actions.edit_folder') }}
        </div>
        <q-space />
        <q-btn v-close-popup icon="close" flat round dense />
      </q-card-section>

      <q-card-section>
        <q-input
          ref="nameInput"
          v-model="folderName"
          :label="$t('app.modals.folders.folder_name')"
          outlined
          class="q-mb-md"
          :rules="[
            val => !!val || $t('app.modals.folders.folder_name_required'),
          ]"
        />
        <div class="text-subtitle2 q-mb-sm">
          {{ $t('app.modals.folders.folder_color') }}
        </div>
        <div class="color-row q-mb-md">
          <q-btn
            v-for="color in folderColors"
            :key="color.value"
            round
            flat
            :style="{ backgroundColor: color.value }"
            class="color-button"
            :class="{ 'color-selected': selectedColor === color.value }"
            @click="selectPresetColor(color.value)"
          /><q-btn
            round
            flat
            icon="colorize"
            class="color-button custom-color-btn"
            :class="{ 'color-selected': isCustomColor }"
            :style="
              isCustomColor
                ? { backgroundColor: customColor }
                : { backgroundColor: 'transparent' }
            "
            @click="openColorPicker"
          />
          <input
            ref="colorPickerInput"
            v-model="customColor"
            type="color"
            class="hidden-color-picker"
            @change="selectCustomColor"
          />
        </div>
      </q-card-section>

      <q-card-actions align="right">
        <q-btn
          v-close-popup
          flat
          label="Cancel"
          color="primary"
          @click="cancel"
        />
        <q-btn
          flat
          label="Save"
          color="primary"
          :disabled="!isValid"
          @click="saveFolder"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { ref, computed, nextTick, watch } from 'vue'
import { Folder } from '@main-types'
import { useI18n } from 'vue-i18n'

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
  folder: {
    type: Object as () => Folder | null,
    default: null,
  },
})

const emit = defineEmits(['update:modelValue', 'save-folder'])

const { t } = useI18n()
const showModal = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
})

const folderName = ref('')
const selectedColor = ref('#DB2955') // Default color
const customColor = ref('#DB2955')
const isCustomColor = ref(false)
const nameInput = ref<HTMLInputElement | null>(null)
const colorPickerInput = ref<HTMLInputElement | null>(null)

const folderColors = [
  { label: t('misc.colors.pink'), value: '#DB2955' },
  { label: t('misc.colors.blue'), value: '#235789' },
  { label: t('misc.colors.green'), value: '#35CE8D' },
  { label: t('misc.colors.purple'), value: '#C490D1' },
  { label: t('misc.colors.orange'), value: '#FE621D' },
  { label: t('misc.colors.yellow'), value: '#f7c548' },
  { label: t('misc.colors.red'), value: '#C03221' },
  { label: t('misc.colors.cyan'), value: '#39A9DB' },
]

// Validate input
const isValid = computed(() => {
  return folderName.value.trim() !== ''
})

// Load folder data when the modal opens and folder prop changes
watch(
  [() => props.modelValue, () => props.folder],
  ([isOpen, folder]) => {
    if (isOpen && folder) {
      nextTick(() => {
        // Initialize values with the current folder data
        folderName.value = folder.name
        selectedColor.value = folder.color
        customColor.value = folder.color

        // Check if it's a preset color or custom
        isCustomColor.value = !folderColors.some(
          color => color.value === folder.color
        )

        // Focus the input
        if (nameInput.value) {
          nameInput.value.focus()
        }
      })
    }
  },
  { immediate: true }
)

const saveFolder = () => {
  if (isValid.value && props.folder) {
    emit('save-folder', {
      id: props.folder.id,
      name: folderName.value.trim(),
      color: selectedColor.value,
    })
    showModal.value = false
  }
}

const openColorPicker = () => {
  if (colorPickerInput.value) {
    colorPickerInput.value.click()
  }
}

const selectCustomColor = () => {
  selectedColor.value = customColor.value
  isCustomColor.value = true
}

const selectPresetColor = (color: string) => {
  selectedColor.value = color
  isCustomColor.value = false
}

const cancel = () => {
  showModal.value = false
}
</script>

<style scoped>
.folder-modal {
  width: 400px;
  background-color: var(--theme-solid);
  color: var(--theme-text);
  border: var(--theme-border) 2px solid;
  backdrop-filter: blur(30px);
}

.color-row {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.color-button {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  transition: all 0.2s ease;
}

.color-button:hover {
  transform: scale(1.1);
  box-shadow: 0 0 0 2px var(--theme-border);
}

.color-selected {
  transform: scale(1.1);
  box-shadow: 0 0 0 3px var(--theme-border) !important;
}

.custom-color-btn {
  background-color: white;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.custom-color-btn.color-selected {
  background-image: none;
}

.custom-color-btn .q-icon {
  color: rgba(0, 0, 0, 0.7);
  position: absolute;
}

.hidden-color-picker {
  position: absolute;
  opacity: 0;
  height: 0;
  width: 0;
  pointer-events: none;
}

/* Add styles for Quasar form elements */
:deep(.q-field__native),
:deep(.q-field__input) {
  color: var(--theme-text) !important;
}

:deep(.q-field__label) {
  color: var(--theme-text-secondary) !important;
}

:deep(.q-field__marginal) {
  color: var(--theme-text-secondary);
}

:deep(input::placeholder) {
  color: var(--theme-text-secondary) !important;
}

:deep(.q-field__messages) {
  color: var(--theme-text-secondary) !important;
}
</style>
