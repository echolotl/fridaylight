<template>
  <div>
    <div v-if="loading" class="loading-content">
      <q-spinner color="primary" size="48px" />
      <div>{{ $t('gamebanana.loading_featured_mods') }}</div>
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
          control-color="primary"
          height="calc(85vh - 152px)"
          navigation-icon="circle"
        >
          <q-carousel-slide
            v-for="(mod, index) in mods"
            :key="mod._idRow"
            :name="index"
            class="featured-slide"
          >
            <div
              class="featured-mod-card"
              @contextmenu.prevent="showContextMenu($event, mod)"
            >
              <q-img
                :src="mod._sImageUrl"
                class="featured-thumbnail"
                :img-style="{
                  filter:
                    mod._sInitialVisibility == 'warn' ? 'blur(20px)' : 'none',
                }"
              >
                <div class="absolute-full featured-overlay"></div>

                <div class="absolute-top-right q-pa-sm row items-center">
                  <div class="category-icon-container">
                    <img
                      :src="mod._aRootCategory._sIconUrl"
                      class="category-icon"
                    />
                    <q-tooltip class="phantom-font">
                      {{ mod._aRootCategory._sName }}
                    </q-tooltip>
                  </div>
                  <div class="author-container">
                    <q-avatar size="40px" square>
                      <img :src="mod._aSubmitter._sAvatarUrl" />
                    </q-avatar>
                    <span v-if="mod._aSubmitter._sUpicUrl" class="author-upic"
                      ><img :src="mod._aSubmitter._sUpicUrl"
                    /></span>
                    <span v-else class="author-name">{{
                      mod._aSubmitter._sName
                    }}</span>
                  </div>
                </div>

                <div class="absolute-top-left featured-period q-pa-sm">
                  <q-badge
                    color="primary"
                    text-color="white"
                    class="mod-period-badge"
                    >{{ formatPeriod(mod._sPeriod) }}</q-badge
                  >
                </div>

                <div class="absolute-bottom featured-info">
                  <div class="featured-title phantom-font-display">
                    {{ mod._sName }}
                  </div>
                  <div v-if="mod._sDescription" class="featured-description">
                    {{ shortenDescription(mod._sDescription) }}
                  </div>

                  <div class="featured-stats-container">
                    <q-btn
                      color="primary"
                      :label="$t('ui.actions.view_details')"
                      class="featured-btn q-mt-sm"
                      @click.stop="
                        $emit('showDetails', mod._idRow, mod._sModelName)
                      "
                    />
                    <div class="featured-stats">
                      <span v-if="mod._sInitialVisibility == 'warn'">
                        <q-icon name="warning" size="sm" color="yellow" />
                        {{ $t('gamebanana.sensitive_content') }}
                      </span>
                      <span v-if="mod._nPostCount">
                        <q-icon name="message" size="sm" />
                        {{ formatNumber(mod._nPostCount) }}
                      </span>
                      <span v-if="mod._nLikeCount">
                        <q-icon name="thumb_up" size="sm" />
                        {{ formatNumber(mod._nLikeCount) }}
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
import type { GBTopSubsItem } from '@custom-types/gamebanana'
import { openUrl } from '@tauri-apps/plugin-opener'
import { useI18n } from 'vue-i18n'

defineProps({
  mods: {
    type: Array as () => GBTopSubsItem[],
    default: () => [],
  },
  loading: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['download', 'showDetails'])

const { t } = useI18n()

const currentSlide = ref(0)

// Context menu handler
const showContextMenu = (event: MouseEvent, mod: GBTopSubsItem) => {
  event.preventDefault()
  event.stopPropagation()

  // Create context menu options
  const contextMenuOptions = [
    {
      icon: 'download',
      label: t('ui.actions.download_mod'),
      action: () => emit('download', mod),
    },
    {
      icon: 'info',
      label: t('ui.actions.view_details'),
      action: () => emit('showDetails', mod._idRow, mod._sModelName),
    },
    {
      icon: 'open_in_new',
      label: t('ui.actions.open_in_browser'),
      action: () => openUrl(mod._sProfileUrl),
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
  today: t('gamebanana.best_of_today'),
  week: t('gamebanana.best_of_this_week'),
  month: t('gamebanana.best_of_this_month'),
  '3month': t('gamebanana.best_of_3_months'),
  '6month': t('gamebanana.best_of_6_months'),
  year: t('gamebanana.best_of_this_year'),
  alltime: t('gamebanana.best_of_all_time'),
}

const formatPeriod = (period: string): string => {
  return periodDisplayMap[period] || t('gamebanana.best_of_unknown', { period })
}

// Shorten description for display
const shortenDescription = (description: string): string => {
  return description?.length > 100
    ? description.slice(0, 100) + '...'
    : description || ''
}
</script>

<style scoped>
.carousel-container {
  position: relative;
  width: 100%;
  height: 100%;
  background: radial-gradient(
    circle at bottom left,
    transparent 0%,
    var(--theme-border) 10%,
    transparent 50%,
    var(--theme-border) 75%,
    transparent 100%
  );
  border-radius: 1rem;
  border: 1px solid var(--theme-border);
}
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

.featured-carousel-inner {
  mask-image: linear-gradient(
    to right,
    transparent 0%,
    white 2%,
    white 98%,
    transparent 100%
  );
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
