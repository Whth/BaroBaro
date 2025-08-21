<template>
  <div class="dashboard-view">
    <div class="dashboard-header">
      <h1>Dashboard</h1>
      <ProfileSelector />
    </div>
    <SearchAndFilter />
    <ModList />
    <ModDetails />
  </div>
</template>

<script setup lang="ts">

</script>

<style scoped>
.dashboard-view {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-l);
}

.dashboard-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.dashboard-header h1 {
  margin: 0;
}
</style>
