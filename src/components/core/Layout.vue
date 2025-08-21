<template>
  <n-layout class="layout animate-fade-in" has-sider>
    <Navigation />
    <n-layout>
      <n-layout-content class="main-content">
        <slot />
      </n-layout-content>
    </n-layout>
  </n-layout>
</template>

<script setup lang="ts">
import Navigation from "./Navigation.vue";
</script>

<style scoped>
.layout {
  min-height: 100vh;
  height: 100vh;
}

.main-content {
  padding: 24px;
  overflow-y: auto;
  background: transparent;
}

/* Ensure consistent page title spacing across all views */
.main-content > *:first-child > .page-title:first-child,
.main-content > .page-title:first-child {
  margin-bottom: 24px;
  display: block;
}

:deep(.n-layout) {
  height: 100%;
}

:deep(.n-layout-content) {
  height: 100%;
}
</style>
