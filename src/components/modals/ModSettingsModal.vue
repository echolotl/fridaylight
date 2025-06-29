<template>
  <q-dialog v-model="showModal" persistent>
    <q-card class="settings-modal phantom-font">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6 phantom-font-difficulty">
          {{ $t('app.modals.mod_settings.title') }}
        </div>
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
            <div class="text-subtitle1 q-mb-md">
              {{ $t('app.modals.mod_settings.general.title') }}
            </div>

            <q-input
              v-model="form.name"
              :label="$t('app.modals.mod_settings.general.mod_name')"
              outlined
              class="q-mb-md"
            />

            <q-input
              v-model="form.description"
              type="textarea"
              :label="$t('app.modals.mod_settings.general.mod_description')"
              outlined
              class="q-mb-md"
              autogrow
              :placeholder="
                $t(
                  'app.modals.mod_settings.general.mod_description_placeholder'
                )
              "
            />
            <q-input
              v-model="form.version"
              :label="$t('app.modals.mod_settings.general.mod_version')"
              outlined
              class="q-mb-md"
              :placeholder="
                $t('app.modals.mod_settings.general.mod_version_placeholder')
              "
            />
            <div class="gamebanana-section q-mt-lg">
              <q-separator class="q-my-md" />
              <div class="text-subtitle1 q-mb-md">
                <q-icon
                  name="M24,6v10.5h-1.5v3h-1.5v1.5h-1.5v1.5h-3v1.5H6v-1.5h-3v-1.5h-1.5v-1.5H0v-4.5h9v-1.5h4.5v-1.5h1.5v-4.5h1.5V3h-1.5V0h4.5v3h1.5v1.5h1.5v1.5h1.5Z"
                  size="sm"
                  class="q-mr-sm"
                />{{ $t('app.modals.mod_settings.general.gamebanana_info') }}
              </div>
              <q-input
                :model-value="gameBananaInfo?.url || ''"
                :label="
                  $t('app.modals.mod_settings.general.gamebanana_profile_url')
                "
                outlined
                class="q-mb-md"
                placeholder="https://gamebanana.com/mods/..."
                @update:model-value="updateGameBananaUrl"
              />
              <div
                v-if="gameBananaInfo && gameBananaInfo.url"
                class="row q-gutter-sm q-mb-md"
              >
                <q-btn
                  :label="
                    $t('app.modals.mod_settings.general.sync_with_gamebanana')
                  "
                  class="col"
                  icon="sync"
                  outline
                  :loading="gamebananaSyncing"
                  @click="syncWithGameBanana"
                />
                <q-btn
                  :label="$t('ui.actions.open_in_browser')"
                  class="col"
                  icon="open_in_new"
                  outline
                  @click="openUrl(gameBananaInfo.url)"
                />
                <div class="full-width">
                  <div class="q-my-xs">
                    <q-checkbox
                      v-model="changeBanner"
                      :label="
                        $t('app.modals.mod_settings.general.change_banner')
                      "
                      class="q-ml-sm"
                      color="primary"
                      dense
                    />
                    <q-checkbox
                      v-model="changeVersion"
                      :label="
                        $t('app.modals.mod_settings.general.change_version')
                      "
                      class="q-ml-sm"
                      color="primary"
                      dense
                    />
                  </div>
                </div>
                <div
                  class="text-caption q-mt-sm"
                  style="color: var(--theme-text-secondary)"
                >
                  {{
                    $t(
                      'app.modals.mod_settings.general.gamebanana_sync_description'
                    )
                  }}
                </div>
              </div>
            </div>
          </q-card-section>

          <!-- Location Section -->
          <q-card-section v-show="activeSection === 'location'">
            <div class="text-subtitle1 q-mb-md">
              {{ $t('app.modals.mod_settings.installation.title') }}
            </div>

            <q-input
              v-model="form.path"
              :label="
                $t('app.modals.mod_settings.installation.folder_location')
              "
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
              :label="
                $t('app.modals.mod_settings.installation.executable_path')
              "
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
            <div class="danger-zone q-mt-lg">
              <q-separator class="q-my-md" />
              <div class="text-subtitle1 q-mb-md">
                {{ $t('ui.danger_zone') }}
              </div>
              <div class="row q-col-gutter-md">
                <div class="col-6">
                  <q-btn
                    icon="delete"
                    :label="$t('mods.actions.remove_mod')"
                    class="full-width"
                    outline
                    @click="showRemoveDialog = true"
                  />
                  <div
                    class="text-caption q-mt-sm"
                    style="color: var(--theme-text-secondary)"
                  >
                    {{
                      $t(
                        'app.modals.mod_settings.installation.remove_mod_description'
                      )
                    }}
                  </div>
                </div>
                <div class="col-6">
                  <q-btn
                    color="negative"
                    icon="delete_forever"
                    :label="$t('mods.actions.delete_mod')"
                    class="full-width"
                    outline
                    @click="showSuperDeleteDialog = true"
                  />
                  <div
                    class="text-caption q-mt-sm"
                    style="color: var(--theme-text-secondary)"
                  >
                    <i18n-t
                      keypath="app.modals.mod_settings.installation.delete_mod_description"
                      tag="span"
                      global
                    >
                      <template #permanentlyDelete>
                        <b style="color: var(--red)">{{
                          $t(
                            'app.modals.mod_settings.installation.permanently_delete'
                          )
                        }}</b>
                      </template>
                    </i18n-t>
                  </div>
                </div>
              </div>
            </div>
            <div class="flex justify-between q-mb-md q-mt-md">
              <q-checkbox
                v-model="form.save_terminal_output"
                label="Save Terminal Output"
                color="primary"
              />
              <div v-if="form.save_terminal_output">
                <q-btn
                  outline
                  class="q-mr-md"
                  color="red"
                  label="Delete Logs"
                  icon="delete"
                  @click="handleDeleteLogsClick"
                />
                <q-btn
                  outline
                  label="Reveal Logs Folder"
                  icon="folder_open"
                  @click="handleOpenFileLocationClick(logsFolderPath)"
                />
              </div>
            </div>
          </q-card-section>

          <!-- Engine Section -->
          <q-card-section v-show="activeSection === 'engine'">
            <div class="text-subtitle1 q-mb-md">
              {{ $t('app.modals.mod_settings.engine.title') }}
            </div>
            <q-select
              v-model="form.engine.engine_type"
              :options="engineTypes"
              :label="$t('app.modals.mod_settings.engine.engine_type')"
              outlined
              class="q-mb-md selector"
              emit-value
              map-options
              :hint="
                $t('app.modals.mod_settings.engine.engine_type_description')
              "
              popup-content-class="phantom-font"
              popup-content-style="background-color: var(--theme-solid); color: var(--theme-text);"
            />

            <q-input
              v-model="form.engine.engine_name"
              :label="$t('app.modals.mod_settings.engine.engine_name')"
              outlined
              class="q-mb-md"
              :placeholder="
                $t('app.modals.mod_settings.engine.engine_name_placeholder')
              "
              :hint="
                $t('app.modals.mod_settings.engine.engine_name_description')
              "
            />

            <div class="q-mb-md">
              <div class="text-subtitle2 q-mb-sm">
                {{ $t('app.modals.mod_settings.engine.engine_icon') }}
              </div>
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
                :label="$t('app.modals.mod_settings.set_icon')"
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
                :label="$t('app.modals.mod_settings.remove_icon')"
                class="q-mt-sm"
                @click="removeEngineIcon"
              />
            </div>

            <q-separator class="q-my-md" />

            <div class="q-mb-md">
              <q-checkbox
                v-model="form.engine!.mods_folder"
                :label="$t('app.modals.mod_settings.engine.has_mods_folder')"
                color="primary"
              />
            </div>

            <q-input
              v-if="form.engine?.mods_folder"
              v-model="form.engine!.mods_folder_path"
              :label="$t('app.modals.mod_settings.engine.mods_folder_path')"
              outlined
              class="q-mb-md"
              :placeholder="
                $t(
                  'app.modals.mod_settings.engine.mods_folder_path_placeholder'
                )
              "
              :hint="
                $t(
                  'app.modals.mod_settings.engine.mods_folder_path_description'
                )
              "
            />
          </q-card-section>
          <!-- Visuals Section -->
          <q-card-section v-show="activeSection === 'visuals'">
            <div class="text-subtitle1 q-mb-md">
              {{ $t('app.modals.mod_settings.appearance.title') }}
            </div>
            <ModBanner
              :mod="form"
              :banner-preview="bannerPreview ? bannerPreview : ''"
              :logo-preview="logoPreview ? logoPreview : ''"
            />

            <div class="row q-col-gutter-md">
              <div class="col-12 col-md-4">
                <div class="icon-upload">
                  <div class="text-subtitle2 q-mb-sm">
                    {{ $t('app.modals.mod_settings.appearance.icon') }}
                  </div>
                  <q-file
                    v-model="iconFile"
                    :label="$t('app.modals.mod_settings.set_icon')"
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
                    <template #append>
                      <div class="flex items-center justify-center">
                        <img
                          v-if="iconPreview || form.icon_data"
                          :src="iconPreview || form.icon_data"
                          width="32"
                          height="32"
                        />
                        <div v-else></div>
                      </div>
                    </template>
                  </q-file>
                  <q-btn
                    v-if="iconPreview || form.icon_data"
                    color="negative"
                    outline
                    class="q-mt-sm full-width"
                    :label="$t('app.modals.mod_settings.remove_icon')"
                    @click="removeIcon"
                  />
                  <div>
                    <div
                      class="text-caption q-mt-md"
                      style="color: var(--theme-text-secondary)"
                    >
                      {{
                        $t('app.modals.mod_settings.appearance.icon_preview')
                      }}
                    </div>
                    <ModListItem
                      :mod="form"
                      :icon="iconPreview || ''"
                      active
                      class="q-mt-md"
                    />
                  </div>
                </div>
              </div>

              <div class="col-12 col-md-4">
                <div class="banner-upload">
                  <div class="text-subtitle2 q-mb-sm">
                    {{ $t('app.modals.mod_settings.appearance.banner') }}
                  </div>
                  <q-file
                    v-model="bannerFile"
                    :label="$t('app.modals.mod_settings.appearance.set_banner')"
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
                    outline
                    color="negative"
                    :label="
                      $t('app.modals.mod_settings.appearance.remove_banner')
                    "
                    class="q-mt-sm full-width"
                    @click="removeBanner"
                  />
                </div>
              </div>

              <div class="col-12 col-md-4">
                <div class="logo-upload">
                  <div class="text-subtitle2 q-mb-sm">
                    {{ $t('app.modals.mod_settings.appearance.logo') }}
                  </div>
                  <q-file
                    v-model="logoFile"
                    :label="$t('app.modals.mod_settings.appearance.set_logo')"
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
                    outline
                    color="negative"
                    :label="
                      $t('app.modals.mod_settings.appearance.remove_logo')
                    "
                    class="q-mt-sm full-width"
                    @click="removeLogo"
                  />
                  <q-select
                    v-if="logoPreview || form.logo_data"
                    v-model="form.logo_position"
                    :options="logoPositionOptions"
                    :label="
                      $t(
                        'app.modals.mod_settings.appearance.logo_position.title'
                      )
                    "
                    popup-content-class="phantom-font"
                    popup-content-style="background-color: var(--theme-solid); color: var(--theme-text);"
                    outlined
                    class="q-mt-md phantom-font selector"
                    emit-value
                    map-options
                    :hint="
                      $t(
                        'app.modals.mod_settings.appearance.logo_position.hint'
                      )
                    "
                  />
                </div>
              </div>
            </div>
          </q-card-section>
        </q-scroll-area>
      </div>
      <q-card-actions align="right">
        <q-btn
          v-close-popup
          flat
          :label="$t('ui.actions.cancel')"
          color="primary"
          @click="cancel"
        />
        <q-btn
          v-close-popup
          flat
          :label="$t('ui.actions.save')"
          color="primary"
          @click="save"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>

  <!-- Remove Mod Confirmation Dialog -->
  <MessageDialog
    v-model="showRemoveDialog"
    :title="$t('mods.actions.remove_mod')"
    icon="warning"
    icon-color="negative"
    :confirm-label="$t('ui.actions.remove')"
    confirm-color="negative"
    @confirm="removeMod"
  >
    <div class="text-h6">{{ form.name }}</div>
    <div class="text-caption">{{ form.path }}</div>
    <p class="text-body2 q-mt-sm">
      {{ $t('ui.actions.conformations.remove_mod') }}
      {{ $t('ui.actions.conformations.remove_mod') }}
    </p>
  </MessageDialog>

  <!-- Super Delete Mod Confirmation Dialog -->
  <MessageDialog
    v-model="showSuperDeleteDialog"
    :title="$t('mods.actions.delete_mod')"
    icon="delete_forever"
    icon-color="negative"
    :confirm-label="$t('ui.actions.delete')"
    confirm-color="negative"
    @confirm="superDeleteMod"
  >
    <div class="text-h6">{{ form.name }}</div>
    <div class="text-caption">{{ form.path }}</div>
    <p class="text-body2 q-mt-sm">
      {{ $t('ui.actions.conformations.delete_mod') }}
    </p>
  </MessageDialog>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { Mod, ModInfoGBData } from '@main-types'
