<template>
  <div class="layout">
    <div class="layout-wrapper" :class="{ 'sidebar-collapsed': sidebarState.isCollapsed.value }">
      <Navigation class="acrylic full-height" />
      <main class="main-content">
        <slot />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject, ref, watch } from "vue";
import Navigation from "./Navigation.vue";

// Inject sidebar state from Navigation component
const sidebarState = inject('sidebarState', {
	isCollapsed: ref(false),
	toggleSidebar: () => {}
});
</script>

<style scoped>
.layout {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.layout-wrapper {
  display: flex;
  flex: 1;
  min-height: 0;
}


.main-content {
   flex: 1;
   padding: var(--spacing-l);
   overflow-y: auto;
   transition: padding 0.3s cubic-bezier(0.4, 0, 0.2, 1);
   background: transparent;
}

.layout-wrapper.sidebar-collapsed .main-content {
  padding-left: calc(var(--spacing-l) + 60px - 250px);
}
</style>
