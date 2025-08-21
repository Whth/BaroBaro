<template>
  <div class="update-mod-tab">
    <h2>Update Mods</h2>
    <div class="update-content">
      <div class="update-header">
        <p>{{ modsToUpdate.length }} mod(s) have available updates</p>
        <button
          v-if="modsToUpdate.length > 0"
          class="update-all-button"
          @click="updateAllMods"
        >
          Update All
        </button>
      </div>
      <div class="mods-to-update">
        <div v-if="modsToUpdate.length === 0" class="no-updates">
          All mods are up to date!
        </div>
        <div
          v-else
          v-for="mod in modsToUpdate"
          :key="mod.id"
          class="mod-update-card"
        >
          <div class="mod-info">
            <h3 class="mod-name">{{ mod.name }}</h3>
            <div class="version-info">
              <span class="current-version"
                >Current: {{ mod.currentVersion }}</span
              >
              <span class="arrow">→</span>
              <span class="new-version">New: {{ mod.newVersion }}</span>
            </div>
            <p class="mod-description">{{ mod.description }}</p>
          </div>
          <div class="update-actions">
            <button
              class="update-button"
              :disabled="mod.updating"
              @click="updateMod(mod.id)"
            >
              {{ mod.updating ? "Updating..." : "Update" }}
            </button>
            <button class="changelog-button" @click="viewChangelog(mod.id)">
              Changelog
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { refreshInstalledMods } from "../../composables/useModManager";

interface ModUpdate {
	id: string;
	name: string;
	currentVersion: string;
	newVersion: string;
	description: string;
	updating?: boolean;
}

const modsToUpdate = ref<ModUpdate[]>([
	{
		id: "1",
		name: "Better Graphics",
		currentVersion: "1.2.3",
		newVersion: "1.3.0",
		description: "Enhanced textures and lighting improvements",
	},
	{
		id: "2",
		name: "New Weapons Pack",
		currentVersion: "2.0.1",
		newVersion: "2.1.0",
		description: "Added 10 new weapons and balanced existing ones",
	},
]);

const _updateMod = async (modId: string) => {
	const mod = modsToUpdate.value.find((m) => m.id === modId);
	if (mod) {
		mod.updating = true;
		console.log(`Updating mod: ${mod.name}`);
		try {
			// In a real app, this would call the Tauri backend to update the mod
			// For now, we'll simulate the update and refresh the installed mods list
			await new Promise((resolve) => setTimeout(resolve, 2000));
			mod.updating = false;
			// Remove from list after update
			modsToUpdate.value = modsToUpdate.value.filter((m) => m.id !== modId);
			alert(`Updated ${mod.name} successfully!`);
			// Refresh the installed mods list after update
			await refreshInstalledMods();
		} catch (error) {
			mod.updating = false;
			console.error("Failed to update mod:", error);
			alert(`Failed to update ${mod.name}.`);
		}
	}
};

const _updateAllMods = async () => {
	for (const mod of modsToUpdate.value) {
		mod.updating = true;
		console.log(`Updating mod: ${mod.name}`);
		try {
			// In a real app, this would call the Tauri backend to update the mod
			// For now, we'll simulate the update and refresh the installed mods list
			await new Promise((resolve) => setTimeout(resolve, 2000));
			mod.updating = false;
			// Refresh the installed mods list after each update
			await refreshInstalledMods();
		} catch (error) {
			mod.updating = false;
			console.error("Failed to update mod:", error);
			alert(`Failed to update ${mod.name}.`);
		}
	}
	// Remove all updated mods from the list
	modsToUpdate.value = modsToUpdate.value.filter((m) => !m.updating);
	alert(`Updated all mods successfully!`);
};

const _viewChangelog = (modId: string) => {
	const mod = modsToUpdate.value.find((m) => m.id === modId);
	if (mod) {
		alert(
			`Changelog for ${mod.name}:\n- Improved performance\n- Fixed bugs\n- Added new features`,
		);
	}
};

onMounted(() => {
	console.log("Update mod tab mounted");
});
</script>

<style scoped>
.update-mod-tab h2 {
  margin-top: 0;
  margin-bottom: var(--spacing-l);
}

.update-content {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-l);
}

.update-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: var(--spacing-m);
  border-bottom: 1px solid var(--color-border);
}

.update-header p {
  margin: 0;
  color: var(--color-text-primary);
  font-weight: var(--font-weight-medium);
}

.update-all-button {
  padding: var(--spacing-s) var(--spacing-m);
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
}

.no-updates {
  text-align: center;
  padding: var(--spacing-xl);
  color: var(--color-text-secondary);
  font-style: italic;
}

.mods-to-update {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-m);
}

.mod-update-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-rounded);
  background-color: var(--color-surface);
}

.mod-info {
  flex: 1;
}

.mod-name {
  margin: 0 0 var(--spacing-xs) 0;
  font-size: var(--font-size-heading-3);
  color: var(--color-text-primary);
}

.version-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-s);
  margin-bottom: var(--spacing-s);
}

.current-version,
.new-version {
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
}

.arrow {
  color: var(--color-text-secondary);
}

.mod-description {
  margin: 0;
  color: var(--color-text-primary);
  font-size: var(--font-size-body-small);
  line-height: 1.4;
}

.update-actions {
  display: flex;
  gap: var(--spacing-s);
}

.update-button {
  padding: var(--spacing-s) var(--spacing-m);
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
}

.update-button:disabled {
  background-color: var(--color-text-secondary);
  cursor: not-allowed;
}

.changelog-button {
  padding: var(--spacing-s) var(--spacing-m);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
}
</style>
