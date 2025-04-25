<template>
  <div class="engine-download">
    <div class="text-h5 phantom-font-difficulty engine-text">
      Engine Downloads
    </div>
    <div class="engine-download-buttons text-center">
      <q-btn
        flat
        label="Psych Engine (1.0.4)"
        @click="$emit('download-engine', 'psych')"
        class="engine-btn"
      >
      <img src="/images/engine_icons/Psych.webp" alt="Psych Engine" class="engine-icon" />
    </q-btn>
      <q-btn
        flat
        label="V-Slice (0.6.3)"
        @click="$emit('download-engine', 'vanilla')"
        class="engine-btn"
      >
      <img src="/images/engine_icons/Vanilla.webp" alt="V-Slice" class="engine-icon" />
    </q-btn>
      <q-btn
        flat
        label="FPS Plus (7.1.0)"
        @click="$emit('download-engine', 'fps-plus')"
        class="engine-btn"
      >
        <img src="/images/engine_icons/Fps-plus.webp" alt="FPS Plus" class="engine-icon" />
      </q-btn>
      <q-btn
        flat
        label="Codename Engine"
        @click="$emit('download-engine', 'codename')"
        class="engine-btn"
      >
        <img src="/images/engine_icons/Codename.webp" alt="Codename Engine" class="engine-icon" />
    
    </q-btn>
    </div>
  </div>
</template>

<script setup lang="ts">
defineEmits(['download-engine']);
</script>

<style scoped>
.engine-download {
  margin-top: 1rem;
}
.engine-text {
  margin-bottom: 1rem;
}
.engine-download-buttons {
  display: flex;
  margin-bottom: 16px;
  gap: 8px;
}
.engine-btn {
  border-radius: 8px;
  padding: 8px 16px;
  width: 100%;
  transition: transform 0.2s ease;
  background-color: var(--theme-card);
  border: 1px solid var(--theme-border);
}
.engine-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

.engine-icon {
  width: 32px;
  height: 32px;
  margin-left: 8px;
}
</style>