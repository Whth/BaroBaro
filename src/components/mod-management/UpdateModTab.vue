<template>
  <div class="update-mod-tab">
    <n-h2>Update Mods</n-h2>
    <n-empty v-if="installed_mod.length === 0" description="No installed mods found."/>
    <n-grid v-else :x-gap="12" :y-gap="12" cols="1 400:2 600:3 800:4" responsive="screen">
      <n-grid-item v-for="mod in installed_mod" :key="mod.steamWorkshopId">
        <n-card
            :bordered="true"
            :class="{ 'selected': selectedMods.has(mod.steamWorkshopId) }"
            :segmented="{ content: true, footer: 'soft' }"
            :title="mod.name || `Mod ${mod.steamWorkshopId}`"
            hoverable
            @click="toggleModSelection(mod.steamWorkshopId)"
        >
          <template #header-extra>
            <n-tag type="info">ID: {{ mod.steamWorkshopId }}</n-tag>
          </template>
          <n-space vertical>
            <n-ellipsis style="font-size: 0.9em">
              {{ mod.description || 'No description available' }}
            </n-ellipsis>
            <n-space justify="space-between">
              <n-text depth="3">Version: {{ mod.version || 'Unknown' }}</n-text>
              <n-text depth="3">Size: {{ formatBytes(mod.size) }}</n-text>
            </n-space>
          </n-space>
          <template #footer>
            <n-space align="center" justify="space-between">
              <n-tag :type="mod.updateRequired ? 'warning' : 'success'">
                {{ mod.updateRequired ? 'Update Available' : 'Up to date' }}
              </n-tag>
              <n-checkbox
                  :checked="selectedMods.has(mod.steamWorkshopId)"
                  @update:checked="toggleModSelection(mod.steamWorkshopId)"
                  @click.stop
              />
            </n-space>
          </template>
        </n-card>
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
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref } from "vue";
import {
	download_mods,
	installed_mod,
	list_installed_mods,
} from "../../invokes";

// Set to keep track of selected mods for update
const selectedMods = ref(new Set<number>());

// Toggle mod selection for update
function toggleModSelection(modId: number) {
	if (selectedMods.value.has(modId)) {
		selectedMods.value.delete(modId);
	} else {
		selectedMods.value.add(modId);
	}
}

// Update selected mods
async function updateSelectedMods() {
	if (selectedMods.value.size === 0) return;

	try {
		const modIds = Array.from(selectedMods.value);
		await download_mods(modIds);

		// Clear selection after update
		selectedMods.value.clear();

		// Refresh the mod list
		await list_installed_mods();
	} catch (error) {
		console.error("Failed to update mods:", error);
	}
}

// Format bytes to human readable format
function formatBytes(bytes: number | undefined): string {
	if (!bytes) return "Unknown";

	const sizes = ["Bytes", "KB", "MB", "GB"];
	if (bytes === 0) return "0 Byte";

	const i = Math.floor(Math.log(bytes) / Math.log(1024));
	return Math.round(bytes / Math.pow(1024, i)) + " " + sizes[i];
}

// Load mods on component mount
onMounted(async () => {
	await list_installed_mods();
});
</script>

<style scoped>
.update-mod-tab {
  padding: 20px;
}

.selected {
  border: 2px solid #409eff;
}
</style>