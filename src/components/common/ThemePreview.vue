<template>
  <div class="theme-preview-container">
    <div :class="['theme-preview', `theme-preview-${themeName}`]">
      <div class="titlebar">
        <div class="close-button"></div>
      </div>
      <div class="window-content">
        <div class="sidebar">
          <div class="sidebar-item active">
            <div class="sidebar-icon" :style="{ backgroundColor: accentColor }"></div>
          </div>
          <div class="sidebar-item">
            <div class="sidebar-icon" :style="{ backgroundColor: accentColor }"></div>
          </div>
          <div class="sidebar-item folder">
            <div class="sidebar-icon" :style="{ backgroundColor: accentColor }"></div>
          </div>
          <div class="folder-contents" :style="{ borderLeftColor: accentColor }">
            <div class="sidebar-item active">
              <div class="sidebar-icon" :style="{ backgroundColor: accentColor }"></div>
            </div>
            <div class="sidebar-item">
              <div class="sidebar-icon" :style="{ backgroundColor: accentColor }"></div>
            </div>
          </div>
        </div>
        <div class="main-content">
          <div class="main-banner-container">
          <div class="main-banner" :style="{ backgroundColor: accentColor }">
          </div>
          <div class="main-banner-title">Theme Preview</div>
        </div>
        <div class="play-button" :style="{ backgroundColor: accentColor }">
        </div>
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
    validator: (value: string) => ['light', 'dark', 'midnight', 'sunset', 'neon', 'forest', 'codename'].includes(value)
  },
  accentColor: {
    type: String,
    default: '#DB2955'
  }
});
</script>

<style scoped>
.theme-preview-container {
  width: 100%;
  height: 30vh;
  margin-bottom: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.theme-preview {
  border-radius: 4px;
  height: 100%;
  width: calc(100vw * 0.3);
  box-shadow: 0 3px 10px rgba(0, 0, 0, 0.3);
  overflow: hidden;
}

/* Light Theme - bright white */
.theme-preview-light {
  background-color: #f5f5f5;
  color: #333;
  border: 2px solid rgba(0, 0, 0, 0.12);
}

/* Dark Theme - standard dark gray */
.theme-preview-dark {
  background-color: #121212;
  color: #fff;
  border: 2px solid rgba(255, 255, 255, 0.12);
}

/* Midnight Theme - deep blue-black */
.theme-preview-midnight {
  background-color: #0a1929;
  color: #e6f1ff;
  border: 2px solid rgba(66, 133, 244, 0.3);
}

/* Sunset Theme - warm orange/dark red gradient */
.theme-preview-sunset {
  background: linear-gradient(135deg, #2c1c2d 0%, #462639 100%);
  color: #ffdab9;
  border: 2px solid rgba(255, 158, 97, 0.3);
}

/* Neon Theme - dark with vibrant highlights */
.theme-preview-neon {
  background-color: #0f0f12;
  color: #fcfcfc;
  border: 2px solid rgba(255, 0, 230, 0.3);
}

/* Forest Theme - earthy tones */
.theme-preview-forest {
  background-color: #6e9489;
  color: #d7e6cf;
  border: 2px solid rgba(47, 83, 66, 0.3);
}

/* Codename Theme - Codename Engine */
.theme-preview-codename {
  background-color: #151c24;
  color: #d5c4f0;
  border: 2px solid rgba(81, 64, 95, 0.12);
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
  background-color: rgba(0, 0, 0, 0.04);
}

.theme-preview-dark .sidebar-item.active {
  background-color: rgba(255, 255, 255, 0.1);
}

.theme-preview-midnight .sidebar-item.active {
  background-color: rgba(66, 133, 244, 0.1);
}

.theme-preview-sunset .sidebar-item.active {
  background-color: rgba(255, 158, 97, 0.1);
}

.theme-preview-neon .sidebar-item.active {
  background-color: rgba(255, 0, 230, 0.1);
}

.theme-preview-forest .sidebar-item.active {
  background-color: rgba(66, 114, 91, 0.1);
}

.theme-preview-codename .sidebar-item.active {
  background-color: rgba(69, 38, 95, 0.2);
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

.theme-preview-light .main-content {
  background-color: rgba(0, 0, 0, 0.04);
}

.theme-preview-dark .main-content {
  background-color: rgba(255, 255, 255, 0.1);
}

.theme-preview-midnight .main-content {
  background-color: rgba(66, 133, 244, 0.1);
}

.theme-preview-sunset .main-content {
  background-color: rgba(255, 158, 97, 0.1);
}

.theme-preview-neon .main-content {
  background-color: rgba(255, 0, 230, 0.1);
}

.theme-preview-forest .main-content {
  background-color: rgba(121, 180, 115, 0.1);
}

.theme-preview-codename .main-content {
  background-color: rgba(69, 38, 95, 0.2);
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
    font-size: .75rem;
    text-align: left;
    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.6);
    padding: 4px;
    bottom: 0;
}

.folder-contents {
    margin-left: 9px;
    border-left: 2px dashed var(--theme-border);
}

.play-button {
  width: 30px;
  height: 15px;
  border-radius: 2px;
  margin-left: 6px;
}
</style>