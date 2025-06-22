<template>
  <q-dialog v-model="showModal" persistent>
    <q-card class="settings-modal phantom-font">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6 phantom-font-difficulty">Mod Settings</div>
        <q-space />
        <q-btn v-close-popup icon="close" flat round dense />
      </q-card-section>

      <div v-if="mod" class="settings-layout">
        <!-- Sidebar Navigation -->
        <div class="settings-sidebar">
          <q-list padding>
            <q-item
              v-for="section in modSettingsSections"
              :key="section.id"
              clickable
              :active="activeSection === section.id"
              active-class="settings-active-item"
              @click="activeSection = section.id"
            >
              <q-item-section avatar>
                <q-icon :name="section.icon" />
              </q-item-section>
              <q-item-section>
                {{ section.label }}
              </q-item-section>
            </q-item>
          </q-list>
        </div>

        <!-- Main Content Area -->
        <q-scroll-area class="settings-content">
          <!-- General Section -->
          <q-card-section v-show="activeSection === 'general'">
            <div class="text-subtitle1 q-mb-md">General Information</div>

            <q-input
              v-model="form.name"
              label="Mod Name"
              outlined
              class="q-mb-md"
            />

            <q-input
              v-model="form.description"
              type="textarea"
              label="Description"
              outlined
              class="q-mb-md"
              autogrow
              placeholder="Enter mod description"
            />
            <q-input
              v-model="form.version"
              label="Version"
              outlined
              class="q-mb-md"
              placeholder="e.g. 1.0.0"
            />
            <div class="gamebanana-section q-mt-lg">
              <q-separator class="q-my-md" />
              <div class="text-subtitle1 q-mb-md">
                <q-icon
                  name="M24,6v10.5h-1.5v3h-1.5v1.5h-1.5v1.5h-3v1.5H6v-1.5h-3v-1.5h-1.5v-1.5H0v-4.5h9v-1.5h4.5v-1.5h1.5v-4.5h1.5V3h-1.5V0h4.5v3h1.5v1.5h1.5v1.5h1.5Z"
                  size="sm"
                  class="q-mr-sm"
                />GameBanana Integration
              </div>
              <q-input
                :model-value="gameBananaInfo?.url || ''"
                label="GameBanana URL"
                outlined
                class="q-mb-md"
                placeholder="https://gamebanana.com/mods/..."
                @update:model-value="updateGameBananaUrl"
              />
              <div v-if="gameBananaInfo && gameBananaInfo.url" class="flex">
                <q-btn
                  label="Sync with Gamebanana"
                  class="flex-grow-1"
                  icon="sync"
                  flat
                  :loading="gamebananaSyncing"
                  @click="syncWithGameBanana"
                />
                <q-btn
                  label="View on GameBanana"
                  class="flex-grow-1 q-ml-sm"
                  icon="open_in_new"
                  flat
                  @click="openUrl(gameBananaInfo.url)"
                />
                <div
                  class="text-caption q-mt-sm"
                  style="color: var(--theme-text-secondary)"
                >
                  Syncing with GameBanana will fetch the latest metadata, and
                  will overwrite any existing metadata. You'll still need to
                  save the mod settings to apply changes.
                  <br />
                  The version you have installed will not change, but the
                  version displayed might.
                </div>
              </div>
            </div>

            <div class="danger-zone q-mt-lg">
              <q-separator class="q-my-md" />
              <div class="text-subtitle1 q-mb-md">Danger Zone</div>
              <q-btn
                color="negative"
                icon="delete"
                label="Remove Mod"
                class="full-width"
                outline
                @click="showRemoveDialog = true"
              />
              <div
                class="text-caption q-mt-sm"
                style="color: var(--theme-text-secondary)"
              >
                This will remove the mod from Fridaylight but keep the files on
                your system.
              </div>

              <q-btn
                color="negative"
                icon="delete_forever"
                label="Delete Mod Files"
                class="full-width q-mt-md"
                outline
                @click="showSuperDeleteDialog = true"
              />
              <div
                class="text-caption q-mt-sm"
                style="color: var(--theme-text-secondary)"
              >
                This will
                <span class="text-negative text-bold">permanently delete</span>
                the mod folder and all its contents from your computer. This
                action cannot be undone!
              </div>
            </div>
          </q-card-section>

          <!-- Location Section -->
          <q-card-section v-show="activeSection === 'location'">
            <div class="text-subtitle1 q-mb-md">File Locations</div>

            <q-input
              v-model="form.path"
              label="Folder Location"
              outlined
              readonly
              class="q-mb-md"
            >
              <template #append>
                <div v-if="form.path" class="icon">
                  <q-btn
                    icon="open_in_new"
                    round
                    flat
                    @click="handleOpenFileLocationClick(form.path)"
                  />
                  <q-btn
                    round
                    flat
                    icon="folder"
                    @click="handleChangeFolderClick"
                  />
                </div>
              </template>
            </q-input>

            <q-input
              v-model="form.executable_path"
              label="Executable Path"
              outlined
              readonly
              class="q-mb-md"
            >
              <template v-if="form.executable_path" #append>
                <div class="icon">
                  <q-btn
                    icon="open_in_new"
                    round
                    flat
                    @click="handleOpenFileLocationClick(form.executable_path)"
                  />
                  <q-btn
                    round
                    flat
                    icon="file_open"
                    @click="handleSelectExecutableClick"
                  />
                </div>
              </template>
            </q-input>
          </q-card-section>

          <!-- Engine Section -->
          <q-card-section v-show="activeSection === 'engine'">
            <div class="text-subtitle1 q-mb-md">Engine Settings</div>
            <q-select
              v-model="form.engine.engine_type"
              :options="engineTypes"
              label="Engine Type"
              outlined
              class="q-mb-md selector"
              emit-value
              map-options
              hint="The type of engine this mod uses"
              popup-content-class="phantom-font"
              popup-content-style="background-color: var(--theme-solid); color: var(--theme-text);"
            />

            <q-input
              v-model="form.engine.engine_name"
              label="Custom Engine Name"
              outlined
              class="q-mb-md"
              placeholder="e.g. My Custom Psych Fork"
              hint="A custom name for the engine (optional)"
            />

            <div class="q-mb-md">
              <div class="text-subtitle2 q-mb-sm">Engine Icon</div>
              <div
                v-if="engineIconPreview || form.engine?.engine_icon"
                class="engine-icon-preview"
              >
                <img
                  :src="engineIconPreview || form.engine?.engine_icon"
                  alt="Engine Icon"
                />
              </div>
              <div v-else class="icon-placeholder">
                <q-icon name="block" size="48px" />
              </div>

              <q-file
                v-model="engineIconFile"
                label="Set Engine Icon"
                outlined
                accept=".jpg, .jpeg, .png, .webp, .ico"
                class="q-mt-sm"
                @update:model-value="handleEngineIconFileChange"
              >
                <template #prepend>
                  <div class="icon">
                    <q-icon name="image" />
                  </div>
                </template>
              </q-file>

              <q-btn
                v-if="engineIconPreview || form.engine?.engine_icon"
                flat
                color="negative"
                label="Remove Icon"
                class="q-mt-sm"
                @click="removeEngineIcon"
              />
            </div>

            <q-separator class="q-my-md" />

            <div class="q-mb-md">
              <q-toggle
                v-model="form.engine!.mods_folder"
                label="Has Mods Folder"
                color="primary"
                hint="Does this engine support a mods folder structure?"
              />
            </div>

            <q-input
              v-if="form.engine?.mods_folder"
              v-model="form.engine!.mods_folder_path"
              label="Mods Folder Path"
              outlined
              class="q-mb-md"
              placeholder="e.g. mods"
              hint="Relative path to the mods folder from the executable directory"
            />
          </q-card-section>

          <!-- Visuals Section -->
          <q-card-section v-show="activeSection === 'visuals'">
            <div class="text-subtitle1 q-mb-md">Appearance</div>

            <div class="icon-upload q-mb-md">
              <div class="text-subtitle2 q-mb-sm">Mod Icon</div>
              <div v-if="iconPreview || form.icon_data" class="icon-preview">
                <img :src="iconPreview || form.icon_data" alt="Mod Icon" />
              </div>
              <div v-else class="icon-placeholder">
                <q-icon name="image" size="48px" />
              </div>
              <q-file
                v-model="iconFile"
                label="Set Mod Icon"
                outlined
                accept=".jpg, .jpeg, .png, .webp, .ico"
                class="q-mt-sm"
                @update:model-value="handleIconFileChange"
              >
                <template #prepend>
                  <div class="icon">
                    <q-icon name="image" />
                  </div>
                </template>
              </q-file>
              <q-btn
                v-if="iconPreview || form.icon_data"
                flat
                color="negative"
                label="Remove Icon"
                class="q-mt-sm"
                @click="removeIcon"
              />
            </div>

            <div class="banner-upload q-mb-md">
              <div class="text-subtitle2 q-mb-sm">Banner Image</div>
              <div
                v-if="bannerPreview || form.banner_data"
                class="banner-preview"
              >
                <img
                  :src="bannerPreview || form.banner_data"
                  alt="Banner Preview"
                />
              </div>
              <div v-else class="banner-placeholder">
                <q-icon name="image" size="48px" />
                <div>No banner image</div>
              </div>
              <q-file
                v-model="bannerFile"
                label="Set Banner Image"
                outlined
                accept=".jpg, .jpeg, .png"
                class="q-mt-sm"
                @update:model-value="handleBannerFileChange"
              >
                <template #prepend>
                  <div class="icon">
                    <q-icon name="panorama" />
                  </div>
                </template>
              </q-file>
              <q-btn
                v-if="bannerPreview || form.banner_data"
                flat
                color="negative"
                label="Remove Banner"
                class="q-mt-sm"
                @click="removeBanner"
              />
            </div>

            <div class="logo-upload q-mb-md">
              <div class="text-subtitle2 q-mb-sm">
                Logo Image (Replaces Title)
              </div>
              <div v-if="logoPreview || form.logo_data" class="logo-preview">
                <img
                  :src="logoPreview || form.logo_data || ''"
                  alt="Logo Preview"
                />
              </div>
              <div v-else class="logo-placeholder">
                <q-icon name="image" size="48px" />
                <div>No logo image</div>
              </div>
              <q-file
                v-model="logoFile"
                label="Set Logo Image"
                outlined
                accept=".jpg, .jpeg, .png"
                class="q-mt-sm"
                @update:model-value="handleLogoFileChange"
              >
                <template #prepend>
                  <div class="icon">
                    <q-icon name="image" />
                  </div>
                </template>
              </q-file>
              <q-btn
                v-if="logoPreview || form.logo_data"
                flat
                color="negative"
                label="Remove Logo"
                class="q-mt-sm"
                @click="removeLogo"
              />

              <q-select
                v-if="logoPreview || form.logo_data"
                v-model="form.logo_position"
                :options="logoPositionOptions"
                label="Logo Position"
                popup-content-class="phantom-font"
                popup-content-style="background-color: var(--theme-solid); color: var(--theme-text);"
                outlined
                class="q-mt-md phantom-font selector"
                emit-value
                map-options
                hint="Position of the logo in the banner"
              />
            </div>
          </q-card-section>
        </q-scroll-area>
      </div>

      <q-card-actions align="right">
        <q-btn
          v-close-popup
          flat
          label="Cancel"
          color="primary"
          @click="cancel"
        />
        <q-btn v-close-popup flat label="Save" color="primary" @click="save" />
      </q-card-actions>
    </q-card>
  </q-dialog>

  <!-- Remove Mod Confirmation Dialog -->
  <MessageDialog
    v-model="showRemoveDialog"
    title="Remove Mod"
    icon="warning"
    icon-color="negative"
    confirm-label="Remove Mod"
    confirm-color="negative"
    @confirm="removeMod"
  >
    <div class="text-h6">{{ form.name }}</div>
    <div class="text-caption">{{ form.path }}</div>
    <p class="text-body2 q-mt-sm">
      Are you sure you want to remove this mod from Fridaylight? The mod files
      will remain on your system.
    </p>
  </MessageDialog>

  <!-- Super Delete Mod Confirmation Dialog -->
  <MessageDialog
    v-model="showSuperDeleteDialog"
    title="Delete Mod"
    icon="delete_forever"
    icon-color="negative"
    confirm-label="Delete Forever"
    confirm-color="negative"
    @confirm="superDeleteMod"
  >
    <div class="text-h6">{{ form.name }}</div>
    <div class="text-caption">{{ form.path }}</div>
    <p class="text-body2 q-mt-sm">
      This will PERMANENTLY DELETE the mod folder and all its contents from your
      computer. This action cannot be undone!
    </p>
  </MessageDialog>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { Mod, ModInfoGBData } from '@main-types'
