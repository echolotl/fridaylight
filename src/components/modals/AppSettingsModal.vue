<template>
  <q-dialog v-model="showModal" persistent>
    <q-card class="settings-modal phantom-font">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6 phantom-font-difficulty">
          {{ $t('app.modals.app_settings.title') }}
        </div>
        <q-space />
        <q-btn v-close-popup icon="close" flat round dense />
      </q-card-section>

      <div class="settings-layout">
        <!-- Sidebar Navigation -->
        <div class="settings-sidebar">
          <q-list padding>
            <q-item
              v-for="section in settingsSections"
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
          <!-- Appearance Section -->
          <q-card-section v-show="activeSection === 'appearance'">
            <div class="text-subtitle1 q-mb-md">
              {{ $t('app.modals.app_settings.appearance.title') }}
            </div>

            <div class="theme-toggle-container row q-mb-md">
              <q-item tag="label" class="full-width">
                <q-item-section>
                  <q-item-label>{{
                    $t('app.modals.app_settings.appearance.use_system_theme')
                  }}</q-item-label>
                  <q-item-label caption>{{
                    $t(
                      'app.modals.app_settings.appearance.use_system_theme_description'
                    )
                  }}</q-item-label>
                </q-item-section>
                <q-item-section side>
                  <q-toggle v-model="settings.useSystemTheme" color="primary" />
                </q-item-section>
              </q-item>
            </div>
            <div v-if="!settings.useSystemTheme" class="theme-selector q-mb-md">
              <!-- Built-in Themes Section -->
              <div class="theme-section">
                <div class="text-subtitle2 q-mb-sm">
                  {{ $t('app.modals.app_settings.appearance.builtin_themes') }}
                </div>
                <div class="theme-grid">
                  <div
                    v-for="theme in builtInThemes"
                    :key="theme.value"
                    class="theme-grid-item"
                    @click="selectTheme(theme.value)"
                  >
                    <ThemePreview
                      :theme-name="theme.value"
                      :theme-display-name="theme.label"
                      :accent-color="getAccentColor()"
                      :compact-mode="settings.compactMode"
                      :cur-selected="getSelectedTheme() === theme.value"
                    />
                  </div>
                </div>
              </div>

              <!-- Custom Themes Section (only if there are custom themes) -->
              <div
                v-if="customThemes.length > 0"
                class="theme-section custom-themes-section"
              >
                <div class="row items-center justify-between q-mb-sm">
                  <div class="text-subtitle2">
                    {{ $t('app.modals.app_settings.appearance.custom_themes') }}
                  </div>
                  <div class="row items-center">
                    <q-btn
                      flat
                      color="primary"
                      icon="folder_open"
                      :label="
                        $t(
                          'app.modals.app_settings.appearance.actions.reveal_themes_folder'
                        )
                      "
                      class="phantom-font q-mr-sm"
                      @click="openThemesFolder"
                    />
                    <q-btn
                      flat
                      color="primary"
                      icon="refresh"
                      :label="
                        $t(
                          'app.modals.app_settings.appearance.actions.refresh_themes'
                        )
                      "
                      class="phantom-font"
                      @click="refreshThemes"
                    />
                  </div>
                </div>
                <div class="theme-grid">
                  <div
                    v-for="theme in customThemes"
                    :key="theme.value"
                    class="theme-grid-item"
                    @click="selectTheme(theme.value)"
                  >
                    <ThemePreview
                      :theme-name="theme.value"
                      :theme-display-name="theme.label"
                      :accent-color="getAccentColor()"
                      :compact-mode="settings.compactMode"
                      :cur-selected="getSelectedTheme() === theme.value"
                    />
                  </div>
                </div>
              </div>
              <div v-else class="text-caption q-mt-sm">
                <div class="row items-center justify-between">
                  <span>
                    <i18n-t
                      keypath="app.modals.app_settings.appearance.custom_themes_description"
                    >
                      <template #themesLink>
                        <a
                          class="text-primary"
                          @click="revealItemInDir(customThemesPath)"
                          >/themes</a
                        >
                      </template>
                    </i18n-t>
                  </span>
                  <q-btn
                    flat
                    color="primary"
                    icon="refresh"
                    :label="
                      $t(
                        'app.modals.app_settings.appearance.actions.refresh_themes'
                      )
                    "
                    class="phantom-font"
                    @click="refreshThemes"
                  />
                </div>
              </div>
            </div>
            <div class="text-subtitle2 q-mb-sm">
              {{ $t('app.modals.app_settings.appearance.accent_color') }}
            </div>
            <div class="color-row q-mb-md">
              <q-btn
                v-for="color in accentColorOptions"
                :key="color.value"
                round
                flat
                :style="{ backgroundColor: color.value }"
                class="color-button"
                :class="{
                  'color-selected':
                    getAccentColor() === color.value && !isCustomAccentColor,
                }"
                @click="selectPresetAccentColor(color.value)"
              />
              <q-btn
                round
                flat
                icon="colorize"
                class="color-button custom-color-btn"
                :class="{ 'color-selected': isCustomAccentColor }"
                :style="
                  isCustomAccentColor
                    ? { backgroundColor: customAccentColor }
                    : { backgroundColor: 'transparent' }
                "
                @click="openAccentColorPicker"
              />
              <input
                ref="accentColorPickerInput"
                v-model="customAccentColor"
                type="color"
                class="hidden-color-picker"
                @change="selectCustomAccentColor"
              />
            </div>

            <q-item tag="label" class="q-mb-md">
              <q-item-section>
                <q-item-label>{{
                  $t('app.modals.app_settings.appearance.compact_mode')
                }}</q-item-label>
                <q-item-label caption>
                  {{
                    $t(
                      'app.modals.app_settings.appearance.compact_mode_description'
                    )
                  }}
                </q-item-label>
              </q-item-section>
              <q-item-section side>
                <q-toggle v-model="settings.compactMode" color="primary" />
              </q-item-section>
            </q-item>
          </q-card-section>

          <!-- Installation Section -->
          <q-card-section v-show="activeSection === 'installation'">
            <div class="text-subtitle1 q-mb-md">
              {{ $t('app.modals.app_settings.installation.title') }}
            </div>

            <q-input
              v-model="settings.installLocation"
              :label="
                $t(
                  'app.modals.app_settings.installation.default_install_location'
                )
              "
              outlined
              readonly
              class="q-mb-md"
            >
              <template #append>
                <div class="icon">
                  <q-btn
                    round
                    flat
                    icon="folder"
                    @click="selectInstallLocation"
                  />
                </div>
              </template>
            </q-input>

            <q-item tag="label" class="q-mt-md">
              <q-item-section>
                <q-item-label>{{
                  $t('app.modals.app_settings.installation.validate_mods')
                }}</q-item-label>
                <q-item-label caption>
                  {{
                    $t(
                      'app.modals.app_settings.installation.validate_mods_description'
                    )
                  }}
                </q-item-label>
              </q-item-section>
              <q-item-section side>
                <q-toggle v-model="settings.validateFnfMods" color="primary" />
              </q-item-section>
            </q-item>

            <q-item tag="label" class="q-mt-md">
              <q-item-section>
                <q-item-label>{{
                  $t(
                    'app.modals.app_settings.installation.show_terminal_output'
                  )
                }}</q-item-label>
                <q-item-label caption>
                  {{
                    $t(
                      'app.modals.app_settings.installation.show_terminal_output_description'
                    )
                  }}
                </q-item-label>
              </q-item-section>
              <q-item-section side>
                <q-toggle
                  v-model="settings.showTerminalOutput"
                  color="primary"
                />
              </q-item-section>
            </q-item>
          </q-card-section>

          <!-- About Section -->
          <q-card-section v-show="activeSection === 'about'">
            <div class="text-subtitle1">
              {{ $t('app.modals.app_settings.about.title') }}
            </div>

            <div class="about-content">
              <div class="about-content-left">
                <img
                  src="/images/fridaylight-colored.svg"
                  class="fridaylight-logo"
                />
                <div class="text-subtitle2 q-mb-sm flex align-center">
                  <i18n-t keypath="app.modals.app_settings.about.created_by">
                    <template #echolotl>
                      <img
                        src="/images/echolotlGB.png"
                        class="q-mx-xs cursor-pointer"
                        @click="openUrl('https://www.echolotl.lol/')"
                      />
                    </template>
                  </i18n-t>
                </div>
                <div class="q-mb-sm">
                  <i18n-t keypath="app.modals.app_settings.about.version">
                    <template #version> 0.11.2 </template>
                  </i18n-t>
                  <span class="text-caption">(Windows, 64 bit)</span>
                </div>
                <div class="flex items-center q-mb-sm">
                  <q-icon name="bug_report" size="sm" color="primary" />
                  <a
                    class="q-ml-xs"
                    @click="
                      openUrl('https://github.com/echolotl/fridaylight/issues')
                    "
                    >{{ $t('app.modals.app_settings.about.bug_disclaimer') }}</a
                  >
                </div>
                <div class="text-caption">
                  {{ $t('app.modals.app_settings.about.beta_disclaimer') }}
                </div>
              </div>
              <div class="about-content-right">
                <div class="text-subtitle2 q-mb-sm">
                  {{ $t('app.modals.app_settings.about.created_with') }}
                </div>
                <div class="logo-grid">
                  <img
                    src="/images/vue.svg"
                    alt="Vue"
                    width="48"
                    height="48"
                    class="logo"
                    @click="openUrl('https://vuejs.org/')"
                  />
                  <img
                    src="/images/tauri.svg"
                    alt="Tauri"
                    width="48"
                    height="48"
                    class="logo"
                    @click="openUrl('https://v2.tauri.app/')"
                  />
                  <img
                    src="/images/quasar.svg"
                    alt="Quasar"
                    width="48"
                    height="48"
                    class="logo"
                    @click="openUrl('https://quasar.dev/')"
                  />
                </div>
                <div class="text-subtitle2 q-mt-md">
                  {{ $t('app.modals.app_settings.about.special_thanks.title') }}
                </div>
                <q-scroll-area class="acknowledgements">
                  <ul style="list-style-type: none; padding-left: 0">
                    <li>
                      <div>Gamebanana</div>
                      <div class="text-caption">
                        {{
                          $t(
                            'app.modals.app_settings.about.special_thanks.gamebanana'
                          )
                        }}
                      </div>
                    </li>
                    <li>
                      <a
                        @click="
                          openUrl('https://gamebanana.com/members/1844732')
                        "
                      >
                        Cracsthor
                      </a>
                      <div class="text-caption">
                        {{
                          $t(
                            'app.modals.app_settings.about.special_thanks.cracsthor'
                          )
                        }}
                      </div>
                    </li>
                    <li>
                      <a
                        @click="
                          openUrl('https://gamebanana.com/members/1844732')
                        "
                      >
                        NoizDynamic
                      </a>
                      <div class="text-caption">
                        {{
                          $t(
                            'app.modals.app_settings.about.special_thanks.noizdynamic'
                          )
                        }}
                      </div>
                    </li>
                    <li>
                      <span>{{
                        $t(
                          'app.modals.app_settings.about.special_thanks.name_flmoddevteam'
                        )
                      }}</span>
                      <div class="text-caption">
                        {{
                          $t(
                            'app.modals.app_settings.about.special_thanks.flmoddevteam'
                          )
                        }}
                      </div>
                    </li>
                    <li>
                      <span> SteamGridDB </span>
                      <div class="text-caption">
                        {{
                          $t(
                            'app.modals.app_settings.about.special_thanks.steamgriddb'
                          )
                        }}
                      </div>
                    </li>
                  </ul>
                </q-scroll-area>
              </div>
            </div>

            <div class="settings-reset-section q-mt-lg">
              <q-separator class="q-my-md" />
              <div class="text-subtitle1 q-mb-md">
                {{ $t('ui.danger_zone') }}
              </div>
              <q-btn
                color="negative"
                icon="restart_alt"
                :label="$t('app.modals.app_settings.reset_settings.title')"
                class="full-width"
                outline
                @click="showResetSettingsDialog = true"
              />
              <div class="text-caption q-mt-sm">
                {{ $t('app.modals.app_settings.reset_settings.description') }}
              </div>

              <q-btn
                color="negative"
                icon="delete_forever"
                :label="$t('app.modals.app_settings.reset_data.title')"
                class="full-width q-mt-md"
                outline
                @click="showResetAppDataDialog = true"
              />
              <div class="text-caption q-mt-sm">
                {{ $t('app.modals.app_settings.reset_data.description') }}
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

  <!-- Reset Settings Confirmation Dialog -->
  <MessageDialog
    v-model="showResetSettingsDialog"
    :title="$t('app.modals.app_settings.reset_settings.title')"
    :message="$t('app.modals.app_settings.reset_settings.confirmation')"
    icon="warning"
    icon-color="negative"
    confirm-label="Reset Settings"
    confirm-color="negative"
    @confirm="resetSettings"
  />

  <!-- Reset App Data Confirmation Dialog -->
  <MessageDialog
    v-model="showResetAppDataDialog"
    :title="$t('app.modals.app_settings.reset_data.title')"
    :message="$t('app.modals.app_settings.reset_data.confirmation')"
    icon="warning"
    icon-color="negative"
    confirm-label="Reset App Data"
    confirm-color="negative"
    @confirm="resetAppData"
  />
