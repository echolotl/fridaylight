<template>
  <q-dialog
    v-model="isOpen"
    persistent
    transition-show="fade"
    transition-hide="fade"
  >
    <q-card class="mod-type-selection-modal phantom-font">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6 phantom-font-difficulty">
          Select Type for "{{ currentModName }}"
        </div>
        <q-space />
        <q-btn v-close-popup icon="close" flat round dense @click="cancel" />
      </q-card-section>

      <q-card-section>
        <p>What type of mod are you downloading?</p>

        <div class="mod-types-grid">
          <!-- Standalone Executable Option -->
          <q-card
            class="mod-type-card"
            :class="{ selected: selectedType === 'executable' }"
            @click="selectedType = 'executable'"
          >
            <q-card-section class="text-center">
              <q-icon name="desktop_windows" size="3rem" />
              <div class="mod-type-name phantom-font-display">Standalone</div>
              <div class="mod-type-description">
                A complete game with its own executable
              </div>
            </q-card-section>
          </q-card>

          <!-- Psych Engine Modpack Option -->
          <q-card
            class="mod-type-card"
            :class="{
              selected: selectedType === 'psych',
              'disabled-card': !engineAvailability.psychCount,
            }"
            @click="
              engineAvailability.psychCount ? (selectedType = 'psych') : null
            "
          >
            <q-card-section class="text-center">
              <img
                src="/images/engine_icons/Psych.webp"
                alt="Psych Engine"
                class="engine-icon"
              />
              <div class="mod-type-name phantom-font-display">Psych Engine</div>
              <div class="mod-type-description">
                {{
                  engineAvailability.psychCount
                    ? `A modpack for the Psych Engine`
                    : 'No Psych Engine installations found'
                }}
              </div>
            </q-card-section>
          </q-card>

          <!-- V-Slice Modpack Option -->
          <q-card
            class="mod-type-card"
            :class="{
              selected: selectedType === 'vanilla',
              'disabled-card': !engineAvailability.vanillaCount,
            }"
            @click="
              engineAvailability.vanillaCount
                ? (selectedType = 'vanilla')
                : null
            "
          >
            <q-card-section class="text-center">
              <img
                src="/images/engine_icons/Vanilla.webp"
                alt="V-Slice"
                class="engine-icon"
              />
              <div class="mod-type-name phantom-font-display">V-Slice</div>
              <div class="mod-type-description">
                {{
                  engineAvailability.vanillaCount
                    ? `A modpack for V-Slice`
                    : 'No V-Slice installations found'
                }}
              </div>
            </q-card-section>
          </q-card>

          <!-- FPS Plus Modpack Option -->
          <q-card
            class="mod-type-card"
            :class="{
              selected: selectedType === 'fps-plus',
              'disabled-card': !engineAvailability.fpsPlusCount,
            }"
            @click="
              engineAvailability.fpsPlusCount
                ? (selectedType = 'fps-plus')
                : null
            "
          >
            <q-card-section class="text-center">
              <img
                src="/images/engine_icons/Fps-plus.webp"
                alt="FPS Plus"
                class="engine-icon"
              />
              <div class="mod-type-name phantom-font-display">FPS Plus</div>
              <div class="mod-type-description">
                {{
                  engineAvailability.fpsPlusCount
                    ? `A modpack for FPS Plus`
                    : 'No FPS Plus installations found'
                }}
              </div>
            </q-card-section>
          </q-card>

          <!-- Codename Engine Modpack Option -->
          <q-card
            class="mod-type-card"
            :class="{
              selected: selectedType === 'codename',
              'disabled-card': !engineAvailability.codenameCount,
            }"
            @click="
              engineAvailability.codenameCount
                ? (selectedType = 'codename')
                : null
            "
          >
            <q-card-section class="text-center">
              <img
                src="/images/engine_icons/Codename.webp"
                alt="Codename Engine"
                class="engine-icon"
              />
              <div class="mod-type-name phantom-font-display">Codename</div>
              <div class="mod-type-description">
                {{
                  engineAvailability.codenameCount
                    ? `A modpack for the Codename Engine`
                    : 'No Codename Engine installations found'
                }}
              </div>
            </q-card-section>
          </q-card>
        </div>

        <!-- Engine list selection (shown when a modpack type is selected) -->
        <div
          v-if="selectedType !== 'executable' && compatibleEngines.length > 0"
          class="q-mt-lg"
        >
          <p>
            Select which {{ formatEngineType(selectedType) }} installation to
            install this modpack to:
          </p>

          <q-list bordered separator class="rounded-borders">
            <q-item
              v-for="engine in compatibleEngines"
              :key="engine.id"
              v-ripple
              clickable
              :active="selectedEngine?.id === engine.id"
              active-class="selected-engine"
              class="engine-item"
              @click="selectedEngine = engine"
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
          <!-- Installation path preview -->
          <div v-if="selectedEngine" class="text-caption q-mt-sm">
            <!-- Codename Engine Addon Option -->
            <div v-if="isCodename" class="q-pt-none q-mb-md">
              <q-toggle
                v-model="isAddon"
                label="Install as Addon (will run on all mods)"
              />
            </div>
            <p>The modpack will be installed to:</p>
            <code>{{ getInstallPath() }}</code>
          </div>
        </div>
      </q-card-section>

      <q-card-actions align="right">
        <q-btn flat label="Back" color="primary" @click="back" />
        <q-btn
          flat
          label="Download"
          color="primary"
          :disable="!isFormValid"
          @click="onSubmit"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, reactive } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { sep } from '@tauri-apps/api/path'

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
  modData: {
    type: Object,
    required: true,
  },
})

