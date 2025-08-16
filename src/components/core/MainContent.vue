<template>
  <main class="main-content">
    <router-view />
  </main>
</template>

<script setup lang="ts">
</script>

<style scoped>
.main-content {
  flex: 1;
  padding: var(--spacing-l);
  overflow-y: auto;
}
</style>