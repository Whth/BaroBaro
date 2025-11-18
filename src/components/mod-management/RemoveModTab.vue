<template>
  <n-scrollbar style="max-height: 60vh">
    <n-empty v-if="installed_mod.length === 0" description="No installed mods found."/>

    <div v-else>
      <n-space justify="center" style="margin-bottom: 16px">
        <n-button
            v-if="selectedMods.size > 0"
            ghost
            type="error"
            @click="showBulkDeleteDialog = true"
        >
          Delete Selected ({{ selectedMods.size }})
        </n-button>
        <n-button
            :disabled="selectedMods.size === 0"
            ghost
            type="warning"
            @click="clearSelection"
        >
          Clear Selection
        </n-button>
      </n-space>

      <n-list :bordered="false" style="background-color: transparent">
        <n-list-item v-for="mod in installed_mod" :key="mod.steamWorkshopId">
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
            <n-space justify="end" style="margin-top: 8px">
              <n-button
                  ghost
                  size="small"
                  type="error"
                  @click.stop="showDeleteDialog(mod)"
              >
                Delete
              </n-button>
            </n-space>
          </div>
        </n-list-item>
      </n-list>
    </div>
  </n-scrollbar>

  <!-- Individual delete confirmation dialog -->
  <n-modal v-model:show="showIndividualDeleteDialog" preset="dialog" title="Delete Mod">
    <div v-if="modToDelete">
      <p>Are you sure you want to delete <strong>{{ modToDelete.name }}</strong>?</p>
      <p style="margin-top: 8px; color: #666; font-size: 12px;">
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
      <p style="margin-top: 8px; color: #666; font-size: 12px;">
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
import ModItemDisplay from "./ModItemDisplay.vue";
import { BarotraumaMod } from "../../proto/mods.ts";
import { useMessage } from "naive-ui";

// Message API for notifications
const message = useMessage();

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