const emit = defineEmits(['update:modelValue', 'submit', 'back', 'cancel'])

// Engine availability tracking
const engineAvailability = reactive({
  psychCount: 0,
  vanillaCount: 0,
  fpsPlusCount: 0,
  codenameCount: 0,
})

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

// Dialog state
const isOpen = ref(props.modelValue)

// Selection state
const selectedType = ref('executable')
const selectedEngine = ref<EngineMod | null>(null)
const compatibleEngines = ref<EngineMod[]>([])
const allEngines = ref<EngineMod[]>([])
const isAddon = ref(false)

// Computed property to check if selected type is Codename Engine
const isCodename = computed(() => {
  return selectedType.value === 'codename'
})

// Computed property to check if form is valid
const isFormValid = computed(() => {
  if (selectedType.value === 'executable') {
    return true
  }
  return !!selectedEngine.value
})

// Watch for changes in props
watch(
  () => props.modelValue,
  val => {
    isOpen.value = val

    // When dialog opens, load all engines to check availability and reset state
    if (val) {
      loadAllEngines()
      isAddon.value = false
    }
  }
)

// Watch for changes in isOpen
watch(isOpen, val => {
  emit('update:modelValue', val)
})

// Watch for changes in selectedType
watch(selectedType, async newType => {
  if (newType !== 'executable') {
    await loadCompatibleEngines(newType)
  } else {
    // Clear selection when switching to executable type
    selectedEngine.value = null
    compatibleEngines.value = []
  }

  // Reset addon flag when changing type
  if (newType !== 'codename') {
    isAddon.value = false
  }
})

// Load all engines on component mount to check availability
onMounted(() => {
  loadAllEngines()
})

// Load all engines to determine which engine types are available
const loadAllEngines = async () => {
  try {
    // Reset counts
    engineAvailability.psychCount = 0
    engineAvailability.vanillaCount = 0
    engineAvailability.fpsPlusCount = 0
    engineAvailability.codenameCount = 0

    // Fetch all mods
    if (window.db && window.db.service) {
      allEngines.value = await window.db.service.getAllMods()
    } else {
      allEngines.value = await invoke<EngineMod[]>('get_mods')
    }

    // Count engines of each type
    allEngines.value.forEach(mod => {
      const engineType = getEngineType(mod).toLowerCase()
      switch (engineType) {
        case 'psych':
          engineAvailability.psychCount++
          break
        case 'vanilla':
          engineAvailability.vanillaCount++
          break
        case 'fps-plus':
          engineAvailability.fpsPlusCount++
          break
        case 'codename':
          engineAvailability.codenameCount++
          break
      }
    })

    console.log('Engine availability:', engineAvailability)
  } catch (error) {
    console.error('Failed to load all engines:', error)
    allEngines.value = []
  }
}

// Get engine type from mod object
const getEngineType = (mod: EngineMod): string => {
  // Check engine structure
  if (mod.engine && mod.engine.engine_type) {
    return mod.engine.engine_type
  }

  // If no engine type found, return unknown
  return 'unknown'
}

