<template>
  <div class="skeleton-mod-card">
    <div class="skeleton-thumbnail" />
    <div class="skeleton-content">
      <div class="skeleton-title"></div>
      <div class="skeleton-author"></div>
      <div class="skeleton-stats">
        <span class="skeleton-stat-item"></span>
        <span class="skeleton-stat-item"></span>
      </div>
      <div v-if="showButton" class="skeleton-button"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps({
  showButton: {
    type: Boolean,
    default: true,
  },
})
</script>
<style scoped>
.skeleton-mod-card {
  background: var(--theme-card);
  border-radius: 0.75rem;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  transition:
    transform 0.2s,
    box-shadow 0.2s;
  border-bottom: 2px solid var(--theme-border);
  border-top: 2px solid var(--theme-surface);
  min-width: 300px;
  max-height: 321px;
  position: relative;
}

.skeleton-mod-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent 0%,
    var(--theme-border) 50%,
    transparent 100%
  );
  animation: skeleton-loading 1.5s ease-in-out infinite;
}

.skeleton-thumbnail {
  height: 160px;
  background: var(--theme-card);
  object-fit: cover;
  border-bottom: 2px solid var(--theme-border);
}

.skeleton-thumbnail::after {
  content: '';
  background-image: url('/images/placeholder.png');
  background-size: cover;
  background-position: center;
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 158px;
  mix-blend-mode: luminosity;
}

.skeleton-content {
  padding: 12px;
  flex-grow: 1;
  opacity: 0.5;
}

.skeleton-title {
  height: 1.1rem;
  width: 60%;
  background: var(--theme-text-secondary);
  margin-bottom: 8px;
  border-radius: 4px;
}

.skeleton-author {
  height: 0.9rem;
  width: 40%;
  background: var(--theme-text-secondary);
  margin-bottom: 8px;
  border-radius: 4px;
}
.skeleton-button {
  height: 2rem;
  width: 100%;
  background: var(--theme-text-secondary);
  border-radius: 0.5rem;
  margin-top: 2rem;
}

.skeleton-stats {
  display: flex;
  gap: 8px;
}
.skeleton-stat-item {
  height: 1rem;
  width: 1rem;
  background: var(--theme-text-secondary);
  border-radius: 4px;
}

@keyframes skeleton-loading {
  0% {
    left: -100%;
  }
  100% {
    left: 100%;
  }
}
</style>