</template>

<script setup lang="ts">
import { ref, watch, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'
import { openUrl, revealItemInDir } from '@tauri-apps/plugin-opener'
import { AppSettings } from '@main-types'
import ThemePreview from '../common/ThemePreview.vue'
import MessageDialog from './MessageDialog.vue'
import { StoreService, DEFAULT_SETTINGS } from '../../services/storeService'
import { DatabaseService } from '../../services/dbService'
import { themeService } from '../../services/themeService'

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['update:modelValue', 'save'])

// Use the singleton directly instead of through a ref
const storeService = StoreService.getInstance()

const settings = ref<AppSettings>({ ...DEFAULT_SETTINGS })
// Sidebar navigation sections
const { t } = useI18n()
const settingsSections = [
  {
    id: 'appearance',
    label: t('app.modals.app_settings.appearance.title'),
    icon: 'palette',
  },
  {
    id: 'installation',
    label: t('app.modals.app_settings.installation.title'),
    icon: 'folder',
  },
  {
    id: 'about',
    label: t('app.modals.app_settings.about.title'),
    icon: 'info',
  },
]

// Track the active section
const activeSection = ref('appearance')

// Theme options - dynamically loaded from theme service
const availableThemes = ref<any[]>([])

const themeOptions = computed(() => {
  return availableThemes.value.map(theme => ({
    label: theme.displayName,
    value: theme.id,
    isCustom: theme.isCustom,
  }))
})

