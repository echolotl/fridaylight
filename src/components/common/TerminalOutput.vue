<template>
  <div v-if="isVisible" class="terminal-output-container">
    <div class="terminal-header">
      <div class="terminal-title phantom-font-difficulty">
        {{ $t('mods.terminal_output') }}
      </div>
      <div class="terminal-actions">
        <q-btn flat round dense icon="content_copy" @click="copyToClipboard" />
        <q-btn
          flat
          round
          dense
          icon="cleaning_services"
          @click="clearTerminal"
        />
        <q-btn flat round dense icon="close" @click="$emit('close')" />
      </div>
    </div>

    <q-virtual-scroll
      ref="terminalContent"
      :items="logs"
      class="terminal-content"
    >
      <template #default="{ item }">
        <pre>{{ item }}</pre>
      </template>
    </q-virtual-scroll>

    <div class="terminal-status-bar">
      <div class="terminal-info">
        {{ logs.length > 0 ? `${logs.length} lines` : 'Ready' }}
      </div>
      <div class="auto-scroll-toggle">
        <q-toggle
          v-model="autoScroll"
          label="Auto-scroll"
          color="primary"
          dense
          size="sm"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onBeforeUnmount, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { QVirtualScroll } from 'quasar'
import { notificationService } from '@services/notificationService'

const props = defineProps({
  modId: {
    type: String,
    default: '',
  },
  isVisible: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['close', 'clear'])

const logs = ref<string[]>([])
const terminalContent = ref<InstanceType<typeof QVirtualScroll> | null>(null)
const autoScroll = ref(true)
const refreshInterval = ref<number | null>(null)

// Function to fetch logs from the backend
const fetchLogs = async () => {
  if (!props.modId || !props.isVisible) return

  try {
    console.info(`Fetching logs for mod ${props.modId}`)
    const newLogs = await invoke<string[]>('get_mod_logs', {
      id: props.modId,
    })

    console.info(`Received logs for mod ${props.modId}:`, newLogs)

    if (Array.isArray(newLogs) && newLogs.length > 0) {
      // Only update if there are new logs to avoid unnecessary renders
      if (JSON.stringify(newLogs) !== JSON.stringify(logs.value)) {
        console.log(
          `Updating logs for mod ${props.modId} (${newLogs.length} entries)`
        )
        logs.value = newLogs

        // Scroll to bottom if auto-scroll is enabled
        if (autoScroll.value) {
          await scrollToBottom()
        }
      }
    } else {
      console.log(`No logs found for mod ${props.modId}`)
    }
  } catch (error) {
    console.error('Failed to fetch mod logs:', error)
  }
}

// Scroll to the bottom of the terminal
const scrollToBottom = async () => {
  if (terminalContent.value) {
    await nextTick()
    terminalContent.value.scrollTo(logs.value.length - 1)
  }
}

// Copy terminal content to clipboard
const copyToClipboard = async () => {
  try {
    await navigator.clipboard.writeText(logs.value.join('\n'))
    notificationService.info({
      message: 'Logs copied to clipboard',
    })
  } catch (err) {
    console.error('Failed to copy:', err)
    notificationService.error({
      message: 'Failed to copy logs to clipboard',
    })
  }
}

// Clear terminal content
const clearTerminal = () => {
  logs.value = []
  emit('clear', props.modId)
}

// Watch for changes in visibility or modId
watch(
  () => props.isVisible,
  newVal => {
    if (newVal) {
      fetchLogs() // Fetch immediately when becoming visible
    }
  }
)

watch(
  () => props.modId,
  (newVal, oldVal) => {
    if (newVal !== oldVal) {
      logs.value = [] // Clear logs when switching mods
      fetchLogs() // Fetch new logs
    }
  }
)

// Start polling for logs when component is mounted
onMounted(() => {
  // Set up interval to refresh logs
  refreshInterval.value = window.setInterval(fetchLogs, 1000)
})

// Clean up on component unmount
onBeforeUnmount(() => {
  if (refreshInterval.value !== null) {
    window.clearInterval(refreshInterval.value)
  }
})
</script>

<style scoped>
.terminal-output-container {
  display: flex;
  flex-direction: column;
  height: 300px;
  border: 2px solid var(--theme-border);
  border-radius: 8px;
  color: var(--theme-text);
  margin: 16px 0;
  overflow: hidden;
}

.terminal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background-color: var(--theme-card);
  border-bottom: 1px solid var(--theme-border);
}

.terminal-title {
  font-size: 1rem;
  color: var(--theme-text);
}

.terminal-actions {
  display: flex;
  gap: 4px;
}

.terminal-content {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
  font-family: 'Courier New', monospace;
  font-size: 0.9rem;
  white-space: pre-wrap;
  line-height: 1.4;
  scrollbar-width: thin;
  background-color: var(--theme-bg);
}

.terminal-content::-webkit-scrollbar {
  width: 6px;
}

.terminal-content::-webkit-scrollbar-thumb {
  background-color: rgba(255, 255, 255, 0.2);
  border-radius: 3px;
}

.terminal-content::-webkit-scrollbar-track {
  background-color: rgba(0, 0, 0, 0.3);
}

.terminal-empty-message {
  color: var(--theme-text-secondary);
  font-style: italic;
  text-align: center;
  margin-top: 20px;
}

.terminal-status-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 4px 12px;
  background-color: var(--theme-card);
  border-top: 1px solid var(--theme-border);
  font-size: 0.8rem;
  color: var(--theme-text-secondary);
}

pre {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-word;
}
</style>