// Load compatible engines based on the selected type
const loadCompatibleEngines = async (engineType: string) => {
  try {
    // Filter allEngines if we've already loaded them
    if (allEngines.value.length > 0) {
      compatibleEngines.value = allEngines.value.filter(mod => {
        const modEngineType = getEngineType(mod).toLowerCase()
        return modEngineType === engineType.toLowerCase()
      })
    } else {
      // Fetch all mods if not already loaded
      let mods: EngineMod[] = []
      if (window.db && window.db.service) {
        mods = await window.db.service.getAllMods()
      } else {
        mods = await invoke<EngineMod[]>('get_mods')
      }

      // Filter mods by engine type
      compatibleEngines.value = mods.filter(mod => {
        const modEngineType = getEngineType(mod).toLowerCase()
        return modEngineType === engineType.toLowerCase()
      })
    }

    // Reset selected engine if not compatible
    if (compatibleEngines.value.length > 0) {
      selectedEngine.value = compatibleEngines.value[0]
    } else {
      selectedEngine.value = null
    }
  } catch (error) {
    console.error('Failed to get compatible engine mods:', error)
    compatibleEngines.value = []
    selectedEngine.value = null
  }
}

// Format engine type for display
const formatEngineType = (engineType: string | null): string => {
  if (!engineType) return 'Unknown'

  switch (engineType.toLowerCase()) {
    case 'psych':
      return 'Psych Engine'
    case 'vanilla':
      return 'V-Slice'
    case 'fps-plus':
      return 'FPS Plus'
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
  if (!selectedEngine.value) return 'Unknown path'

  // If it's a Codename Engine addon, use addons folder instead of mods
  if (isCodename.value && isAddon.value) {
    const basePath = selectedEngine.value.path
    const executablePath = selectedEngine.value.executable_path || ''

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
  return getModsFolderPath(selectedEngine.value)
}

// Form submission
const onSubmit = () => {
  if (isFormValid.value) {
    emit('submit', {
      modType: selectedType.value,
      engineMod: selectedEngine.value,
      isAddon: isCodename.value ? isAddon.value : false,
    })
  }
}

// Go back to the previous modal
const back = () => {
  emit('back')
}

// Cancel form
const cancel = () => {
  emit('cancel')
}

// Type declaration for window.db
declare global {
  interface Window {
    db: any
  }
}

// Watch for modData changes to populate with mod information
watch(
  () => props.modData,
  newModData => {
    if (newModData && Object.keys(newModData).length > 0) {
      console.log('Received mod data in ModTypeSelectionModal:', newModData)
      currentModName.value = newModData.name || 'Unknown Mod'
    }
  }
)

const currentModName = ref<string>('Unknown Mod')
</script>

<style scoped>
.mod-type-selection-modal {
  width: 700px;
  max-width: 90vw;
  background-color: var(--theme-solid);
  color: var(--theme-text);
  border: var(--theme-border) 2px solid;
  backdrop-filter: blur(30px);
  max-height: 90vh;
  overflow-y: auto;
}

.mod-types-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 12px;
  margin-bottom: 16px;
}

.mod-type-card {
  cursor: pointer;
  transition: all 0.3s ease;
  border: 2px solid transparent;
  background-color: var(--theme-card);
}

.mod-type-card:not(.disabled-card):hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.mod-type-card.selected {
  border-color: var(--q-primary);
  background-color: var(--theme-solid);
}

.disabled-card {
  opacity: 0.6;
  cursor: not-allowed;
}

.engine-icon {
  width: 3rem;
  height: 3rem;
  margin-bottom: 8px;
  object-fit: contain;
}

.mod-type-name {
  font-size: 1.2rem;
  margin-bottom: 4px;
}

.mod-type-description {
  font-size: 0.9rem;
  color: var(--theme-text-secondary);
}

/* Engine list styling */
.engine-item {
  color: var(--theme-text);
  border-radius: 1rem;
}

.selected-engine {
  background-color: var(--q-primary);
  color: white;
}

.engine-path-caption {
  color: var(--theme-text-secondary);
}

.selected-engine .q-item__label {
  color: white;
}

.q-list {
  border: none;
  margin-top: 8px;
  margin-bottom: 16px;
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

/* Field styling */
:deep(.q-field__native),
:deep(.q-field__input) {
  color: var(--theme-text) !important;
}

:deep(.q-field__label) {
  color: var(--theme-text-secondary) !important;
}

:deep(.q-field__marginal) {
  color: var(--theme-text-secondary);
}

:deep(input::placeholder) {
  color: var(--theme-text-secondary) !important;
}

:deep(.q-field__messages) {
  color: var(--theme-text-secondary) !important;
}
</style>