// Separate computed properties for built-in and custom themes
const builtInThemes = computed(() => {
  return themeOptions.value.filter(theme => !theme.isCustom)
})

const customThemes = computed(() => {
  return themeOptions.value.filter(theme => theme.isCustom)
})

const accentColorOptions = [
  { label: t('misc.colors.pink'), value: '#DB2955' },
  { label: t('misc.colors.blue'), value: '#235789' },
  { label: t('misc.colors.green'), value: '#35CE8D' },
  { label: t('misc.colors.purple'), value: '#C490D1' },
  { label: t('misc.colors.orange'), value: '#FE621D' },
  { label: t('misc.colors.yellow'), value: '#f7c548' },
  { label: t('misc.colors.red'), value: '#C03221' },
  { label: t('misc.colors.cyan'), value: '#39dbce' },
]

// Helper function to get current accent color
const getAccentColor = () => {
  if (typeof settings.value.accentColor === 'string') {
    return settings.value.accentColor
  }
  return settings.value.accentColor?.value || '#DB2955'
}

const showModal = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
})

const showResetSettingsDialog = ref(false)
const showResetAppDataDialog = ref(false)

// Custom accent color variables
const customAccentColor = ref('#DB2955')
const isCustomAccentColor = ref(false)
const accentColorPickerInput = ref<HTMLInputElement | null>(null)