import { formatEngineName } from '../../utils'
import MessageDialog from './MessageDialog.vue'
import { revealItemInDir, openUrl } from '@tauri-apps/plugin-opener'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
  mod: {
    type: Object as () => Mod | null,
    default: null,
  },
})

const emit = defineEmits([
  'update:modelValue',
  'save',
  'change-folder',
  'select-executable',
  'delete-mod',
  'super-delete-mod',
])

const form = ref<Mod>({
  id: '',
  name: '',
  path: '',
  executable_path: '',
  icon_data: '',
  display_order: 0,
  banner_data: '',
  logo_data: '',
  version: '',
  engine: {
    engine_type: '',
    engine_name: '',
    engine_icon: '',
    mods_folder: true,
    mods_folder_path: '',
  },
})

const bannerFile = ref<File | null>(null)
const bannerPreview = ref<string | null>(null)
const logoFile = ref<File | null>(null)
const logoPreview = ref<string | null>(null)
const engineIconFile = ref<File | null>(null)
const engineIconPreview = ref<string | null>(null)
const iconFile = ref<File | null>(null)
const iconPreview = ref<string | null>(null)

// GameBanana metadata state
const gameBananaInfo = ref<ModInfoGBData | null>(null)
const metadataData = ref<any>(null)
const metadataLoaded = ref(false)
const gamebananaSyncing = ref(false)

