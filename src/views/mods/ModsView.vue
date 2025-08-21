<template>
  <div class="mods-view">
    <h1>Mod Management</h1>
    <ModManagementTabs />
  </div>
</template>

<script setup lang="ts">
import ModManagementTabs from "../../components/mod-management/ModManagementTabs.vue";
</script>

<style scoped>
.mods-view {
  padding: var(--spacing-l);
  background: transparent;
  max-height: calc(100vh - 2 * var(--spacing-l));
  overflow: hidden;
}

.mods-view h1 {
  margin: 0 0 var(--spacing-l) 0;
}
</style>
