<template>
  <div>
    <div v-if="loading" class="loading-content">
      <q-spinner color="primary" size="48px" />
      <div>Loading featured mods...</div>
    </div>

    <div v-else class="featured-carousel">
      <div class="carousel-container">
        <q-carousel
          v-model="currentSlide"
          animated
          arrows
          autoplay
          navigation-position="bottom"
          infinite
          transition-prev="slide-right"
          transition-next="slide-left"
          class="featured-carousel-inner"
          height="75vh"
          control-color="primary"
          navigation-icon="circle"
        >
          <q-carousel-slide
            v-for="(mod, index) in mods"
            :key="mod.id"
            :name="index"
            class="featured-slide"
          >
            <div
              class="featured-mod-card"
              @click="$emit('showDetails', mod.id, mod.model_name)"
              @contextmenu.prevent="showContextMenu($event, mod)"
            >
              <q-img
                :src="mod.image_url"
                class="featured-thumbnail"
                :img-style="{
                  filter:
                    mod.initial_visibility == 'warn' ? 'blur(20px)' : 'none',
                }"
              >
                <div class="absolute-full featured-overlay"></div>

                <div class="absolute-top-right q-pa-sm row items-center">
                  <div class="category-icon-container">
                    <img
                      :src="
                        mod.category_icon_url ||
                        'https://gamebanana.com/static/img/defaults/icon.png'
                      "
                      class="category-icon"
                    />
                    <q-tooltip class="phantom-font">
                      {{ mod.category_name }}
                    </q-tooltip>
                  </div>
                  <div class="author-container">
                    <q-avatar size="40px" square>
                      <img
                        :src="
                          mod.submitter_avatar_url ||
                          'https://gamebanana.com/static/img/defaults/avatar.gif'
                        "
                      />
                    </q-avatar>
                    <span v-if="mod.submitter_u_pic" class="author-upic"
                      ><img :src="mod.submitter_u_pic"
                    /></span>
                    <span v-else class="author-name">{{ mod.owner }}</span>
                  </div>
                </div>

                <div class="absolute-top-left featured-period q-pa-sm">
                  <q-badge
                    color="primary"
                    text-color="white"
                    class="mod-period-badge"
                    >{{ formatPeriod(mod.period) }}</q-badge
                  >
                </div>

                <div class="absolute-bottom featured-info">
                  <div class="featured-title phantom-font-display">
                    {{ mod.name }}
                  </div>
                  <div class="featured-description">
                    {{ shortenDescription(mod.description) }}
                  </div>

                  <div class="featured-stats-container">
                    <q-btn
                      color="primary"
                      label="Download"
                      class="featured-btn q-mt-sm"
                      @click.stop="$emit('download', mod)"
                    />
                    <div class="featured-stats">
                      <span v-if="mod.initial_visibility == 'warn'">
                        <q-icon name="warning" size="sm" color="yellow" />
                        Has sensitive content!
                      </span>
                      <span>
                        <q-icon name="message" size="sm" />
                        {{ formatNumber(mod.post_count) }}
                      </span>
                      <span>
                        <q-icon name="thumb_up" size="sm" />
                        {{ formatNumber(mod.likes) }}
                      </span>
                    </div>
                  </div>
                </div>
              </q-img>
            </div>
          </q-carousel-slide>
        </q-carousel>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { GameBananaMod } from '../../types'
import { openUrl } from '@tauri-apps/plugin-opener'

const props = defineProps({
  mods: {
    type: Array as () => GameBananaMod[],
    default: () => [],
  },
  loading: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['download', 'showDetails'])

const currentSlide = ref(0)

console.log('Mods in carousel:', props.mods)

// Context menu handler
const showContextMenu = (event: MouseEvent, mod: GameBananaMod) => {
  event.preventDefault()
  event.stopPropagation()

  // Create context menu options
  const contextMenuOptions = [
    {
      icon: 'download',
      label: 'Download Mod',
      action: () => emit('download', mod),
    },
    {
      icon: 'info',
      label: 'Show Details',
      action: () => emit('showDetails', mod.id),
    },
    {
      icon: 'open_in_new',
      label: 'Open GameBanana Page',
      action: () => openUrl(mod.profile_url),
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

// Helper function to format file sizes
const formatNumber = (num: number): string => {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + 'M'
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'K'
  }
  return num.toString()
}

// Format period labels
const periodDisplayMap: Record<string, string> = {
  today: 'Best of Today',
  week: 'Best of this Week',
  month: 'Best of this Month',
  '3month': 'Best of 3 Months',
  '6month': 'Best of 6 Months',
  year: 'Best of this Year',
  alltime: 'Best of All Time',
}

const formatPeriod = (period: string): string => {
  return periodDisplayMap[period] || `Best of ${period}`
}

// Shorten description for display
const shortenDescription = (description: string): string => {
  return description?.length > 100
    ? description.slice(0, 100) + '...'
    : description || ''
}
</script>

<style scoped>
.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.featured-slide {
  display: flex;
  justify-content: center;
  align-items: center;
}

.featured-mod-card {
  position: relative;
  width: 100%;
  height: 100%;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

.featured-thumbnail {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.featured-overlay {
  background: linear-gradient(to top, rgba(0, 0, 0, 0.8), transparent);
}

.featured-info {
  padding: 12px;
  color: white;
  line-height: 1.2;
}

.featured-title {
  font-size: 3rem;
  margin-bottom: 4px;
}

.featured-description {
  font-size: 1.1rem;
  margin-bottom: 8px;
}

.featured-stats-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.featured-stats {
  display: flex;
  gap: 12px;
  font-size: 1rem;
  align-items: center;
}

.featured-btn {
  margin-top: 8px;
  border: 2px solid var(--theme-border);
  border-radius: 0.25rem;
}

.featured-period {
  font-size: 0.9rem;
  z-index: 10;
}

.mod-period-badge {
  font-size: 0.8rem;
  padding: 0.5rem 0.5rem;
  border-radius: 0.25rem;
}

.absolute-top-right {
  z-index: 10;
}

.author-container {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 1rem;
  background-color: rgba(0, 0, 0, 0.5);
  border-radius: 0.25rem;
}

.category-icon-container {
  margin-right: 1rem;
}

.author-name {
  font-size: 0.9rem;
  color: white;
}
.author-upic {
  margin-top: 5px;
}

.loading-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 200px;
  color: var(--theme-text-secondary);
}

:deep(.q-carousel) {
  background-color: transparent;
}

:deep(.q-img__content > div) {
  background-color: transparent;
}
</style>
