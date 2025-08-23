<template>
  <n-config-provider :theme="currentTheme">
    <div class="app-container">
      <Layout>
        <router-view/>
      </Layout>
    </div>
  </n-config-provider>
</template>

<script lang="ts" setup>
import {onMounted} from "vue";
import Layout from "./components/core/Layout.vue";
import {Theme} from "./proto/config.ts";
import {config, refresh_config} from "./invokes.ts";
import {currentTheme, theme_mapping} from "./composables/useTheme.ts";


onMounted(async () => {
  await refresh_config()
  currentTheme.value = theme_mapping[config.value.uiConfig?.theme || Theme.Light];
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
