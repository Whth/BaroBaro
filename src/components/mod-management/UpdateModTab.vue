<template>
  <n-scrollbar style="max-height: 60vh">
    <n-empty v-if="installed_mod.length === 0" description="No installed mods found."/>

    <n-grid v-else :x-gap="12" :y-gap="12" cols="1 400:2 600:3 800:4" responsive="screen">
      <n-grid-item v-for="mod in installed_mod" :key="mod.steamWorkshopId">
        <!-- Use ModItemDisplay instead of n-card -->
        <ModItemDisplay
            :class="{ 'selected': selectedMods.has(mod.steamWorkshopId) }"
            :mod="mod"
            :style="{ cursor: 'pointer', borderRadius: 'var(--n-border-radius)' }"
            @click="toggleModSelection(mod.steamWorkshopId)"
        />
      </n-grid-item>
    </n-grid>

    <n-space v-if="installed_mod.length > 0" justify="center" style="margin-top: 20px">
      <n-button
          :disabled="selectedMods.size === 0"
          ghost
          type="primary"
          @click="updateSelectedMods"
      >
        Update Selected ({{ selectedMods.size }})
      </n-button>
    </n-space>
  </n-scrollbar>
</template>

<script lang="ts" setup>
import { onMounted, ref } from "vue";
import {
	download_mods,
	installed_mod,
	list_installed_mods,
} from "../../invokes";
import ModItemDisplay from "./ModItemDisplay.vue";

// Set to track selected mods for update
const selectedMods = ref(new Set<number>());

// Toggle selection of a mod
function toggleModSelection(modId: number) {
	const isSelected = selectedMods.value.has(modId);
	if (isSelected) {
		selectedMods.value.delete(modId);
	} else {
		selectedMods.value.add(modId);
	}
}

// Update all selected mods
async function updateSelectedMods() {
	if (selectedMods.value.size === 0) return;

	try {
		const modIds = Array.from(selectedMods.value);
		await download_mods(modIds);

		// Clear selection after update
		selectedMods.value.clear();

		// Refresh the installed mod list
		await list_installed_mods();
	} catch (error) {
		console.error("Failed to update mods:", error);
	}
}

// Load installed mods on mount
onMounted(async () => {
	await list_installed_mods();
});
</script>

<style scoped>
/* Optional: Highlight selected mod with border or shadow */
.selected {
  box-shadow: 0 0 0 2px #409eff !important;
}
</style>