<template>
  <div class="theme-preview-container">
    <div :class="['theme-preview', `theme-preview-${themeName}`]">
      <div class="titlebar">
        <div class="close-button"></div>
      </div>
      <div class="window-content">
        <div class="sidebar" :class="compactMode ? 'compact' : ''">
          <div
            class="sidebar-item active"
            :class="compactMode ? 'compact' : ''"
          >
            <div
              class="sidebar-icon"
              :style="{ backgroundColor: accentColor }"
            ></div>
          </div>
          <div class="sidebar-item" :class="compactMode ? 'compact' : ''">
            <div
              class="sidebar-icon"
              :style="{ backgroundColor: accentColor }"
            ></div>
          </div>
          <div
            class="sidebar-item folder"
            :class="compactMode ? 'compact' : ''"
          >
            <div
              class="sidebar-icon"
              :style="{ backgroundColor: accentColor }"
            ></div>
          </div>
          <div
            class="folder-contents"
            :style="{
              borderLeftColor: accentColor,
              borderTopColor: accentColor,
            }"
            :class="compactMode ? 'compact' : ''"
          >
            <div
              class="sidebar-item active"
              :class="compactMode ? 'compact' : ''"
            >
              <div
                class="sidebar-icon"
                :style="{ backgroundColor: accentColor }"
              ></div>
            </div>
            <div class="sidebar-item" :class="compactMode ? 'compact' : ''">
              <div
                class="sidebar-icon"
                :style="{ backgroundColor: accentColor }"
              ></div>
            </div>
          </div>
        </div>
        <div class="main-content" :class="compactMode ? 'compact' : ''">
          <div class="main-banner-container">
            <div
              class="main-banner"
              :style="{ backgroundColor: accentColor }"
            ></div>
            <div class="main-banner-title">Theme Preview</div>
          </div>
          <div
            class="play-button"
            :style="{ backgroundColor: accentColor }"
          ></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps({
  themeName: {
    type: String,
    default: 'dark',
    validator: (value: string) =>
      [
        'light',
        'dark',
        'yourself',
        'hotline',
        'corruption',
        'shaggy',
        'qt',
        'garcello',
        'pump',
        'doe',
        'boo',
      ].includes(value),
  },
  accentColor: {
    type: String,
    default: '#DB2955',
  },
  compactMode: {
    type: Boolean,
    default: false,
  },
})
</script>

