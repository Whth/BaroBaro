<template>
  <div ref="listContainer" style="height: 69vh; overflow-y: auto;">
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
    </n-empty>
  </div>
  <div v-if="orderChanged" style="margin-top: 12px; display: flex; justify-content: flex-end;">
    <n-button :loading="saving" type="primary" @click="saveOrder">
      {{ $t('modList.saveOrder') }}
    </n-button>
  </div>
</template>

<script lang="ts" setup>
import {
	clear_active_profile,
	enabled_mods,
	list_enabled_mods,
	reorder_enabled_mods,
} from "../../invokes.ts";
import ModCard from "./ModCard.vue";
import type { BarotraumaMod } from "../../proto/mods.ts";
import { ArchiveOutline, MenuOutline } from "@vicons/ionicons5";
import Sortable from "sortablejs";
import { nextTick, onMounted, onUnmounted, ref } from "vue";
import { useMessage } from "naive-ui";
import { useI18n } from "vue-i18n";

const emit = defineEmits(["viewingMod"]);
const message = useMessage();
const { t } = useI18n();

const orderChanged = ref(false);
const saving = ref(false);
const listContainer = ref<HTMLElement | null>(null);

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
	console.log("[ModList] Initializing SortableJS on", el, "children:", el.children.length);
	sortable = Sortable.create(el, {
		animation: 150,
		handle: ".drag-handle",
		draggable: "[data-draggable]",
		forceFallback: true,
		onEnd(evt) {
			console.log("[ModList] Sortable onEnd:", evt.oldIndex, "->", evt.newIndex);
			const { oldIndex, newIndex } = evt;
			if (oldIndex == null || newIndex == null || oldIndex === newIndex) return;
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
});

async function saveOrder() {
	saving.value = true;
	try {
		const orderedIds = enabled_mods.value.map((m) => m.steamWorkshopId);
		await reorder_enabled_mods(orderedIds);
		await list_enabled_mods();
		await clear_active_profile();
		orderChanged.value = false;
		message.success(t("modList.orderSaved"));
	} catch (error) {
		message.error(String(error));
	} finally {
		saving.value = false;
	}
}
</script>

<style scoped>
.sortable-ghost {
	opacity: 0.4;
}
</style>

