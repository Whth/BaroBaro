<template>
  <div class="profile-selector">
    <label for="profile-select" class="profile-label">Active Profile:</label>
    <select
      id="profile-select"
      v-model="selectedProfile"
      class="profile-select"
    >
      <option
        v-for="profile in mod_lists"
        :key="profile.profileName"
        :value="profile.profileName"
      >
        {{ profile.profileName }}
      </option>
    </select>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { mod_lists } from "../../composables/useModManager";

const selectedProfile = ref("");

onMounted(() => {
	// Set the first mod list as the default selected profile
	if (mod_lists.value.length > 0) {
		selectedProfile.value = mod_lists.value[0].profileName;
	}
	console.log(
		"Profile selector mounted with",
		mod_lists.value.length,
		"profiles",
	);
});
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
