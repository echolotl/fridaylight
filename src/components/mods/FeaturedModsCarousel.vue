<template>
  <div>

    <div class="loading-content" v-if="loading">
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
          height="50em"
          control-color="primary"
          navigation-icon="circle"
        >
          <q-carousel-slide
            v-for="(mod, index) in mods"
            :key="mod.id"
            :name="index"
            class="featured-slide"
          >
            <div class="featured-mod-card">
              <q-img :src="mod.imageUrl" class="featured-thumbnail">
                <div class="absolute-full featured-overlay"></div>

                <!-- Author profile picture with name -->
                <div class="absolute-top-right q-pa-sm">
                  <div class="author-container">
                    <q-avatar size="40px">
                      <img
                        :src="mod.submitterAvatarUrl || 'https://gamebanana.com/static/img/defaults/avatar.gif'"
                      />
                    </q-avatar>
                    <span class="author-name">{{ mod.owner }}</span>
                  </div>
                </div>

                <!-- Period tag -->
                <div class="absolute-top-left featured-period q-pa-sm">
                  <q-badge
                    color="primary"
                    text-color="white"
                    class="mod-period-badge"
                  >{{ formatPeriod(mod.period) }}</q-badge>
                </div>

                <div class="absolute-bottom featured-info">
                  <div class="featured-title phantom-font-display">{{ mod.name }}</div>
                  <div class="featured-description">
                    {{ shortenDescription(mod.description) }}
                  </div>

                  <div class="featured-stats-container">
                    <q-btn
                      color="primary"
                      label="Download"
                      class="featured-btn q-mt-sm"
                      @click="$emit('download', mod)"
                    />
                    <div class="featured-stats">
                      <span>
                        <q-icon name="message" size="sm" />
                        {{ formatNumber(mod.postCount) }}
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
import { ref } from 'vue';
import type { GameBananaMod } from "../../types";

defineProps({
  mods: {
    type: Array as () => GameBananaMod[],
    default: () => []
  },
  loading: {
    type: Boolean,
    default: false
  }
});

defineEmits(['download']);

const currentSlide = ref(0);

// Helper function to format numbers (e.g., 1000 -> 1K)
const formatNumber = (num: number): string => {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + "M";
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + "K";
  }
  return num.toString();
};

// Format period labels
const periodDisplayMap: Record<string, string> = {
  today: "Best of Today",
  week: "Best of this Week",
  month: "Best of this Month",
  "3month": "Best of 3 Months",
  "6month": "Best of 6 Months",
  year: "Best of this Year",
  alltime: "Best of All Time",
};

const formatPeriod = (period: string): string => {
  return periodDisplayMap[period] || `Best of ${period}`;
};

// Shorten description for display
const shortenDescription = (description: string): string => {
  return description?.length > 100 
    ? description.slice(0, 100) + "..." 
    : description || "";
};
</script>

<style scoped>
.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.featured-carousel-inner {
  width: 100%;
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
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
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
}

.featured-period {
  font-size: 0.9rem;
  z-index: 10;
}

.mod-period-badge {
  font-size: 0.8rem;
  padding: 0.5rem 0.5rem;
  border-radius: .25rem;
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
  border-radius: .25rem;
}

.author-name {
  font-size: 0.9rem;
  color: white;
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