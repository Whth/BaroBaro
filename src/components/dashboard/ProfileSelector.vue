<template>
  <div class="profile-selector">
    <label for="profile-select" class="profile-label">Active Profile:</label>
    <select id="profile-select" v-model="selectedProfile" class="profile-select">
      <option v-for="profile in profiles" :key="profile.id" :value="profile.id">
        {{ profile.name }}
      </option>
    </select>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface Profile {
  id: string
  name: string
}

const profiles = ref<Profile[]>([
  { id: '1', name: 'Survival Profile' },
  { id: '2', name: 'Creative Profile' },
  { id: '3', name: 'Adventure Profile' }
])

const selectedProfile = ref('1')

onMounted(() => {
  // In a real app, this would be loaded from the store or backend
  console.log('Profile selector mounted')
})
</script>

<style scoped>
.profile-selector {
  display: flex;
  align-items: center;
  gap: var(--spacing-s);
}

.profile-label {
  font-weight: var(--font-weight-medium);
  color: var(--color-text-primary);
}

.profile-select {
  padding: var(--spacing-xs) var(--spacing-s);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--font-size-body-regular);
}
</style>