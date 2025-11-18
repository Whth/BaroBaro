<template>
  <n-scrollbar style="max-height: 60vh">
    <n-empty v-if="installed_mod.length === 0" description="No installed mods found."/>

    <div v-else>
      <!-- Compact bulk actions -->
      <div v-if="selectedMods.size > 0" class="bulk-actions">
        <n-button
            class="bulk-delete-btn"
            size="small"
            type="error"
            @click="showBulkDeleteDialog = true"
        >
          🗑️ Delete Selected ({{ selectedMods.size }})
        </n-button>
        <n-button
            class="clear-selection-btn"
            ghost
            size="small"
            @click="clearSelection"
        >
          ✖️ Clear
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
            <div class="mod-actions">
              <n-button
                  class="delete-btn"
                  size="tiny"
                  type="error"
                  @click.stop="showDeleteDialog(mod)"
              >
                🗑️
              </n-button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </n-scrollbar>

  <!-- Individual delete confirmation dialog -->
  <n-modal v-model:show="showIndividualDeleteDialog" preset="dialog" title="Delete Mod">
    <div v-if="modToDelete">
      <p>Are you sure you want to delete <strong>{{ modToDelete.name }}</strong>?</p>
      <p style="margin-top: 8px; color: var(--n-text-color-2); font-size: 12px;">
        This action cannot be undone and will permanently remove the mod from your system.
      </p>
    </div>
    <template #action>
      <n-button ghost @click="cancelDelete">Cancel</n-button>
      <n-button :loading="isDeleting" type="error" @click="confirmDelete">Delete</n-button>
    </template>
  </n-modal>

  <!-- Bulk delete confirmation dialog -->
  <n-modal v-model:show="showBulkDeleteDialog" preset="dialog" title="Delete Selected Mods">
    <div>
      <p>Are you sure you want to delete <strong>{{ selectedMods.size }}</strong> selected mod(s)?</p>
      <p style="margin-top: 8px; color: var(--n-text-color-2); font-size: 12px;">
        This action cannot be undone and will permanently remove the selected mods from your system.
      </p>
    </div>
    <template #action>
      <n-button ghost @click="cancelBulkDelete">Cancel</n-button>
      <n-button :loading="isDeleting" type="error" @click="confirmBulkDelete">Delete Selected</n-button>
    </template>
  </n-modal>
</template>

<script lang="ts" setup>
import { onMounted, ref } from "vue";
import {
	installed_mod,
	list_installed_mods,
	remove_mods,
} from "../../invokes.ts";
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
const showIndividualDeleteDialog = ref(false);
const showBulkDeleteDialog = ref(false);
const modToDelete = ref<BarotraumaMod | null>(null);
const isDeleting = ref(false);

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

// Show delete confirmation for single mod
function showDeleteDialog(mod: BarotraumaMod) {
	modToDelete.value = mod;
	showIndividualDeleteDialog.value = true;
}

// Cancel individual delete
function cancelDelete() {
	showIndividualDeleteDialog.value = false;
	modToDelete.value = null;
}

// Cancel bulk delete
function cancelBulkDelete() {
	showBulkDeleteDialog.value = false;
}

// Confirm individual delete
async function confirmDelete() {
	if (!modToDelete.value) return;

	isDeleting.value = true;
	try {
		await remove_mods([modToDelete.value.steamWorkshopId]);
		message.success(`Successfully deleted "${modToDelete.value.name}"`);

		// Refresh the mod list
		await list_installed_mods();

		// Clear selection if this mod was selected
		selectedMods.value.delete(modToDelete.value.steamWorkshopId);
	} catch (error) {
		console.error("Failed to delete mod:", error);
		message.error(`Failed to delete "${modToDelete.value.name}"`);
	} finally {
		isDeleting.value = false;
		cancelDelete();
	}
}

// Confirm bulk delete
async function confirmBulkDelete() {
	if (selectedMods.value.size === 0) return;

	isDeleting.value = true;
	const modIds = Array.from(selectedMods.value);
	const selectedModCount = modIds.length;

	try {
		await remove_mods(modIds);
		message.success(`Successfully deleted ${selectedModCount} mod(s)`);

		// Refresh the mod list
		await list_installed_mods();

		// Clear all selections
		clearSelection();
	} catch (error) {
		console.error("Failed to delete mods:", error);
		message.error(`Failed to delete ${selectedModCount} mod(s)`);
	} finally {
		isDeleting.value = false;
		cancelBulkDelete();
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

.bulk-delete-btn {
  font-weight: 600;
  box-shadow: 0 2px 8px rgba(244, 67, 54, 0.2);
  transition: all 0.2s ease;
}

.bulk-delete-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(244, 67, 54, 0.3);
}

.clear-selection-btn {
  color: var(--n-text-color-2);
  border: 1px solid var(--n-border-color);
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

/* Compact action button */
.mod-actions {
  flex-shrink: 0;
}

.delete-btn {
  width: 32px;
  height: 32px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  transition: all 0.2s ease;
  box-shadow: 0 2px 6px rgba(244, 67, 54, 0.15);
}

.delete-btn:hover {
  transform: scale(1.05);
  box-shadow: 0 4px 12px rgba(244, 67, 54, 0.25);
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
  color: #e5e7eb;
}

:global(.dark) .mod-item {
  background: rgba(255, 255, 255, 0.05);
}

:global(.dark) .mod-item:hover {
  background: rgba(255, 255, 255, 0.1);
}

:global(.dark) .detail-item {
  background: rgba(255, 255, 255, 0.1);
  color: #d1d5db;
}

:global(.dark) .mod-details {
  color: #9ca3af;
}

:global(.dark) .bulk-actions {
  background: rgba(255, 255, 255, 0.05);
}
</style>
