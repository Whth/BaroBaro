<template>
    <n-config-provider :theme="naiveTheme">
      <div class="app-container">
        <Layout>
          <router-view />
        </Layout>
      </div>
    </n-config-provider>
  </template>

<script setup lang="ts">
import { onMounted } from "vue";
import Layout from "./components/core/Layout.vue";
import { initializeApplication, getStoredLanguage } from "./composables/useAppInit";
import { useTheme } from "./composables/useTheme";
import i18n from "./i18n";

// Setup naive-ui theme provider
const { naiveTheme } = useTheme();

// Initialize the application
onMounted(async () => {
	try {
		await initializeApplication();
		const language = getStoredLanguage();
		i18n.global.locale = language as "en" | "zh";
	} catch (e) {
		console.error("Failed to initialize app:", e);
	}
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
