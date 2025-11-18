<template>
  <n-scrollbar style="max-height: 60vh">
    <n-empty v-if="installed_mod.length === 0" description="No installed mods found."/>

    <div v-else>
      <n-space justify="center" style="margin-bottom: 16px">
        <n-button
            v-if="selectedMods.size > 0"
            type="primary"
            @click="showUpdateDialog = true"
        >
          Update Selected ({{ selectedMods.size }})
        </n-button>
        <n-button
            :disabled="selectedMods.size === 0"
            ghost
            @click="clearSelection"
        >
          Clear Selection
        </n-button>
        <n-button
            :loading="isRefreshing"
            ghost
            type="info"
            @click="refreshMods"
        >
          Refresh Mods
        </n-button>
      </n-space>

      <n-grid v-if="installed_mod.length > 0" :x-gap="12" :y-gap="12" cols="1 400:2 600:3 800:4" responsive="screen">
        <n-grid-item v-for="mod in installed_mod" :key="mod.steamWorkshopId">
          <div
              :style="{
              cursor: 'pointer',
              borderRadius: 'var(--n-border-radius)',
              padding: '4px',
              backgroundColor: selectedMods.has(mod.steamWorkshopId) ? 'rgba(64, 158, 255, 0.1)' : 'transparent',
              border: selectedMods.has(mod.steamWorkshopId) ? '2px solid #409eff' : '2px solid transparent',
              transition: 'all 0.2s ease'
            }"
              @click="toggleModSelection(mod.steamWorkshopId)"
          >
            <ModItemDisplay :mod="mod"/>
          </div>
        </n-grid-item>
      </n-grid>
    </div>
  </n-scrollbar>

  <!-- Update confirmation dialog -->
  <n-modal v-model:show="showUpdateDialog" preset="dialog" title="Update Mods">
    <div>
      <p>Are you sure you want to update <strong>{{ selectedMods.size }}</strong> selected mod(s)?</p>
      <p style="margin-top: 8px; color: #666; font-size: 12px;">
        This will delete the current versions and redownload the latest versions from Steam Workshop.
      </p>
    </div>
    <template #action>
      <n-button ghost @click="cancelUpdate">Cancel</n-button>
      <n-button :loading="isUpdating" type="primary" @click="confirmUpdate">Update</n-button>
    </template>
  </n-modal>

  <!-- Update progress dialog -->
  <n-modal v-model:show="showProgressDialog" :title="`Updating ${updatingMods.length} mod(s)`" preset="dialog">
    <div>
      <n-progress
          :percentage="updateProgress"
          :processing="true"
          style="margin-bottom: 16px"
          type="line"
      />

      <n-list>
        <n-list-item v-for="mod in updatingMods" :key="mod.steamWorkshopId">
          <div style="display: flex; justify-content: space-between; align-items: center;">
            <span>{{ mod.name }}</span>
            <n-tag
                :type="getUpdateStatusType(mod.steamWorkshopId)"
                size="small"
            >
              {{ getUpdateStatusText(mod.steamWorkshopId) }}
            </n-tag>
          </div>
        </n-list-item>
      </n-list>
    </div>
    <template #action>
      <n-button
          v-if="!isUpdating"
          type="primary"
          @click="closeProgressDialog"
      >
        Close
      </n-button>
    </template>
  </n-modal>
</template>

<script lang="ts" setup>
import { onMounted, ref } from "vue";
import {
	download_mods,
	installed_mod,
	list_installed_mods,
	remove_mods,
} from "../../invokes";
import ModItemDisplay from "./ModItemDisplay.vue";
import { BarotraumaMod } from "../../proto/mods.ts";
import { useMessage } from "naive-ui";

// Message API for notifications
const message = useMessage();

// Selection management
const selectedMods = ref(new Set<number>());

// Dialog states
const showUpdateDialog = ref(false);
const showProgressDialog = ref(false);
const updatingMods = ref<BarotraumaMod[]>([]);

