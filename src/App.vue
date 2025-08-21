<template>
  <div class="app-background" :style="backgroundStyle">
    <Layout>
      <router-view />
    </Layout>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import Layout from "./components/core/Layout.vue";
import { initializeApplication, getStoredLanguage } from "./composables/useAppInit";
import { config } from "./invokes";
import i18n from "./i18n";

// Background customization state
const backgroundSettings = ref({
	backgroundImage: "",
	backgroundOpacity: 1,
	backgroundBlur: 0,
});

// Theme state
const currentTheme = ref("LIGHT");
const currentLanguage = ref("en");

// Computed background style
const backgroundStyle = computed(() => {
	return {
		backgroundImage: backgroundSettings.value.backgroundImage
			? `url(${backgroundSettings.value.backgroundImage})`
			: "none",
		backgroundOpacity: backgroundSettings.value.backgroundOpacity,
		backdropFilter:
			backgroundSettings.value.backgroundBlur > 0
				? `blur(${backgroundSettings.value.backgroundBlur}px)`
				: "none",
	};
});

// Theme application functions
const applyTheme = (theme: string) => {
	const root = document.documentElement;

	// Add transition class for smooth theme change
	root.style.setProperty("--theme-transition-duration", "300ms");
	root.classList.add("theme-transitioning");

	// Set the theme
	root.setAttribute("data-theme", theme.toLowerCase());

	// Remove transition class after animation completes
	setTimeout(() => {
		root.classList.remove("theme-transitioning");
		root.style.removeProperty("--theme-transition-duration");
	}, 300);
};

// Load all preferences from backend config
onMounted(async () => {
	try {
		// Initialize the application (loads config and sets up everything)
		await initializeApplication();

		if (config.value.uiConfig) {
			const uiConfig = config.value.uiConfig;

			// Map backend theme enum to frontend string
			const themeMap: { [key: number]: string } = { 0: "DARK", 1: "LIGHT" };
			const languageMap: { [key: number]: string } = { 0: "en", 1: "zh" };

			currentTheme.value = themeMap[uiConfig.theme] || "LIGHT";
			currentLanguage.value = languageMap[uiConfig.language] || "en";

			// Update background settings
			backgroundSettings.value.backgroundImage = uiConfig.backgroundImage || "";
			backgroundSettings.value.backgroundOpacity =
				uiConfig.backgroundOpacity || 1;
			backgroundSettings.value.backgroundBlur =
				Number(uiConfig.backgroundBlur) || 0;
		}

		// Get stored language from the initialization
		currentLanguage.value = getStoredLanguage();

		// Set i18n locale
		i18n.global.locale = currentLanguage.value as "en" | "zh";
	} catch (e) {
		console.error("Failed to initialize app", e);
	}

	// Apply theme immediately on app load
	applyTheme(currentTheme.value);
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
}

#app {
  height: 100vh;
  overflow: hidden;
}

.app-background {
  height: 100vh;
  width: 100vw;
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  transition: background 0.3s ease;
}

/* Scrollbar styles */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: var(--color-surface);
}

::-webkit-scrollbar-thumb {
  background: var(--color-text-secondary);
  border-radius: var(--border-radius-soft);
}

::-webkit-scrollbar-thumb:hover {
  background: var(--color-text-primary);
}
</style>
