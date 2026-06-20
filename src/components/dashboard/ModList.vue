<template>
  <div ref="listContainer">
    <div v-for="(element, index) in enabled_mods" :key="modKey(element)" data-draggable
         style="display: flex; align-items: center; margin-bottom: 12px; touch-action: none; user-select: none;">
      <div class="drag-handle" style="cursor: grab; padding: 8px; display: flex; align-items: center;">
        <n-icon size="20">
          <menu-outline/>
        </n-icon>
      </div>
      <div style="flex: 1;">
        <ModCard :index="index + 1" :mod="element" @mod-selected="viewMod"/>
      </div>
    </div>
    <n-empty v-if="enabled_mods.length === 0" :description="$t('modList.empty')" style="margin-top: 20px">
      <template #icon>
        <n-icon>
          <archive-outline/>
        </n-icon>
      </template>
      <template #extra>
        <n-button type="primary" @click="$router.push('/mods')">
          {{ $t('modList.browseWorkshop') }}
        </n-button>
      </template>
    </n-empty>
  </div>
  <div v-if="orderChanged" style="margin-top: 12px; display: flex; justify-content: flex-end; gap: 8px;">
    <n-button v-if="undoStack.length > 0" @click="undoOrder">
      {{ $t('modList.undo') }}
    </n-button>
    <n-button :loading="saving" type="primary" @click="saveOrder">
      {{ $t('modList.saveOrder') }}
    </n-button>
  </div>
</template>

<script lang="ts" setup>
import { ArchiveOutline, MenuOutline } from "@vicons/ionicons5";
import { useMessage } from "naive-ui";
import Sortable from "sortablejs";
import { nextTick, onMounted, onUnmounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import {
	clear_active_profile,
	enabled_mods,
	list_enabled_mods,
	reorder_enabled_mods,
} from "../../invokes.ts";
import type { BarotraumaMod } from "../../proto/mods.ts";
import ModCard from "./ModCard.vue";

const emit = defineEmits(["viewingMod"]);
const message = useMessage();
const { t } = useI18n();

const orderChanged = ref(false);
const saving = ref(false);
const listContainer = ref<HTMLElement | null>(null);
const undoStack = ref<BarotraumaMod[][]>([]);

let sortable: Sortable | null = null;
function modKey(mod: BarotraumaMod): string {
	return String(mod.steamWorkshopId) || `${mod.name}-${mod.homeDir ?? ""}`;
}

function viewMod(mod: BarotraumaMod) {
	emit("viewingMod", mod);
}

onMounted(async () => {
	await nextTick();
	const el = listContainer.value;
	if (!el) {
		console.error("[ModList] listContainer ref is null");
		return;
	}
	console.log(
		"[ModList] Initializing SortableJS on",
		el,
		"children:",
		el.children.length,
	);
	sortable = Sortable.create(el, {
		animation: 150,
		handle: ".drag-handle",
		draggable: "[data-draggable]",
		forceFallback: true,
		onEnd(evt) {
			const { oldIndex, newIndex } = evt;
			if (oldIndex == null || newIndex == null || oldIndex === newIndex) return;
			// Snapshot current order before modifying
			undoStack.value.push([...enabled_mods.value]);
			const items = [...enabled_mods.value];
			const [moved] = items.splice(oldIndex, 1);
			items.splice(newIndex, 0, moved);
			enabled_mods.value = items;
			orderChanged.value = true;
		},
	});
	console.log("[ModList] SortableJS instance:", sortable);
});

onUnmounted(() => {
	sortable?.destroy();
	undoStack.value = [];
});

async function saveOrder() {
	saving.value = true;
	try {
		const orderedIds = enabled_mods.value.map((m) => m.steamWorkshopId);
		await reorder_enabled_mods(orderedIds);
		await list_enabled_mods();
		await clear_active_profile();
		orderChanged.value = false;
		undoStack.value = [];
		message.success(t("modList.orderSaved"));
	} catch (error) {
		message.error(String(error));
	} finally {
		saving.value = false;
	}
}

function undoOrder() {
	if (undoStack.value.length === 0) return;
	enabled_mods.value = undoStack.value.pop()!;
	if (undoStack.value.length === 0) {
		orderChanged.value = false;
	}
	message.info(t("modList.orderUndone"));
}
</script>

<style scoped>
.sortable-ghost {
	opacity: 0.4;
}
</style>

