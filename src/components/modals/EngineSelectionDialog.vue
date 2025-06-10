<template>
  <q-dialog
    v-model="isOpen"
    persistent
    transition-show="fade"
    transition-hide="fade"
  >
    <q-card class="engine-selection-dialog phantom-font">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6 phantom-font-difficulty">
          Select Installation Location
        </div>
        <q-space />
        <q-btn v-close-popup icon="close" flat round dense @click="cancel" />
      </q-card-section>

      <q-card-section class="q-mb-sm">
        <p>
          Select which {{ formatEngineType(engineType) }} installation to add
          this modpack to:
        </p>

        <q-list bordered separator class="rounded-borders">
          <q-item
            v-for="engine in compatibleEngines"
            :key="engine.id"
            v-ripple
            clickable
            :active="selectedEngineMod?.id === engine.id"
            active-class="selected-engine"
            class="engine-item"
            @click="selectedEngineMod = engine"
          >
            <q-item-section avatar>
              <q-avatar>
                <img
                  :src="
                    engine.icon_data ||
                    `/images/engine_icons/${
                      engine.engine?.engine_type || engine.engine_type
                    }.webp`
                  "
                  onerror="this.src='/images/engine_icons/Psych.webp'"
                />
              </q-avatar>
            </q-item-section>

            <q-item-section>
              <q-item-label>{{ engine.name }}</q-item-label>
              <q-item-label caption class="engine-path-caption">{{
                engine.path
              }}</q-item-label>
            </q-item-section>
          </q-item>
        </q-list>
      </q-card-section>
      <!-- Codename Engine Addon Option -->
      <q-card-section
        v-if="isCodename && selectedEngineMod"
        class="q-pt-none q-pb-none"
      >
        <q-toggle
          v-model="isAddon"
          label="Install as Addon (will run on all mods)"
        />
      </q-card-section>
      <q-card-section v-if="selectedEngineMod" class="text-caption">
        <p>The modpack will be installed to:</p>
        <code>{{ getInstallPath() }}</code>
      </q-card-section>

      <q-card-actions align="right">
        <q-btn
          v-close-popup
          flat
          label="Cancel"
          color="primary"
          @click="cancel"
        />
        <q-btn
          flat
          label="Download & Install"
          color="primary"
          :disable="!selectedEngineMod"
          @click="confirm"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { sep } from '@tauri-apps/api/path'
import { ref, watch, computed } from 'vue'

interface EngineMod {
  id: string
  name: string
  path: string
  icon_data?: string
  engine?: {
    engine_type: string
    mods_folder_path?: string
    mods_folder?: boolean
  }
  engine_type?: string
  executable_path?: string
}

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
  compatibleEngines: {
    type: Array as () => EngineMod[],
    default: () => [],
  },
  engineType: {
    type: String as () => string | null,
    default: null,
  },
  modName: {
    type: String,
    default: 'Unknown Mod',
  },
})

const emit = defineEmits(['update:modelValue', 'select', 'cancel'])

const isOpen = ref(props.modelValue)
const selectedEngineMod = ref<EngineMod | null>(null)
const isAddon = ref(false)

// Computed property to check if the selected engine is Codename
const isCodename = computed(() => {
  return props.engineType?.toLowerCase() === 'codename'
})

// Set the default selected engine to the first one in the list
watch(
  () => props.compatibleEngines,
  newEngines => {
    if (newEngines && newEngines.length > 0) {
      selectedEngineMod.value = newEngines[0]
    }
  },
  { immediate: true }
)

watch(
  () => props.modelValue,
  val => {
    isOpen.value = val
  }
)

watch(isOpen, val => {
  emit('update:modelValue', val)
})

const confirm = () => {
  if (selectedEngineMod.value) {
    // Include isAddon in the emitted data
    emit('select', {
      ...selectedEngineMod.value,
      isAddon: isAddon.value,
    })
    isOpen.value = false
  }
}

const cancel = () => {
  emit('cancel')
  isOpen.value = false
}

const formatEngineType = (engineType: string | null): string => {
  if (!engineType) return 'Unknown'

  switch (engineType.toLowerCase()) {
    case 'psych':
      return 'Psych Engine'
    case 'vanilla':
      return 'V-Slice'
    case 'codename':
      return 'Codename Engine'
    default:
      return engineType.charAt(0).toUpperCase() + engineType.slice(1)
  }
}

// Function to get the mods folder path for an engine mod
const getModsFolderPath = (engineMod: EngineMod): string => {
  // Get base directory first in all cases
  const basePath = engineMod.path
  const executablePath = engineMod.executable_path || ''

  if (!basePath) return 'Unknown path'

  // Get parent directory of executable if it exists
  let baseDir = basePath
  if (executablePath) {
    // Extract the directory from the executable path
    const lastSlashIndex = executablePath.lastIndexOf('/')
    if (lastSlashIndex > 0) {
      baseDir = executablePath.substring(0, lastSlashIndex)
    } else {
      const lastBackslashIndex = executablePath.lastIndexOf('\\')
      if (lastBackslashIndex > 0) {
        baseDir = executablePath.substring(0, lastBackslashIndex)
      }
    }
  }

  // Then check if the engine has a specified custom mods folder path
  if (
    engineMod.engine &&
    engineMod.engine.mods_folder &&
    engineMod.engine.mods_folder_path
  ) {
    // Combine the base directory with the custom mods folder path
    return `${baseDir}${sep()}${engineMod.engine.mods_folder_path}`
  }

  // If no custom path specified, use default mods folder
  return `${baseDir}${sep()}mods`
}

// Get the installation path based on engine type and addon setting
const getInstallPath = (): string => {
  if (!selectedEngineMod.value) return 'Unknown path'

  // If it's a Codename Engine addon, use addons folder instead of mods
  if (isCodename.value && isAddon.value) {
    const basePath = selectedEngineMod.value.path
    const executablePath = selectedEngineMod.value.executable_path || ''

    if (!basePath) return 'Unknown path'

    // Get parent directory of executable if it exists
    let baseDir = basePath
    if (executablePath) {
      // Extract the directory from the executable path
      const lastSlashIndex = executablePath.lastIndexOf('/')
      if (lastSlashIndex > 0) {
        baseDir = executablePath.substring(0, lastSlashIndex)
      } else {
        const lastBackslashIndex = executablePath.lastIndexOf('\\')
        if (lastBackslashIndex > 0) {
          baseDir = executablePath.substring(0, lastBackslashIndex)
        }
      }
    }

    return `${baseDir}${sep()}addons`
  }

  // Otherwise use the regular mods folder path
  return getModsFolderPath(selectedEngineMod.value)
}
</script>

<style scoped>
.engine-selection-dialog {
  width: 700px;
  max-width: 90vw;
  max-height: 90vh;
  background-color: var(--theme-solid);
  color: var(--theme-text);
  border: var(--theme-border) 2px solid;
  backdrop-filter: blur(30px);
}

.engine-item {
  color: var(--theme-text) !important;
}

.selected-engine {
  background-color: var(--q-primary) !important;
  color: white !important;
}

.q-list {
  background-color: var(--theme-card);
  border: var(--theme-border) 1px solid;
}

.q-item {
  padding: 12px;
}

.q-item-label {
  font-weight: 500;
}

.q-item-label + .q-item-label {
  margin-top: 4px;
}

code {
  display: block;
  padding: 8px;
  background-color: var(--theme-card);
  border-radius: 4px;
  font-family: monospace;
  white-space: pre-wrap;
  word-break: break-all;
  border: var(--theme-border) 1px solid;
}

p {
  margin: 8px 0 16px;
}
</style>
