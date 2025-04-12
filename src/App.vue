<template>
  <q-layout view="lHh Lpr lFf" class="transparent-bg">
    <!-- Redesigned Sidebar that contains both the sidebar and main content -->
    <Sidebar v-model="sidebarOpen" @resize="handleSidebarResize">
      <!-- Main content is now handled within the Sidebar component -->
    </Sidebar>
  </q-layout>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { onOpenUrl } from '@tauri-apps/plugin-deep-link';
import Sidebar from './components/Sidebar.vue';
import { DatabaseService } from './services/dbService';

// Define window.db
declare global {
  interface Window {
    db: any;
  }
}

const sidebarOpen = ref(true);
const sidebarWidth = ref(250);

const handleSidebarResize = (width: number) => {
  sidebarWidth.value = width;
};

// Get the current system theme (light or dark)
const getSystemTheme = (): boolean => {
  return window.matchMedia && window.matchMedia('(prefers-color-scheme: light)').matches;
};

// Apply theme based on system or user preference
const applyTheme = async (useLightTheme: boolean) => {
  console.log('Applying theme:', useLightTheme ? 'light' : 'dark');
  
  // First check if we're running on Windows 11
  const isWindows11 = await invoke<boolean>('is_windows_11');
  console.log('Is Windows 11:', isWindows11);
  
  // Apply CSS classes for theme
  if (useLightTheme) {
    document.body.classList.add('light-theme');
    document.body.classList.remove('dark-theme');
  } else {
    document.body.classList.remove('light-theme');
    document.body.classList.add('dark-theme');
  }
  
  // Apply solid theme if not on Windows 11
  if (!isWindows11) {
    document.body.classList.add('solid-theme');
    
    // Remove transparent background styles
    document.documentElement.style.setProperty('--transparent-bg-override', 'none');
    
    // Set background to solid color instead of transparent
    const bgColor = useLightTheme ? 'var(--theme-bg)' : 'var(--theme-bg)';
    document.body.style.background = bgColor;
    document.querySelector('.q-layout')?.setAttribute('style', 'background: ' + bgColor + ' !important');
  } else {
    document.body.classList.remove('solid-theme');
    document.documentElement.style.setProperty('--transparent-bg-override', 'transparent');
    document.body.style.background = 'transparent';
    
    // Apply Mica effect based on theme (Windows 11 only)
    try {
      await invoke('change_mica_theme', { 
        window: 'main',
        dark: !useLightTheme
      });
      console.log('Applied Mica theme effect:', !useLightTheme ? 'dark' : 'light');
    } catch (error) {
      console.error('Failed to apply Mica effect:', error);
    }
  }
};

// Handle system theme changes
const handleSystemThemeChange = async (event: MediaQueryListEvent) => {
  // Only react to system theme changes if that setting is enabled
  const useSystemTheme = await getUseSystemThemeSetting();
  if (useSystemTheme) {
    const isLightTheme = event.matches;
    applyTheme(isLightTheme);
  }
};

// Get the current "use system theme" setting from the database
const getUseSystemThemeSetting = async (): Promise<boolean> => {
  try {
    const dbService = DatabaseService.getInstance();
    const value = await dbService.getSetting('useSystemTheme');
    return value === 'true';
  } catch (error) {
    console.error('Error fetching useSystemTheme setting:', error);
    return true; // Default to true on error
  }
};

// Get the manually set theme preference (only used if not using system theme)
const getThemePreference = async (): Promise<boolean> => {
  try {
    const dbService = DatabaseService.getInstance();
    const value = await dbService.getSetting('enableLightTheme');
    return value === 'true';
  } catch (error) {
    console.error('Error fetching theme preference:', error);
    return false; // Default to dark on error
  }
};

// Initialize the app
onMounted(async () => {
  try {
    // Set up deep link handler
    onOpenUrl((urls) => {
      console.log('deep link:', urls);
    });
    
    // Initialize the database service
    const dbService = DatabaseService.getInstance();
    await dbService.initialize();
    
    // Make the database service available globally
    window.db = {
      // Legacy interface for compatibility
      select: async (query: string, params?: any[]) => {
        // Pass this to the actual database via dbService when needed
        const db = await dbService['db'];
        return db.select(query, params);
      },
      execute: async (query: string, params?: any[]) => {
        // Pass this to the actual database via dbService when needed
        const db = await dbService['db'];
        return db.execute(query, params);
      },
      // Add the dbService instance for direct access
      service: dbService
    };
    
    // Load mods from the database
    const mods = await dbService.getAllMods();
    console.log('Loaded mods from database:', mods);
    if (mods && mods.length > 0) {
      console.log(`Loading ${mods.length} mods from database to backend`);
      // No need to sync with backend as the dbService handles this internally
    } else {
      console.log('No mods found in database');
    }
    
    // Apply initial theme based on system or user preference
    const useSystemTheme = await getUseSystemThemeSetting();
    const useLightTheme = useSystemTheme ? getSystemTheme() : await getThemePreference();
    applyTheme(useLightTheme);
    
    // Listen for system theme changes
    const mediaQuery = window.matchMedia('(prefers-color-scheme: light)');
    mediaQuery.addEventListener('change', handleSystemThemeChange);
    
    console.log('App initialized and database tables updated');
  } catch (error) {
    console.error('Failed to initialize database:', error);
  }
});

onUnmounted(() => {
  // Clean up the event listener for system theme changes
  const mediaQuery = window.matchMedia('(prefers-color-scheme: light)');
  mediaQuery.removeEventListener('change', handleSystemThemeChange);
});
</script>

<style>
html, body {
  background: transparent !important;
  margin: 0;
  padding: 0;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  color: var(--theme-text);
}

.transparent-bg {
  background: transparent !important;
}

/* Make sure any Quasar containers are also transparent */
.q-layout, .q-page-container, .q-page {
  background: transparent !important;
}

/* Remove all shadows from Quasar components */
.q-shadow-1, .q-shadow-2, .q-shadow-3, .q-shadow-4, .q-shadow-5, 
.q-shadow-6, .q-shadow-7, .q-shadow-8, .q-shadow-9, .q-shadow-10 {
  box-shadow: none !important;
}

/* Custom font face for phantom muff font */
@font-face {
  font-family: 'PhantomMuffFull';
  src: url('./assets/fonts/PhantomMuffFull.ttf') format('truetype');
  font-weight: normal;
  font-style: normal;
}

@font-face {
  font-family: 'PhantomMuffEmpty';
  src: url('./assets/fonts/PhantomMuffEmpty.ttf') format('truetype');
  font-weight: normal;
  font-style: normal;
}
</style>