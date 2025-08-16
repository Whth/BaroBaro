<template>
  <div class="search-filter">
    <div class="search-container">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search mods..."
        class="search-input"
      />
      <button class="search-button">🔍</button>
    </div>
    <div class="filter-container">
      <select v-model="statusFilter" class="filter-select">
        <option value="all">All Status</option>
        <option value="enabled">Enabled</option>
        <option value="disabled">Disabled</option>
      </select>
      <select v-model="typeFilter" class="filter-select">
        <option value="all">All Types</option>
        <option value="local">Local</option>
        <option value="remote">Remote</option>
      </select>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

const searchQuery = ref('')
const statusFilter = ref('all')
const typeFilter = ref('all')

// Emit events when filters change
watch([searchQuery, statusFilter, typeFilter], () => {
  console.log('Filters changed:', { searchQuery: searchQuery.value, statusFilter: statusFilter.value, typeFilter: typeFilter.value })
  // In a real app, this would emit an event to update the mod list
})
</script>

<style scoped>
.search-filter {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-m);
  background-color: var(--color-surface);
  border-radius: var(--border-radius-rounded);
  box-shadow: var(--shadow-level-1);
}

.search-container {
  display: flex;
  align-items: center;
  flex: 1;
  max-width: 400px;
}

.search-input {
  flex: 1;
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--font-size-body-regular);
}

.search-button {
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-left: none;
  border-radius: var(--border-radius-soft);
  border-top-left-radius: 0;
  border-bottom-left-radius: 0;
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  cursor: pointer;
}

.filter-container {
  display: flex;
  gap: var(--spacing-m);
}

.filter-select {
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--font-size-body-regular);
}
</style>