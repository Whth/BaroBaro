<template>
  <n-scrollbar style="max-height: 60vh">
    <n-empty v-if="installed_mod.length === 0" :description="$t('modManagement.noInstalledMods')"/>

    <div v-else>
      <!-- Compact bulk actions -->
      <div class="bulk-actions">
        <n-button
            :disabled="selectedMods.size === 0"
            class="action-btn"
            size="medium"
            type="primary"
            @click="showUpdateDialog = true"
        >
          {{ $t('modManagement.updateSelected', {count: selectedMods.size}) }}
        </n-button>
        <n-button
            :disabled="selectedMods.size === 0"
            class="action-btn"
            ghost
            size="medium"
            @click="clearSelection"
        >
          {{ $t('modManagement.clearSelection') }}
        </n-button>
        <n-button
            :loading="isRefreshing"
            class="action-btn"
            ghost
            size="medium"
            type="info"
            @click="refreshMods"
        >
          {{ $t('modManagement.refreshMods') }}
        </n-button>
      </div>


      <!-- Compact mod list -->
      <div class="mod-grid">
        <div
            v-for="mod in installed_mod"
            :key="mod.steamWorkshopId"
            :class="{ selected: selectedMods.has(mod.steamWorkshopId) }"
            class="mod-item"
            @click="toggleModSelection(mod.steamWorkshopId)"
        >
          <div class="mod-content">
            <div class="mod-info">
              <div class="mod-header">
                <h4 class="mod-title">{{ mod.name }}</h4>
                <div class="mod-tags">
                  <n-tag
                      v-for="tag in mod.tags.slice(0, 3)"
                      :key="tag"
                      :style="getTagStyle(tag)"
                      round
                      size="small"
                  >
                    {{ tag }}
                  </n-tag>
                  <span v-if="mod.tags.length > 3" class="more-tags">+{{ mod.tags.length - 3 }}</span>
                </div>
              </div>
              <div class="mod-details">
                <span class="detail-item">v{{ mod.modVersion }}</span>
                <span class="detail-item">Game: {{ mod.gameVersion }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </n-scrollbar>

  <!-- Update confirmation dialog -->
  <n-modal v-model:show="showUpdateDialog" :title="$t('modManagement.updateMod')" preset="dialog">
    <div>
      <p>{{ $t('modManagement.confirmUpdateMultiple', {count: selectedMods.size}) }}</p>
      <p style="margin-top: 8px; color: var(--n-text-color-2); font-size: 12px;">
        {{ $t('modManagement.updateWarning') }}
      </p>
    </div>
    <template #action>
      <n-button ghost @click="cancelUpdate">{{ $t('app.cancel') }}</n-button>
      <n-button :loading="isUpdating" type="primary" @click="confirmUpdate">{{
          $t('modManagement.updateMod')
        }}
      </n-button>
    </template>
  </n-modal>

  <!-- Update progress dialog -->
  <n-modal v-model:show="showProgressDialog" :title="$t('modManagement.updateMod') + ` (${updatingMods.length})`"
           preset="dialog">
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
        {{ $t('app.cancel') }}
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
import { BarotraumaMod } from "../../proto/mods.ts";
import { useMessage } from "naive-ui";
import getTagColorConfig from "../../composables/coloredTag.ts";

// Message API for notifications
const message = useMessage();

// Get tag style configuration
function getTagStyle(tag: string) {
	const config = getTagColorConfig(tag);
	return {
		cursor: "pointer",
		transition: "all 0.2s ease",
		opacity: "0.85",
		...config,
	};
}

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
/* Bulk actions - compact and positioned */
.bulk-actions {
  display: flex;
  gap: 8px;
  justify-content: center;
  margin-bottom: 12px;
  padding: 8px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  backdrop-filter: blur(10px);
}

.action-btn {
  transition: all 0.15s ease;
}

.action-btn:hover:not(:disabled) {
  transform: translateY(-1px);
}

.actions-row {
  display: flex;
  justify-content: center;
  margin-bottom: 16px;
}

/* Compact mod grid layout */
.mod-grid {
  display: grid;
  gap: 8px;
  grid-template-columns: 1fr;
}

.mod-item {
  padding: 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 2px solid transparent;
  background: rgba(255, 255, 255, 0.03);
}

.mod-item:hover {
  background: rgba(255, 255, 255, 0.08);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.mod-item.selected {
  background: rgba(64, 158, 255, 0.1);
  border-color: #409eff;
  box-shadow: 0 0 12px rgba(64, 158, 255, 0.2);
}

/* Compact content layout */
.mod-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.mod-info {
  flex: 1;
  min-width: 0;
}

.mod-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: 8px;
  gap: 12px;
}

.mod-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--n-text-color);
  line-height: 1.2;
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.mod-tags {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
  max-width: 200px;
  align-items: center;
}

.more-tags {
  color: var(--n-text-color-2);
  font-size: 11px;
  font-weight: 500;
}

.mod-details {
  display: flex;
  gap: 16px;
  color: var(--n-text-color-2);
  font-size: 12px;
}

.detail-item {
  background: rgba(0, 0, 0, 0.05);
  padding: 2px 8px;
  border-radius: 12px;
  font-weight: 500;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .mod-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .mod-tags {
    max-width: none;
  }

  .mod-details {
    flex-wrap: wrap;
  }
}

/* Dark theme adjustments */
:global(.dark) .mod-title {
  color: var(--n-text-color);
}

:global(.dark) .mod-item {
  background: rgba(255, 255, 255, 0.05);
}

:global(.dark) .mod-item:hover {
  background: rgba(255, 255, 255, 0.1);
}

:global(.dark) .detail-item {
  background: rgba(255, 255, 255, 0.1);
  color: var(--n-text-color-2);
}

:global(.dark) .mod-details {
  color: var(--n-text-color-2);
}

:global(.dark) .bulk-actions {
  background: rgba(255, 255, 255, 0.05);
}
</style>