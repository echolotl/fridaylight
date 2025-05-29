<template>
  <q-item
    :key="`dl-${download.id}`"
    class="downloading-mod"
    :class="{ 'compact-mode': compactMode }"
  >
    <q-item-section avatar>
      <q-spinner
        v-if="!download.isComplete && !download.isError"
        size="32px"
        color="primary"
      />
      <q-icon
        v-else-if="download.isComplete"
        name="check_circle"
        color="positive"
        size="32px"
      />
      <q-icon
        v-else-if="download.isError"
        name="error"
        color="negative"
        size="32px"
      />
    </q-item-section>

    <q-item-section v-if="!compactMode">
      <q-item-label>{{ download.name }}</q-item-label>
      <q-item-label caption style="color: var(--theme-text-secondary)">{{
        download.step
      }}</q-item-label>

      <q-linear-progress
        v-if="!download.isComplete && !download.isError"
        :value="download.percentage / 100"
        color="primary"
        class="q-mt-xs"
        rounded
        size="8px"
      />

      <q-item-label v-if="download.isError" caption class="text-negative">
        {{ download.error }}
      </q-item-label>
    </q-item-section>

    <!-- Show progress in compact mode -->
    <q-linear-progress
      v-if="compactMode && !download.isComplete && !download.isError"
      :value="download.percentage / 100"
      color="primary"
      class="compact-progress"
      rounded
      size="4px"
    />

    <q-tooltip v-if="compactMode">
      {{ download.name }} - {{ download.step }}
      <div v-if="download.isError" class="text-negative">
        {{ download.error }}
      </div>
    </q-tooltip>
  </q-item>
</template>

<script setup lang="ts">
import { DownloadProgress } from '@stores/downloadState'

defineProps({
  download: {
    type: Object as () => DownloadProgress,
    required: true,
  },
  compactMode: {
    type: Boolean,
    default: false,
  },
})
</script>

<style scoped>
.downloading-mod {
  margin-bottom: 8px;
}

.compact-mode {
  padding: 4px 8px;
  min-height: 40px;
  position: relative;
}

.compact-progress {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  margin: 0;
  height: 3px;
}
</style>
