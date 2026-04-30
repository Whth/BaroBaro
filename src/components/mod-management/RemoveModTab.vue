<template>
  <ModGrid
      :mods="installed_mod"
      :selected-mods="selectedMods"
      @toggle-select="toggleModSelection"
      @clear-selection="clearSelection"
  >
    <template #bulk-actions>
      <n-button
          :disabled="selectedMods.size === 0"
          class="mg-action-btn"
          size="medium"
          type="error"
          @click="showBulkDeleteDialog = true"
      >
        {{ $t('modManagement.deleteSelected', {count: selectedMods.size}) }}
      </n-button>
    </template>

    <template #mod-actions="{ mod }">
      <div class="rm-actions">
        <n-button
            class="rm-delete-btn"
            type="error"
            @click.stop="showDeleteDialog(mod)"
        >
          {{ $t('modManagement.deleteMod') }}
        </n-button>
      </div>
    </template>
  </ModGrid>

  <!-- Individual delete confirmation dialog -->
  <n-modal v-model:show="showIndividualDeleteDialog" :title="$t('modManagement.deleteMod')" preset="dialog">
    <div v-if="modToDelete">
      <p>{{ $t('modManagement.confirmDeleteSingle', {name: modToDelete.name}) }}</p>
      <p style="margin-top: 8px; color: var(--n-text-color-2); font-size: 12px;">
        {{ $t('modManagement.deleteWarning') }}
      </p>
    </div>
    <template #action>
      <n-button ghost @click="cancelDelete">{{ $t('app.cancel') }}</n-button>
      <n-button :loading="isDeleting" type="error" @click="confirmDelete">{{ $t('app.delete') }}</n-button>
    </template>
  </n-modal>

  <!-- Bulk delete confirmation dialog -->
  <n-modal v-model:show="showBulkDeleteDialog" :title="$t('modManagement.deleteMod')" preset="dialog">
    <div>
      <p>{{ $t('modManagement.confirmDeleteMultiple', {count: selectedMods.size}) }}</p>
      <p style="margin-top: 8px; color: var(--n-text-color-2); font-size: 12px;">
        {{ $t('modManagement.deleteWarning') }}
      </p>
    </div>
    <template #action>
      <n-button ghost @click="cancelBulkDelete">{{ $t('app.cancel') }}</n-button>
      <n-button :loading="isDeleting" type="error" @click="confirmBulkDelete">
        {{ $t('modManagement.deleteSelected', {count: selectedMods.size}) }}
      </n-button>
    </template>
  </n-modal>
</template>

<script lang="ts" setup>
import { onMounted, ref } from "vue";
import {
	installed_mod,
	list_installed_mods,
	uninstall_mods,
} from "../../invokes.ts";
import { BarotraumaMod } from "../../proto/mods.ts";
import { useMessage } from "naive-ui";
import ModGrid from "./ModGrid.vue";

const message = useMessage();

// Selection management
const selectedMods = ref(new Set<number>());

// Dialog states
const showIndividualDeleteDialog = ref(false);
const showBulkDeleteDialog = ref(false);
const modToDelete = ref<BarotraumaMod | null>(null);
const isDeleting = ref(false);

function toggleModSelection(modId: number) {
	const isSelected = selectedMods.value.has(modId);
	if (isSelected) {
		selectedMods.value.delete(modId);
	} else {
		selectedMods.value.add(modId);
	}
}

function clearSelection() {
	selectedMods.value.clear();
}

function showDeleteDialog(mod: BarotraumaMod) {
	modToDelete.value = mod;
	showIndividualDeleteDialog.value = true;
}

function cancelDelete() {
	showIndividualDeleteDialog.value = false;
	modToDelete.value = null;
}

function cancelBulkDelete() {
	showBulkDeleteDialog.value = false;
}

async function confirmDelete() {
	if (!modToDelete.value) return;

	isDeleting.value = true;
	try {
		await uninstall_mods([modToDelete.value.steamWorkshopId]);
		message.success(`Successfully deleted "${modToDelete.value.name}"`);

		await list_installed_mods();

		selectedMods.value.delete(modToDelete.value.steamWorkshopId);
	} catch (error) {
		console.error("Failed to delete mod:", error);
		message.error(`Failed to delete "${modToDelete.value.name}"`);
	} finally {
		isDeleting.value = false;
		cancelDelete();
	}
}

async function confirmBulkDelete() {
	if (selectedMods.value.size === 0) return;

	isDeleting.value = true;
	const modIds = Array.from(selectedMods.value);
	const selectedModCount = modIds.length;

	try {
		await uninstall_mods(modIds);
		message.success(`Successfully deleted ${selectedModCount} mod(s)`);

		await list_installed_mods();

		clearSelection();
	} catch (error) {
		console.error("Failed to delete mods:", error);
		message.error(`Failed to delete ${selectedModCount} mod(s)`);
	} finally {
		isDeleting.value = false;
		cancelBulkDelete();
	}
}

onMounted(async () => {
	await list_installed_mods();
});
</script>

<style scoped>
.rm-actions {
  flex-shrink: 0;
}

.rm-delete-btn {
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  transition: all 0.2s ease;
  box-shadow: 0 2px 6px rgba(244, 67, 54, 0.15);
}

.rm-delete-btn:hover {
  transform: scale(1.05);
  box-shadow: 0 4px 12px rgba(244, 67, 54, 0.25);
}
</style>
