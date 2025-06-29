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
          {{ $t('app.modals.mod_type_selection.title') }}
        </div>
        <q-space />
        <q-btn v-close-popup icon="close" flat round dense @click="cancel" />
      </q-card-section>

      <q-card-section>
        <i18n-t tag="p" keypath="app.modals.mod_type_selection.description">
          <template #modName>
            {{ currentModName }}
          </template>
        </i18n-t>

        <div class="mod-types-grid">
          <!-- Standalone Executable Option -->
          <q-card
            class="mod-type-card"
            :class="{ selected: selectedType === 'executable' }"
            @click="selectedType = 'executable'"
          >
            <q-card-section class="text-center">
              <q-icon name="terminal" size="3rem" />
              <div class="mod-type-name phantom-font-display">
                {{ $t('app.modals.mod_type_selection.standalone') }}
              </div>
              <div class="mod-type-description">
                {{ $t('app.modals.mod_type_selection.standalone_description') }}
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
              <div class="mod-type-name phantom-font-display">Psych</div>
              <div class="mod-type-description">
                {{
                  engineAvailability.psychCount
                    ? $t('app.modals.mod_type_selection.psych_description')
                    : $t('app.modals.mod_type_selection.no_psych_installed')
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
                    ? $t('app.modals.mod_type_selection.vslice_description')
                    : $t('app.modals.mod_type_selection.no_vslice_installed')
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
                    ? $t('app.modals.mod_type_selection.fpsplus_description')
                    : $t('app.modals.mod_type_selection.no_fpsplus_installed')
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
                    ? $t('app.modals.mod_type_selection.codename_description')
                    : $t('app.modals.mod_type_selection.no_codename_installed')
                }}
              </div>
            </q-card-section>
          </q-card>
        </div>

        <div
          class="text-caption text-center"
          style="color: var(--theme-text-secondary)"
        >
          If you're unsure what type of mod this is, you should check this mod's
          description.
          <q-btn
            outline
            class="full-width q-mt-sm"
            label="Open Mod Details"
            icon="open_in_new"
            @click="handleOpenModDetails"
          />
        </div>

        <!-- Engine list selection (shown when a modpack type is selected) -->
        <div
          v-if="selectedType !== 'executable' && compatibleEngines.length > 0"
          class="q-mt-lg"
        >
          <p>
            {{
              $t('app.modals.mod_type_selection.select_where_to_install', {
                engine: formattedEngineName,
              })
            }}
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
                      `/images/engine_icons/${engine.engine?.engine_type}.webp`
                    "
                    onerror="this.src='/images/engine_icons/Psych.webp'"
                  />
                </q-avatar>
              </q-item-section>

              <q-item-section>
                <q-item-label
                  >{{ engine.name
                  }}<span
                    v-if="engine.version"
                    style="
                      color: var(--theme-text-secondary);
                      font-size: 0.75rem;
                    "
                    class="q-ml-xs"
                    >({{ engine.version }})</span
                  ></q-item-label
                >
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
                :label="$t('app.modals.engine_select.install_as_addon')"
              />
            </div>
            <p style="color: var(--theme-text-secondary)" class="q-mb-xs">
              {{ $t('app.modals.engine_select.will_be_installed_to') }}
            </p>
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
import { formatEngineName } from '@utils/index'
import { Mod } from '@main-types'

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
  modName: {
    type: String,
    required: false,
  },
})

const emit = defineEmits([
  'update:modelValue',
  'submit',
  'back',
  'cancel',
  'open-mod-details',
])

// Engine availability tracking
const engineAvailability = reactive({
  psychCount: 0,
  vanillaCount: 0,
  fpsPlusCount: 0,
  codenameCount: 0,
})

// Dialog state
const isOpen = ref(props.modelValue)

// Selection state
const selectedType = ref('executable')
const selectedEngine = ref<Mod | null>(null)
const compatibleEngines = ref<Mod[]>([])
const allEngines = ref<Mod[]>([])
const isAddon = ref(false)
const formattedEngineName = ref('')

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
  async val => {
    isOpen.value = val

    // When dialog opens, load all engines to check availability and reset state
    if (val) {
      loadAllEngines()
      isAddon.value = false
      // Initialize formatted engine name for default selection
      if (selectedType.value !== 'executable') {
        formattedEngineName.value = await formatEngineName(selectedType.value)
      }
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
    // Update formatted engine name
    formattedEngineName.value = await formatEngineName(newType)
  } else {
    // Clear selection when switching to executable type
    selectedEngine.value = null
    compatibleEngines.value = []
    formattedEngineName.value = ''
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
      allEngines.value = await invoke<Mod[]>('get_mods')
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
const getEngineType = (mod: Mod): string => {
  // Check engine structure
  if (mod.engine && mod.engine.engine_type) {
    return mod.engine.engine_type
  }

  // If no engine type found, return unknown
  return 'unknown'
}

const handleOpenModDetails = () => {
  emit('open-mod-details')
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
      let mods: Mod[] = []
      if (window.db && window.db.service) {
        mods = await window.db.service.getAllMods()
      } else {
        mods = await invoke<Mod[]>('get_mods')
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

// Function to get the mods folder path for an engine mod
const getModsFolderPath = (engineMod: Mod): string => {
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

// Watch for modName changes to populate with mod information
watch(
  () => props.modName,
  newModName => {
    if (newModName) {
      console.log('Received mod name in ModTypeSelectionModal:', newModName)
      currentModName.value = newModName
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
  border: 2px solid var(--theme-border);
  background-color: var(--theme-card);
}

.mod-type-card:not(.disabled-card):hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.mod-type-card.selected {
  border-color: var(--q-primary);
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
  background-color: var(--theme-card);
  border: 2px solid transparent !important;
}

.selected-engine {
  border: 2px solid var(--q-primary) !important;
  color: white;
  border-radius: 0.25rem;
}

.engine-path-caption {
  color: var(--theme-text-secondary) !important;
}

.selected-engine .q-item__label {
  color: white;
}

.q-list {
  border: none;
  margin-top: 8px;
  margin-bottom: 16px;
  border: 1px solid var(--theme-border);
  background-color: var(--theme-card);
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
