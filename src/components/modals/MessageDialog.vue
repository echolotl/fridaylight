<template>
  <q-dialog v-model="showDialog" persistent>
    <q-card
      class="phantom-font"
    >
      <q-card-section class="row items-center">
        <q-icon :name="icon" :color="iconColor" text-color="white" size="md" />
        <h6 class="dialog-title phantom-font-difficulty" >{{ title }}</h6>
      </q-card-section>
      
      <q-card-section>
        <div v-if="heading" class="text-h6">{{ heading }}</div>
        <div v-if="subHeading" class="text-caption">{{ subHeading }}</div>
        <p v-if="message" class="text-body2 q-mt-sm">{{ message }}</p>
        <slot></slot> <!-- For custom content -->
      </q-card-section>
      
      <q-card-actions align="right">
        <q-btn 
          flat 
          :label="cancelLabel" 
          color="primary" 
          v-close-popup 
          @click="onCancel" 
        />
        <q-btn
          flat
          :label="confirmLabel"
          :color="confirmColor"
          @click="onConfirm"
          v-close-popup
        />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { computed } from 'vue';

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
  }
});

const emit = defineEmits(['update:modelValue', 'confirm', 'cancel']);

// Two-way binding for dialog visibility
const showDialog = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
});

const onConfirm = () => {
  emit('confirm');
};

const onCancel = () => {
  emit('cancel');
};
</script>

<style>
.dialog-title {
    margin: 0 .5rem;
}

.q-card {
    background-color: var(--solid);
    color: var(--text-color);
    border: 2px solid var(--theme-border);
}
</style>