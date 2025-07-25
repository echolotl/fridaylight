<template>
  <q-dialog v-model="showDialog" :persistent="persistent">
    <q-card
      class="phantom-font dialog"
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

      <q-card-section class="q-pt-none dialog-content">
        <div v-if="heading" class="text-h6">{{ heading }}</div>
        <div
          v-if="subHeading"
          class="text-caption"
          style="color: var(--theme-text-secondary)"
        >
          {{ subHeading }}
        </div>
        <p v-if="message" class="text-body2 q-mt-sm">{{ message }}</p>
        <slot></slot>
        <!-- For custom content -->
      </q-card-section>

      <q-card-actions align="right" style="flex-shrink: 0">
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
import { useI18n } from 'vue-i18n'

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
  title: {
    type: String,
    default: () => {
      const { t } = useI18n()
      return t('misc.confirmation')
    },
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
    default: () => {
      const { t } = useI18n()
      return t('ui.actions.confirm')
    },
  },
  cancelLabel: {
    type: String,
    default: () => {
      const { t } = useI18n()
      return t('ui.actions.cancel')
    },
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
.dialog {
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
}
.scrollarea {
  width: 100%;
  height: 100%;
  overflow-y: auto;
}
</style>