// Load saved settings when modal is opened
watch(
  () => props.modelValue,
  async newVal => {
    if (newVal) {
      await loadSettings()
    }
  }
)

const loadSettings = async () => {
  try {
    if (!storeService) {
      console.warn('Store service not initialized yet, using default settings')
      return
    }

    // Get all settings from StoreService
    const storedSettings = await storeService.getAllSettings() // Apply stored settings to our local settings ref
    settings.value = { ...settings.value, ...storedSettings }

    // Initialize custom accent color state
    const currentAccentColor = getAccentColor()
    const isPresetColor = accentColorOptions.some(
      option => option.value === currentAccentColor
    )

    if (!isPresetColor) {
      // Current color is custom
      isCustomAccentColor.value = true
      customAccentColor.value = currentAccentColor
    } else {
      // Current color is a preset
      isCustomAccentColor.value = false
      customAccentColor.value = '#DB2955' // Reset to default
    }

    console.log('Settings loaded from store service:', settings.value)

    // Apply theme immediately upon loading
    await updateTheme()
  } catch (error) {
    console.error('Failed to load settings:', error)
  }
}

const selectInstallLocation = async () => {
  try {
    // Use the Rust backend to select a folder
    const folderPath = await invoke<string>('select_settings_folder')
    if (folderPath) {
      settings.value.installLocation = folderPath
    }
  } catch (error) {
    console.error('Failed to select install location:', error)
  }
}

