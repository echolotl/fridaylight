<template>
  <q-dialog v-model="showModal" persistent>
    <q-card class="folder-modal phantom-font">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6 phantom-font-difficulty">Create Folder</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-card-section>
        <q-input
          v-model="folderName"
          label="Folder Name"
          outlined
          class="q-mb-md"
          :rules="[(val) => !!val || 'Folder name is required']"
          ref="nameInput"
        />
        <div class="text-subtitle2 q-mb-sm">Folder Color</div>
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
            type="color"
            ref="colorPickerInput"
            v-model="customColor"
            class="hidden-color-picker"
            @change="selectCustomColor"
          />
        </div>
      </q-card-section>

      <q-card-actions align="right">
        <q-btn
          flat
          label="Cancel"
          color="primary"
          v-close-popup
          @click="cancel"
        />
        <q-btn
          flat
          label="Create"
          color="primary"
          @click="createFolder"
          :disabled="!isValid"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { ref, computed, nextTick, watch } from "vue";

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(["update:modelValue", "create-folder"]);

const showModal = computed({
  get: () => props.modelValue,
  set: (value) => emit("update:modelValue", value),
});

const folderName = ref("");
const selectedColor = ref("#DB2955"); // Default color
const customColor = ref("#DB2955");
const isCustomColor = ref(false);
const nameInput = ref<HTMLInputElement | null>(null);
const colorPickerInput = ref<HTMLInputElement | null>(null);

const folderColors = [
  { label: "Pink", value: "#DB2955" },
  { label: "Blue", value: "#235789" },
  { label: "Green", value: "#35CE8D" },
  { label: "Purple", value: "#C490D1" },
  { label: "Orange", value: "#FE621D" },
  { label: "Yellow", value: "#f7c548" },
  { label: "Red", value: "#C03221" },
  { label: "Cyan", value: "#39A9DB" },
];

// Validate input
const isValid = computed(() => {
  return folderName.value.trim() !== "";
});

// Focus the name input when the modal opens
watch(
  () => props.modelValue,
  (isOpen) => {
    if (isOpen) {
      nextTick(() => {
        // Reset values when opening the modal
        folderName.value = "";
        selectedColor.value = "#DB2955";
        customColor.value = "#DB2955";
        isCustomColor.value = false;

        // Focus the input
        if (nameInput.value) {
          nameInput.value.focus();
        }
      });
    }
  }
);

const createFolder = () => {
  if (isValid.value) {
    emit("create-folder", {
      name: folderName.value.trim(),
      color: selectedColor.value,
    });
    showModal.value = false;
  }
};

const openColorPicker = () => {
  if (colorPickerInput.value) {
    colorPickerInput.value.click();
  }
};

const selectCustomColor = () => {
  selectedColor.value = customColor.value;
  isCustomColor.value = true;
};

const selectPresetColor = (color: string) => {
  selectedColor.value = color;
  isCustomColor.value = false;
};

const cancel = () => {
  folderName.value = "";
  selectedColor.value = "#DB2955";
  customColor.value = "#DB2955";
  isCustomColor.value = false;
};
</script>

<style scoped>
.folder-modal {
  width: 400px;
  background-color: var(--solid);
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
