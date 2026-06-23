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
          type="primary"
          @click="showUpdateDialog = true"
      >
        {{ $t('modManagement.updateSelected', {count: selectedMods.size}) }}
      </n-button>
      <n-button
          :loading="isRefreshing"
          class="mg-action-btn"
          ghost
          size="medium"
          type="info"
          @click="refreshMods"
      >
        {{ $t('modManagement.refreshMods') }}
      </n-button>
    </template>
  </ModGrid>

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
import { useMessage } from "naive-ui";
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import {
	check_mod_updates,
	download_mods,
	installed_mod,
	list_installed_mods,
	uninstall_mods,
} from "../../invokes";
import type { BarotraumaMod } from "../../proto/mods.ts";
import ModGrid from "./ModGrid.vue";

const { t } = useI18n();

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
	Map<
		number,
		"pending" | "updating" | "success" | "error" | "checking" | "uptodate"
	>
>(new Map());

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

function getUpdateStatusType(
	modId: number,
): "default" | "success" | "error" | "warning" | "info" {
	const status = updateStatuses.value.get(modId);
	switch (status) {
		case "success":
		case "uptodate":
			return "success";
		case "error":
			return "error";
		case "updating":
		case "checking":
			return "info";
		default:
			return "default";
	}
}

function getUpdateStatusText(modId: number): string {
	const status = updateStatuses.value.get(modId);
	switch (status) {
		case "success":
			return t("modManagement.statusSuccess");
		case "error":
			return t("modManagement.statusError");
		case "updating":
			return t("modManagement.statusUpdating");
		case "checking":
			return t("modManagement.statusChecking");
		case "uptodate":
			return t("modManagement.statusUpToDate");
		default:
			return t("modManagement.statusPending");
	}
}

function cancelUpdate() {
	showUpdateDialog.value = false;
}

function closeProgressDialog() {
	showProgressDialog.value = false;
	updateProgress.value = 0;
	updateStatuses.value.clear();
}

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

	// Start all as "checking"
	selectedModList.forEach((mod) => {
		updateStatuses.value.set(mod.steamWorkshopId, "checking");
	});

	// Determine which mods actually need updating
	let toUpdate: BarotraumaMod[] = [];
	try {
		const results = await check_mod_updates(modIds);
		for (const result of results) {
			const mod = selectedModList.find(
				(m) => m.steamWorkshopId === result.modId,
			);
			if (!mod) continue;
			if (result.needsUpdate) {
				updateStatuses.value.set(mod.steamWorkshopId, "pending");
				toUpdate.push(mod);
			} else {
				updateStatuses.value.set(mod.steamWorkshopId, "uptodate");
			}
		}
	} catch (error) {
		console.error("Failed to check mod updates:", error);
		// Fall back to updating all
		selectedModList.forEach((mod) => {
			updateStatuses.value.set(mod.steamWorkshopId, "pending");
		});
		toUpdate = [...selectedModList];
	}

	// If nothing needs updating
	if (toUpdate.length === 0) {
		message.success(t("modManagement.allAlreadyUpToDate"));
		isUpdating.value = false;
		return;
	}

	let completedCount = 0;
	const totalCount = toUpdate.length;

	try {
		for (const mod of toUpdate) {
			updateStatuses.value.set(mod.steamWorkshopId, "updating");

			try {
				await uninstall_mods([mod.steamWorkshopId]);
				await download_mods([mod.steamWorkshopId]);

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

		await list_installed_mods();
		clearSelection();

		const uptodateCount = selectedModList.length - totalCount;
		if (completedCount === totalCount) {
			if (uptodateCount > 0) {
				message.success(
					t("modManagement.someUpToDate", {
						updated: completedCount,
						uptodate: uptodateCount,
					}),
				);
			} else {
				message.success("All mods updated successfully!");
			}
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

onMounted(async () => {
	await list_installed_mods();
});
</script>