// Get the current system theme (light or dark)
const getSystemTheme = (): { label: string; value: string } => {
  return window.matchMedia &&
    window.matchMedia('(prefers-color-scheme: light)').matches
    ? { label: 'Light', value: 'light' }
    : { label: 'Dark', value: 'dark' }
}

// Apply theme function
const updateTheme = async () => {
  // Determine which theme to use based on settings
  let activeThemeValue: string

  // If using system theme, override with the system preference
  if (settings.value.useSystemTheme) {
    activeThemeValue = getSystemTheme().value
  } else {
    // Handle both string and object theme values
    activeThemeValue =
      typeof settings.value.theme === 'string'
        ? settings.value.theme
        : (settings.value.theme as unknown as { value: string })?.value ||
          'dark'
  }

  console.log('Applying theme via themeService:', activeThemeValue)

  try {
    // Use the theme service to apply the theme
    await themeService.applyTheme(activeThemeValue)

    // Check if we're running on Windows 11 for additional styling
    const isWindows11 = await invoke<boolean>('is_windows_11')
    console.log('Is Windows 11:', isWindows11, 'Theme:', activeThemeValue)

    // Apply platform-specific styling
    if (!isWindows11) {
      // Apply solid theme styling for non-Windows 11
      if (activeThemeValue === 'light' || activeThemeValue === 'dark') {
        document.body.classList.add('solid-theme')
        console.log(
          'Using solid background for non-Windows 11 theme:',
          activeThemeValue
        )
      } else {
        document.body.classList.remove('solid-theme')
        console.log(
          'Using theme background for non-Windows 11 theme:',
          activeThemeValue
        )
      }

      // Remove transparent background styles
      document.documentElement.style.setProperty(
        '--transparent-bg-override',
        'none'
      )

      // Set background to solid color based on the theme
      const bgColor = `var(--theme-bg)`

      document.body.style.removeProperty('background')
      document.body.style.backgroundColor = bgColor

      document
        .querySelector('.q-layout')
        ?.setAttribute('style', 'background: ' + bgColor + ' !important')
    } else {
      // On Windows 11, handle Mica effect for light and dark themes
      if (themeService.supportsWindowsMica(activeThemeValue)) {
        document.body.classList.remove('solid-theme')
        document.documentElement.style.setProperty(
          '--transparent-bg-override',
          'transparent'
        )

        // Make background transparent for Mica
        document.body.style.removeProperty('background')
        document.body.setAttribute(
          'style',
          'background: transparent !important'
        )

        // Make sure q-layout is also transparent for Mica to work properly
        const qLayout = document.querySelector('.q-layout')
        if (qLayout) {
          qLayout.removeAttribute('style')
          qLayout.setAttribute('style', 'background: transparent !important')
        }

        // Apply Mica effect via Rust backend
        try {
          const isDarkMica = activeThemeValue !== 'light'
          await invoke('change_mica_theme', {
            window: 'main',
            dark: isDarkMica,
          })
          console.log(
            'Applied Mica theme effect:',
            isDarkMica ? 'dark' : 'light'
          )
        } catch (error) {
          console.error('Failed to apply Mica effect:', error)
        }
      } else {
        // For other themes on Windows 11, use solid background
        document.body.classList.remove('solid-theme')
        document.documentElement.style.setProperty(
          '--transparent-bg-override',
          'none'
        )

        const bgColor = `var(--theme-bg)`

        document.body.style.removeProperty('background')
        document.body.style.backgroundColor = bgColor
        document
          .querySelector('.q-layout')
          ?.setAttribute('style', 'background: ' + bgColor + ' !important')
      }
    }

    // Dispatch an event so other components can know about theme changes
    window.dispatchEvent(
      new CustomEvent('theme-changed', {
        detail: {
          theme: activeThemeValue,
          useSystemTheme: settings.value.useSystemTheme,
        },
      })
    )
  } catch (error) {
    console.error('Failed to apply theme:', error)
  }
}