const engineTypes = [
  { label: 'Vanilla', value: 'vanilla' },
  { label: 'Psych Engine', value: 'psych' },
  { label: 'Codename Engine', value: 'codename' },
  { label: 'FPS Plus', value: 'fps-plus' },
  { label: 'Kade Engine', value: 'kade' },
  { label: 'Pre-VSlice', value: 'pre-vslice' },
  { label: 'Other', value: 'other' },
]

const logoPositionOptions = [
  { label: 'Bottom Left', value: 'left_bottom' },
  { label: 'Middle Left', value: 'left_middle' },
  { label: 'Middle', value: 'middle' },
]

const showModal = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
})

const modSettingsSections = [
  { id: 'general', label: 'General', icon: 'info' },
  { id: 'location', label: 'Location', icon: 'folder' },
  { id: 'engine', label: 'Engine', icon: 'code' },
  { id: 'visuals', label: 'Appearance', icon: 'palette' },
]

const activeSection = ref('general')
const showRemoveDialog = ref(false)
const showSuperDeleteDialog = ref(false)

// Reset form when modal is opened
watch(
  () => props.modelValue,
  async newVal => {
    if (newVal && props.mod) {
      // Clone the mod object to form
      form.value = JSON.parse(JSON.stringify(props.mod))
      // Ensure engine object exists in the cloned form
      if (!form.value.engine) {
        form.value.engine = {
          engine_type: 'unknown',
          engine_name: '',
          engine_icon: '',
          mods_folder: false,
          mods_folder_path: '',
        }
      }
      // Initialize preview values with existing data
      bannerPreview.value = form.value.banner_data || null
      logoPreview.value = form.value.logo_data || null
      engineIconPreview.value = form.value.engine?.engine_icon || null
      iconPreview.value = form.value.icon_data || null

      bannerFile.value = null // Clear file input ref
      logoFile.value = null // Clear file input ref
      engineIconFile.value = null // Clear file input ref
      iconFile.value = null // Clear file input ref
      activeSection.value = 'general' // Reset to general tab

      // Load metadata for GameBanana info
      await loadMetadataData()
    }
  }
)

