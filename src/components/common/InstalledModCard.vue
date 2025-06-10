<template>
  <div class="mod-card">
    <div
      class="mod-card-content"
      @click="$emit('showDetails', mod)"
      @contextmenu.prevent="showContextMenu"
    >
      <div class="mod-thumbnail-container">
        <!-- Display banner if available, otherwise use a placeholder or background color -->
        <div class="mod-thumbnail" :style="bannerStyle">
          <!-- Show engine icon -->
          <img v-if="engineIconUrl" :src="engineIconUrl" class="engine-icon" />

          <!-- Show logo if available -->
          <img
            v-if="mod.logo_data"
            :src="mod.logo_data"
            class="mod-logo"
            :class="logoPositionClass"
          />
        </div>
      </div>

      <div class="mod-info">
        <div class="mod-title">{{ mod.name }}</div>

        <div class="mod-metadata">
          <!-- Show version if available -->
          <span v-if="mod.version" class="badge">v{{ mod.version }}</span>

          <!-- Show engine name -->
          <span v-if="mod.engine.engine_name != 'unknown'" class="badge">{{
            mod.engine.engine_name
          }}</span>
        </div>

        <!-- Contributors if available -->
        <div v-if="hasContributors" class="contributors">
          <div class="contributor-list">
            <span>by {{ primaryContributor }}</span>
            <span v-if="contributorCount > 1"
              >+{{ contributorCount - 1 }} more</span
            >
          </div>
        </div>

        <div class="mod-stats">
          <span v-if="mod.last_played">
            <q-icon name="schedule" size="xs" />
            {{ formatDate(mod.last_played) }}
          </span>
          <span v-if="mod.date_added">
            <q-icon name="add_circle_outline" size="xs" />
            {{ formatDate(mod.date_added) }}
          </span>
        </div>
      </div>
    </div>
    <div class="action-buttons flex row justify-end">
      <q-btn
        color="primary"
        label="Play"
        class="play-btn"
        text-color="white"
        flat
        @click.stop="$emit('play', mod.id)"
      />
      <q-btn
        color="var(--theme-text)"
        icon="settings"
        class="settings-btn"
        flat
        round
        @click.stop="$emit('configure', mod)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Mod } from '@main-types'
import { revealItemInDir } from '@tauri-apps/plugin-opener'

const props = defineProps({
  mod: {
    type: Object as () => Mod,
    required: true,
  },
})

const emit = defineEmits(['play', 'configure', 'showDetails'])

// Create banner style based on available data
const bannerStyle = computed(() => {
  if (props.mod.banner_data) {
    return {
      backgroundImage: `url(${props.mod.banner_data})`,
      backgroundSize: 'cover',
      backgroundPosition: 'center',
    }
  }
  return {
    backgroundImage: 'url("/images/menuBG.png")',
    backgroundSize: 'cover',
    backgroundPosition: 'center',
  }
})

// Get engine icon URL
const engineIconUrl = computed(() => {
  if (props.mod.engine && props.mod.engine.engine_icon) {
    return props.mod.engine.engine_icon
  }
  return null
})

// Handle logo positioning
const logoPositionClass = computed(() => {
  switch (props.mod.logo_position) {
    case 'left_bottom':
      return 'logo-left-bottom'
    case 'left_middle':
      return 'logo-left-middle'
    case 'middle':
      return 'logo-middle'
    default:
      return 'logo-left-bottom'
  }
})

// Format date to relative time (e.g. "2 days ago")
const formatDate = (timestamp: number): string => {
  // Ensure timestamp is always treated as seconds and convert to milliseconds for Date operations
  // If the timestamp appears to be in seconds (less than year 2100 in milliseconds), convert to milliseconds
  const normalizedTimestamp =
    timestamp < 4102444800000
      ? Math.trunc(timestamp) * 1000
      : Math.trunc(timestamp)

  const now = Date.now()
  const difference = now - normalizedTimestamp

  // Debug info
  console.log(`Current time: ${now}`)
  console.log(`Original timestamp: ${timestamp}`)
  console.log(`Normalized timestamp: ${normalizedTimestamp}`)

  // Convert milliseconds to days, hours, etc.
  const days = Math.floor(difference / (1000 * 60 * 60 * 24))
  const hours = Math.floor(
    (difference % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60)
  )

  if (days > 30) {
    return new Date(normalizedTimestamp).toLocaleDateString()
  } else if (days > 0) {
    return `${days}d ago`
  } else if (hours > 0) {
    return `${hours}h ago`
  } else {
    return 'Just now'
  }
}

