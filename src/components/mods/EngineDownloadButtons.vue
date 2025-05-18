<template>
  <div class="engine-download">
    <h6 class="phantom-font-difficulty q-mb-md"> Engine Downloads <hr/></h6>
    <div class="engine-download-buttons text-center">
      <q-btn
        flat
        label="Psych Engine"
        @click="$emit('download-engine', 'psych')"
        class="engine-btn"
      >
      <img src="/images/engine_icons/Psych.webp" alt="Psych Engine" class="engine-icon" />
    </q-btn>
      <q-btn
        flat
        label="V-Slice"
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
    <q-expansion-item class="more-engines" dense>
      <template v-slot:header>
        <div class="engine-text">
          <span>More Engines</span>
        </div>
      </template>
      <div class="more-engines-content">
        None yet, but you can help suggest what engines to include on the <span @click="openUrl('https://github.com/echolotl/fridaylight')" style="cursor: pointer; text-decoration: underline; color: var(--q-primary)">Github</span>!
      </div>
    </q-expansion-item>
  </div>
</template>

<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener';

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
  transition: transform 0.2s ease, box-shadow 0.2s ease;
  background-color: var(--theme-card);
  border-bottom: 2px solid var(--theme-border);
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

.engine-text {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  text-transform: uppercase;
  margin-bottom: 0;
}
.more-engines {
  border-radius: 1rem;
}
.more-engines-content {
  color: var(--theme-text-secondary);
  padding: 1rem;
  text-align: center;
}
</style>