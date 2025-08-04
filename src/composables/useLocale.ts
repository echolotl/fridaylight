import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { StoreService } from '@services/storeService'

export interface LocaleInfo {
  code: string
  name: string
  key: string
}

export const AVAILABLE_LOCALES: LocaleInfo[] = [
  { code: 'en', name: 'English', key: 'english' },
  { code: 'tr', name: 'Türkçe', key: 'turkish' },
  { code: 'ua', name: 'Ukraynaca', key: 'ukrainian' },
  { code: 'ru', name: 'Русский', key: 'russian' },
]

const storeService = StoreService.getInstance()

// Reactive locale state
const currentLocale = ref<string>('en')
const isLoading = ref(false)

export function useLocale() {
  // Try to get i18n instance, but handle cases where it's not available
  // Linting will complain about using `any`, but we need to handle cases where i18n is not available

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let locale: any = null
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let setLocaleMessage: any = null
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let availableLocales: any = null

  try {
    const i18n = useI18n()
    locale = i18n.locale
    setLocaleMessage = i18n.setLocaleMessage
    availableLocales = i18n.availableLocales
  } catch {
    // This will happen if called outside of component context
    console.warn('useI18n not available in current context')
  }

  // Load locale messages dynamically
  async function loadLocale(localeCode: string): Promise<boolean> {
    try {
      // Check if already loaded (only if we have access to availableLocales)
      if (availableLocales && availableLocales.includes(localeCode)) {
        return true
      }

      // Import locale file
      const messages = await import(`../locales/${localeCode}.json`)

      // Set the locale message in i18n (only if setLocaleMessage is available)
      if (setLocaleMessage) {
        setLocaleMessage(localeCode, messages.default)
      }

      return true
    } catch (error) {
      console.error(`Failed to load locale ${localeCode}:`, error)
      return false
    }
  }

  // Change locale and save to store
  async function changeLocale(newLocale: string): Promise<boolean> {
    if (isLoading.value) return false

    try {
      isLoading.value = true

      // Load the locale first
      const loaded = await loadLocale(newLocale)
      if (!loaded) return false // Update i18n locale (only if locale is available)
      if (locale) {
        locale.value = newLocale
      }
      currentLocale.value = newLocale

      // Save to Tauri store
      await storeService.saveSetting('locale', newLocale)

      return true
    } catch (error) {
      console.error('Failed to change locale:', error)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // Initialize locale from store
  async function initializeLocale(): Promise<void> {
    try {
      isLoading.value = true

      // Get saved locale from store
      const savedLocale = await storeService.getSetting('locale') // Load and set the locale
      await loadLocale(savedLocale)
      if (locale) {
        locale.value = savedLocale
      }
      currentLocale.value = savedLocale
    } catch (error) {
      console.error('Failed to initialize locale from store:', error)
      // Fallback to default
      await changeLocale('en')
    } finally {
      isLoading.value = false
    }
  }

  // Watch for external locale changes (if needed)
  async function syncFromStore(): Promise<void> {
    try {
      const storeLocale = await storeService.getSetting('locale')
      if (storeLocale !== currentLocale.value) {
        await changeLocale(storeLocale)
      }
    } catch (error) {
      console.error('Failed to sync locale from store:', error)
    }
  }

  return {
    currentLocale,
    isLoading,
    changeLocale,
    initializeLocale,
    loadLocale,
    syncFromStore,
  }
}

// Global locale watcher composable for components that need to react to locale changes
export function useLocaleWatcher() {
  const { currentLocale } = useLocale()
  const { t, locale } = useI18n()

  // Watch for locale changes and trigger reactivity
  watch(
    currentLocale,
    newLocale => {
      if (locale.value !== newLocale) {
        locale.value = newLocale
      }
    },
    { immediate: true }
  )

  return {
    t,
    currentLocale,
    locale: currentLocale,
  }
}