import MessageDialog from './MessageDialog.vue'
import { revealItemInDir, openUrl } from '@tauri-apps/plugin-opener'
import { invoke } from '@tauri-apps/api/core'
import { appDataDir, sep } from '@tauri-apps/api/path'
import { DEFAULT_ENGINE } from '@services/dbService'
import { notificationService } from '@services/notificationService'
import ModBanner from '@components/mods/ModBanner.vue'
import { useI18n } from 'vue-i18n'
import ModListItem from '@components/mods/list/ModListItem.vue'

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

const engine = ref({
  engine_type: 'unknown',
  engine_name: '',
  engine_icon: '',
  mods_folder: false,
  mods_folder_path: '',
})

const { t } = useI18n()

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
  engine: engine.value,
  save_terminal_output: false,
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
const changeBanner = ref(true)
const changeVersion = ref(true)

const engineTypes = [
  { label: 'Not Selected', value: 'unknown' },
  { label: 'Vanilla', value: 'vanilla' },
  { label: 'Psych Engine', value: 'psych' },
  { label: 'Codename Engine', value: 'codename' },
  { label: 'FPS Plus', value: 'fps-plus' },
  { label: 'Kade Engine', value: 'kade' },
  { label: 'Pre-VSlice', value: 'pre-vslice' },
  { label: 'Other', value: 'other' },
]