const save = async () => {
  try {
    if (!storeService) {
      console.warn('Store service not initialized yet, cannot save settings')
      return
    }

    // Process the accent color for storage - ensure it's stored as a string
    // For accent color, we need to handle both string and object values
    if (typeof settings.value.accentColor !== 'string') {
      // If it's an object with a value property
      if (settings.value.accentColor?.value) {
        console.log(
          'Converting accent color object to string value:',
          settings.value.accentColor
        )
        settings.value.accentColor = settings.value.accentColor.value
      }
      // If it's null or undefined or another non-string type
      else {
        console.log('Using default accent color')
        settings.value.accentColor = '#DB2955' // Default
      }
    }

    // Ensure accentColor is a valid CSS color value (starts with #)
    if (
      typeof settings.value.accentColor !== 'string' ||
      !settings.value.accentColor.startsWith('#')
    ) {
      console.log('Fixing invalid accent color format')
      settings.value.accentColor = '#DB2955' // Default
    }

    // Process the theme value to ensure it's a string
    if (
      typeof settings.value.theme === 'object' &&
      settings.value.theme !== null &&
      'value' in settings.value.theme
    ) {
      console.log('Converting theme object to string value')
      settings.value.theme = (settings.value.theme as { value: string }).value
    }

    console.log(
      'Saving settings with accent color:',
      settings.value.accentColor
    )

    // Save all settings at once using the StoreService
    await storeService.saveSettings(settings.value)
    console.log('Settings saved via StoreService:', settings.value)

    // Apply the accent color to CSS custom properties
    document.documentElement.style.setProperty(
      '--q-primary',
      settings.value.accentColor
    )

    // Apply theme
    await updateTheme()

    // Dispatch compact mode changed event
    window.dispatchEvent(
      new CustomEvent('compact-mode-changed', {
        detail: {
          compactMode: settings.value.compactMode,
        },
      })
    )

    // Emit save event
    emit('save', settings.value)
  } catch (error) {
    console.error('Failed to save settings:', error)
  }
}

const cancel = () => {
  // Reset any unsaved changes by reloading from DB
  loadSettings()
}

// Theme management functions
const customThemesPath = ref('')

const refreshThemes = async () => {
  try {
    // Make sure theme service is initialized
    await themeService.initialize()

    await themeService.refreshThemes()
    const themes = themeService.getThemes()
    availableThemes.value = themes
    console.log('Refreshed themes:', themes)
  } catch (error) {
    console.error('Failed to refresh themes:', error)
  }
}

const openThemesFolder = async () => {
  try {
    const themesDir = themeService.getCustomThemesDirectory()
    await revealItemInDir(themesDir)
  } catch (error) {
    console.error('Failed to open themes folder:', error)
  }
}

// Accent color management functions
const openAccentColorPicker = () => {
  if (accentColorPickerInput.value) {
    accentColorPickerInput.value.click()
  }
}

const selectCustomAccentColor = () => {
  settings.value.accentColor = customAccentColor.value
  isCustomAccentColor.value = true
}

const selectPresetAccentColor = (color: string) => {
  settings.value.accentColor = color
  isCustomAccentColor.value = false
}

// Theme selection functions
const getSelectedTheme = (): string => {
  if (settings.value.useSystemTheme) {
    return getSystemTheme().value
  }

  // Handle both string and object theme values
  if (typeof settings.value.theme === 'string') {
    return settings.value.theme
  } else {
    // Handle case where theme is an object
    const themeObj = settings.value.theme as unknown as { value: string }
    return themeObj?.value || 'dark'
  }
}

const selectTheme = (themeId: string) => {
  settings.value.theme = themeId
}

const resetSettings = () => {
  // Reset all settings to default values
  settings.value = { ...DEFAULT_SETTINGS }
  console.log('Settings have been reset to default values.')
}