<style scoped>
.theme-preview-container {
  width: 100%;
  height: 30vh;
  margin-bottom: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.theme-preview {
  border-radius: 4px;
  height: 100%;
  width: calc(100vw * 0.3);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
  overflow: hidden;
}

/* Light Theme */
.theme-preview-light {
  background-color: #f5f5f5;
  color: #333;
  border: 2px solid rgba(0, 0, 0, 0.12);
}

/* Dark Theme*/
.theme-preview-dark {
  background-color: #121212;
  color: #fff;
  border: 2px solid rgba(255, 255, 255, 0.12);
}

/* Yourself Theme */
.theme-preview-yourself {
  background-color: #0f142c;
  color: #e6f1ff;
  border: 2px solid rgba(66, 133, 244, 0.3);
}

/* Hotline Theme */
.theme-preview-hotline {
  background: linear-gradient(135deg, #2c1c2d 0%, #462639 100%);
  color: #ffdab9;
  border: 2px solid rgba(255, 158, 97, 0.3);
}

/* Corruption Theme */
.theme-preview-corruption {
  background-color: #0f0f12;
  color: #ffa0b0;
  border: 2px solid rgba(255, 160, 184, 0.25);
}

/* Shaggy Theme */
.theme-preview-shaggy {
  background-color: #152423;
  color: #d5c4f0;
  border: 2px solid rgba(213, 196, 240, 0.25);
}

/* QT Theme */
.theme-preview-qt {
  background-color: #eecfd8;
  color: #49343f;
  border: 2px solid rgba(73, 52, 63, 0.25);
}

/* Garcello Theme */
.theme-preview-garcello {
  background-color: #dbeecf;
  color: #343b49;
  border: 2px solid rgba(52, 59, 73, 0.25);
}

/* Pump Theme */
.theme-preview-pump {
  background-color: #e6e0ad;
  color: #311c24;
  border: 2px solid rgba(49, 28, 36, 0.25);
}

/* Doe Theme */
.theme-preview-doe {
  background: linear-gradient(to top, #ffffff 20%, #9cedfc 50%, #7ba2f5);
  color: #2c1d07;
  border: 2px solid rgba(44, 29, 7, 0.25);
}

/* Boo Theme */
.theme-preview-boo {
  background-color: #cfe2ee;
  color: #614545;
  border: 2px solid rgba(97, 69, 69, 0.25);
}

.window-content {
  display: flex;
  height: 100%;
}

.sidebar {
  width: 30%;
  padding: 30px 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.sidebar.compact {
  width: 30px;
  margin-top: 20px;
  padding: 30px 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.sidebar-item.active {
  background-color: rgba(255, 255, 255, 0.25);
  height: 20px;
}

.sidebar-item {
  height: 20px;
  width: 100%;
  border-radius: 0 8px 8px 0;
  display: flex;
  align-items: center;
  justify-content: left;
}

.sidebar-item.folder {
  height: 10px;
}

.sidebar-item.folder .sidebar-icon {
  width: 10px;
  height: 5px;
}

.theme-preview-light .sidebar-item.active {
  background-color: #f0f0f0;
}

.theme-preview-dark .sidebar-item.active {
  background-color: #282828;
}

.theme-preview-yourself .sidebar-item.active {
  background-color: #121b38;
}

.theme-preview-hotline .sidebar-item.active {
  background-color: #462639;
}

.theme-preview-corruption .sidebar-item.active {
  background-color: #140f16;
}

.theme-preview-shaggy .sidebar-item.active {
  background-color: #1e3129;
}

/* Add styles for new themes */
.theme-preview-qt .sidebar-item.active {
  background-color: #e4bad1;
}

.theme-preview-garcello .sidebar-item.active {
  background-color: #bfe4ba;
}

.theme-preview-pump .sidebar-item.active {
  background-color: #d8c395;
}

.theme-preview-doe .sidebar-item.active {
  background-color: rgba(255, 255, 255, 0.5);
}

.theme-preview-boo .sidebar-item.active {
  background-color: #bac9e4;
}

.sidebar-icon {
  width: 10px;
  height: 10px;
  border-radius: 2px;
  margin-left: 5px;
}

.main-content {
  width: 70%;
  height: calc(100% - 18px);
  margin: 10px;
  margin-bottom: 0;
  border-radius: 4px 4px 0 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: 4px;
}
.main-content.compact {
  width: 100%;
  height: calc(100% - 18px);
  margin: 10px;
  margin-bottom: 0;
  border-radius: 4px 4px 0 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: 4px;
}

.theme-preview-light .main-content {
  background-color: #f0f0f0;
}

.theme-preview-dark .main-content {
  background-color: #282828;
}

.theme-preview-yourself .main-content {
  background-color: #121b38;
}

.theme-preview-hotline .main-content {
  background-color: #462639;
}

.theme-preview-corruption .main-content {
  background-color: #140f16;
}

.theme-preview-shaggy .main-content {
  background-color: #1e3129;
}

/* Main content styles for new themes */
.theme-preview-qt .main-content {
  background-color: #e4bad1;
}

.theme-preview-garcello .main-content {
  background-color: #bfe4ba;
}

.theme-preview-pump .main-content {
  background-color: #d8c395;
}

.theme-preview-doe .main-content {
  background-color: rgba(255, 255, 255, 0.5);
}

.theme-preview-boo .main-content {
  background-color: #bac9e4;
}

.titlebar {
  width: 100%;
  height: 8px;
  display: flex;
  align-items: center;
  justify-content: right;
  background-color: white;
}

.close-button {
  width: 15px;
  height: 100%;
  background-color: var(--red);
}
.main-banner-container {
  position: relative;
  width: 100%;
  height: 40%;
  overflow: hidden;
}
.main-banner {
  width: 100%;
  height: 100%;
  border-radius: 3px 3px 0 0;
  mask-image: linear-gradient(to bottom, black, transparent);
}
.main-banner-title {
  position: absolute;
  font-size: 0.75rem;
  text-align: left;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.6);
  padding: 4px;
  bottom: 0;
}

.folder-contents {
  margin-left: 9px;
  border-left: 2px dashed var(--theme-border);
}

.folder-contents.compact {
  margin-left: 0;
  border-left: 2px solid var(--theme-border);
  border-top: 2px solid var(--theme-border);
}

.play-button {
  width: 30px;
  height: 15px;
  border-radius: 2px;
  margin-left: 6px;
}
</style>
