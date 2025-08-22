<template>
  <n-layout class="layout" has-sider>
    <Navigation/>
    <n-layout>
      <n-layout-content class=main-content>
        <slot/>
      </n-layout-content>
    </n-layout>
  </n-layout>
</template>

<script lang="ts" setup>
import Navigation from "./Navigation.vue";
</script>

<style scoped>
.layout {
  min-height: 100vh;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--n-color-background);
}

.main-content {
  padding: 24px;
  overflow-y: auto;
  background: var(--n-color-background);
  flex: 1;
  position: relative;
  color: var(--n-text-color-primary);
}

:deep(.n-layout) {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--n-color-background);
}

:deep(.n-layout-content) {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: var(--n-color-background);
  color: var(--n-text-color-primary);
}

:deep(.n-layout-sider) {
  background: var(--n-menu-color);
  border-right: 1px solid var(--n-border-color);
}
</style>