// Load metadata and GameBanana info when modal opens
const loadMetadataData = async () => {
  if (!props.mod?.path) return

  try {
    const metadata = await invoke('get_mod_metadata', {
      modPath: props.mod.path,
    })
    metadataData.value = metadata

    // Extract GameBanana info from metadata
    if (
      metadata &&
      typeof metadata === 'object' &&
      (metadata as any).gamebanana
    ) {
      gameBananaInfo.value = (metadata as any).gamebanana
    } else {
      gameBananaInfo.value = null
    }

    metadataLoaded.value = true
  } catch (error) {
    console.warn('Failed to load metadata data:', error)
    metadataLoaded.value = true
  }
}

// Save GameBanana info to metadata
const saveGameBananaMetadata = async () => {
  if (!props.mod?.path || !metadataData.value) return

  try {
    const updatedMetadata = { ...metadataData.value }

    if (gameBananaInfo.value) {
      updatedMetadata.gamebanana = gameBananaInfo.value
    } else {
      delete updatedMetadata.gamebanana
    }

    await invoke('save_mod_metadata', {
      modPath: props.mod.path,
      metadata: updatedMetadata,
    })
  } catch (error) {
    console.warn('Failed to save GameBanana metadata:', error)
  }
}

const handleBannerFileChange = (file: File | null) => {
  bannerFile.value = file // Store the file reference
  if (file) {
    const reader = new FileReader()
    reader.onload = e => {
      bannerPreview.value = e.target?.result as string
    }
    reader.readAsDataURL(file)
  } else {
    bannerPreview.value = null
    // If file is cleared, explicitly set form data to undefined for save logic
    form.value.banner_data = undefined
  }
}

