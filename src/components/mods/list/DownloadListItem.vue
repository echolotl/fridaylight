<template>
  <q-item :key="`dl-${download.id}`" class="downloading-mod">
    <q-item-section avatar>
      <q-spinner
        size="32px"
        v-if="!download.isComplete && !download.isError"
        color="primary"
      />
      <q-icon
        name="check_circle"
        color="positive"
        size="32px"
        v-else-if="download.isComplete"
      />
      <q-icon
        name="error"
        color="negative"
        size="32px"
        v-else-if="download.isError"
      />
    </q-item-section>

    <q-item-section>
      <q-item-label>{{ download.name }}</q-item-label>
      <q-item-label caption style="color: var(--theme-text-secondary)">{{ download.step }}</q-item-label>

      <q-linear-progress
        v-if="!download.isComplete && !download.isError"
        :value="download.percentage / 100"
        color="primary"
        class="q-mt-xs"
        rounded
        size="8px"
      />

      <q-item-label
        caption
        class="text-negative"
        v-if="download.isError"
      >
        {{ download.error }}
      </q-item-label>
    </q-item-section>
  </q-item>
</template>

<script setup lang="ts">
import { DownloadProgress } from "@stores/downloadState";

defineProps({
  download: {
    type: Object as () => DownloadProgress,
    required: true,
  }
});
</script>

<style scoped>
.downloading-mod {
  margin-bottom: 8px;
}
</style>