// Loading states
const isUpdating = ref(false);
const isRefreshing = ref(false);

// Progress tracking
const updateProgress = ref(0);
const updateStatuses = ref<
	Map<number, "pending" | "updating" | "success" | "error">
>(new Map());

// Toggle mod selection
function toggleModSelection(modId: number) {
	const isSelected = selectedMods.value.has(modId);
	if (isSelected) {
		selectedMods.value.delete(modId);
	} else {
		selectedMods.value.add(modId);
	}
}

// Clear all selections
function clearSelection() {
	selectedMods.value.clear();
}

// Get update status type for display
function getUpdateStatusType(
	modId: number,
): "default" | "success" | "error" | "warning" | "info" {
	const status = updateStatuses.value.get(modId);
	switch (status) {
		case "success":
			return "success";
		case "error":
			return "error";
		case "updating":
			return "info";
		default:
			return "default";
	}
}

// Get update status text for display
function getUpdateStatusText(modId: number): string {
	const status = updateStatuses.value.get(modId);
	switch (status) {
		case "success":
			return "Updated";
		case "error":
			return "Failed";
		case "updating":
			return "Updating...";
		default:
			return "Pending";
	}
}

// Cancel update
function cancelUpdate() {
	showUpdateDialog.value = false;
}

// Close progress dialog
function closeProgressDialog() {
	showProgressDialog.value = false;
	updateProgress.value = 0;
	updateStatuses.value.clear();
}

// Refresh mods list
async function refreshMods() {
	isRefreshing.value = true;
	try {
		await list_installed_mods();
		message.success("Mods refreshed");
	} catch (error) {
		console.error("Failed to refresh mods:", error);
		message.error("Failed to refresh mods");
	} finally {
		isRefreshing.value = false;
	}
}

// Confirm update
async function confirmUpdate() {
	if (selectedMods.value.size === 0) return;

	isUpdating.value = true;
	showUpdateDialog.value = false;
	showProgressDialog.value = true;

	const modIds = Array.from(selectedMods.value);
	const selectedModList = installed_mod.value.filter((mod) =>
		modIds.includes(mod.steamWorkshopId),
	);
	updatingMods.value = selectedModList;

	// Initialize status tracking
	selectedModList.forEach((mod) => {
		updateStatuses.value.set(mod.steamWorkshopId, "pending");
	});

	let completedCount = 0;
	const totalCount = selectedModList.length;

	try {
		// Update each mod individually to track progress
		for (const mod of selectedModList) {
			updateStatuses.value.set(mod.steamWorkshopId, "updating");

			try {
				// Step 1: Remove the old mod
				await remove_mods([mod.steamWorkshopId]);

				// Step 2: Download the updated mod
				await download_mods([mod.steamWorkshopId]);

				// Mark as successful
				updateStatuses.value.set(mod.steamWorkshopId, "success");
				message.success(`Updated "${mod.name}"`);
			} catch (error) {
				console.error(`Failed to update mod ${mod.name}:`, error);
				updateStatuses.value.set(mod.steamWorkshopId, "error");
				message.error(`Failed to update "${mod.name}"`);
			}

			completedCount++;
			updateProgress.value = Math.round((completedCount / totalCount) * 100);
		}

		// Refresh the mod list after all updates
		await list_installed_mods();

		// Clear selections
		clearSelection();

		if (completedCount === totalCount) {
			message.success("All mods updated successfully!");
		} else {
			message.warning(`Updated ${completedCount} out of ${totalCount} mods`);
		}
	} catch (error) {
		console.error("Failed to update mods:", error);
		message.error("Failed to update mods");
	} finally {
		isUpdating.value = false;
	}
}

// Load installed mods on mount
onMounted(async () => {
	await list_installed_mods();
});
</script>

<style scoped>
.selected {
  box-shadow: 0 0 0 2px #409eff !important;
}
</style>