const handleLogoFileChange = (file: File | null) => {
  logoFile.value = file // Store the file reference
  if (file) {
    const reader = new FileReader()
    reader.onload = e => {
      logoPreview.value = e.target?.result as string
    }
    reader.readAsDataURL(file)
  } else {
    logoPreview.value = null
  }
}

const handleEngineIconFileChange = (file: File | null) => {
  engineIconFile.value = file // Store the file reference
  if (file) {
    const reader = new FileReader()
    reader.onload = e => {
      engineIconPreview.value = e.target?.result as string
    }
    reader.readAsDataURL(file)
  } else {
    engineIconPreview.value = null
    // Don't clear form.value.engine.engine_icon here, handle removal in removeEngineIcon or save()
  }
}

const handleIconFileChange = (file: File | null) => {
  iconFile.value = file // Store the file reference
  if (file) {
    const reader = new FileReader()
    reader.onload = e => {
      iconPreview.value = e.target?.result as string
    }
    reader.readAsDataURL(file)
  } else {
    iconPreview.value = null
    // If file is cleared, explicitly set form data to undefined for save logic
    form.value.icon_data = undefined
  }
}

const removeLogo = () => {
  form.value.logo_data = null
  logoPreview.value = null
  logoFile.value = null // Clear the file input ref
}

