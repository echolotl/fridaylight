import { createApp } from 'vue'
import { createI18n } from 'vue-i18n'
import App from './App.vue'
import router from './router'

// Import Quasar
import { Quasar, Notify } from 'quasar'

// Import Quasar CSS
import 'quasar/src/css/index.sass'

// Import icon libraries
import '@quasar/extras/material-icons/material-icons.css'

// Import store and locale utilities
import { StoreService } from '@services/storeService'

// Create i18n instance with minimal initial setup
const i18n = createI18n({
  legacy: false,
  locale: 'en',
  fallbackLocale: 'en',
  messages: {
    en: {},
  },
})

// Function to initialize locale from store
async function initializeLocaleFromStore() {
  try {
    const storeService = StoreService.getInstance()
    const savedLocale = await storeService.getSetting('locale') // Load locale messages
    try {
      const messages = await import(`./locales/${savedLocale}.json`)
      i18n.global.setLocaleMessage(savedLocale, messages.default)
      // Cast to avoid type issues with dynamic locale setting
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      ;(i18n.global.locale as any).value = savedLocale
    } catch (error) {
      console.warn(
        `Failed to load locale ${savedLocale}, using default:`,
        error
      )
      // Load default English locale
      const enMessages = await import('./locales/en.json')
      i18n.global.setLocaleMessage('en', enMessages.default)
    }
  } catch (error) {
    console.error('Failed to initialize locale from store:', error)
    // Load default English locale as fallback
    try {
      const enMessages = await import('./locales/en.json')
      i18n.global.setLocaleMessage('en', enMessages.default)
    } catch (enError) {
      console.error('Failed to load default locale:', enError)
    }
  }
}

async function initializeApp() {
  const app = createApp(App)

  try {
    // Initialize store service first
    const storeService = StoreService.getInstance()
    await storeService.initialize()

    // Initialize locale from store
    await initializeLocaleFromStore()

    console.info('App initialization completed successfully')
  } catch (error) {
    console.error('Failed to initialize app services:', error)
    // Continue with app initialization even if store/locale fails
  }

  // Use plugins
  app.use(router)
  app.use(i18n)
  app.use(Quasar, {
    plugins: { Notify },
    config: {
      brand: {},
      notify: {
        position: 'bottom-right',
        timeout: 3000,
        textColor: 'white',
        classes: 'phantom-font',
        badgeStyle: 'border: 2px solid var(--theme-border);',
        attrs: {
          style: 'border: 2px solid var(--theme-border);',
        },
      },
    },
  })

  // Mount the app
  app.mount('#app')
}

// Initialize and mount the app
initializeApp().catch(error => {
  console.error('Critical error during app initialization:', error)
  // Mount app anyway as fallback
  const app = createApp(App)
  app.use(router)
  app.use(i18n)
  app.use(Quasar, {
    plugins: { Notify },
    config: {
      brand: {},
      notify: {
        position: 'bottom-right',
        timeout: 3000,
        textColor: 'white',
        classes: 'phantom-font',
        badgeStyle: 'border: 2px solid var(--theme-border);',
        attrs: {
          style: 'border: 2px solid var(--theme-border);',
        },
      },
    },
  })
  app.mount('#app')
})

export default i18n
