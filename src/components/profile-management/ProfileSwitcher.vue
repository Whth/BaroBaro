<template>
  <div class="profile-switcher">
    <label for="profile-switch" class="switcher-label">Profile:</label>
    <select
      id="profile-switch"
      v-model="selectedProfile"
      class="switcher-select"
    >
      <option v-for="profile in profiles" :key="profile.id" :value="profile.id">
        {{ profile.name }}
      </option>
    </select>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";

interface Profile {
	id: string;
	name: string;
}

const _profiles = ref<Profile[]>([
	{ id: "1", name: "Survival Profile" },
	{ id: "2", name: "Creative Profile" },
	{ id: "3", name: "Adventure Profile" },
]);

const _selectedProfile = ref("1");

onMounted(() => {
	console.log("Profile switcher mounted");
});
</script>

<style scoped>
.profile-switcher {
  display: flex;
  align-items: center;
  gap: var(--spacing-s);
}

.switcher-label {
  font-weight: var(--font-weight-medium);
  color: var(--color-text-primary);
}

.switcher-select {
  padding: var(--spacing-xs) var(--spacing-s);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--font-size-body-regular);
}
</style>
