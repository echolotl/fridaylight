<template>
  <q-item
    :key="`dl-${props.download.id}`"
    class="downloading-mod"
    :class="{ 'compact-mode': props.compactMode }"
  >
    <q-item-section avatar>
      <q-spinner
        v-if="!props.download.isComplete && !props.download.isError"
        size="32px"
        color="primary"
      />
      <q-icon
        v-else-if="props.download.isComplete"
        name="check_circle"
        color="positive"
        size="32px"
      />
      <q-icon
        v-else-if="props.download.isError"
        name="error"
        color="negative"
        size="32px"
      />
    </q-item-section>

    <q-item-section v-if="!props.compactMode">
      <q-item-label>{{ props.download.name }}</q-item-label>
      <q-item-label caption style="color: var(--theme-text-secondary)">{{
        props.download.step
      }}</q-item-label>

      <q-linear-progress
        v-if="!props.download.isComplete && !props.download.isError"
        :value="props.download.percentage / 100"
        color="primary"
        class="q-mt-xs"
        rounded
        size="8px"
      />

      <q-item-label
        v-if="
          !props.download.isComplete &&
          !props.download.isError &&
          downloadSpeed &&
          props.download.bytesDownloaded !== 0 &&
          !props.download.step.includes('Extracting') &&
          !compactMode
        "
        caption
        class="text-primary q-mt-xs"
      >
        {{ downloadSpeed }}
      </q-item-label>

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
import { computed, ref, watch } from 'vue'

const props = defineProps({
  download: {
    type: Object as () => DownloadProgress,
    required: true,
  },
  compactMode: {
    type: Boolean,
    default: false,
  },
})

const startTime = ref<number | null>(null)
const lastUpdateTime = ref<number | null>(null)
const lastBytesDownloaded = ref(0)
const speedSamples = ref<number[]>([])
const maxSamples = 10 // Keep last 10 speed samples for smoothing

// Initialize start time when download actually begins
watch(
  () => props.download.bytesDownloaded,
  (newBytes, _oldBytes) => {
    if (newBytes > 0 && startTime.value === null) {
      startTime.value = Date.now()
      lastUpdateTime.value = Date.now()
      lastBytesDownloaded.value = newBytes
    } else if (newBytes > lastBytesDownloaded.value && lastUpdateTime.value) {
      // Calculate instantaneous speed
      const now = Date.now()
      const timeDiff = (now - lastUpdateTime.value) / 1000 // seconds
      const bytesDiff = newBytes - lastBytesDownloaded.value

      if (timeDiff > 0.1) {
        // Only update if enough time has passed (100ms)
        const instantSpeed = bytesDiff / timeDiff

        // Add to speed samples for smoothing
        speedSamples.value.push(instantSpeed)
        if (speedSamples.value.length > maxSamples) {
          speedSamples.value.shift()
        }

        lastUpdateTime.value = now
        lastBytesDownloaded.value = newBytes
      }
    }
  },
  { immediate: true }
)

// Reset when download completes or errors
watch(
  () => [props.download.isComplete, props.download.isError],
  ([isComplete, isError]) => {
    if (isComplete || isError) {
      speedSamples.value = []
    }
  }
)

const downloadSpeed = computed(() => {
  // Return early states
  if (props.download.percentage === 0 || startTime.value === null)
    return 'Starting...'
  if (props.download.bytesDownloaded === 0) return 'Connecting...'

  // Use smoothed speed if we have samples
  if (speedSamples.value.length > 0) {
    // Calculate average of recent speed samples
    const avgSpeed =
      speedSamples.value.reduce((sum, speed) => sum + speed, 0) /
      speedSamples.value.length
    return formatSpeed(avgSpeed)
  }

  // Fallback to overall average speed
  const elapsedTime = (Date.now() - startTime.value) / 1000
  if (elapsedTime < 0.5) return 'Calculating...' // Wait at least 500ms

  const overallSpeed = props.download.bytesDownloaded / elapsedTime
  if (overallSpeed <= 0 || !isFinite(overallSpeed)) return 'Stalled'

  return formatSpeed(overallSpeed)
})

function formatSpeed(bytesPerSecond: number): string {
  if (bytesPerSecond >= 1024 * 1024 * 1024) {
    return `${(bytesPerSecond / (1024 * 1024 * 1024)).toFixed(2)} GB/s`
  } else if (bytesPerSecond >= 1024 * 1024) {
    return `${(bytesPerSecond / (1024 * 1024)).toFixed(2)} MB/s`
  } else if (bytesPerSecond >= 1024) {
    return `${(bytesPerSecond / 1024).toFixed(2)} KB/s`
  } else {
    return `${bytesPerSecond.toFixed(0)} B/s`
  }
}
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
