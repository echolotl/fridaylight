<template>
  <q-layout view="lHh Lpr lFf" class="transparent-bg">
    <!-- Redesigned Sidebar that contains both the sidebar and main content -->
    <Sidebar v-model="sidebarOpen" @resize="handleSidebarResize">
      <!-- Main content is now handled within the Sidebar component -->
    </Sidebar>
  </q-layout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import Database from '@tauri-apps/plugin-sql';
import Sidebar from './components/Sidebar.vue';

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

// Initialize the app
onMounted(async () => {
  try {
    const db = await Database.load('sqlite:mods.db');
    window.db = db; // Make the database available globally
    
    // First check if the mods table exists and what columns it has
    const tableInfo: any[] = await db.select(`PRAGMA table_info(mods)`);
    
    if (tableInfo.length > 0) {
      // Table exists, check for missing columns
      const columns = tableInfo.map((col: any) => col.name);
      
      // Add missing columns if they don't exist
      if (!columns.includes('banner_data')) {
        await db.execute(`ALTER TABLE mods ADD COLUMN banner_data TEXT`);
        console.log('Added banner_data column to mods table');
      }
      
      if (!columns.includes('logo_data')) {
        await db.execute(`ALTER TABLE mods ADD COLUMN logo_data TEXT`);
        console.log('Added logo_data column to mods table');
      }
      
      if (!columns.includes('version')) {
        await db.execute(`ALTER TABLE mods ADD COLUMN version TEXT`);
        console.log('Added version column to mods table');
      }
      
      if (!columns.includes('engine_type')) {
        await db.execute(`ALTER TABLE mods ADD COLUMN engine_type TEXT`);
        console.log('Added engine_type column to mods table');
      }
      
      if (!columns.includes('display_order')) {
        await db.execute(`ALTER TABLE mods ADD COLUMN display_order INTEGER DEFAULT 9999`);
        console.log('Added display_order column to mods table');
      }
    } else {
      // Table doesn't exist, create it with all columns
      await db.execute(`
        CREATE TABLE IF NOT EXISTS mods (
          id TEXT PRIMARY KEY,
          name TEXT NOT NULL,
          path TEXT NOT NULL,
          executable_path TEXT,
          icon_data TEXT,
          banner_data TEXT,
          logo_data TEXT,
          version TEXT,
          engine_type TEXT,
          display_order INTEGER DEFAULT 9999
        )
      `);
      console.log('Created mods table with all columns');
    }
    
    // Check if settings table exists and create it if it doesn't
    try {
      const settingsTableInfo: any[] = await db.select(`PRAGMA table_info(settings)`);
      
      if (settingsTableInfo.length === 0) {
        // Table doesn't exist, create it
        await db.execute(`
          CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
          )
        `);
        
        // Insert default settings
        await db.execute(`
          INSERT INTO settings (key, value) VALUES 
          ('accentColor', '#FF0088'),
          ('installLocation', 'C:\\Users\\Public\\Documents\\FNF Mods')
        `);
        console.log('Created settings table with default values');
      }
    } catch (error) {
      console.error('Failed to initialize settings table:', error);
    }
    
    // Load mods from the database and sync with backend
    const mods = await db.select<any[]>('SELECT * FROM mods ORDER BY display_order ASC');
    console.log('Loaded mods from database:', mods);
    if (mods && mods.length > 0) {
      console.log(`Loading ${mods.length} mods from database to backend`);
      
      // Send the mods to the Rust backend to sync with ModsState
      await invoke('sync_mods_from_database', { modsData: mods });
      console.log('Successfully synced mods with backend');
    } else {
      console.log('No mods found in database');
    }
    
    console.log('App initialized and database tables updated');
  } catch (error) {
    console.error('Failed to initialize database:', error);
  }
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