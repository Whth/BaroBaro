<template>
  <n-config-provider :theme="naiveTheme" :themeOverrides="themeOverrides">
    <div class="app-container">
      <Layout>
        <router-view/>
      </Layout>
    </div>
  </n-config-provider>
</template>

<script lang="ts" setup>
import {computed, onMounted} from "vue";
import Layout from "./components/core/Layout.vue";
import {getStoredLanguage, initializeApplication,} from "./composables/useAppInit";
import {useTheme} from "./composables/useTheme";
import i18n from "./i18n";

// Setup naive-ui theme provider
const {themeMode, naiveTheme, initializeTheme, getStoredTheme, setTheme} = useTheme();

// Custom theme overrides for better integration
const themeOverrides = computed(() => {
  const baseOverrides = {
    common: {
      primaryColor: "#3b82f6",
      primaryColorHover: "#1d4ed8",
      primaryColorPressed: "#1e40af",
      primaryColorSuppl: "#3b82f6",
    },
  };

  // Add theme-specific overrides
  if (themeMode.value === "dark") {
    return {
      ...baseOverrides,
      Button: {
        color: "#374151",
        colorHover: "#4b5563",
        colorPressed: "#1f2937",
        colorFocus: "#4b5563",
        border: "#4b5563",
        borderHover: "#6b7280",
        borderPressed: "#374151",
      },
      Card: {
        color: "#1f2937",
        borderColor: "#374151",
      },
      Menu: {
        color: "#111827",
        groupTextColor: "#9ca3af",
        itemColorActive: "#1d4ed8",
        itemColorActiveHover: "#1e40af",
        itemTextColor: "#f9fafb",
        itemTextColorHover: "#ffffff",
        itemTextColorActive: "#ffffff",
      },
    };
  } else {
    return {
      ...baseOverrides,
      Button: {
        color: "#ffffff",
        colorHover: "#f3f4f6",
        colorPressed: "#e5e7eb",
        colorFocus: "#f3f4f6",
        border: "#d1d5db",
        borderHover: "#9ca3af",
        borderPressed: "#6b7280",
      },
      Card: {
        color: "#ffffff",
        borderColor: "#e5e7eb",
      },
      Menu: {
        color: "#ffffff",
        groupTextColor: "#6b7280",
        itemColorActive: "#dbeafe",
        itemColorActiveHover: "#bfdbfe",
        itemTextColor: "#111827",
        itemTextColorHover: "#000000",
        itemTextColorActive: "#1e40af",
      },
    };
  }
});

// Initialize the application
onMounted(async () => {
  try {
    await initializeApplication();

    // After config is loaded, get the fresh theme from localStorage
    const storedTheme = getStoredTheme();
    console.log("Applying theme from config:", storedTheme);

    // Apply theme and initialize theme system
    await setTheme(storedTheme);
    await initializeTheme();

    // Apply language
    const language = getStoredLanguage();
    i18n.global.locale = language as "en" | "zh";

    console.log("App initialization complete - theme and language applied");
  } catch (e) {
    console.error("Failed to initialize app:", e);
  }

  // Listen for theme change events
  window.addEventListener("theme-changed", (event: any) => {
    console.log("Theme changed to:", event.detail.theme);
  });

  // Load background image on app start
  const loadInitialBackground = async () => {
    try {
      const {get_background_image} = await import("./invokes");
      const backgroundDataUrl = await get_background_image();
      if (backgroundDataUrl) {
        document.documentElement.style.setProperty(
            "--background-image",
            backgroundDataUrl,
        );
        console.log("Initial background image loaded");
      } else {
        document.documentElement.style.setProperty(
            "--background-image",
            "none",
        );
        console.log("No initial background image configured");
      }
    } catch (error) {
      console.error("Failed to load initial background image:", error);
      document.documentElement.style.setProperty("--background-image", "none");
    }
  };
  await loadInitialBackground();
});
</script>

<style>
/* Global styles */
:root {
  font-family: var(--font-family-primary);
  font-size: 16px;
  line-height: 24px;
  font-weight: var(--font-weight-regular);
  color: var(--color-text-primary);
  background-color: var(--color-background);
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  margin: 0;
  padding: 0;
  background: var(--background-image, none);
  background-size: var(--background-size);
  background-position: var(--background-position);
  background-repeat: var(--background-repeat);
  transition: all 0.1s ease;
}

body::before {
  content: '';
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: var(--background-image, var(--color-background));
  opacity: var(--background-opacity);
  backdrop-filter: blur(var(--background-blur));
  -webkit-backdrop-filter: blur(var(--background-blur));
  z-index: -1;
  pointer-events: none;
}


.app-container {
  min-height: 100vh;
  background: var(--color-background);
}

/* Simple scrollbar styles */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: var(--color-surface);
}

::-webkit-scrollbar-thumb {
  background: var(--color-text-secondary);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--color-text-primary);
}
</style>