const resetAppData = async () => {
  try {
    console.log('Resetting application data...')

    // Get instances of our services
    const dbService = DatabaseService.getInstance()

    // First, reset settings to default values
    await storeService.clearSettings()
    console.log('Settings have been cleared successfully')

    // Then clear the database (delete all data from tables)
    await dbService.clearDatabase()
    console.log('Database has been cleared successfully')

    // Reload the application to apply changes
    window.location.reload()
  } catch (error) {
    console.error('Failed to reset app data:', error)
    window.alert(t('app.modals.app_settings.reset_data.failed', { error }))
  }
}

onMounted(async () => {
  // Initialize StoreService
  await storeService.initialize()

  // Initialize theme service first
  try {
    await themeService.initialize()
  } catch (error) {
    console.error('Failed to initialize theme service:', error)
  }

  // Load available themes from theme service
  try {
    const themes = themeService.getThemes()
    availableThemes.value = themes
    customThemesPath.value = themeService.getCustomThemesDirectory()
    console.log('Loaded themes for settings:', themes)
  } catch (error) {
    console.error('Failed to load themes:', error)
    // Fallback to basic themes if service fails
    availableThemes.value = [
      {
        id: 'light',
        name: 'light',
        displayName: 'Light',
        isBuiltIn: true,
        isCustom: false,
      },
      {
        id: 'dark',
        name: 'dark',
        displayName: 'Dark',
        isBuiltIn: true,
        isCustom: false,
      },
    ]
  }
})

// Initialize settings on component creation
loadSettings()
</script>

<style scoped>
.settings-modal {
  width: 90vw;
  height: 90vh;
  max-width: 90vw;
  max-height: 90vh;
  color: var(--theme-text);
  border: var(--theme-border) 2px solid;
  backdrop-filter: blur(30px);
  scrollbar-width: none;
  background-color: var(--theme-solid);
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

.color-preview {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  border: 1px solid var(--theme-border);
}

.color-row {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  padding: 1rem;
}

.color-button {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  transition: all 0.2s ease;
}

.color-button:hover {
  transform: scale(1.1);
  box-shadow: 0 0 0 2px var(--theme-border);
}

.color-selected {
  transform: scale(1.1);
  box-shadow: 0 0 0 3px var(--theme-border) !important;
}

.custom-color-btn {
  background-color: white;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.custom-color-btn.color-selected {
  background-image: none;
}

.custom-color-btn .q-icon {
  color: rgba(0, 0, 0, 0.7);
  position: absolute;
}

.hidden-color-picker {
  position: absolute;
  opacity: 0;
  height: 0;
  width: 0;
  pointer-events: none;
}

.acknowledgements {
  height: 200px;
  line-height: 1.5;
  padding: 1rem;
}

a {
  cursor: pointer;
}

.acknowledgements ul {
  padding-left: 20px;
}

.acknowledgements li {
  margin-bottom: 4px;
}

.text-caption {
  color: var(--theme-text-secondary);
}

.logo-grid {
  display: inline-flex;
  gap: 8px;
}

.logo {
  cursor: pointer;
  transition: transform 0.2s;
  display: inline-block;
  width: 48px;
  height: 48px;
}

.logo:hover {
  transform: scale(1.05);
}

.center-credits {
  display: flex;
  flex-direction: column;
  margin-top: 16px;
}

.list-item {
  background-color: var(--theme-surface);
}

:deep(.q-field__messages, .q-field__native::placeholder, .q-field__label) {
  color: var(--theme-text-secondary);
}

:deep(.q-field__native::placeholder) {
  color: var(--theme-text-secondary);
}

.theme-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 16px;
  margin-top: 8px;
}

.theme-grid-item {
  cursor: pointer;
  transition: transform 0.2s ease;
}

.theme-grid-item:hover {
  transform: scale(1.02);
}

.theme-section {
  margin-bottom: 24px;
}

.theme-section:last-child {
  margin-bottom: 0;
}

.custom-themes-section {
  padding-top: 16px;
  border-top: 1px solid var(--theme-border);
}

.about-content {
  display: flex;
  justify-content: space-between;
}
.about-content-left {
  flex: 1;
}
.about-content-right {
  flex: 1;
  text-align: right;
}
.fridaylight-logo {
  max-height: 125px;
  width: auto;
}
</style>
