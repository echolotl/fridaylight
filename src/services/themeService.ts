import { appDataDir, sep } from '@tauri-apps/api/path'
import { readDir, exists, mkdir, readTextFile } from '@tauri-apps/plugin-fs'

export interface Theme {
  id: string
  name: string
  displayName: string
  isBuiltIn: boolean
  isCustom: boolean
  filePath?: string
  cssContent?: string
}

class ThemeService {
  private themes: Theme[] = []
  private loadedThemeStyleElement: HTMLStyleElement | null = null
  private customThemesDir: string = ''

  // Built-in themes that come with the app
  private builtInThemes: Omit<Theme, 'isBuiltIn' | 'isCustom'>[] = [
    { id: 'dark', name: 'dark', displayName: 'Dark' },
    { id: 'light', name: 'light', displayName: 'Light' },
    { id: 'yourself', name: 'yourself', displayName: 'Yourself' },
    { id: 'hotline', name: 'hotline', displayName: 'Hotline' },
    { id: 'corruption', name: 'corruption', displayName: 'Corruption' },
    { id: 'shaggy', name: 'shaggy', displayName: 'Shaggy' },
    { id: 'qt', name: 'qt', displayName: 'QT' },
    { id: 'garcello', name: 'garcello', displayName: 'Garcello' },
    { id: 'pump', name: 'pump', displayName: 'Pump' },
    { id: 'boo', name: 'boo', displayName: 'Boo' },
    { id: 'doe', name: 'doe', displayName: 'Doe' },
  ]
  async initialize(): Promise<void> {
    try {
      // Set up custom themes directory
      this.customThemesDir = await appDataDir()
      this.customThemesDir = `${this.customThemesDir}${sep()}themes`

      // Ensure custom themes directory exists
      if (!(await exists(this.customThemesDir))) {
        await mkdir(this.customThemesDir)
      }

      // Load all themes
      await this.loadThemes()
    } catch (error) {
      console.error('Failed to initialize theme service:', error)
      // Fallback to built-in themes only
      this.themes = this.builtInThemes.map(theme => ({
        ...theme,
        isBuiltIn: true,
        isCustom: false,
      }))
    }

    // Ensure we have at least the light theme available
    if (!this.getTheme('light')) {
      console.warn('Light theme not found in themes list, adding fallback')
      this.themes.push({
        id: 'light',
        name: 'light',
        displayName: 'Light',
        isBuiltIn: true,
        isCustom: false,
      })
    }
  }

  private async loadThemes(): Promise<void> {
    this.themes = []

    // Load built-in themes
    for (const theme of this.builtInThemes) {
      this.themes.push({
        ...theme,
        isBuiltIn: true,
        isCustom: false,
      })
    }

    // Load custom themes
    await this.loadCustomThemes()
  }

  private async loadCustomThemes(): Promise<void> {
    try {
      const entries = await readDir(this.customThemesDir)

      for (const entry of entries) {
        if (entry.name && entry.name.endsWith('.css')) {
          const themeName = entry.name.replace('.css', '')
          const themeId = `custom-${themeName}`

          // Read the CSS content
          const cssContent = await readTextFile(
            `${this.customThemesDir}${sep()}${entry.name}`
          )

          this.themes.push({
            id: themeId,
            name: themeName,
            displayName: this.formatThemeName(themeName),
            isBuiltIn: false,
            isCustom: true,
            filePath: `${this.customThemesDir}${sep()}${entry.name}`,
            cssContent,
          })
        }
      }
    } catch (error) {
      console.warn('Failed to load custom themes:', error)
    }
  }

  private formatThemeName(name: string): string {
    // Convert snake_case or kebab-case to Title Case
    return name.replace(/[-_]/g, ' ').replace(/\b\w/g, l => l.toUpperCase())
  }

  async refreshThemes(): Promise<void> {
    await this.loadThemes()
  }

  getThemes(): Theme[] {
    return [...this.themes]
  }

