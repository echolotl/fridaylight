<template>
  <div class="theme-preview-container">
    <div
      class="theme-preview"
      :style="{
        background: computedThemeStyles.backgroundColor,
        color: computedThemeStyles.color,
        border: curSelected
          ? `2px solid ${accentColor}`
          : computedThemeStyles.border,
      }"
    >
      <div class="titlebar">
        <div class="close-button"></div>
      </div>
      <div class="window-content">
        <div class="sidebar" :class="compactMode ? 'compact' : ''">
          <div
            class="sidebar-item active"
            :class="compactMode ? 'compact' : ''"
            :style="{ backgroundColor: computedThemeStyles.sidebarActiveColor }"
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
              :style="{
                backgroundColor: computedThemeStyles.sidebarActiveColor,
              }"
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
        <div
          class="main-content"
          :class="compactMode ? 'compact' : ''"
          :style="{ backgroundColor: computedThemeStyles.mainContentColor }"
        >
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
    <div class="theme-name">
      {{ themeDisplayName }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { themeService } from "../../services/themeService";

const props = defineProps({
  themeName: {
    type: String,
    default: "dark",
  },
  themeDisplayName: {
    type: String,
    default: "Dark",

  },
  accentColor: {
    type: String,
    default: '#DB2955',
  },
  compactMode: {
    type: Boolean,
    default: false,
  },
  curSelected: {
    type: Boolean,
    default: false,
  },
});

// Reactive variables for theme styles
const themeStyles = ref({
  backgroundColor: "#121212",
  color: "#fff",
  border: "2px solid rgba(255, 255, 255, 0.12)",
  sidebarActiveColor: "#282828",
  mainContentColor: "#282828",
});

// Predefined theme color mappings (fallback for dynamic generation)
const themeColorMap: Record<string, any> = {
  light: {
    backgroundColor: "#f5f5f5",
    color: "#333",
    border: "2px solid rgba(0, 0, 0, 0.12)",
    sidebarActiveColor: "#f0f0f0",
    mainContentColor: "#f0f0f0",
  },
  dark: {
    backgroundColor: "#121212",
    color: "#fff",
    border: "2px solid rgba(255, 255, 255, 0.12)",
    sidebarActiveColor: "#282828",
    mainContentColor: "#282828",
  },
  yourself: {
    backgroundColor: "#0f142c",
    color: "#e6f1ff",
    border: "2px solid rgba(66, 133, 244, 0.3)",
    sidebarActiveColor: "#121b38",
    mainContentColor: "#121b38",
  },
  hotline: {
    backgroundColor: "linear-gradient(135deg, #2c1c2d 0%, #462639 100%)",
    color: "#ffdab9",
    border: "2px solid rgba(255, 158, 97, 0.3)",
    sidebarActiveColor: "#462639",
    mainContentColor: "#462639",
  },
  corruption: {
    backgroundColor: "#0f0f12",
    color: "#ffa0b0",
    border: "2px solid rgba(255, 160, 184, 0.25)",
    sidebarActiveColor: "#140f16",
    mainContentColor: "#140f16",
  },
  shaggy: {
    backgroundColor: "#152423",
    color: "#d5c4f0",
    border: "2px solid rgba(213, 196, 240, 0.25)",
    sidebarActiveColor: "#1e3129",
    mainContentColor: "#1e3129",
  },
  qt: {
    backgroundColor: "#eecfd8",
    color: "#49343f",
    border: "2px solid rgba(73, 52, 63, 0.25)",
    sidebarActiveColor: "#e4bad1",
    mainContentColor: "#e4bad1",
  },
  garcello: {
    backgroundColor: "#dbeecf",
    color: "#343b49",
    border: "2px solid rgba(52, 59, 73, 0.25)",
    sidebarActiveColor: "#bfe4ba",
    mainContentColor: "#bfe4ba",
  },
  pump: {
    backgroundColor: "#e6e0ad",
    color: "#311c24",
    border: "2px solid rgba(49, 28, 36, 0.25)",
    sidebarActiveColor: "#d8c395",
    mainContentColor: "#d8c395",
  },
  doe: {
    backgroundColor:
      "linear-gradient(to top, #ffffff 20%, #9cedfc 50%, #7ba2f5)",
    color: "#2c1d07",
    border: "2px solid rgba(44, 29, 7, 0.25)",
    sidebarActiveColor: "rgba(255, 255, 255, 0.5)",
    mainContentColor: "rgba(255, 255, 255, 0.5)",
  },
  boo: {
    backgroundColor: "#cfe2ee",
    color: "#614545",
    border: "2px solid rgba(97, 69, 69, 0.25)",
    sidebarActiveColor: "#bac9e4",
    mainContentColor: "#bac9e4",
  },
};

// Computed property to get current theme styles
const computedThemeStyles = computed(() => {
  return themeStyles.value;
});

// Generate theme styles based on CSS variables or fallback
const generateThemeStyles = async () => {
  try {
    const theme = themeService.getTheme(props.themeName);
    if (!theme) {
      console.warn(
        `Theme '${props.themeName}' not found, using default dark theme`
      );
      themeStyles.value = themeColorMap.dark;
      return;
    }

    // For custom themes, try to extract CSS variables
    if (theme.isCustom && theme.cssContent) {
      const extractedStyles = extractCSSVariables(theme.cssContent);
      if (extractedStyles) {
        themeStyles.value = extractedStyles;
        return;
      }
    }

    // For built-in themes, use predefined mappings
    const themeName = theme.name;
    if (themeColorMap[themeName]) {
      themeStyles.value = themeColorMap[themeName];
    } else {
      // Fallback for unknown themes
      themeStyles.value = themeColorMap.dark;
    }
  } catch (error) {
    console.warn("Failed to generate theme styles:", error);
    themeStyles.value = themeColorMap.dark;
  }
};

// Extract CSS variables from custom theme CSS content
const extractCSSVariables = (cssContent: string): any | null => {
  try {
    // Parse CSS variables from the content
    const bgMatch = cssContent.match(/--theme-bg:\s*([^;]+);/);
    const cardMatch = cssContent.match(/--theme-card:\s*([^;]+);/);
    const textMatch = cssContent.match(/--theme-text:\s*([^;]+);/);
    const borderMatch = cssContent.match(/--theme-border:\s*([^;]+);/);

    if (!bgMatch || !textMatch) {
      return null; // Need at least background and text colors
    }

    const backgroundColor = bgMatch[1].trim();
    const color = textMatch[1].trim();
    const borderColor = borderMatch
      ? borderMatch[1].trim()
      : "rgba(128, 128, 128, 0.3)";
    const surfaceColor = cardMatch ? cardMatch[1].trim() : backgroundColor;

    return {
      backgroundColor,
      color,
      border: `2px solid ${borderColor}`,
      sidebarActiveColor: surfaceColor,
      mainContentColor: surfaceColor,
    };
  } catch (error) {
    console.warn("Failed to extract CSS variables:", error);
    return null;
  }
};

// Load theme styles when component mounts or theme changes
const loadThemeStyles = () => {
  generateThemeStyles();
};

onMounted(() => {
  loadThemeStyles();
});

// Watch for theme name changes
watch(
  () => props.themeName,
  () => {
    loadThemeStyles();
  },
  { immediate: true }
);

</script>

<style scoped>
.theme-preview-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
  margin: 10px;
  gap: 0.25rem;
}
.theme-preview {
  border-radius: 4px;
  height: 175px;
  width: 100%;
  max-width: 250px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
  overflow: hidden;
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

.sidebar-icon {
  width: 10px;
  height: 10px;
  border-radius: 2px;
  margin-left: 5px;
}

.main-content {
  width: 70%;
  height: calc(100%);
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
  height: calc(100%);
  margin: 10px;
  margin-bottom: 0;
  border-radius: 4px 4px 0 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: 4px;
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