// Contributor-related computed properties
const hasContributors = computed(() => {
  return (
    props.mod.contributors &&
    props.mod.contributors.length > 0 &&
    props.mod.contributors.some(
      group => group.members && group.members.length > 0
    )
  )
})

const primaryContributor = computed(() => {
  if (!hasContributors.value) {
    return 'Unknown'
  }

  // Try to find a "Creator" or "Main" group first
  const mainGroups = ['Creator', 'Main', 'Developer', 'Author']
  for (const groupName of mainGroups) {
    const group = props.mod.contributors?.find(
      g => g.group.toLowerCase() === groupName.toLowerCase()
    )
    if (group && group.members.length > 0) {
      return group.members[0].name
    }
  }

  // Fallback to first contributor in the first group
  return props.mod.contributors?.[0]?.members[0]?.name || 'Unknown'
})

const contributorCount = computed(() => {
  if (!props.mod.contributors) return 0

  return props.mod.contributors.reduce((total, group) => {
    return total + (group.members ? group.members.length : 0)
  }, 0)
})

// Context menu handler
const showContextMenu = (event: MouseEvent) => {
  event.preventDefault()
  event.stopPropagation()

  // Create context menu options
  const contextMenuOptions = [
    {
      icon: 'play_arrow',
      label: 'Launch Mod',
      action: () => emit('play', props.mod.id),
    },
    {
      icon: 'settings',
      label: 'Edit Settings',
      action: () => emit('configure', props.mod.id),
    },
    {
      icon: 'folder_open',
      label: 'Open Mod Folder',
      action: () => {
        revealItemInDir(props.mod.path)
      },
    },
    {
      icon: 'info',
      label: 'Show Details',
      action: () => emit('showDetails', props.mod.id),
    },
  ]

  // Create and dispatch custom event to show context menu
  const customEvent = new CustomEvent('show-context-menu', {
    detail: {
      position: { x: event.clientX, y: event.clientY },
      options: contextMenuOptions,
    },
    bubbles: true,
  })

  // Safely handle the case where event.target could be null
  if (event.target) {
    event.target.dispatchEvent(customEvent)
  } else {
    // Fallback to document if target is null
    document.dispatchEvent(customEvent)
  }
}
</script>

<style scoped>
.mod-card {
  background: var(--theme-card);
  border-radius: 0.75rem;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  transition:
    transform 0.2s,
    box-shadow 0.2s;
  border-bottom: 2px solid var(--theme-border);
}

.mod-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 0px 20px rgba(0, 0, 0, 0.2);
}

.mod-card-content {
  cursor: pointer;
  display: flex;
  flex-direction: column;
  flex-grow: 1;
}

.mod-thumbnail-container {
  position: relative;
  height: 160px;
  overflow: hidden;
  border-bottom: 2px solid var(--theme-border);
}

.mod-thumbnail {
  height: 100%;
  width: 100%;
  display: flex;
  align-items: flex-end;
  background: var(--theme-card-accent);
}

.mod-logo {
  max-height: 60px;
  max-width: 180px;
  margin: 10px;
  object-fit: contain;
}

.logo-left-bottom {
  position: absolute;
  bottom: 5px;
  left: 5px;
}

.logo-left-middle {
  position: absolute;
  top: 50%;
  left: 10px;
  transform: translateY(-50%);
}

.logo-middle {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  max-width: 140px;
  max-height: 100px;
}

.engine-icon {
  position: absolute;
  top: 8px;
  right: 8px;
  width: auto;
  height: 32px;
  padding: 2px;
  background: rgba(0, 0, 0, 0.5);
  border-radius: 5px;
}

.mod-info {
  padding: 12px;
  flex-grow: 1;
}

.mod-title {
  font-weight: bold;
  font-size: 1.1rem;
  margin-bottom: 4px;
  color: var(--theme-text);
}

.mod-metadata {
  display: flex;
  gap: 6px;
  margin-bottom: 8px;
}

.badge {
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 0.7rem;
  background-color: var(--theme-card);
  color: var(--theme-text-secondary);
}

.contributors {
  font-size: 0.9rem;
  color: var(--theme-text-secondary);
  margin-bottom: 8px;
}

.contributor-list {
  display: flex;
  align-items: center;
  gap: 4px;
}

.mod-stats {
  display: flex;
  gap: 12px;
  font-size: 0.8rem;
  color: var(--theme-text-secondary);
  margin-top: auto;
}

.action-buttons {
  display: flex;
  justify-content: flex-end;
  padding: 0 12px 12px;
  gap: 10px;
}

.play-btn {
  width: 100%;
  background-color: var(--theme-accent, #42b883);
  color: white;
  flex: 1;
  border-radius: 1rem;
  border: 2px solid var(--theme-border);
}
</style>