const logoPositionOptions = [
  {
    label: t('app.modals.mod_settings.appearance.logo_position.bottom_left'),
    value: 'left_bottom',
  },
  {
    label: t('app.modals.mod_settings.appearance.logo_position.middle_left'),
    value: 'left_middle',
  },
  {
    label: t('app.modals.mod_settings.appearance.logo_position.middle'),
    value: 'middle',
  },
]

const showModal = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
})

const logsFolderPath = ref<string>('')

watch(
  () => props.modelValue,
  async newVal => {
    if (newVal && props.mod) {
      logsFolderPath.value =
        (await appDataDir()) + sep() + 'mod_logs' + sep() + form.value.id
    }
  }
)

const modSettingsSections = [
  {
    id: 'general',
    label: t('app.modals.mod_settings.general.label'),
    icon: 'info',
  },
  {
    id: 'location',
    label: t('app.modals.mod_settings.installation.title'),
    icon: 'folder',
  },
  {
    id: 'engine',
    label: t('app.modals.mod_settings.engine.label'),
    icon: 'code',
  },
  {
    id: 'visuals',
    label: t('app.modals.mod_settings.appearance.title'),
    icon: 'palette',
  },
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

      // Properly handle engine object - parse if it's a string, otherwise use as-is
      if (props.mod.engine) {
        if (typeof props.mod.engine === 'string') {
          try {
            form.value.engine = JSON.parse(props.mod.engine)
          } catch (e) {
            console.warn('Failed to parse engine string:', e)
            form.value.engine = DEFAULT_ENGINE
          }
        } else {
          form.value.engine = { ...props.mod.engine }
        }
      } else {
        form.value.engine = {
          engine_type: 'unknown',
          engine_name: '',
          engine_icon: '',
          mods_folder: false,
          mods_folder_path: '',
        }
      }

      // Update the engine ref to match the form
      engine.value = {
        engine_type: form.value.engine.engine_type,
        engine_name: form.value.engine.engine_name || '',
        engine_icon: form.value.engine.engine_icon || '',
        mods_folder: form.value.engine.mods_folder,
        mods_folder_path: form.value.engine.mods_folder_path || '',
      }

      // Ensure save_terminal_output defaults to false if not explicitly set to true
      form.value.save_terminal_output = form.value.save_terminal_output === true

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

      console.info(
        `Form values initialized for mod: ${form.value.id}:`,
        form.value
      )

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

    console.info('GameBanana API response received')

    // Update form with the fetched data
    form.value.name = response?._sName || form.value.name
    form.value.description = response?._sDescription || form.value.description
    if (changeVersion.value) {
      form.value.version = response?._sVersion || form.value.version
    }

    // Handle banner image with better error handling
    if (response?._aPreviewMedia?._aImages?.[0] && changeBanner.value) {
      const bannerImage = response._aPreviewMedia._aImages[0]
      const bannerUrl = `${bannerImage._sBaseUrl}/${bannerImage._sFile}`
      console.info('Constructed banner URL:', bannerUrl)

      // Validate URL before attempting to fetch
      if (bannerUrl && bannerUrl !== 'undefined/undefined') {
        try {
          console.info('Attempting to fetch banner from URL:', bannerUrl)
          const bannerData = await invoke<string>('get_url_as_base64', {
            url: bannerUrl,
          })
          console.info(
            'Successfully fetched banner data, length:',
            bannerData?.length || 0
          )
          if (bannerData) {
            form.value.banner_data = bannerData
            bannerPreview.value = bannerData
          }
        } catch (bannerError) {
          console.error('Failed to fetch banner image:', bannerError)
          console.error('Banner URL that failed:', bannerUrl)
        }
      } else {
        console.warn('Invalid banner URL constructed:', bannerUrl)
      }
    } else {
      console.info('No preview images found in response')
    }

    console.info('GameBanana sync completed successfully')
    gamebananaSyncing.value = false
  } catch (error) {
    console.error('Failed to sync with GameBanana:', error)
    gamebananaSyncing.value = false
  }
}

const handleDeleteLogsClick = async () => {
  try {
    await invoke('clear_all_mod_logs', { id: form.value.id })
    console.info('Mod logs deleted successfully')
  } catch (error) {
    console.error('Failed to delete mod logs:', error)
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

  // Handle Mod Icon Image
  if (iconPreview.value) {
    updatedMod.icon_data = iconPreview.value
  } else if (iconFile.value === null && form.value.icon_data === null) {
    updatedMod.icon_data = undefined
  }
  // Otherwise preserve existing icon_data

  // First, make sure engine object exists
  if (!updatedMod.engine) {
    // Then parse the string to an object
    updatedMod.engine = { ...DEFAULT_ENGINE, ...form.value.engine }
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
  try {
    await revealItemInDir(path)
  } catch (error) {
    console.error('Failed to open file location:', error)

    notificationService.operationError(
      'Failed to open file location',
      String(error)
    )
  }
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
:deep(.q-checkbox__bg) {
  border-color: var(--theme-border);
  background-color: var(--theme-solid);
}
.list-item {
  background-color: var(--theme-surface);
}
</style>