  getTheme(id: string): Theme | undefined {
    return this.themes.find(theme => theme.id === id)
  }
  async loadThemeCSS(themeId: string): Promise<string> {
    const theme = this.getTheme(themeId)
    if (!theme) {
      throw new Error(`Theme '${themeId}' not found`)
    }

    if (theme.isCustom && theme.cssContent) {
      return theme.cssContent
    }

    if (theme.isBuiltIn) {
      // For built-in themes, import the CSS file as text
      try {
        const cssModule = await import(`../themes/${theme.name}.css?raw`)
        return cssModule.default
      } catch (error) {
        console.error(`Failed to load built-in theme '${themeId}':`, error)

        // If this is not already the light theme, try to fall back to it
        if (themeId !== 'light') {
          console.warn(
            `Attempting to load light theme as fallback for '${themeId}'`
          )
          try {
            const fallbackModule = await import(`../themes/light.css?raw`)
            return fallbackModule.default
          } catch (fallbackError) {
            console.error('Failed to load light theme fallback:', fallbackError)
          }
        }

        throw error
      }
    }

    throw new Error(`Unable to load CSS for theme '${themeId}'`)
  }
  async applyTheme(themeId: string): Promise<void> {
    try {
      // Remove previous theme CSS
      if (this.loadedThemeStyleElement) {
        this.loadedThemeStyleElement.remove()
        this.loadedThemeStyleElement = null
      }

      // Load and apply new theme CSS
      const cssContent = await this.loadThemeCSS(themeId)

      this.loadedThemeStyleElement = document.createElement('style')
      this.loadedThemeStyleElement.textContent = cssContent
      this.loadedThemeStyleElement.id = 'dynamic-theme-css'
      document.head.appendChild(this.loadedThemeStyleElement)

      // Apply theme class to body (for backward compatibility)
      const theme = this.getTheme(themeId)
      if (theme) {
        // Remove all existing theme classes
        document.body.classList.forEach(className => {
          if (className.endsWith('-theme')) {
            document.body.classList.remove(className)
          }
        })

        // Add new theme class
        document.body.classList.add(`${theme.name}-theme`)
      }
    } catch (error) {
      console.error(`Failed to apply theme '${themeId}':`, error)

      // Reset to light theme as fallback
      if (themeId !== 'light') {
        console.warn('Falling back to light theme due to error')
        try {
          await this.applyTheme('light')
        } catch (fallbackError) {
          console.error('Failed to apply fallback light theme:', fallbackError)
          // If even the light theme fails, apply a minimal safe theme
          this.applyMinimalTheme()
        }
      } else {
        // If light theme itself failed, apply minimal theme
        this.applyMinimalTheme()
      }
    }
  }
  getCustomThemesDirectory(): string {
    return this.customThemesDir
  }

  private applyMinimalTheme(): void {
    // Apply a basic light theme as an absolute fallback (shouldn't happen?)
    const minimalCSS = `
      body {
        background-color: #ffffff;
        color: #000000;
      }
    `

    // Remove previous theme CSS
    if (this.loadedThemeStyleElement) {
      this.loadedThemeStyleElement.remove()
    }

    this.loadedThemeStyleElement = document.createElement('style')
    this.loadedThemeStyleElement.textContent = minimalCSS
    this.loadedThemeStyleElement.id = 'minimal-theme-css'
    document.head.appendChild(this.loadedThemeStyleElement)

    // Remove all existing theme classes and add light theme class
    document.body.classList.forEach(className => {
      if (className.endsWith('-theme')) {
        document.body.classList.remove(className)
      }
    })
    document.body.classList.add('light-theme')

    console.info('Applied minimal fallback theme')
  }

  // Check if a theme supports Windows 11 Mica effect
  supportsWindowsMica(themeId: string): boolean {
    const theme = this.getTheme(themeId)
    if (!theme) return false

    // Only light and dark themes support Mica effect
    return theme.name === 'light' || theme.name === 'dark'
  }
}

// Export singleton instance
export const themeService = new ThemeService()
