<template>
  <transition name="fade">
    <div
      v-if="visible"
      class="context-menu shadow-10 phantom-font"
      :style="{ top: `${position.y}px`, left: `${position.x}px` }"
    >
      <div v-for="(option, index) in options" :key="index">
        <q-separator v-if="option.separator" />
        <div
          v-else
          class="context-menu-item"
          :class="{ 'text-negative': option.danger }"
          @click="handleOptionClick(option)"
        >
          <q-icon :name="option.icon" size="sm" class="q-mr-sm" />
          {{ option.label }}
        </div>
      </div>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, defineProps, defineEmits } from 'vue'

// Define context menu option type
export interface ContextMenuOption {
  icon: string
  label: string
  action: () => void
  danger?: boolean
  separator?: boolean
}

// Props
defineProps({
  options: {
    type: Array as () => ContextMenuOption[],
    required: true,
  },
  visible: {
    type: Boolean,
    default: false,
  },
  position: {
    type: Object as () => { x: number; y: number },
    default: () => ({ x: 0, y: 0 }),
  },
})

// Emits
const emit = defineEmits(['close'])

// Handle option click
const handleOptionClick = (option: ContextMenuOption) => {
  if (option.action) {
    option.action()
  }
  emit('close')
}

// Close the menu when clicking outside
const handleGlobalClick = (event: MouseEvent) => {
  const contextMenu = document.querySelector('.context-menu') as HTMLElement
  if (contextMenu && !contextMenu.contains(event.target as Node)) {
    emit('close')
  }
}

// Close the menu when pressing escape
const handleEscapeKey = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    emit('close')
  }
}

// Add global event listeners when component is mounted
onMounted(() => {
  document.addEventListener('click', handleGlobalClick)
  document.addEventListener('keydown', handleEscapeKey)

  // Adjust position if menu would go off screen
  setTimeout(() => {
    const contextMenu = document.querySelector('.context-menu') as HTMLElement
    if (contextMenu) {
      const rect = contextMenu.getBoundingClientRect()
      const windowWidth = window.innerWidth
      const windowHeight = window.innerHeight

      if (rect.right > windowWidth) {
        contextMenu.style.left = `${windowWidth - rect.width - 10}px`
      }

      if (rect.bottom > windowHeight) {
        contextMenu.style.top = `${windowHeight - rect.height - 10}px`
      }
    }
  }, 0)
})

// Remove global event listeners when component is unmounted
onUnmounted(() => {
  document.removeEventListener('click', handleGlobalClick)
  document.removeEventListener('keydown', handleEscapeKey)
})
</script>

<style scoped>
.context-menu {
  position: fixed;
  z-index: 9999;
  background-color: var(--theme-solid);
  border: 2px solid var(--theme-border);
  border-radius: 0.5rem;
  min-width: 150px;
  max-width: 250px;
  padding: 8px 0;
  backdrop-filter: blur(15px);
}

.context-menu-item {
  padding: 0.25rem 1rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  color: var(--theme-text);
  transition: background-color 0.2s ease;
}

.context-menu-item:hover {
  background-color: var(--theme-surface);
}

.context-menu-item.text-negative:hover {
  background-color: rgba(255, 0, 0, 0.1);
}

.context-menu-item .q-icon {
  margin-right: 8px;
}

/* Fade transition animations */
.fade-enter-active,
.fade-leave-active {
  transition:
    opacity 0.2s ease,
    transform 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: scale(0.95);
}

.fade-enter-to,
.fade-leave-from {
  opacity: 1;
  transform: scale(1);
}
</style>
