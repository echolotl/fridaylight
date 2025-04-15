<template>
  <div class="engine-download">
    <div class="text-h4 phantom-font-difficulty text-center engine-text">
      Engine Downloads
    </div>
    <div class="engine-download-buttons text-center">
      <q-btn
        flat
        label="Psych Engine"
        color="purple"
        @click="$emit('download-engine', 'psych')"
        class="engine-btn"
      />
      <q-btn
        flat
        label="V-Slice"
        color="blue"
        @click="$emit('download-engine', 'vslice')"
        class="engine-btn"
      />
      <q-btn
        flat
        label="FPS Plus"
        color="pink"
        @click="$emit('download-engine', 'fpsplus')"
        class="engine-btn"
      />
      <q-btn
        flat
        label="Codename Engine"
        color="orange"
        @click="$emit('download-engine', 'codename')"
        class="engine-btn"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
defineEmits(['download-engine']);
</script>

<style scoped>
.engine-text {
  margin-bottom: 1rem;
}
.engine-download-buttons {
  display: flex;
  gap: 8px;
  justify-content: center;
  margin-bottom: 16px;
}
.engine-btn {
  border-radius: 8px;
  padding: 8px 16px;
  transition: transform 0.2s ease;
}
.engine-btn:hover {
  transform: translateY(-2px);
}
</style>