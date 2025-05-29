<template>
  <q-dialog v-model="showDialog" :persistent="persistent">
    <q-card
      class="phantom-font"
      style="
        background: var(--solid);
        color: var(--text-color);
        border: 2px solid var(--theme-border);
        backdrop-filter: blur(30px);
      "
    >
      <q-card-section class="row items-center">
        <q-icon
          v-if="!disableIcon"
          :name="icon"
          :color="iconColor"
          text-color="white"
          size="md"
          class="q-mr-sm"
        />
        <h6 class="dialog-title phantom-font-difficulty">{{ title }}</h6>
      </q-card-section>

      <q-card-section class="q-pt-none">
        <div v-if="heading" class="text-h6">{{ heading }}</div>
        <div v-if="subHeading" class="text-caption">{{ subHeading }}</div>
        <p v-if="message" class="text-body2 q-mt-sm">{{ message }}</p>
        <slot></slot>
        <!-- For custom content -->
      </q-card-section>

      <q-card-actions align="right">
        <q-btn
          v-if="!singleOption"
          v-close-popup
          flat
          :label="cancelLabel"
          color="primary"
          @click="onCancel"
        />
        <q-btn
          v-close-popup
          flat
          :label="confirmLabel"
          :color="confirmColor"
          @click="onConfirm"
        />
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
  title: {
    type: String,
    default: 'Confirmation',
  },
  heading: {
    type: String,
    default: '',
  },
  subHeading: {
    type: String,
    default: '',
  },
  message: {
    type: String,
    default: '',
  },
  icon: {
    type: String,
    default: 'help',
  },
  disableIcon: {
    type: Boolean,
    default: false,
  },
  iconColor: {
    type: String,
    default: 'primary',
  },
  confirmLabel: {
    type: String,
    default: 'Confirm',
  },
  cancelLabel: {
    type: String,
    default: 'Cancel',
  },
  confirmColor: {
    type: String,
    default: 'primary',
  },
  persistent: {
    type: Boolean,
    default: true,
  },
  singleOption: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['update:modelValue', 'confirm', 'cancel'])

// Two-way binding for dialog visibility
const showDialog = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
})

const onConfirm = () => {
  emit('confirm')
}

const onCancel = () => {
  emit('cancel')
}
</script>

<style scoped>
.dialog-title {
  margin: 0;
}
.scrollarea {
  width: 100%;
  height: 100%;
  overflow-y: auto;
}
</style>
