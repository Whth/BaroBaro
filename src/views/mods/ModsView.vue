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
}

.mods-view h1 {
  margin: 0 0 var(--spacing-l) 0;
}
</style>