const removeBanner = () => {
  form.value.banner_data = undefined
  bannerPreview.value = null
  bannerFile.value = null // Clear the file input ref
}

const removeEngineIcon = () => {
  if (form.value.engine) {
    form.value.engine.engine_icon = ''
  }
  engineIconPreview.value = null
  engineIconFile.value = null // Clear the file input ref
}

const removeIcon = () => {
  form.value.icon_data = undefined
  iconPreview.value = null
  iconFile.value = null // Clear the file input ref
}

const removeMod = () => {
  console.log('Removing mod:', form.value.name)
  // Close the dialog
  showRemoveDialog.value = false
  // Close the settings modal
  showModal.value = false
  // Emit the event to the parent component to handle the actual deletion
  emit('delete-mod', props.mod?.id)
}

const superDeleteMod = () => {
  console.log('Super deleting mod:', form.value.name)
  // Close the dialog
  showSuperDeleteDialog.value = false
  // Close the settings modal
  showModal.value = false
  // Emit the event to the parent component to handle the actual deletion
  emit('super-delete-mod', props.mod?.id)
}

const updateGameBananaUrl = (url: string | number | null) => {
  const urlString = url?.toString() || ''
  if (!gameBananaInfo.value) {
    gameBananaInfo.value = {
      id: parseInt(urlString.split('/').pop() || '0') || 0,
      url: urlString,
      model_type: formatGameBananaModelType(urlString.split('/')[3] || 'mods'),
    }
    console.log('Initialized GameBanana info:', gameBananaInfo.value)
  } else {
    gameBananaInfo.value = {
      id: parseInt(urlString.split('/').pop() || '0') || 0,
      url: urlString,
      model_type: formatGameBananaModelType(urlString.split('/')[3] || 'mods'),
    }
    console.log('Updated GameBanana info:', gameBananaInfo.value)
  }
}

const formatGameBananaModelType = (modelType: string) => {
  switch (modelType.toLowerCase()) {
    case 'mods':
      return 'Mod'
    case 'wips':
      return 'Wip'
    case 'tools':
      return 'Tool'
    default:
      return 'Mod'
  }
}

const syncWithGameBanana = async () => {
  if (!gameBananaInfo.value || !gameBananaInfo.value.url) {
    console.warn('No GameBanana URL set, cannot sync')
    return
  }
  gamebananaSyncing.value = true
  try {
    const response = await invoke<any>('get_mod_info_command', {
      modId: gameBananaInfo.value.id,
      modelType: gameBananaInfo.value.model_type,
    })

    // Update form with the fetched data
    form.value.name = response?._sName || form.value.name
    form.value.description = response?._sDescription || form.value.description
    form.value.version = response?._sVersion || form.value.version
    console.log('GameBanana sync response:', response)
    gamebananaSyncing.value = false
  } catch (error) {
    console.error('Failed to sync with GameBanana:', error)
    gamebananaSyncing.value = false
  }
}

