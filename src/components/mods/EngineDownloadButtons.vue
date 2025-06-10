<template>
  <div class="engine-download">
    <h6 class="phantom-font-difficulty q-mb-md">
      Engine Downloads
      <hr />
    </h6>
    <div class="engine-download-buttons text-center">
      <q-btn
        v-for="engine in primaryEngines"
        :key="engine.engineType"
        flat
        class="engine-btn"
        no-caps
        @click="$emit('download-engine', engine.engineType)"
      >
        <template #default>
          <div class="engine-text">
            <span>{{ engine.engineName }}</span>
          </div>
          <span v-if="engine.data.engine_version" class="engine-version">
            (v{{ engine.data.engine_version }})
          </span>
          <img
            v-if="
              engine.data.engine_icon && engineIcons[engine.data.engine_icon]
            "
            :src="engineIcons[engine.data.engine_icon]"
            :alt="engine.engineName"
            class="engine-icon"
          />
        </template>
      </q-btn>
    </div>
    <q-expansion-item
      v-if="secondaryEngines.length > 0"
      class="more-engines full-width"
      dense
      expand-icon-class="dropdown-icon"
    >
      <template #header>
        <div class="flex items-center justify-between">
          <span>More Engines</span>
        </div>
      </template>
      <div class="more-engines-content">
        <q-btn
          v-for="engine in secondaryEngines"
          :key="engine.engineType"
          flat
          no-caps
          class="engine-btn"
          @click="$emit('download-engine', engine.engineType)"
        >
          <template #default>
            <div class="engine-text">
              <span>{{ engine.engineName }}</span>
            </div>
            <span v-if="engine.data.engine_version" class="engine-version">
              (v{{ engine.data.engine_version }})
            </span>
            <img
              v-if="
                engine.data.engine_icon && engineIcons[engine.data.engine_icon]
              "
              :src="engineIcons[engine.data.engine_icon]"
              :alt="engine.engineName"
              class="engine-icon"
            />
          </template>
        </q-btn>
      </div>
      <div>
        Have a suggestion for an engine? Ask on the
        <a
          class="cursor-pointer"
          @click="openUrl('https://www.github.com/echolotl/fridaylight/issues')"
          >GitHub</a
        >!
      </div>
    </q-expansion-item>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { openUrl } from '@tauri-apps/plugin-opener'
import { getAllEngineTypes, type EngineTypeInfo } from '../../utils'
import { resolveResource } from '@tauri-apps/api/path'
import { invoke } from '@tauri-apps/api/core'

defineEmits(['download-engine'])

const engineTypes = ref<EngineTypeInfo[]>([])
const engineIcons = ref<Record<string, string>>({})

const primaryEngines = computed(() =>
  engineTypes.value.filter((engine: EngineTypeInfo) => engine.isPrimary)
)

const secondaryEngines = computed(() =>
  engineTypes.value.filter((engine: EngineTypeInfo) => !engine.isPrimary)
)

onMounted(async () => {
  try {
    engineTypes.value = await getAllEngineTypes()
    await loadEngineIcons()
  } catch (error) {
    console.error('Failed to load engine types:', error)
  }
})

async function getIconAsBase64(iconPath: string): Promise<string> {
  try {
    const resourcePath = await resolveResource(iconPath)
    const iconData = await invoke<string>('get_file_as_base64', {
      filePath: resourcePath,
    })
    return iconData
  } catch (error) {
    console.error(`Error loading icon from ${iconPath}:`, error)
    return ''
  }
}

async function loadEngineIcons() {
  for (const engine of engineTypes.value) {
    if (engine.data.engine_icon) {
      try {
        engineIcons.value[engine.data.engine_icon] = await getIconAsBase64(
          `resources/${engine.data.engine_icon}`
        )
      } catch (error) {
        console.error(`Failed to load icon for ${engine.engineName}:`, error)
      }
    }
  }
}
</script>

<style scoped>
.engine-download {
  margin-top: 1rem;
}
.engine-download-buttons {
  display: flex;
  margin-bottom: 16px;
  gap: 8px;
}
.engine-btn {
  border-radius: 8px;
  padding: 8px 16px;
  max-width: fit-content;
  width: 100%;
  transition:
    transform 0.2s ease,
    box-shadow 0.2s ease;
  background-color: var(--theme-card);
  border-bottom: 2px solid var(--theme-border);
  display: flex;
  flex-direction: row;
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
  margin-bottom: none;
}
.engine-version {
  font-size: 0.75rem;
  color: var(--theme-text-secondary);
  margin-left: 4px;
}
.more-engines {
  border-radius: 1rem;
}
.more-engines-content {
  flex-wrap: wrap;
  display: flex;
  padding: 1rem 0;
  gap: 8px;
  text-align: center;
}
:deep(.dropdown-icon) {
  color: var(--theme-text-secondary);
}
</style>
