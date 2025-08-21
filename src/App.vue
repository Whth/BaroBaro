<template>
  <div class="app-background" :style="backgroundStyle">
    <Layout>
      <router-view />
    </Layout>
  </div>
</template>

<script setup lang="ts">
import Layout from "./components/core/Layout.vue";
import { ref, computed, onMounted } from "vue";

// Background customization state
const backgroundSettings = ref({
  backgroundImage: "",
  backgroundOpacity: 1,
  backgroundBlur: 0,
});

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

// Load background settings from localStorage
onMounted(() => {
  const savedSettings = localStorage.getItem("backgroundSettings");
  if (savedSettings) {
    try {
      backgroundSettings.value = JSON.parse(savedSettings);
    } catch (e) {
      console.error("Failed to parse background settings", e);
    }
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