const save = async () => {
  const updatedMod = { ...form.value }

  // Handle Banner Image
  if (bannerPreview.value) {
    updatedMod.banner_data = bannerPreview.value
  }

  // Handle Logo Image
  if (logoPreview.value) {
    updatedMod.logo_data = logoPreview.value
  } else if (logoFile.value === null && form.value.logo_data === null) {
    updatedMod.logo_data = undefined
  }
  // Otherwise preserve existing logo_data

  // Handle Engine Icon Image
  if (engineIconPreview.value && updatedMod.engine) {
    updatedMod.engine.engine_icon = engineIconPreview.value
  } else if (
    engineIconFile.value === null &&
    updatedMod.engine &&
    form.value.engine?.engine_icon === ''
  ) {
    updatedMod.engine.engine_icon = ''
  }

  // Handle Mod Icon Image
  if (iconPreview.value) {
    updatedMod.icon_data = iconPreview.value
  } else if (iconFile.value === null && form.value.icon_data === null) {
    updatedMod.icon_data = undefined
  }
  // Otherwise preserve existing icon_data

  // First, make sure engine object exists
  if (!updatedMod.engine) {
    updatedMod.engine = {
      engine_type: 'unknown',
      engine_name: await formatEngineName('unknown'),
      engine_icon: '',
      mods_folder: false,
      mods_folder_path: '',
    }
  } else {
    const defaultName = await formatEngineName(updatedMod.engine.engine_type)
    const originalDefaultName = await formatEngineName(
      props.mod?.engine?.engine_type || 'unknown'
    )
    if (
      !updatedMod.engine.engine_name ||
      updatedMod.engine.engine_name === originalDefaultName
    ) {
      updatedMod.engine.engine_name = defaultName
    }
  }

  // Save GameBanana metadata separately
  await saveGameBananaMetadata()

  console.info(
    'MODAL: Emitting save with mod data:',
    JSON.stringify(updatedMod)
  )

  emit('save', updatedMod)
}
const cancel = () => {
  bannerPreview.value = null
  logoPreview.value = null
  bannerFile.value = null
  logoFile.value = null
  engineIconPreview.value = null
  engineIconFile.value = null
  iconPreview.value = null
  iconFile.value = null
}

const handleChangeFolderClick = () => {
  emit('change-folder', (newPath: string) => {
    form.value.path = newPath
  })
}

const handleSelectExecutableClick = () => {
  emit('select-executable', (newExecutablePath: string) => {
    form.value.executable_path = newExecutablePath
  })
}

const handleOpenFileLocationClick = async (path: string) => {
  await revealItemInDir(path)
}
</script>

<style scoped>
.settings-modal {
  width: 90vw;
  height: 90vh;
  max-width: 90vw;
  max-height: 90vh;
  background-color: var(--theme-solid);
  color: var(--theme-text);
  border: var(--theme-border) 2px solid;
  backdrop-filter: blur(30px);
  scrollbar-width: none;
}

.settings-layout {
  display: flex;
  height: calc(100% - 100px); /* Account for header and footer */
  overflow: hidden;
}

.settings-sidebar {
  width: 200px;
  border-right: 1px solid var(--theme-border);
  overflow-y: auto;
}

.settings-content {
  flex: 1;
  overflow-y: auto;
  padding-right: 8px;
}

.settings-active-item {
  background-color: var(--q-primary);
  color: white;
}

.settings-sidebar .q-icon {
  color: var(--theme-text-secondary);
}

.settings-active-item .q-icon {
  color: white;
}

.banner-upload,
.logo-upload,
.icon-upload {
  display: flex;
  flex-direction: column;
}

.banner-preview img,
.logo-preview img {
  width: 100%;
  height: auto;
  max-height: 150px;
  object-fit: contain;
  border-radius: 4px;
}

.icon-preview img {
  width: 64px;
  height: 64px;
  object-fit: contain;
  border-radius: 4px;
  image-rendering: pixelated;
}

.banner-placeholder,
.logo-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--theme-text-secondary);
  border: 1px dashed var(--theme-border);
  border-radius: 4px;
  padding: 16px;
}

.engine-icon-preview img {
  width: 64px;
  height: 64px;
  object-fit: contain;
  border-radius: 4px;
}

.icon-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--theme-text-secondary);
  border: 1px dashed var(--theme-border);
  border-radius: 4px;
  padding: 16px;
  width: 64px;
  height: 64px;
}

.q-field :deep(.q-field__label) {
  color: var(--theme-text) !important;
}

.q-field.q-field--outlined :deep(.q-field__control) {
  color: var(--theme-text);
}
:deep(.q-field__native) {
  color: var(--theme-text);
}
:deep(.q-field__messages) {
  color: var(--theme-text-secondary);
}
.list-item {
  background-color: var(--theme-surface);
}
</style>
