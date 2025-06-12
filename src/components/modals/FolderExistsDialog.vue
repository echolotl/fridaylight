<template>
  <q-dialog v-model="showDialog" persistent>
    <q-card
      class="phantom-font"
      style="
        background: var(--theme-solid);
        color: var(--text-color);
        border: 2px solid var(--theme-border);
        backdrop-filter: blur(30px);
        min-width: 400px;
      "
    >
      <q-card-section class="row items-center">
        <q-icon
          name="warning"
          color="warning"
          text-color="white"
          size="md"
          class="q-mr-sm"
        />
        <h6 class="dialog-title phantom-font-difficulty">
          Folder Already Exists
        </h6>
      </q-card-section>

      <q-card-section class="q-pt-none">
        <div class="text-h6">{{ modName }}</div>
        <p class="text-body1 q-mt-sm">
          The mod folder already exists in your mods directory.
        </p>
        <p class="text-body2 q-mt-sm">
          This mod has already been downloaded with Fridaylight. You can either
          update the existing mod (overwriting any existing files!) or download
          into a separate instance.
        </p>
      </q-card-section>

      <q-card-actions align="right" class="q-gutter-sm">
        <q-btn flat label="Cancel" color="grey" @click="onCancel" />
        <q-btn
          flat
          label="Download Anyway"
          color="primary"
          @click="onDownloadAnyway"
        />
        <q-btn flat label="Update" color="positive" @click="onUpdate" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
  modName: {
    type: String,
    default: '',
  },
})

const emit = defineEmits([
  'update:modelValue',
  'update',
  'downloadAnyway',
  'cancel',
])

// Two-way binding for dialog visibility
const showDialog = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
})

const onUpdate = () => {
  emit('update')
  showDialog.value = false
}

const onDownloadAnyway = () => {
  emit('downloadAnyway')
  showDialog.value = false
}

const onCancel = () => {
  emit('cancel')
  showDialog.value = false
}
</script>

<style scoped>
.dialog-title {
  margin: 0;
}
</style>
